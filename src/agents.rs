use crate::types::AgentAnalysis;
use serde_json::json;

async fn call_ollama_agent(system_prompt: &str, user_text: &str) -> Result<AgentAnalysis, String> {
    let client = reqwest::Client::new();
    
    // --- DÜZENLENEN KISIM BAŞLANGICI ---
    // Çevre değişkeninden OLLAMA_BASE_URL'i okuyoruz. Docker içinde host.docker.internal'a bağlanacak.
    let ollama_base = std::env::var("OLLAMA_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());
    
    let url = format!("{}/api/generate", ollama_base);
    // --- DÜZENLENEN KISIM BİTİŞİ ---

    let payload = json!({
        "model": "llama3", 
        "system": system_prompt,
        "prompt": user_text,
        "stream": false,
        "format": "json"
    });

    let response = client.post(&url) // Burayı da dinamik url referansına çevirdik
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

pub async fn analyze_marketing(text: &str) -> Result<AgentAnalysis, String> {
    let prompt = "Sen Ticari Yönlendirme ve Tüketici Manipülasyonu Analiz uzmanısın. \
    Metni, kullanıcının bilinçaltında hangi ürünü, hizmeti veya sektörü satın almaya zorlandığı/yennlendirildiği açısından incele. \
    Cevabını KESİNLİKLE TÜRKÇE ver. 'aciklama' kısmına KESİNLİKLE sadece şu kalıba uygun bir cümle yaz: 'Kişi [X ürününü/hizmetini] satın almaya veya yönelmeye meyilli olabilir.' \
    ([X ürününü/hizmetini] kısmını metinden yola çıkarak akıllıca tahmin et, örn: 'anti-aging kremini', 'siber güvenlik yazılımını', 'kripto para paketini'). \
    Çıktı formatı kesinlikle şu JSON şemasında olmalı: \
    {\"manipulation_type\": \"Pazarlama\", \"detected\": true/false, \"confidence_score\": 0.0-1.0, \"aciklama\": \"Kişi X ürününü almak isteyebilir kalıbındaki cümle buraya.\", \"target_sentences\": [\"yönlendirme yapılan cümle\"]}";
    
    call_ollama_agent(prompt, text).await
}