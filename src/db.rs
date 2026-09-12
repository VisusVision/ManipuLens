//! PostgreSQL veri katmanı.
//!
//! Önce users.json / history.jsonl dosyaları vardı (her kayıtta tüm dosya
//! yeniden yazılıyordu), sonra tek bir SQLite dosyası. Artık kalıcı veri
//! PostgreSQL'de: kullanıcı profili üstüne kurulacak reklam hedefleme katmanı
//! "şu ilgi alanına sahip kullanıcılar" gibi sorgular soracak ve bu, profilin
//! `jsonb` olarak indekslenebildiği bir veritabanı ister.
//!
//! Eski kaynaklardan içe aktarım iki yoldan yapılır ve ikisi de kaynağa
//! dokunmaz: `migrate_from_json_files` (JSON dosyaları) ve
//! `import_from_sqlite` (`--import-sqlite manipulens.db`).
//!
//! Sorgular derleme zamanı denetimli `query!` makroları yerine çalışma zamanı
//! API'si (`sqlx::query`) ile yazıldı: makro, `cargo build` sırasında ayakta
//! bir veritabanı ya da `sqlx prepare` ile üretilmiş offline veri ister;
//! bu projede derlemenin veritabanından bağımsız kalması daha değerli.

use crate::ads::Ad;
use crate::types::{AgentVerdict, HistoryEntry, User, UserProfile};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{PgPool, Row};

/// Varsayılan bağlantı adresi; `DATABASE_URL` ile ezilir.
pub const DEFAULT_DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1:5433/manipulens";

pub struct Db {
    pool: PgPool,
}

/// Oturum kaydı (Authorization: Bearer <token> ile doğrulanır)
pub struct Session {
    pub user_id: String,
    pub email: String,
    pub expires_at: i64,
}

impl Db {
    /// Havuzu açar ve şema göçlerini uygular.
    pub async fn connect(url: &str) -> Result<Self, String> {
        let pool = PgPoolOptions::new()
            // Analiz isteği sırasında profil tazeleme ayrı bir görevde koşuyor;
            // beş bağlantı tek kullanıcılı yerel kurulum için fazlasıyla yeter.
            .max_connections(5)
            .connect(url)
            .await
            .map_err(|e| format!("PostgreSQL bağlantısı kurulamadı: {e}"))?;

        let db = Db { pool };
        db.run_migrations().await?;
        Ok(db)
    }

    async fn run_migrations(&self) -> Result<(), String> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| format!("şema göçü başarısız: {e}"))
    }

    /// Testler için izole şema: her çağrı kendi `test_<rastgele>` şemasını
    /// açar ve göçleri orada koşar. Böylece testler paralel çalışsa da
    /// birbirinin satırlarını görmez; tek gereksinim ayakta bir PostgreSQL.
    #[cfg(test)]
    pub async fn connect_test() -> Self {
        let url = std::env::var("DATABASE_URL_TEST").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:5433/manipulens_test".to_string()
        });
        let schema = format!("test_{}", uuid::Uuid::new_v4().simple());

        let setup = PgPoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .expect("test PostgreSQL'ine bağlanılamadı (DATABASE_URL_TEST)");
        sqlx::query(&format!("CREATE SCHEMA \"{schema}\""))
            .execute(&setup)
            .await
            .expect("test şeması oluşturulamadı");
        setup.close().await;

        let schema_for_hook = schema.clone();
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .after_connect(move |conn, _| {
                let schema = schema_for_hook.clone();
                Box::pin(async move {
                    sqlx::query(&format!("SET search_path TO \"{schema}\""))
                        .execute(conn)
                        .await?;
                    Ok(())
                })
            })
            .connect(&url)
            .await
            .expect("test havuzu açılamadı");

        let db = Db { pool };
        db.run_migrations().await.expect("test şema göçü başarısız");
        db
    }

    /// Eski JSON dosyalarını (varsa) bir kez içe aktarır. Tablolar boş
    /// değilse hiçbir şey yapmaz; kaynak dosyalara dokunulmaz.
    pub async fn migrate_from_json_files(&self) {
        if self.count("users").await == 0 {
            if let Ok(content) = std::fs::read_to_string("users.json") {
                if let Ok(users) = serde_json::from_str::<Vec<User>>(&content) {
                    let mut imported = 0;
                    for u in &users {
                        if self.insert_user(u).await.is_ok() {
                            imported += 1;
                        }
                    }
                    tracing::info!(imported, "users.json içe aktarıldı (dosya korunuyor)");
                }
            }
        }

        if self.count("history").await == 0 {
            if let Ok(content) = std::fs::read_to_string("history.jsonl") {
                let mut imported = 0;
                for line in content.lines() {
                    if let Ok(entry) = serde_json::from_str::<HistoryEntry>(line) {
                        self.insert_history(&entry).await;
                        imported += 1;
                    }
                }
                tracing::info!(imported, "history.jsonl içe aktarıldı (dosya korunuyor)");
            }
        }
    }

    async fn count(&self, table: &str) -> i64 {
        // Tablo adı sabit listeden gelir (users/history); kullanıcı girdisi değil.
        sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) FROM {table}"))
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0)
    }

    /// Göç araçları için: hedef tablo boş mu? Dolu tabloya ikinci kez aktarım
    /// yapılmaz, yoksa komut iki kez çalıştırıldığında geçmiş ikiye katlanır.
    pub async fn is_table_empty(&self, table: &str) -> bool {
        self.count(table).await == 0
    }

    // ===== Kullanıcılar =====

    pub async fn user_by_email(&self, email: &str) -> Option<User> {
        sqlx::query("SELECT id, email, password_hash, created_at, verified FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
            .map(|r: PgRow| User {
                id: r.get("id"),
                email: r.get("email"),
                password_hash: r.get("password_hash"),
                created_at: r.get("created_at"),
                verified: r.get("verified"),
            })
    }

    /// UNIQUE(email) ihlalinde Err döner (yarış durumunda çifte kayıt imkânsız).
    pub async fn insert_user(&self, user: &User) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at, verified)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.created_at)
        .bind(user.verified)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
    }

    /// Doğrulandı olarak işaretler; güncel kullanıcıyı döner.
    pub async fn set_verified(&self, email: &str) -> Option<User> {
        sqlx::query("UPDATE users SET verified = true WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await
            .ok()?;
        self.user_by_email(email).await
    }

    pub async fn update_password(&self, email: &str, password_hash: &str) -> bool {
        matches!(
            sqlx::query("UPDATE users SET password_hash = $1 WHERE email = $2")
                .bind(password_hash)
                .bind(email)
                .execute(&self.pool)
                .await,
            Ok(res) if res.rows_affected() > 0
        )
    }

    // ===== Geçmiş =====

    pub async fn insert_history(&self, entry: &HistoryEntry) {
        // Ajan kararları tek jsonb sütununda: 6 satır yerine 1 satır, ve
        // ajan listesi değişirse şema göçü gerekmez.
        let agents_json = entry
            .agents
            .as_ref()
            .and_then(|a| serde_json::to_value(a).ok());

        let result = sqlx::query(
            "INSERT INTO history (timestamp, client_id, text_preview, is_manipulated, dominant_manipulation, genel_sonuc, lang, user_id, agents_json, predicted_product, text_len)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(&entry.timestamp)
        .bind(&entry.client_id)
        .bind(&entry.text_preview)
        .bind(entry.is_manipulated)
        .bind(&entry.dominant_manipulation)
        .bind(&entry.genel_sonuc)
        .bind(&entry.lang)
        .bind(&entry.user_id)
        .bind(agents_json)
        .bind(&entry.predicted_product)
        .bind(entry.text_len)
        .execute(&self.pool)
        .await;

        if let Err(e) = result {
            tracing::warn!(error = %e, "geçmiş kaydı yazılamadı");
        }
    }

    /// history satırından HistoryEntry kurar. Sütun adları tüm SELECT'lerde
    /// aynıdır; `user_id` dışa aktarımda COALESCE ile üretilir.
    fn row_to_entry(r: &PgRow) -> HistoryEntry {
        let agents_json: Option<serde_json::Value> = r.try_get("agents_json").ok().flatten();
        HistoryEntry {
            timestamp: r.get("timestamp"),
            client_id: r.get("client_id"),
            text_preview: r.get("text_preview"),
            is_manipulated: r.get("is_manipulated"),
            dominant_manipulation: r.get("dominant_manipulation"),
            genel_sonuc: r.get("genel_sonuc"),
            lang: r.get("lang"),
            user_id: r.get("user_id"),
            agents: agents_json.and_then(|j| serde_json::from_value::<Vec<AgentVerdict>>(j).ok()),
            predicted_product: r.get("predicted_product"),
            text_len: r.get("text_len"),
        }
    }

    /// En yeni kayıt önce, en fazla `limit` kayıt. Satır id'leri de döner:
    /// çeviri sonrası özetin kalıcı güncellenmesi (update_history_summary) için.
    pub async fn history_for_client(&self, client_id: &str, limit: i64) -> Vec<(i64, HistoryEntry)> {
        let rows = sqlx::query(
            "SELECT id, timestamp, client_id, text_preview, is_manipulated, dominant_manipulation, genel_sonuc, lang, user_id, agents_json, predicted_product, text_len
             FROM history WHERE client_id = $1 ORDER BY id DESC LIMIT $2",
        )
        .bind(client_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await;

        match rows {
            Ok(rows) => rows
                .iter()
                .map(|r| (r.get::<i64, _>("id"), Self::row_to_entry(r)))
                .collect(),
            Err(e) => {
                tracing::warn!(error = %e, "geçmiş okunamadı");
                Vec::new()
            }
        }
    }

    /// Çevrilen özeti kalıcı yazar: aynı kayıt bir daha Ollama'ya gitmez.
    /// (Dil tutarlılığı düzeltmesi — geçmiş her açılışta yeniden çevrilmesin.)
    pub async fn update_history_summary(&self, id: i64, genel_sonuc: &str, lang: &str) {
        let _ = sqlx::query("UPDATE history SET genel_sonuc = $1, lang = $2 WHERE id = $3")
            .bind(genel_sonuc)
            .bind(lang)
            .bind(id)
            .execute(&self.pool)
            .await;
    }

    /// Veri seti dışa aktarımı için TÜM geçmiş, en eski kayıt önce.
    /// Kullanıcı ayrımı `user_id` ile yapılır; eski kayıtlarda bu alan boşsa
    /// `users` tablosundan e-postayla çözülür (çözülemezse None kalır).
    pub async fn history_for_export(&self) -> Vec<HistoryEntry> {
        let rows = sqlx::query(
            "SELECT h.timestamp, h.client_id, h.text_preview, h.is_manipulated, h.dominant_manipulation, h.genel_sonuc, h.lang,
                    COALESCE(h.user_id, u.id) AS user_id, h.agents_json, h.predicted_product, h.text_len
             FROM history h LEFT JOIN users u ON u.email = h.client_id
             ORDER BY h.id ASC",
        )
        .fetch_all(&self.pool)
        .await;

        match rows {
            Ok(rows) => rows.iter().map(Self::row_to_entry).collect(),
            Err(e) => {
                tracing::warn!(error = %e, "dışa aktarım için geçmiş okunamadı");
                Vec::new()
            }
        }
    }

    // ===== Kullanıcı profilleri =====

    /// Profili yazar veya günceller (user_id birincil anahtar).
    pub async fn upsert_profile(&self, profile: &UserProfile) {
        let Ok(profile_json) = serde_json::to_value(profile) else {
            return;
        };
        let _ = sqlx::query(
            "INSERT INTO user_profiles (user_id, profile_json, analyzed_count, model_version, updated_at)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (user_id) DO UPDATE SET
                profile_json = EXCLUDED.profile_json,
                analyzed_count = EXCLUDED.analyzed_count,
                model_version = EXCLUDED.model_version,
                updated_at = EXCLUDED.updated_at",
        )
        .bind(&profile.user_id)
        .bind(profile_json)
        .bind(profile.stats.total)
        .bind(&profile.model_version)
        .bind(&profile.updated_at)
        .execute(&self.pool)
        .await;
    }

    pub async fn profile_for_user(&self, user_id: &str) -> Option<UserProfile> {
        let json: serde_json::Value =
            sqlx::query_scalar("SELECT profile_json FROM user_profiles WHERE user_id = $1")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await
                .ok()
                .flatten()?;
        serde_json::from_value(json).ok()
    }

    /// Kullanıcı profilini siler (KVKK: kullanıcı kendi profilini kaldırabilir).
    /// Profilden türetilmiş reklam kararları da silinir — profil gittiyse onun
    /// üstüne kurulmuş hedefleme kaydının durması anlamsız olurdu.
    pub async fn delete_profile(&self, user_id: &str) -> bool {
        let deleted = sqlx::query("DELETE FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() > 0)
            .unwrap_or(false);
        self.delete_ad_decisions_for_user(user_id).await;
        deleted
    }

    // ===== Reklam hedefleme =====

    /// Kullanıcı reklam hedeflemesine rıza verdi mi? Varsayılan false;
    /// rıza yoksa profil hedeflemede hiç okunmaz.
    pub async fn ads_consent(&self, user_id: &str) -> bool {
        sqlx::query_scalar::<_, bool>("SELECT ads_consent FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
            .unwrap_or(false)
    }

    /// Rızayı açar/kapatır. Kapatıldığında geçmiş kararlar da silinir:
    /// rıza geri alındıysa o rızayla üretilmiş kayıt da durmamalı.
    pub async fn set_ads_consent(&self, user_id: &str, consent: bool) -> bool {
        let updated = sqlx::query("UPDATE users SET ads_consent = $1 WHERE id = $2")
            .bind(consent)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() > 0)
            .unwrap_or(false);

        if updated && !consent {
            self.delete_ad_decisions_for_user(user_id).await;
        }
        updated
    }

    /// Aktif kampanyalar. Envanter elle doldurulur; dış reklam ağı yoktur.
    pub async fn active_inventory(&self) -> Vec<Ad> {
        let rows = sqlx::query(
            "SELECT id, brand, category, title, body, target, sensitive, urgency
             FROM ad_inventory WHERE active = true ORDER BY id ASC",
        )
        .fetch_all(&self.pool)
        .await;

        match rows {
            Ok(rows) => rows
                .iter()
                .map(|r| Ad {
                    id: r.get("id"),
                    brand: r.get("brand"),
                    category: r.get("category"),
                    title: r.get("title"),
                    body: r.get("body"),
                    target: r
                        .try_get::<serde_json::Value, _>("target")
                        .ok()
                        .and_then(|v| serde_json::from_value(v).ok())
                        .unwrap_or_default(),
                    sensitive: r.get("sensitive"),
                    urgency: r.get("urgency"),
                })
                .collect(),
            Err(e) => {
                tracing::warn!(error = %e, "reklam envanteri okunamadı");
                Vec::new()
            }
        }
    }

    /// Kampanya ekler veya günceller (yönetim ucu).
    pub async fn upsert_ad(&self, ad: &Ad, active: bool) -> Result<(), String> {
        let target = serde_json::to_value(&ad.target).map_err(|e| e.to_string())?;
        sqlx::query(
            "INSERT INTO ad_inventory (id, brand, category, title, body, target, sensitive, urgency, active)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (id) DO UPDATE SET
                brand = EXCLUDED.brand,
                category = EXCLUDED.category,
                title = EXCLUDED.title,
                body = EXCLUDED.body,
                target = EXCLUDED.target,
                sensitive = EXCLUDED.sensitive,
                urgency = EXCLUDED.urgency,
                active = EXCLUDED.active",
        )
        .bind(&ad.id)
        .bind(&ad.brand)
        .bind(&ad.category)
        .bind(&ad.title)
        .bind(&ad.body)
        .bind(target)
        .bind(ad.sensitive)
        .bind(ad.urgency)
        .bind(active)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
    }

    /// Kararı kaydeder ve satır id'sini döner; geri bildirim bu id ile gelir.
    pub async fn record_ad_decision(
        &self,
        user_id: &str,
        ad_id: &str,
        score: f32,
        reason: &str,
        model_version: &str,
    ) -> Option<i64> {
        sqlx::query_scalar::<_, i64>(
            "INSERT INTO ad_decisions (user_id, ad_id, score, reason, model_version)
             VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(user_id)
        .bind(ad_id)
        .bind(score)
        .bind(reason)
        .bind(model_version)
        .fetch_one(&self.pool)
        .await
        .ok()
    }

    /// Gösterim/tıklama/gizleme kaydeder. Karar başka kullanıcıya aitse
    /// hiçbir şey yazılmaz — kimse başkasının kararına olay ekleyemez.
    pub async fn record_ad_event(&self, decision_id: i64, user_id: &str, kind: &str) -> bool {
        if !matches!(kind, "impression" | "click" | "dismiss") {
            return false;
        }
        sqlx::query(
            "INSERT INTO ad_events (decision_id, kind)
             SELECT $1, $2 WHERE EXISTS (
                 SELECT 1 FROM ad_decisions WHERE id = $1 AND user_id = $3
             )",
        )
        .bind(decision_id)
        .bind(kind)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .unwrap_or(false)
    }

    /// Kullanıcının reklam kararlarını siler (olaylar ON DELETE CASCADE ile
    /// gider). Profil silindiğinde ve rıza geri alındığında çağrılır.
    pub async fn delete_ad_decisions_for_user(&self, user_id: &str) -> u64 {
        sqlx::query("DELETE FROM ad_decisions WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected())
            .unwrap_or(0)
    }

    // ===== Oturumlar =====

    pub async fn create_session(
        &self,
        token: &str,
        user_id: &str,
        email: &str,
        now: i64,
        expires_at: i64,
    ) {
        let _ = sqlx::query(
            "INSERT INTO sessions (token, user_id, email, created_at, expires_at)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(token)
        .bind(user_id)
        .bind(email)
        .bind(now)
        .bind(expires_at)
        .execute(&self.pool)
        .await;

        // Süresi dolan oturumları fırsattan temizle
        let _ = sqlx::query("DELETE FROM sessions WHERE expires_at < $1")
            .bind(now)
            .execute(&self.pool)
            .await;
    }

    /// Geçerli (süresi dolmamış) oturumu döner; dolmuşsa siler.
    pub async fn session_by_token(&self, token: &str, now: i64) -> Option<Session> {
        let row = sqlx::query("SELECT user_id, email, expires_at FROM sessions WHERE token = $1")
            .bind(token)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()?;

        let session = Session {
            user_id: row.get("user_id"),
            email: row.get("email"),
            expires_at: row.get("expires_at"),
        };

        if session.expires_at < now {
            let _ = sqlx::query("DELETE FROM sessions WHERE token = $1")
                .bind(token)
                .execute(&self.pool)
                .await;
            return None;
        }
        Some(session)
    }

    /// Şifre değişince kullanıcının tüm oturumlarını düşür (çalınmış token ölür).
    pub async fn delete_sessions_for_user(&self, email: &str) {
        let _ = sqlx::query("DELETE FROM sessions WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_user(email: &str) -> User {
        User {
            id: format!("id-{}", email),
            email: email.to_string(),
            password_hash: "hash".to_string(),
            created_at: "2026-01-01T00:00:00+03:00".to_string(),
            verified: false,
        }
    }

    #[tokio::test]
    async fn user_roundtrip_and_unique_email() {
        let db = Db::connect_test().await;
        db.insert_user(&sample_user("a@b.com")).await.unwrap();
        // Aynı e-posta ikinci kez eklenemez
        assert!(db.insert_user(&sample_user("a@b.com")).await.is_err());

        let u = db.user_by_email("a@b.com").await.unwrap();
        assert!(!u.verified);

        let u = db.set_verified("a@b.com").await.unwrap();
        assert!(u.verified);

        assert!(db.update_password("a@b.com", "newhash").await);
        assert_eq!(
            db.user_by_email("a@b.com").await.unwrap().password_hash,
            "newhash"
        );
        assert!(!db.update_password("yok@b.com", "x").await);
    }

    #[tokio::test]
    async fn history_isolated_per_client_and_ordered() {
        let db = Db::connect_test().await;
        for i in 0..3 {
            db.insert_history(&HistoryEntry {
                timestamp: format!("t{}", i),
                client_id: "user1".to_string(),
                text_preview: format!("metin {}", i),
                is_manipulated: i % 2 == 0,
                dominant_manipulation: "Dilsel".to_string(),
                genel_sonuc: "sonuç".to_string(),
                lang: Some("tr".to_string()),
                user_id: None,
                agents: None,
                predicted_product: None,
                text_len: None,
            })
            .await;
        }
        db.insert_history(&HistoryEntry {
            timestamp: "tx".to_string(),
            client_id: "user2".to_string(),
            text_preview: "başka".to_string(),
            is_manipulated: false,
            dominant_manipulation: "Yok".to_string(),
            genel_sonuc: "temiz".to_string(),
            lang: None,
            user_id: None,
            agents: None,
            predicted_product: None,
            text_len: None,
        })
        .await;

        let h = db.history_for_client("user1", 100).await;
        assert_eq!(h.len(), 3);
        // En yeni önce
        assert_eq!(h[0].1.timestamp, "t2");
        // Başka kullanıcının kaydı sızmaz
        assert!(h.iter().all(|(_, e)| e.client_id == "user1"));
        // lang alanı korunur
        assert_eq!(h[0].1.lang.as_deref(), Some("tr"));

        let h2 = db.history_for_client("user1", 2).await;
        assert_eq!(h2.len(), 2);

        // lang'sız (eski) kayıt None döner
        let h3 = db.history_for_client("user2", 10).await;
        assert_eq!(h3[0].1.lang, None);
    }

    #[tokio::test]
    async fn agent_verdicts_survive_jsonb_roundtrip() {
        let db = Db::connect_test().await;
        db.insert_history(&HistoryEntry {
            timestamp: "t0".to_string(),
            client_id: "user1".to_string(),
            text_preview: "önizleme".to_string(),
            is_manipulated: true,
            dominant_manipulation: "Pazarlama".to_string(),
            genel_sonuc: "sonuç".to_string(),
            lang: Some("tr".to_string()),
            user_id: Some("uid-1".to_string()),
            agents: Some(vec![AgentVerdict {
                t: "Pazarlama".to_string(),
                d: true,
                c: 0.9,
            }]),
            predicted_product: Some("Kişi Kripto-X satın almaya meyilli olabilir.".to_string()),
            text_len: Some(120),
        })
        .await;

        let (_, entry) = db.history_for_client("user1", 1).await[0].clone();
        let agents = entry.agents.expect("ajan kararları jsonb'den dönmeli");
        assert_eq!(agents[0].t, "Pazarlama");
        assert!(agents[0].d);
        assert_eq!(entry.text_len, Some(120));
        assert_eq!(entry.user_id.as_deref(), Some("uid-1"));
    }

    #[tokio::test]
    async fn history_summary_update_persists_translation() {
        let db = Db::connect_test().await;
        db.insert_history(&HistoryEntry {
            timestamp: "t0".to_string(),
            client_id: "user1".to_string(),
            text_preview: "önizleme".to_string(),
            is_manipulated: true,
            dominant_manipulation: "Dilsel".to_string(),
            genel_sonuc: "Türkçe özet".to_string(),
            lang: Some("tr".to_string()),
            user_id: None,
            agents: None,
            predicted_product: None,
            text_len: None,
        })
        .await;

        let (id, _) = db.history_for_client("user1", 1).await[0].clone();
        db.update_history_summary(id, "English summary", "en").await;

        let (_, updated) = db.history_for_client("user1", 1).await[0].clone();
        assert_eq!(updated.genel_sonuc, "English summary");
        assert_eq!(updated.lang.as_deref(), Some("en"));
    }

    #[tokio::test]
    async fn session_lifecycle() {
        let db = Db::connect_test().await;
        let now = 1_000_000;
        db.create_session("tok1", "uid", "a@b.com", now, now + 100)
            .await;

        let s = db.session_by_token("tok1", now).await.unwrap();
        assert_eq!(s.email, "a@b.com");

        // Süresi dolunca None döner ve silinir
        assert!(db.session_by_token("tok1", now + 101).await.is_none());
        assert!(db.session_by_token("tok1", now).await.is_none());

        // Şifre sıfırlanınca tüm oturumlar düşer
        db.create_session("tok2", "uid", "a@b.com", now, now + 100)
            .await;
        db.delete_sessions_for_user("a@b.com").await;
        assert!(db.session_by_token("tok2", now).await.is_none());
    }
}
