use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub text: String,
    pub client_id: Option<String>, // Yeni eklendi
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

// Yeni: Geçmiş kaydı için
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub client_id: String,
    pub text_preview: String,      // Metnin ilk 120 karakteri
    pub is_manipulated: bool,
    pub dominant_manipulation: String,
    pub genel_sonuc: String,
}