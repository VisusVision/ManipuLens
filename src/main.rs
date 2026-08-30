mod agents;
mod audit;
mod auth;
mod db;
mod orchestrator;
mod types;

use audit::audit;
use auth::{LoginGuard, RateWindow};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use db::Db;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use types::*;
use uuid::Uuid;

// ===== Ortak State =====

/// Bekleyen doğrulama / sıfırlama kodu
struct PendingCode {
    code: String,
    expires_at: i64, // unix saniye
    created_at: i64, // rate-limit için
    /// Yanlış deneme sayısı — MAX_CODE_ATTEMPTS aşılırsa kod iptal edilir
    /// (6 haneli kodun brute-force ile bulunmasını engeller).
    attempts: u32,
}

/// E-posta gönderiminin gerçek sonucu — kullanıcıya net mesaj verebilmek için
enum MailResult {
    /// SMTP üzerinden başarıyla gönderildi
    Sent,
    /// SMTP yapılandırılmamış; kod sunucu konsoluna yazıldı (geliştirici modu)
    DevConsole,
    /// SMTP yapılandırılmış ama gönderim başarısız (yanlış şifre, ağ vb.)
    Failed,
}

struct AppState {
    db: Db,
    /// Anahtar: "verify:<email>" veya "reset:<email>"
    codes: Mutex<HashMap<String, PendingCode>>,
    /// E-posta başına başarısız giriş kilidi
    login_guards: Mutex<HashMap<String, LoginGuard>>,
    /// Kullanıcı başına /v1/analyze hız limiti
    analyze_rate: Mutex<HashMap<String, RateWindow>>,
}

type SharedState = Arc<AppState>;

#[derive(Deserialize)]
struct HistoryQuery {
    /// İstemcinin arayüz dili ("tr"/"en"). Verilirse, yanlış dilde saklanmış
    /// geçmiş özetleri tek Ollama çağrısıyla bu dile çevrilerek döndürülür.
    lang: Option<String>,
}

// ===== Yardımcılar =====

const CODE_TTL_SECS: i64 = 600; // Kodlar 10 dakika geçerli
const RESEND_COOLDOWN_SECS: i64 = 30; // Aynı e-postaya yeni kod için bekleme süresi
const MAX_CODE_ATTEMPTS: u32 = 5; // Kod başına yanlış deneme hakkı
const HISTORY_LIMIT: i64 = 200; // /v1/history en fazla bu kadar kayıt döner

/// Geliştirme kolaylığı bayrağı: `AUTH_MAIL_DISABLED=1` iken 6 haneli e-posta
/// kodu akışı tamamen atlanır — kayıt anında doğrulanmış sayılır ve şifre
/// sıfırlama kod istemez. SMTP kodu (`lettre`, `send_code_email`) yerinde
/// kalır; bayrak kaldırıldığında eski davranış birebir geri döner.
///
/// GÜVENLİK: bu mod açıkken `/v1/reset`, e-postayı bilen HERKESİN şifreyi
/// değiştirmesine izin verir. Yalnızca yerel geliştirmede açılmalıdır;
/// varsayılan kapalıdır ve açıkken sunucu başlangıçta uyarı basar.
fn mail_disabled() -> bool {
    std::env::var("AUTH_MAIL_DISABLED")
        .map(|v| matches!(v.trim(), "1" | "true" | "TRUE"))
        .unwrap_or(false)
}

/// Proje kökündeki .env dosyasını okur ve ortam değişkeni olarak yükler.
/// (Harici dotenv bağımlılığına gerek kalmadan SMTP ayarlarını dosyadan almayı sağlar.)
fn load_dotenv() {
    let Ok(content) = fs::read_to_string(".env") else { return };
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if std::env::var(key).is_err() {
                std::env::set_var(key, value);
            }
        }
    }
    println!(".env dosyası yüklendi.");
}

/// SMTP transportunu kurar. Port 465 = implicit TLS, diğerleri (587) = STARTTLS.
fn build_mailer(
    host: &str,
    port: u16,
    smtp_user: String,
    smtp_pass: String,
) -> Result<AsyncSmtpTransport<Tokio1Executor>, String> {
    let builder = if port == 465 {
        AsyncSmtpTransport::<Tokio1Executor>::relay(host)
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host)
    }
    .map_err(|e| e.to_string())?;

    Ok(builder
        .port(port)
        .credentials(Credentials::new(smtp_user, smtp_pass))
        .build())
}

fn norm_lang(l: &Option<String>) -> &'static str {
    match l.as_deref() {
        Some("en") => "en",
        _ => "tr",
    }
}

fn pick<'a>(lang: &str, tr: &'a str, en: &'a str) -> &'a str {
    if lang == "en" { en } else { tr }
}

fn is_valid_password(password: &str) -> bool {
    password.len() >= 8
        && password.chars().any(|c| c.is_uppercase())
        && password.chars().any(|c| c.is_lowercase())
        && password.chars().any(|c| c.is_ascii_digit())
}

/// 6 haneli rastgele kod (uuid'in rastgeleliğinden türetilir, ek bağımlılık gerekmez)
fn generate_code() -> String {
    format!("{:06}", (Uuid::new_v4().as_u128() % 1_000_000) as u32)
}

/// bcrypt CPU-yoğun (~100ms): async worker thread'lerini bloklamasın diye
/// spawn_blocking ile ayrı thread'de çalıştırılır.
async fn hash_password(password: String) -> Result<String, ()> {
    tokio::task::spawn_blocking(move || hash(password, DEFAULT_COST))
        .await
        .map_err(|_| ())?
        .map_err(|_| ())
}

async fn verify_password(password: String, hash_str: String) -> bool {
    tokio::task::spawn_blocking(move || verify(&password, &hash_str).unwrap_or(false))
        .await
        .unwrap_or(false)
}

/// Authorization: Bearer <token> başlığını doğrular; geçerli oturumu döner.
fn authenticate(state: &SharedState, headers: &HeaderMap) -> Option<db::Session> {
    let value = headers.get(axum::http::header::AUTHORIZATION)?.to_str().ok()?;
    let token = value.strip_prefix("Bearer ")?.trim();
    if token.is_empty() {
        return None;
    }
    state.db.session_by_token(token, Local::now().timestamp())
}

/// Başarılı kimlik doğrulama sonrası oturum açar, token döner.
fn open_session(state: &SharedState, user_id: &str, email: &str) -> String {
    let token = auth::new_token();
    let now = Local::now().timestamp();
    state
        .db
        .create_session(&token, user_id, email, now, now + auth::SESSION_TTL_SECS);
    token
}

fn unauthorized_msg(lang: &str) -> String {
    pick(
        lang,
        "Oturum bulunamadı veya süresi doldu. Lütfen tekrar giriş yapın.",
        "Session not found or expired. Please log in again.",
    )
    .to_string()
}

/// Kodu üret, sakla ve e-posta ile gönder.
/// Err(kalan_saniye): aynı e-postaya çok kısa süre önce kod gönderilmiş (rate-limit).
async fn store_and_send_code(
    state: &SharedState,
    purpose: &str,
    email: &str,
    lang: &str,
) -> Result<MailResult, i64> {
    let now = Local::now().timestamp();
    let code = generate_code();
    {
        let mut codes = state.codes.lock().await;

        // Süresi dolan kodları temizle (map sınırsız büyümesin)
        codes.retain(|_, pc| pc.expires_at >= now);

        let key = format!("{}:{}", purpose, email);
        if let Some(existing) = codes.get(&key) {
            let elapsed = now - existing.created_at;
            if elapsed < RESEND_COOLDOWN_SECS {
                return Err(RESEND_COOLDOWN_SECS - elapsed);
            }
        }

        codes.insert(
            key,
            PendingCode {
                code: code.clone(),
                expires_at: now + CODE_TTL_SECS,
                created_at: now,
                attempts: 0,
            },
        );
    }
    audit("code_sent", json!({ "email": email, "purpose": purpose }));
    Ok(send_code_email(email, &code, lang, purpose == "reset").await)
}

/// store_and_send_code sonucunu kullanıcıya gösterilecek mesaja çevirir.
fn mail_result_message(result: &Result<MailResult, i64>, lang: &str) -> String {
    match result {
        Ok(MailResult::Sent) => pick(
            lang,
            "Kod e-posta adresinize gönderildi.",
            "The code has been sent to your email.",
        )
        .to_string(),
        Ok(MailResult::DevConsole) => pick(
            lang,
            "SMTP yapılandırılmamış: kod sunucu konsoluna yazıldı (geliştirici modu).",
            "SMTP is not configured: the code was printed to the server console (developer mode).",
        )
        .to_string(),
        Ok(MailResult::Failed) => pick(
            lang,
            "E-posta gönderilemedi. Sunucunun SMTP ayarlarını ve loglarını kontrol edin.",
            "The email could not be sent. Check the server's SMTP settings and logs.",
        )
        .to_string(),
        Err(remaining) => {
            if lang == "en" {
                format!("A code was sent recently. Please wait {} seconds and try again.", remaining)
            } else {
                format!("Kısa süre önce kod gönderildi. Lütfen {} saniye sonra tekrar deneyin.", remaining)
            }
        }
    }
}

/// Kod doğrulama sonucu
enum CodeCheck {
    /// Kod doğru; tüketildi (tek kullanımlık)
    Ok,
    /// Kod yanlış veya süresi dolmuş
    Wrong,
    /// Çok fazla yanlış deneme — kod iptal edildi, yenisi istenmeli
    TooManyAttempts,
}

/// Kodu doğrula; geçerliyse tüketir. Yanlış denemeleri sayar:
/// MAX_CODE_ATTEMPTS aşılırsa kod tamamen iptal edilir (brute-force koruması).
async fn consume_code(state: &SharedState, purpose: &str, email: &str, code: &str) -> CodeCheck {
    let key = format!("{}:{}", purpose, email);
    let mut codes = state.codes.lock().await;
    let now = Local::now().timestamp();

    let Some(pc) = codes.get_mut(&key) else {
        return CodeCheck::Wrong;
    };

    if pc.expires_at < now {
        codes.remove(&key);
        return CodeCheck::Wrong;
    }

    if pc.code == code {
        codes.remove(&key);
        return CodeCheck::Ok;
    }

    pc.attempts += 1;
    if pc.attempts >= MAX_CODE_ATTEMPTS {
        codes.remove(&key);
        audit(
            "code_brute_force_blocked",
            json!({ "email": email, "purpose": purpose, "attempts": MAX_CODE_ATTEMPTS }),
        );
        return CodeCheck::TooManyAttempts;
    }
    CodeCheck::Wrong
}

/// SMTP üzerinden kod e-postası gönderir. SMTP env değişkenleri
/// (SMTP_HOST, SMTP_USER, SMTP_PASS, opsiyonel SMTP_PORT/SMTP_FROM)
/// tanımlı değilse geliştirme modu: kod sunucu konsoluna yazılır.
async fn send_code_email(to: &str, code: &str, lang: &str, is_reset: bool) -> MailResult {
    let host = std::env::var("SMTP_HOST").ok();
    let smtp_user = std::env::var("SMTP_USER").ok();
    let smtp_pass = std::env::var("SMTP_PASS").ok();

    let (Some(host), Some(smtp_user), Some(smtp_pass)) = (host, smtp_user, smtp_pass) else {
        println!(
            "[DEV MODE] SMTP yapılandırılmamış → {} için {} kodu: {}",
            to,
            if is_reset { "sıfırlama" } else { "doğrulama" },
            code
        );
        return MailResult::DevConsole;
    };

    let subject = if is_reset {
        pick(lang, "ManipuLens Şifre Sıfırlama Kodu", "ManipuLens Password Reset Code")
    } else {
        pick(lang, "ManipuLens E-posta Doğrulama Kodu", "ManipuLens Email Verification Code")
    };

    let body = match (is_reset, lang) {
        (true, "en") => format!(
            "Your ManipuLens password reset code: {}\n\nThis code is valid for 10 minutes. If you did not request it, you can ignore this email.",
            code
        ),
        (true, _) => format!(
            "ManipuLens şifre sıfırlama kodunuz: {}\n\nKod 10 dakika geçerlidir. Bu isteği siz yapmadıysanız bu e-postayı yok sayabilirsiniz.",
            code
        ),
        (false, "en") => format!(
            "Your ManipuLens email verification code: {}\n\nThis code is valid for 10 minutes.",
            code
        ),
        (false, _) => format!(
            "ManipuLens e-posta doğrulama kodunuz: {}\n\nKod 10 dakika geçerlidir.",
            code
        ),
    };

    let from_addr = std::env::var("SMTP_FROM").unwrap_or_else(|_| smtp_user.clone());
    let port: u16 = std::env::var("SMTP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(587);

    let from_mb: lettre::message::Mailbox = match from_addr.parse() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("SMTP_FROM adresi geçersiz: {}", e);
            return MailResult::Failed;
        }
    };
    let to_mb: lettre::message::Mailbox = match to.parse() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Alıcı e-posta adresi geçersiz ({}): {}", to, e);
            return MailResult::Failed;
        }
    };

    let email_msg = match Message::builder()
        .from(from_mb)
        .to(to_mb)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
    {
        Ok(m) => m,
        Err(e) => {
            eprintln!("E-posta oluşturulamadı: {}", e);
            return MailResult::Failed;
        }
    };

    let mailer = match build_mailer(&host, port, smtp_user, smtp_pass) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("SMTP bağlantısı kurulamadı: {}", e);
            return MailResult::Failed;
        }
    };

    match mailer.send(email_msg).await {
        Ok(_) => MailResult::Sent,
        Err(e) => {
            eprintln!("E-posta gönderilemedi ({}): {}", to, e);
            MailResult::Failed
        }
    }
}

fn auth_fail(message: String) -> Json<AuthResponse> {
    Json(AuthResponse {
        success: false,
        message,
        client_id: None,
        email: None,
        needs_verification: None,
        token: None,
    })
}

// ========== REGISTER ==========
async fn handle_register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<AuthResponse> {
    let lang = norm_lang(&payload.lang);
    let email = payload.email.trim().to_lowercase();
    let password = payload.password;

    if email.is_empty() || !email.contains('@') {
        return auth_fail(
            pick(lang, "Geçerli bir e-posta adresi giriniz.", "Please enter a valid email address.").to_string(),
        );
    }

    if !is_valid_password(&password) {
        return auth_fail(
            pick(
                lang,
                "Şifre en az 8 karakter olmalı, en az 1 büyük harf, 1 küçük harf ve 1 rakam içermelidir.",
                "Password must be at least 8 characters and include an uppercase letter, a lowercase letter, and a digit.",
            )
            .to_string(),
        );
    }

    if state.db.user_by_email(&email).is_some() {
        return auth_fail(
            pick(lang, "Bu e-posta adresi zaten kayıtlı.", "This email is already registered.").to_string(),
        );
    }

    let Ok(password_hash) = hash_password(password).await else {
        return auth_fail(
            pick(lang, "Şifre işlenirken bir hata oluştu.", "An error occurred while processing the password.")
                .to_string(),
        );
    };

    // Mail devre dışıyken kullanıcı doğrudan doğrulanmış başlar (kod beklenmez).
    let mail_off = mail_disabled();

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        email: email.clone(),
        password_hash,
        created_at: Local::now().to_rfc3339(),
        verified: mail_off,
    };

    // UNIQUE(email) kısıtı yarış durumunda bile çifte kaydı engeller
    if state.db.insert_user(&new_user).is_err() {
        return auth_fail(
            pick(lang, "Bu e-posta adresi zaten kayıtlı.", "This email is already registered.").to_string(),
        );
    }

    audit("register", json!({ "email": email }));

    // Mail devre dışı: kod üretme, doğrudan oturum aç.
    if mail_off {
        let token = open_session(&state, &new_user.id, &new_user.email);
        audit("register_auto_verified", json!({ "email": email }));
        return Json(AuthResponse {
            success: true,
            message: pick(
                lang,
                "Hesabınız oluşturuldu ve giriş yapıldı (e-posta doğrulaması devre dışı).",
                "Your account has been created and you are logged in (email verification is disabled).",
            )
            .to_string(),
            client_id: Some(new_user.id.clone()),
            email: Some(email),
            needs_verification: None,
            token: Some(token),
        });
    }

    // Doğrulama kodu üret ve gönder — sonucu kullanıcıya net bildir
    let mail_result = store_and_send_code(&state, "verify", &email, lang).await;
    let mail_note = mail_result_message(&mail_result, lang);

    Json(AuthResponse {
        success: true,
        message: format!(
            "{} {}",
            pick(lang, "Hesabınız oluşturuldu.", "Your account has been created."),
            mail_note
        ),
        client_id: None,
        email: Some(email),
        needs_verification: Some(true),
        token: None,
    })
}

// ========== LOGIN ==========
async fn handle_login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Json<AuthResponse> {
    let lang = norm_lang(&payload.lang);
    let email = payload.email.trim().to_lowercase();
    let password = payload.password;
    let now = Local::now().timestamp();

    // Brute-force kilidi: art arda çok hata varsa deneme bile yapılmaz
    {
        let mut guards = state.login_guards.lock().await;
        let guard = guards.entry(email.clone()).or_insert_with(LoginGuard::new);
        if let Err(remaining) = guard.check(now) {
            audit("login_locked", json!({ "email": email, "remaining_secs": remaining }));
            let msg = if lang == "en" {
                format!(
                    "Too many failed attempts. This account is temporarily locked; try again in {} minutes.",
                    (remaining + 59) / 60
                )
            } else {
                format!(
                    "Çok fazla hatalı deneme. Hesap geçici olarak kilitlendi; {} dakika sonra tekrar deneyin.",
                    (remaining + 59) / 60
                )
            };
            return auth_fail(msg);
        }
    }

    let user = state.db.user_by_email(&email);
    let password_ok = match &user {
        Some(u) => verify_password(password, u.password_hash.clone()).await,
        // Kullanıcı yoksa da sahte doğrulama yap: yanıt süresi üzerinden
        // e-posta var/yok bilgisi sızmasın (timing side-channel)
        None => {
            let _ = verify_password("dummy-password".to_string(), DUMMY_HASH.to_string()).await;
            false
        }
    };

    match (user, password_ok) {
        // Mail devre dışıyken eski (doğrulanmamış) kayıtlar da giriş yapabilir:
        // kaydı bir kez doğrulanmış işaretleyip normal akışa devam ederiz.
        (Some(u), true) if u.verified || mail_disabled() => {
            if !u.verified {
                state.db.set_verified(&email);
                audit("login_auto_verified", json!({ "email": email }));
            }
            {
                let mut guards = state.login_guards.lock().await;
                if let Some(g) = guards.get_mut(&email) {
                    g.record_success();
                }
            }
            let token = open_session(&state, &u.id, &u.email);
            audit("login_success", json!({ "email": email }));
            Json(AuthResponse {
                success: true,
                message: pick(lang, "Giriş başarılı.", "Login successful.").to_string(),
                client_id: Some(u.id),
                email: Some(email),
                needs_verification: None,
                token: Some(token),
            })
        }
        (Some(_), true) => {
            // Şifre doğru ama e-posta doğrulanmamış → yeni kod göndermeyi dene
            let mail_result = store_and_send_code(&state, "verify", &email, lang).await;
            let mail_note = mail_result_message(&mail_result, lang);
            Json(AuthResponse {
                success: false,
                message: format!(
                    "{} {}",
                    pick(
                        lang,
                        "E-posta adresiniz henüz doğrulanmamış.",
                        "Your email is not verified yet."
                    ),
                    mail_note
                ),
                client_id: None,
                email: Some(email),
                needs_verification: Some(true),
                token: None,
            })
        }
        _ => {
            {
                let mut guards = state.login_guards.lock().await;
                let guard = guards.entry(email.clone()).or_insert_with(LoginGuard::new);
                guard.record_failure(now);
            }
            audit("login_fail", json!({ "email": email }));
            auth_fail(pick(lang, "E-posta veya şifre hatalı.", "Incorrect email or password.").to_string())
        }
    }
}

/// Timing side-channel savunması için sabit bcrypt hash'i (login'de kullanıcı
/// bulunamayınca da aynı maliyette doğrulama koşulur).
const DUMMY_HASH: &str = "$2b$12$C6UzMDM.H6dfI/f/IKcEeO7ZBpUqoAtE.LGmSuo6oDcO0Zl0kKMWq";

// ========== VERIFY (e-posta doğrulama) ==========
async fn handle_verify(
    State(state): State<SharedState>,
    Json(payload): Json<VerifyRequest>,
) -> Json<AuthResponse> {
    let lang = norm_lang(&payload.lang);
    let email = payload.email.trim().to_lowercase();
    let code = payload.code.trim().to_string();

    if mail_disabled() {
        return auth_fail(
            pick(
                lang,
                "E-posta doğrulaması devre dışı; doğrudan giriş yapabilirsiniz.",
                "Email verification is disabled; you can log in directly.",
            )
            .to_string(),
        );
    }

    match consume_code(&state, "verify", &email, &code).await {
        CodeCheck::Ok => {}
        CodeCheck::TooManyAttempts => {
            return auth_fail(
                pick(
                    lang,
                    "Çok fazla yanlış deneme yapıldı; kod iptal edildi. Lütfen yeni kod isteyin.",
                    "Too many wrong attempts; the code has been cancelled. Please request a new code.",
                )
                .to_string(),
            );
        }
        CodeCheck::Wrong => {
            return auth_fail(
                pick(lang, "Kod hatalı veya süresi dolmuş.", "The code is incorrect or has expired.").to_string(),
            );
        }
    }

    if let Some(user) = state.db.set_verified(&email) {
        let token = open_session(&state, &user.id, &user.email);
        audit("verify_success", json!({ "email": email }));
        return Json(AuthResponse {
            success: true,
            message: pick(
                lang,
                "E-posta doğrulandı. Giriş yapıldı.",
                "Email verified. You are now logged in.",
            )
            .to_string(),
            client_id: Some(user.id),
            email: Some(user.email),
            needs_verification: None,
            token: Some(token),
        });
    }

    auth_fail(pick(lang, "Kullanıcı bulunamadı.", "User not found.").to_string())
}

// ========== RESEND (doğrulama kodunu tekrar gönder) ==========
async fn handle_resend(
    State(state): State<SharedState>,
    Json(payload): Json<ResendRequest>,
) -> Json<AuthResponse> {
    let lang = norm_lang(&payload.lang);
    let email = payload.email.trim().to_lowercase();

    if mail_disabled() {
        return auth_fail(
            pick(
                lang,
                "E-posta doğrulaması devre dışı; kod gönderilmiyor.",
                "Email verification is disabled; no code is sent.",
            )
            .to_string(),
        );
    }

    let needs_code = matches!(state.db.user_by_email(&email), Some(u) if !u.verified);

    if needs_code {
        let mail_result = store_and_send_code(&state, "verify", &email, lang).await;
        let success = !matches!(mail_result, Ok(MailResult::Failed) | Err(_));
        return Json(AuthResponse {
            success,
            message: mail_result_message(&mail_result, lang),
            client_id: None,
            email: Some(email),
            needs_verification: None,
            token: None,
        });
    }

    Json(AuthResponse {
        success: true,
        message: pick(lang, "Kod gönderildi.", "Code sent.").to_string(),
        client_id: None,
        email: Some(email),
        needs_verification: None,
        token: None,
    })
}

// ========== FORGOT (şifremi unuttum) ==========
async fn handle_forgot(
    State(state): State<SharedState>,
    Json(payload): Json<ForgotRequest>,
) -> Json<AuthResponse> {
    let lang = norm_lang(&payload.lang);
    let email = payload.email.trim().to_lowercase();

    if mail_disabled() {
        // Kod üretilmez; istemci doğrudan yeni şifre ekranına geçer.
        return Json(AuthResponse {
            success: true,
            message: pick(
                lang,
                "Doğrulama kodu devre dışı; yeni şifrenizi doğrudan belirleyebilirsiniz.",
                "Verification codes are disabled; you can set a new password directly.",
            )
            .to_string(),
            client_id: None,
            email: Some(email),
            needs_verification: None,
            token: None,
        });
    }

    if state.db.user_by_email(&email).is_some() {
        let mail_result = store_and_send_code(&state, "reset", &email, lang).await;
        // Rate-limit ve gönderim hatası kullanıcıya bildirilir; ancak
        // e-postanın kayıtlı olup olmadığı sızdırılmaz (mesajlar nötr).
        match &mail_result {
            Err(_) | Ok(MailResult::Failed) | Ok(MailResult::DevConsole) => {
                let success = !matches!(mail_result, Ok(MailResult::Failed) | Err(_));
                return Json(AuthResponse {
                    success,
                    message: mail_result_message(&mail_result, lang),
                    client_id: None,
                    email: Some(email),
                    needs_verification: None,
                    token: None,
                });
            }
            Ok(MailResult::Sent) => {}
        }
    }

    // Güvenlik: e-postanın kayıtlı olup olmadığını dışarı sızdırma
    Json(AuthResponse {
        success: true,
        message: pick(
            lang,
            "Eğer bu e-posta kayıtlıysa, sıfırlama kodu gönderildi.",
            "If this email is registered, a reset code has been sent.",
        )
        .to_string(),
        client_id: None,
        email: Some(email),
        needs_verification: None,
        token: None,
    })
}

// ========== RESET (yeni şifre belirle) ==========
async fn handle_reset(
    State(state): State<SharedState>,
    Json(payload): Json<ResetRequest>,
) -> Json<AuthResponse> {
    let lang = norm_lang(&payload.lang);
    let email = payload.email.trim().to_lowercase();
    let code = payload.code.trim().to_string();

    if !is_valid_password(&payload.new_password) {
        return auth_fail(
            pick(
                lang,
                "Şifre en az 8 karakter olmalı, en az 1 büyük harf, 1 küçük harf ve 1 rakam içermelidir.",
                "Password must be at least 8 characters and include an uppercase letter, a lowercase letter, and a digit.",
            )
            .to_string(),
        );
    }

    // Mail devre dışıyken kod kontrolü atlanır (yalnızca yerel geliştirme modu).
    let skip_code = mail_disabled();
    if skip_code {
        audit("password_reset_no_code", json!({ "email": email }));
    }

    let code_check = if skip_code {
        CodeCheck::Ok
    } else {
        consume_code(&state, "reset", &email, &code).await
    };

    match code_check {
        CodeCheck::Ok => {}
        CodeCheck::TooManyAttempts => {
            return auth_fail(
                pick(
                    lang,
                    "Çok fazla yanlış deneme yapıldı; kod iptal edildi. Lütfen yeni kod isteyin.",
                    "Too many wrong attempts; the code has been cancelled. Please request a new code.",
                )
                .to_string(),
            );
        }
        CodeCheck::Wrong => {
            return auth_fail(
                pick(lang, "Kod hatalı veya süresi dolmuş.", "The code is incorrect or has expired.").to_string(),
            );
        }
    }

    let Ok(password_hash) = hash_password(payload.new_password).await else {
        return auth_fail(
            pick(lang, "Şifre işlenirken bir hata oluştu.", "An error occurred while processing the password.")
                .to_string(),
        );
    };

    if state.db.update_password(&email, &password_hash) {
        // Güvenlik: şifre değişince eski tüm oturumlar geçersiz olur
        // (token'ı ele geçirmiş biri varsa dışarı atılır).
        state.db.delete_sessions_for_user(&email);
        audit("password_reset", json!({ "email": email }));
        return Json(AuthResponse {
            success: true,
            message: pick(
                lang,
                "Şifreniz güncellendi. Şimdi giriş yapabilirsiniz.",
                "Your password has been updated. You can now log in.",
            )
            .to_string(),
            client_id: None,
            email: Some(email),
            needs_verification: None,
            token: None,
        });
    }

    auth_fail(pick(lang, "Kullanıcı bulunamadı.", "User not found.").to_string())
}

// ========== ANALYZE ==========
async fn handle_analyze(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<FinalReport>, (StatusCode, String)> {
    let lang = norm_lang(&payload.lang);

    // KİMLİK: geçerli oturum token'ı zorunlu. Eskiden bu uç tamamen açıktı;
    // URL'i bilen herkes sınırsız analiz (7 LLM çağrısı) tetikleyebiliyordu.
    let Some(session) = authenticate(&state, &headers) else {
        audit("analyze_unauthorized", json!({ "lang": lang }));
        return Err((StatusCode::UNAUTHORIZED, unauthorized_msg(lang)));
    };

    // HIZ LİMİTİ: kullanıcı başına 10 analiz/dakika
    {
        let now = Local::now().timestamp();
        let mut rates = state.analyze_rate.lock().await;
        let rw = rates.entry(session.email.clone()).or_insert_with(RateWindow::new);
        if let Err(retry_secs) = rw.allow(now, auth::ANALYZE_WINDOW_SECS, auth::ANALYZE_MAX_PER_WINDOW) {
            audit("analyze_rate_limited", json!({ "email": session.email, "retry_secs": retry_secs }));
            let msg = if lang == "en" {
                format!("Too many analyses. Please wait {} seconds and try again.", retry_secs)
            } else {
                format!("Çok fazla analiz isteği. Lütfen {} saniye sonra tekrar deneyin.", retry_secs)
            };
            return Err((StatusCode::TOO_MANY_REQUESTS, msg));
        }
    }

    let text = payload.text.trim().to_string();

    if text.is_empty() {
        let msg = pick(lang, "Analiz edilecek metin boş olamaz.", "The text to analyze cannot be empty.");
        return Err((StatusCode::BAD_REQUEST, msg.to_string()));
    }

    const MAX_TEXT_LEN: usize = 1000;
    if text.chars().count() > MAX_TEXT_LEN {
        let msg = if lang == "en" {
            format!("Text is too long (maximum {} characters).", MAX_TEXT_LEN)
        } else {
            format!("Metin çok uzun (maksimum {} karakter).", MAX_TEXT_LEN)
        };
        return Err((StatusCode::BAD_REQUEST, msg));
    }

    let started = Instant::now();
    match orchestrator::run_orchestrator(&text, lang).await {
        Ok(report) => {
            let duration_ms = started.elapsed().as_millis() as u64;

            // client_id artık istemcinin beyanı değil, oturumdan gelen e-posta:
            // kimse başkası adına kayıt oluşturamaz.
            let entry = HistoryEntry {
                timestamp: Local::now().to_rfc3339(),
                client_id: session.email.clone(),
                text_preview: audit::preview(&text),
                is_manipulated: report.is_manipulated,
                dominant_manipulation: report.dominant_manipulation.clone(),
                genel_sonuc: report.genel_sonuc.clone(),
                // Özet hangi dilde üretildiyse kaydet: /v1/history dil
                // tutarlılığı artık tahmine değil bu alana dayanır.
                lang: Some(lang.to_string()),
            };
            state.db.insert_history(&entry);

            // Yapılandırılmış denetim logu (tam metin loglanmaz; önizleme +
            // uzunluk yeterli — tam sonuç kullanıcının geçmişinde saklı).
            let detected: Vec<&str> = report
                .detailed_analyses
                .iter()
                .filter(|a| a.detected)
                .map(|a| a.manipulation_type.as_str())
                .collect();
            audit(
                "analyze",
                json!({
                    "email": session.email,
                    "user_id": session.user_id,
                    "lang": lang,
                    "text_len": text.chars().count(),
                    "text_preview": audit::preview(&text),
                    "is_manipulated": report.is_manipulated,
                    "dominant": report.dominant_manipulation,
                    "detected_agents": detected,
                    "duration_ms": duration_ms,
                }),
            );

            Ok(Json(report))
        }
        Err(err) => {
            audit(
                "analyze_error",
                json!({ "email": session.email, "error": err, "duration_ms": started.elapsed().as_millis() as u64 }),
            );
            Err((StatusCode::INTERNAL_SERVER_ERROR, err))
        }
    }
}

// ========== TRANSLATE REPORT ==========
/// Önceden üretilmiş bir raporu hedef dile çevirir. Analizi yeniden çalıştırmaz;
/// yalnızca metin alanlarını tek Ollama çağrısıyla çevirir. Arayüz dili
/// değiştirildiğinde istemci tarafından çağrılır.
async fn handle_translate_report(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(payload): Json<TranslateReportRequest>,
) -> Result<Json<FinalReport>, (StatusCode, String)> {
    let lang = match payload.lang.as_str() {
        "en" => "en",
        _ => "tr",
    };
    // Oturum zorunlu: bu uç da Ollama kaynağı tüketiyor
    if authenticate(&state, &headers).is_none() {
        return Err((StatusCode::UNAUTHORIZED, unauthorized_msg(lang)));
    }
    Ok(Json(orchestrator::translate_report(payload.report, lang).await))
}

// ========== HISTORY ==========
async fn handle_history(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<HistoryEntry>>, (StatusCode, String)> {
    let lang_q = norm_lang(&query.lang);

    // GÜVENLİK: Eskiden geçmiş, istemcinin beyan ettiği client_id ile
    // filtreleniyordu — başkasının kimliğini bilen herkes onun geçmişini
    // çekebiliyordu (IDOR). Artık kimlik yalnızca oturum token'ından türetilir.
    let Some(session) = authenticate(&state, &headers) else {
        return Err((StatusCode::UNAUTHORIZED, unauthorized_msg(lang_q)));
    };

    let mut rows = state.db.history_for_client(&session.email, HISTORY_LIMIT);

    // DİL TUTARLILIĞI (v2): Yeni kayıtlar hangi dilde üretildiğini `lang`
    // alanında taşır — tespit tahmine değil bu alana dayanır (eski lang'sız
    // kayıtlar için sezgisel kontrol korunur). Yanlış dildeki özetler TEK
    // toplu Ollama çağrısıyla çevrilir ve sonuç VERİTABANINA KALICI yazılır:
    // aynı kayıt bir daha çevrilmez → geçmiş açılışı hızlanır, timeout ve
    // "karışık dil" sorunu tekrarlamaz. (text_preview orijinal alıntı olduğu
    // için asla çevrilmez.)
    if let Some(lang) = query.lang.as_deref() {
        let lang = if lang == "en" { "en" } else { "tr" };

        let mut wrong_idx: Vec<usize> = Vec::new();
        let mut wrong_texts: Vec<String> = Vec::new();
        for (i, (_, entry)) in rows.iter().enumerate() {
            if summary_needs_translation(entry.lang.as_deref(), &entry.genel_sonuc, lang) {
                wrong_idx.push(i);
                wrong_texts.push(entry.genel_sonuc.clone());
            }
        }

        if !wrong_texts.is_empty() {
            if let Some(translations) = orchestrator::translate_texts(&wrong_texts, lang).await {
                for (i, translated) in wrong_idx.into_iter().zip(translations) {
                    if let Some((id, entry)) = rows.get_mut(i) {
                        // Kalıcılaştır: bir sonraki açılış Ollama'sız döner
                        state.db.update_history_summary(*id, &translated, lang);
                        entry.genel_sonuc = translated;
                        entry.lang = Some(lang.to_string());
                    }
                }
            }
            // Çeviri başarısız olursa kayıtlar orijinal dilde döner (veri kaybı yok).
        }
    }

    let entries: Vec<HistoryEntry> = rows.into_iter().map(|(_, e)| e).collect();
    Ok(Json(entries))
}

/// Geçmiş özetinin hedef dile çevrilmesi gerekiyor mu?
/// Kayıtta `lang` varsa kesin karşılaştırma; yoksa (eski kayıt) sezgisel tespit.
fn summary_needs_translation(entry_lang: Option<&str>, text: &str, target: &str) -> bool {
    match entry_lang {
        Some(l) => l != target,
        None => orchestrator::wrong_language(text, target),
    }
}

// ========== HEALTH ==========
async fn handle_health() -> Json<serde_json::Value> {
    // `mail_disabled`: istemci (uzantı) doğrulama/şifre-sıfırlama ekranlarını
    // buna göre gizler; sunucuyla arayüz aynı modda kalır.
    Json(json!({ "status": "ok", "mail_disabled": mail_disabled() }))
}

#[tokio::main]
async fn main() {
    // Yapılandırılmış log: RUST_LOG ile seviye ayarlanabilir (varsayılan info)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // .env dosyasındaki SMTP vb. ayarları yükle (varsa)
    load_dotenv();

    if mail_disabled() {
        tracing::warn!(
            "AUTH_MAIL_DISABLED etkin: e-posta dogrulamasi ve sifre sifirlama kodu ATLANIYOR. \
             Bu modda /v1/reset, e-postayi bilen herkesin sifreyi degistirmesine izin verir - \
             yalnizca yerel gelistirmede kullanin."
        );
    }

    // SQLite aç + eski JSON verilerini (varsa) bir kez içe aktar
    let db = Db::open("manipulens.db").expect("SQLite veritabanı açılamadı");
    db.migrate_from_json_files();

    // Chrome Extension + ngrok için düzeltilmiş CORS ayarı
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            "ngrok-skip-browser-warning".parse().unwrap(),
        ])
        .expose_headers(Any)
        .max_age(Duration::from_secs(86400));

    let state = Arc::new(AppState {
        db,
        codes: Mutex::new(HashMap::new()),
        login_guards: Mutex::new(HashMap::new()),
        analyze_rate: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/v1/register", post(handle_register))
        .route("/v1/login", post(handle_login))
        .route("/v1/verify", post(handle_verify))
        .route("/v1/resend", post(handle_resend))
        .route("/v1/forgot", post(handle_forgot))
        .route("/v1/reset", post(handle_reset))
        .route("/v1/analyze", post(handle_analyze))
        .route("/v1/translate-report", post(handle_translate_report))
        .route("/v1/history", get(handle_history))
        .route("/healthz", get(handle_health))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Manipülasyon Tespit Backend Servisi {} adresinde çalışıyor...", addr);
    tracing::info!("Auth endpointleri hazır → /v1/register /v1/login /v1/verify /v1/resend /v1/forgot /v1/reset");
    tracing::info!("Korumalı uçlar (Bearer token gerekir) → /v1/analyze /v1/translate-report /v1/history");

    // === SMTP SAĞLIK KONTROLÜ (açılışta) ===
    // Mail sorunlarını dakikalarca kör debug etmek yerine sunucu açılırken netleştir.
    match (
        std::env::var("SMTP_HOST"),
        std::env::var("SMTP_USER"),
        std::env::var("SMTP_PASS"),
    ) {
        (Ok(host), Ok(smtp_user), Ok(smtp_pass)) => {
            let port: u16 = std::env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587);
            match build_mailer(&host, port, smtp_user.clone(), smtp_pass) {
                Ok(mailer) => match mailer.test_connection().await {
                    Ok(true) => tracing::info!("✓ SMTP bağlantısı doğrulandı: {}:{} ({})", host, port, smtp_user),
                    Ok(false) => tracing::warn!("✗ SMTP sunucusuna bağlanılamadı ({}:{}). Kodlar mail ile GİDEMEYECEK. SETUP_MAIL.md dosyasına bakın.", host, port),
                    Err(e) => tracing::warn!("✗ SMTP bağlantı/kimlik testi başarısız: {}. Kodlar mail ile GİDEMEYECEK. Şifre bir 'uygulama şifresi' olmalı — SETUP_MAIL.md dosyasına bakın.", e),
                },
                Err(e) => tracing::warn!("✗ SMTP yapılandırma hatası: {}. SETUP_MAIL.md dosyasına bakın.", e),
            }
        }
        _ => {
            tracing::warn!("✗ SMTP yapılandırılmamış (SMTP_HOST/SMTP_USER/SMTP_PASS eksik). Mail gönderilemez; kodlar konsola yazılır. Kurulum: SETUP_MAIL.md");
        }
    }

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_policy() {
        assert!(is_valid_password("Abcdef12"));
        assert!(!is_valid_password("kisa1A")); // 8 karakterden kısa
        assert!(!is_valid_password("hepsikucuk1")); // büyük harf yok
        assert!(!is_valid_password("HEPSIBUYUK1")); // küçük harf yok
        assert!(!is_valid_password("RakamYokAa")); // rakam yok
    }

    #[test]
    fn code_is_six_digits() {
        for _ in 0..100 {
            let c = generate_code();
            assert_eq!(c.len(), 6);
            assert!(c.chars().all(|ch| ch.is_ascii_digit()));
        }
    }

    #[test]
    fn summary_translation_decision() {
        // lang alanı varsa KESİN karar: içerik ne olursa olsun alan konuşur
        assert!(summary_needs_translation(Some("tr"), "Türkçe özet", "en"));
        assert!(!summary_needs_translation(Some("en"), "English summary", "en"));
        assert!(!summary_needs_translation(Some("tr"), "The text looks English but field says tr", "tr"));

        // lang alanı yoksa (eski kayıt) sezgisel tespite düşülür
        assert!(summary_needs_translation(
            None,
            "The text is using fear to push the reader toward a purchase.",
            "tr"
        ));
        assert!(!summary_needs_translation(
            None,
            "Bu metin korku yaratarak okuyucuyu bir ürüne yönlendiriyor.",
            "tr"
        ));
    }

    #[test]
    fn lang_normalization() {
        assert_eq!(norm_lang(&Some("en".to_string())), "en");
        assert_eq!(norm_lang(&Some("tr".to_string())), "tr");
        assert_eq!(norm_lang(&Some("fr".to_string())), "tr"); // bilinmeyen → tr
        assert_eq!(norm_lang(&None), "tr");
    }

    #[test]
    fn dummy_hash_is_valid_bcrypt() {
        // Timing savunmasında kullanılan sabit hash gerçekten doğrulanabilir olmalı
        assert!(!verify("dummy-password-wrong", DUMMY_HASH).unwrap_or(true));
        assert!(verify("x", DUMMY_HASH).is_ok());
    }

    fn test_state() -> SharedState {
        Arc::new(AppState {
            db: Db::open_in_memory(),
            codes: Mutex::new(HashMap::new()),
            login_guards: Mutex::new(HashMap::new()),
            analyze_rate: Mutex::new(HashMap::new()),
        })
    }

    #[tokio::test]
    async fn code_brute_force_cancels_code() {
        let state = test_state();
        let now = Local::now().timestamp();
        state.codes.lock().await.insert(
            "verify:a@b.com".to_string(),
            PendingCode {
                code: "123456".to_string(),
                expires_at: now + 600,
                created_at: now,
                attempts: 0,
            },
        );

        // 4 yanlış deneme: kod hâlâ yaşıyor
        for _ in 0..(MAX_CODE_ATTEMPTS - 1) {
            assert!(matches!(
                consume_code(&state, "verify", "a@b.com", "000000").await,
                CodeCheck::Wrong
            ));
        }
        // 5. yanlış deneme: kod iptal
        assert!(matches!(
            consume_code(&state, "verify", "a@b.com", "000000").await,
            CodeCheck::TooManyAttempts
        ));
        // Doğru kod bile artık çalışmaz (kod silindi)
        assert!(matches!(
            consume_code(&state, "verify", "a@b.com", "123456").await,
            CodeCheck::Wrong
        ));
    }

    #[tokio::test]
    async fn correct_code_consumed_once() {
        let state = test_state();
        let now = Local::now().timestamp();
        state.codes.lock().await.insert(
            "reset:a@b.com".to_string(),
            PendingCode {
                code: "654321".to_string(),
                expires_at: now + 600,
                created_at: now,
                attempts: 0,
            },
        );

        assert!(matches!(
            consume_code(&state, "reset", "a@b.com", "654321").await,
            CodeCheck::Ok
        ));
        // Tek kullanımlık: ikinci kez geçmez
        assert!(matches!(
            consume_code(&state, "reset", "a@b.com", "654321").await,
            CodeCheck::Wrong
        ));
    }

    #[tokio::test]
    async fn expired_code_rejected() {
        let state = test_state();
        let now = Local::now().timestamp();
        state.codes.lock().await.insert(
            "verify:a@b.com".to_string(),
            PendingCode {
                code: "111111".to_string(),
                expires_at: now - 1, // süresi dolmuş
                created_at: now - 700,
                attempts: 0,
            },
        );
        assert!(matches!(
            consume_code(&state, "verify", "a@b.com", "111111").await,
            CodeCheck::Wrong
        ));
    }

    #[test]
    fn authenticate_rejects_bad_headers() {
        let state = test_state();
        // Header yok
        assert!(authenticate(&state, &HeaderMap::new()).is_none());
        // Bearer değil
        let mut h = HeaderMap::new();
        h.insert(axum::http::header::AUTHORIZATION, "Basic abc".parse().unwrap());
        assert!(authenticate(&state, &h).is_none());
        // Geçersiz token
        let mut h = HeaderMap::new();
        h.insert(axum::http::header::AUTHORIZATION, "Bearer gecersiz".parse().unwrap());
        assert!(authenticate(&state, &h).is_none());
    }

    #[test]
    fn authenticate_accepts_valid_session() {
        let state = test_state();
        let token = open_session(&state, "uid-1", "a@b.com");
        let mut h = HeaderMap::new();
        h.insert(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        let s = authenticate(&state, &h).expect("oturum geçerli olmalı");
        assert_eq!(s.email, "a@b.com");
        assert_eq!(s.user_id, "uid-1");
    }
}
