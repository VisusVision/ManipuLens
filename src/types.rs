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

/// Bir analizde tek uzman ajanın kompakt kararı. Geçmiş satırında JSON dizi
/// olarak saklanır; alan adları kısa tutuldu çünkü kullanıcı başına yüzlerce
/// satırda tekrarlanıyor.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentVerdict {
    /// Manipülasyon tipi: "Dilsel" | "Psikolojik" | "Davranışsal" | "Algısal" | "Sosyal" | "Pazarlama"
    pub t: String,
    /// Ajan tespit etti mi
    pub d: bool,
    /// Güven skoru (0.0-1.0)
    pub c: f32,
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
    /// Kullanıcının UUID'si. Veri seti dışa aktarımı e-posta yerine bunu
    /// kullanır. Eski kayıtlarda yoktur (None) — dışa aktarımda `users`
    /// tablosundan e-postayla çözülür.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 6 uzman ajanın o analizdeki kararları. Eskiden hiç saklanmıyordu;
    /// kullanıcı profili bu alan olmadan yalnızca baskın tipi görebiliyordu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<AgentVerdict>>,
    /// Pazarlama ajanının çıkardığı ürün/sektör cümlesi (varsa).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicted_product: Option<String>,
    /// Analiz edilen metnin karakter uzunluğu (tam metin saklanmaz).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_len: Option<i64>,
}

/// Demografi ajanının tek bir özellik için ürettiği tahmin.
/// `guven` 0.60'ın altındaysa `deger` "bilinmiyor" olarak sabitlenir —
/// zayıf tahmini kesin bilgi gibi göstermeyiz.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DemographicTrait {
    pub deger: String,
    pub guven: f32,
    /// Tahminin geçmişteki hangi gözleme dayandığı (tek cümle).
    #[serde(default)]
    pub dayanak: String,
}

/// Demografi ajanının çıktısı. YASAK ALANLAR — bu yapıya bilinçli olarak
/// eklenmemiştir ve ajan prompt'unda da açıkça yasaklanır: etnik köken, din
/// veya inanç, sağlık durumu, cinsel yönelim, siyasi görüş. Bunlar KVKK ve
/// GDPR'da özel nitelikli kişisel veridir; tarama geçmişinden çıkarımı hem
/// hukuken riskli hem istatistiksel olarak dayanaksızdır.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DemographicInference {
    pub yas_araligi: DemographicTrait,
    pub cinsiyet: DemographicTrait,
    pub egitim_seviyesi: DemographicTrait,
    pub tuketici_egilimi: DemographicTrait,
    /// En fazla 5 ilgi alanı etiketi.
    #[serde(default)]
    pub ilgi_alanlari: Vec<String>,
    /// En fazla 2 cümlelik insan okuru için özet.
    #[serde(default)]
    pub ozet: String,
}

/// Kullanıcı profili: sayaç katmanı (her analizde, LLM'siz) + çıkarım katmanı
/// (N analizde bir, LLM ile). Kart 2'de demografi ajanı `inference` alanını
/// doldurur; `stats` bu turda da üretilir.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub user_id: String,
    /// Deterministik sayaçlar — tahmin değil, ölçüm.
    pub stats: ProfileStats,
    /// LLM çıkarımı (demografi ajanı). Henüz üretilmediyse None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference: Option<serde_json::Value>,
    /// Çıkarımı üreten sürüm etiketi ("stats-v1", "demographic-v1", ...).
    pub model_version: String,
    pub updated_at: String,
    /// Çıkarım katmanının en son ne zaman üretildiği.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_at: Option<String>,
    /// Çıkarım üretildiğinde kullanıcının kaç analizi vardı. Bayatlık
    /// (kaç yeni analiz biriktiği) bu sayıyla ölçülür.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_count: Option<i64>,
}

/// Kullanıcının analiz geçmişinden doğrudan sayılan büyüklükler.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProfileStats {
    /// Toplam analiz sayısı
    pub total: i64,
    /// Manipülatif bulunan analiz sayısı
    pub manipulated: i64,
    /// Manipülasyon tipi -> kaç kez baskın çıktı
    pub dominant_counts: std::collections::BTreeMap<String, i64>,
    /// Manipülasyon tipi -> ajanın kaç kez tespit ettiği (baskın olmasa da)
    pub agent_detect_counts: std::collections::BTreeMap<String, i64>,
    /// Dil -> analiz sayısı
    pub lang_counts: std::collections::BTreeMap<String, i64>,
    /// Pazarlama ajanının en sık çıkardığı ürün/sektör cümleleri (en fazla 10)
    pub top_products: Vec<String>,
    /// Ortalama metin uzunluğu (karakter)
    pub avg_text_len: i64,
    /// En eski ve en yeni analiz zaman damgası
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
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