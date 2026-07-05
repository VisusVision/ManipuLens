use crate::types::{FinalReport};
use crate::agents::*;
use serde_json::json;

pub async fn run_orchestrator(text: &str) -> Result<FinalReport, String> {
    let (r1, r2, r3, r4, r5) = tokio::join!(
        analyze_linguistic(text),
        analyze_psychological(text),
        analyze_behavioral(text),
        analyze_perceptual(text),
        analyze_social(text)
    );

    let mut detailed_analyses = Vec::new();
    if let Ok(a) = r1 { detailed_analyses.push(a); }
    if let Ok(a) = r2 { detailed_analyses.push(a); }
    if let Ok(a) = r3 { detailed_analyses.push(a); }
    if let Ok(a) = r4 { detailed_analyses.push(a); }
    if let Ok(a) = r5 { detailed_analyses.push(a); }

    let client = reqwest::Client::new();
    // Yönetici promptunu Türkçe ve net olacak şekilde güncelledik
    let manager_prompt = "Sen baş analizörsün. Sana gelen uzman raporlarını sentezle. Cevabını KESİNLİKLE TÜRKÇE ver. 'genel_sonuc' kısmını karmaşık terimlerden uzak, son kullanıcının rahatça anlayacağı maksimum 2 cümlelik bir özet halinde yaz. Çıktı formatı kesinlikle şu şemada olmalı: {\"is_manipulated\": true/false, \"dominant_manipulation\": \"En baskın tür adı\", \"genel_sonuc\": \"Sade Türkçe genel özet.\"}";
    
    let user_payload = json!({
        "original_text": text,
        "expert_reports": detailed_analyses
    });

    let payload = json!({
        "model": "llama3",
        "system": manager_prompt,
        "prompt": user_payload.to_string(),
        "stream": false,
        "format": "json"
    });

    let response = client.post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let res_body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        if let Some(response_str) = res_body.get("response").and_then(|r| r.as_str()) {
            
            #[derive(serde::Deserialize)]
            struct ManagerOutput {
                is_manipulated: bool,
                dominant_manipulation: String,
                genel_sonuc: String,
            }
            
            let manager_out: ManagerOutput = serde_json::from_str(response_str).map_err(|e| e.to_string())?;
            
            return Ok(FinalReport {
                is_manipulated: manager_out.is_manipulated,
                dominant_manipulation: manager_out.dominant_manipulation,
                genel_sonuc: manager_out.genel_sonuc,
                detailed_analyses,
                predicted_product,
            });
        }
    }

    Err("Yönetici ajan raporu oluşturamadı.".to_string())
}
