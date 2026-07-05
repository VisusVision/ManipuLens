use crate::types::AgentAnalysis;
use serde_json::json;

async fn call_ollama_agent(system_prompt: &str, user_text: &str) -> Result<AgentAnalysis, String> {
    let client = reqwest::Client::new();
    let url = "http://localhost:11434/api/generate";

    let payload = json!({
        "model": "llama3", 
        "system": system_prompt,
        "prompt": user_text,
        "stream": false,
        "format": "json"
    });

    let response = client.post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let res_body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        
        if let Some(response_str) = res_body.get("response").and_then(|r| r.as_str()) {
            let analysis: AgentAnalysis = serde_json::from_str(response_str).map_err(|e| e.to_string())?;
            return Ok(analysis);
        }
    }
    
    Err("Ollama'dan geçerli bir yanıt alınamadı.".to_string())
}

pub async fn analyze_linguistic(text: &str) -> Result<AgentAnalysis, String> {
    let prompt = "Sen Dilsel Manipülasyon Analiz uzmanısın. Metni kelime oyunları, mugalata ve çarpıtmalar açısından incele. Cevabını KESİNLİKLE TÜRKÇE ver. 'aciklama' kısmını son kullanıcıya hitaben, çok net, sade ve en fazla 1-2 cümle olacak şekilde yaz. Çıktı formatı kesinlikle şu JSON şemasında olmalı: {\"manipulation_type\": \"Dilsel\", \"detected\": true/false, \"confidence_score\": 0.0-1.0, \"aciklama\": \"Net Türkçe açıklama buraya.\", \"target_sentences\": [\"tespit edilen cümle\"]}";
    call_ollama_agent(prompt, text).await
}

pub async fn analyze_psychological(text: &str) -> Result<AgentAnalysis, String> {
    let prompt = "Sen Psikolojik Manipülasyon Analiz uzmanısın. Metni gaslighting, korku ve suçluluk duygusu yaratma açısından incele. Cevabını KESİNLİKLE TÜRKÇE ver. 'aciklama' kısmını son kullanıcıya hitaben, çok net, sade ve en fazla 1-2 cümle olacak şekilde yaz. Çıktı formatı kesinlikle şu JSON şemasında olmalı: {\"manipulation_type\": \"Psikolojik\", \"detected\": true/false, \"confidence_score\": 0.0-1.0, \"aciklama\": \"Net Türkçe açıklama buraya.\", \"target_sentences\": [\"tespit edilen cümle\"]}";
    call_ollama_agent(prompt, text).await
}

pub async fn analyze_behavioral(text: &str) -> Result<AgentAnalysis, String> {
    let prompt = "Sen Davranışsal Manipülasyon Analiz uzmanısın. Metni fevri aksiyona zorlama (FOMO) ve sahte aciliyet yaratma açısından incele. Cevabını KESİNLİKLE TÜRKÇE ver. 'aciklama' kısmını son kullanıcıya hitaben, çok net, sade ve en fazla 1-2 cümle olacak şekilde yaz. Çıktı formatı kesinlikle şu JSON şemasında olmalı: {\"manipulation_type\": \"Davranışsal\", \"detected\": true/false, \"confidence_score\": 0.0-1.0, \"aciklama\": \"Net Türkçe açıklama buraya.\", \"target_sentences\": [\"tespit edilen cümle\"]}";
    call_ollama_agent(prompt, text).await
}

pub async fn analyze_perceptual(text: &str) -> Result<AgentAnalysis, String> {
    let prompt = "Sen Algısal Manipülasyon Analiz uzmanısın. Metni gerçekleri seçici sunma (cherry-picking) ve yanlış çerçeveleme açısından incele. Cevabını KESİNLİKLE TÜRKÇE ver. 'aciklama' kısmını son kullanıcıya hitaben, çok net, sade ve en fazla 1-2 cümle olacak şekilde yaz. Çıktı formatı kesinlikle şu JSON şemasında olmalı: {\"manipulation_type\": \"Algısal\", \"detected\": true/false, \"confidence_score\": 0.0-1.0, \"aciklama\": \"Net Türkçe açıklama buraya.\", \"target_sentences\": [\"tespit edilen cümle\"]}";
    call_ollama_agent(prompt, text).await
}

pub async fn analyze_social(text: &str) -> Result<AgentAnalysis, String> {
    let prompt = "Sen Sosyal Manipülasyon Analiz uzmanısın. Metni mahalle baskısı, sürü psikolojisi ve kutuplaştırma taktikleri açısından incele. Cevabını KESİNLİKLE TÜRKÇE ver. 'aciklama' kısmını son kullanıcıya hitaben, çok net, sade ve en fazla 1-2 cümle olacak şekilde yaz. Çıktı formatı kesinlikle şu JSON şemasında olmalı: {\"manipulation_type\": \"Sosyal\", \"detected\": true/false, \"confidence_score\": 0.0-1.0, \"aciklama\": \"Net Türkçe açıklama buraya.\", \"target_sentences\": [\"tespit edilen cümle\"]}";
    call_ollama_agent(prompt, text).await
}

pub async fn analyze_consumer_intent(dominant_type: &str, summary: &str) -> Result<String, String> {
    let prompt = "Sen bir Tüketici Davranışları ve Finansal Psikoloji uzmanısın. \
                  Sana verilen manipülasyon türü ve özet analize bakarak, bu manipülasyona maruz kalan \
                  bir hedef kitlenin/kişinin psikolojik zaafiyetinden faydalanarak hangi spesifik ürün veya \
                  hizmeti satın almaya yönlendirilebileceğini tahmin et. \
                  Cevabını KESİNLİKLE TÜRKÇE ver. Yanıtın son derece kısa, net ve tek bir cümle olmalı. \
                  Format KESİNLİKLE şu JSON şemasında olmalı: {\"intent_prediction\": \"Kişi X ürününü/hizmetini almak isteyebilir.\"}";

    let user_input = format!("Baskın Manipülasyon: {}\nAnaliz Özeti: {}", dominant_type, summary);

    let client = reqwest::Client::new();
    let url = "http://localhost:11434/api/generate";

    let payload = serde_json::json!({
        "model": "llama3", 
        "system": prompt,
        "prompt": user_input,
        "stream": false,
        "format": "json"
    });

    let response = client.post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let res_body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        if let Some(response_str) = res_body.get("response").and_then(|r| r.as_str()) {
            #[derive(serde::Deserialize)]
            struct IntentOutput {
                intent_prediction: String,
            }
            if let Ok(out) = serde_json::from_str::<IntentOutput>(response_str) {
                return Ok(out.intent_prediction);
            }
        }
    }
    Err("Tüketici eğilim ajanı yanıt üretemedi.".to_string())
}
