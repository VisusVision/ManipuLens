use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentAnalysis {
    pub manipulation_type: String,
    pub detected: bool,
    pub confidence_score: f32,
    pub aciklama: String, // Kısa ve net Türkçe açıklama
    pub target_sentences: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalReport {
    pub is_manipulated: bool,
    pub dominant_manipulation: String,
    pub genel_sonuc: String, // Yönetici ajanın Türkçe özeti
    pub detailed_analyses: Vec<AgentAnalysis>,
    pub predicted_product: Option<String>,
}
