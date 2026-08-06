use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub text: String,
    /// Çıktı dili: "tr" (varsayılan) veya "en"
    pub lang: Option<String>,
    // NOT: eski istemcilerin gönderdiği client_id alanı artık KULLANILMAZ;
    // kimlik yalnızca Authorization token'ından türetilir (serde bilinmeyen
    // alanları zaten yok sayar).
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentAnalysis {
    pub manipulation_type: String,
    pub detected: bool,
    pub confidence_score: f32,
    pub aciklama: String,
    pub target_sentences: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinalReport {
    pub is_manipulated: bool,
    pub dominant_manipulation: String,
    pub genel_sonuc: String,
    pub predicted_product: Option<String>,
    pub detailed_analyses: Vec<AgentAnalysis>,
}

#[derive(Deserialize)]
pub struct TranslateReportRequest {
    /// Daha önce üretilmiş analiz raporu (istemcinin önbelleğindeki hali)
    pub report: FinalReport,
    /// Hedef dil: "tr" veya "en"
    pub lang: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub client_id: String,
    pub text_preview: String,
    pub is_manipulated: bool,
    pub dominant_manipulation: String,
    pub genel_sonuc: String,
    /// genel_sonuc'un ŞU AN hangi dilde olduğu ("tr"/"en"). Eski kayıtlarda
    /// yoktur (None) — o durumda dil sezgisel olarak tespit edilir.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

// ===== Auth =====
fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
    /// E-posta doğrulaması. Eski kayıtlarda alan yoksa true kabul edilir
    /// (mevcut kullanıcılar kilitlenmesin); yeni kayıtlar false başlar.
    #[serde(default = "default_true")]
    pub verified: bool,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub lang: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub lang: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub code: String,
    pub lang: Option<String>,
}

#[derive(Deserialize)]
pub struct ResendRequest {
    pub email: String,
    pub lang: Option<String>,
}

#[derive(Deserialize)]
pub struct ForgotRequest {
    pub email: String,
    pub lang: Option<String>,
}

#[derive(Deserialize)]
pub struct ResetRequest {
    pub email: String,
    pub code: String,
    pub new_password: String,
    pub lang: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub client_id: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_verification: Option<bool>,
    /// Oturum token'ı: başarılı login/verify sonrası döner. İstemci bunu
    /// saklayıp korumalı uçlara `Authorization: Bearer <token>` ile gönderir.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}