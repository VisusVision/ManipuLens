use crate::agents::*;
use crate::types::{AgentAnalysis, FinalReport};
use serde_json::json;

pub async fn run_orchestrator(text: &str, lang: &str) -> Result<FinalReport, String> {
    // 6 ajanı paralel çalıştır
    let (r1, r2, r3, r4, r5, r6) = tokio::join!(
        analyze_linguistic(text, lang),
        analyze_psychological(text, lang),
        analyze_behavioral(text, lang),
        analyze_perceptual(text, lang),
        analyze_social(text, lang),
        analyze_marketing(text, lang)
    );

    // Hata durumunda kullanılacak varsayılan ajan (seçili dilde)
    let error_msg = if lang == "en" {
        "An error occurred during analysis."
    } else {
        "Analiz sırasında hata oluştu."
    };

    let fallback = |name: &str| AgentAnalysis {
        manipulation_type: name.to_string(),
        detected: false,
        confidence_score: 0.0,
        aciklama: error_msg.to_string(),
        target_sentences: vec![],
    };

    // Her ajanı al + tipini zorla doğru isme çek
    let detailed_analyses = vec![
        {
            let mut a = r1.unwrap_or_else(|_| fallback("Dilsel"));
            a.manipulation_type = "Dilsel".to_string();
            a
        },
        {
            let mut a = r2.unwrap_or_else(|_| fallback("Psikolojik"));
            a.manipulation_type = "Psikolojik".to_string();
            a
        },
        {
            let mut a = r3.unwrap_or_else(|_| fallback("Davranışsal"));
            a.manipulation_type = "Davranışsal".to_string();
            a
        },
        {
            let mut a = r4.unwrap_or_else(|_| fallback("Algısal"));
            a.manipulation_type = "Algısal".to_string();
            a
        },
        {
            let mut a = r5.unwrap_or_else(|_| fallback("Sosyal"));
            a.manipulation_type = "Sosyal".to_string();
            a
        },
        {
            let mut a = r6.unwrap_or_else(|_| fallback("Pazarlama"));
            a.manipulation_type = "Pazarlama".to_string();
            a
        },
    ];

    // Marketing ajanından predicted_product al. Model şablondaki [X] slotunu
    // doldurmayıp literal bırakırsa paneli hiç gösterme (bozuk metin > yok).
    let mut predicted_product_str = None;
    if let Some(marketing) = detailed_analyses.iter().find(|a| a.manipulation_type == "Pazarlama") {
        if marketing.detected && !marketing.aciklama.contains('[') {
            predicted_product_str = Some(marketing.aciklama.clone());
        }
    }

    // === YÖNETİCİ (SENTEZÖR) AJAN ===
    let out_lang = if lang == "en" { "English" } else { "Turkish" };

    let manager_prompt = format!(
        r#"You are the CHIEF ORCHESTRATOR synthesizing 6 specialist reports into one calibrated verdict. Strict, fair, realistic.

INPUT: JSON with "original_text" and "expert_reports".
DECIDE:
1. Weigh by confidence, not count: one 0.95 report with concrete target_sentences outweighs three vague 0.55 ones.
2. Ignore reports with detected = true but confidence < 0.60, or whose target_sentences do not appear in original_text.
3. is_manipulated = true ONLY if a reliable report (>= 0.60, plausible evidence) remains. dominant_manipulation = highest-confidence reliable type; "Yok" if none. Never invent findings.

"genel_sonuc": max 2 natural sentences for an everyday reader, saying WHAT technique is used and HOW it targets them; if clean, briefly reassure. LANGUAGE: "genel_sonuc" MUST be written in {out_lang} - mandatory even if original_text or the expert reports are in a different language. Do NOT mirror the input's language, ALWAYS write in {out_lang}.

Output ONLY one valid JSON object, no markdown:
{{"is_manipulated":true|false,"dominant_manipulation":"Dilsel"|"Psikolojik"|"Davranışsal"|"Algısal"|"Sosyal"|"Pazarlama"|"Yok","genel_sonuc":"..."}}"#
    );

    let user_payload = json!({
        "original_text": text,
        "expert_reports": detailed_analyses
    });

    let payload = json!({
        "model": "llama3",
        "system": manager_prompt,
        "prompt": user_payload.to_string(),
        "stream": false,
        "format": "json",
        "keep_alive": "30m",
        "options": {
            "temperature": 0.2,
            "top_p": 0.9
        }
    });

    // Yönetici ajan çağrısını dene; başarısız olursa 6 uzman raporundan
    // yerel bir özet üreterek analizi yine de tamamla (tek hata noktası olmasın).
    let manager_result: Option<(bool, String, String)> = async {
        let response = http_client()
            .post(format!("{}/api/generate", ollama_url()))
            .json(&payload)
            .send()
            .await
            .ok()?;

        if !response.status().is_success() {
            return None;
        }

        let res_body: serde_json::Value = response.json().await.ok()?;
        let response_str = res_body.get("response").and_then(|r| r.as_str())?;

        #[derive(serde::Deserialize)]
        struct ManagerOutput {
            is_manipulated: bool,
            dominant_manipulation: String,
            genel_sonuc: String,
        }

        let manager_out: ManagerOutput = serde_json::from_str(response_str).ok()?;
        Some((
            manager_out.is_manipulated,
            manager_out.dominant_manipulation,
            manager_out.genel_sonuc,
        ))
    }
    .await;

    let (is_manipulated, dominant_manipulation, genel_sonuc) = match manager_result {
        Some(result) => result,
        None => fallback_summary(&detailed_analyses, lang),
    };

    let mut report = FinalReport {
        is_manipulated,
        dominant_manipulation,
        genel_sonuc,
        predicted_product: predicted_product_str,
        detailed_analyses,
    };

    // === DİL TUTARLILIĞI ONARIMI ===
    // llama3 gibi küçük modeller, talimata rağmen bazen girdinin dilinde cevap
    // verir. Yanlış dilde kalan alanları tespit edip TEK toplu çeviri çağrısıyla
    // onarırız. Model doğru dilde cevap verdiyse ek maliyet sıfırdır.
    repair_language(&mut report, lang).await;

    Ok(report)
}

// ===================== DİL ONARIM YARDIMCILARI =====================

/// Tırnak içi bölümleri (orijinal metinden alıntılar) çıkarır — alıntıların
/// diğer dilde olması normaldir, dil tespitini yanıltmamalı.
fn strip_quoted(s: &str) -> String {
    let mut out = String::new();
    let mut in_quote: Option<char> = None;
    for c in s.chars() {
        match in_quote {
            Some(closing) => {
                if c == closing {
                    in_quote = None;
                }
            }
            None => match c {
                '\'' => in_quote = Some('\''),
                '"' => in_quote = Some('"'),
                '\u{2018}' => in_quote = Some('\u{2019}'), // ‘ ’
                '\u{201C}' => in_quote = Some('\u{201D}'), // “ ”
                _ => out.push(c),
            },
        }
    }
    out
}

/// Basit sezgisel dil kontrolü: metin hedef dilden belirgin şekilde sapmış mı?
/// En az 2 sinyal arar; alıntılar sayılmaz (yanlış pozitifleri önler).
/// (pub: /v1/history dil tutarlılığı kontrolü de bunu kullanır.)
pub fn wrong_language(s: &str, lang: &str) -> bool {
    let stripped = format!(" {} ", strip_quoted(s).to_lowercase());

    let tr_signals = ["ğ", "ş", "ı", "ç", "ö", "ü", " ve ", " bir ", " bu ", " için ", " metin ", " değil "];
    let en_signals = [" the ", " and ", " is ", " of ", " to ", " this ", " reader ", " text ", " by ", " a "];

    let tr_score = tr_signals.iter().filter(|m| stripped.contains(*m)).count();
    let en_score = en_signals.iter().filter(|m| stripped.contains(*m)).count();

    if lang == "en" {
        tr_score >= 2 && tr_score > en_score
    } else {
        en_score >= 2 && en_score > tr_score
    }
}

/// Yanlış dildeki metinleri tek Ollama çağrısıyla hedef dile çevirir.
pub async fn translate_texts(texts: &[String], lang: &str) -> Option<Vec<String>> {
    let target = if lang == "en" { "English" } else { "Turkish" };

    // NOT: Kurallar sıkılaştırıldı — model bazen cümle içinde kaynak dilden
    // parçalar bırakıyordu (ör. "sunarak rather than bir mausoleum").
    let system = format!(
        r#"You are a professional translator. Translate each string in the given JSON array into {target}.
STRICT RULES:
- Translate EVERY word and EVERY sentence COMPLETELY into {target}.
- The output must contain ZERO words from the source language. NEVER mix two languages in one sentence.
- Only exception: fragments enclosed in quotation marks are citations from an original document — keep those EXACTLY as-is.
- Keep the tone and meaning; do not add, remove or summarize information.
- Use natural, fluent {target}.
- Output ONLY a valid JSON object with the same number of items in the same order:
{{"translations":["..."]}}"#
    );

    let payload = json!({
        "model": "llama3",
        "system": system,
        "prompt": serde_json::to_string(texts).ok()?,
        "stream": false,
        "format": "json",
        "keep_alive": "30m",
        "options": { "temperature": 0.1 }
    });

    let response = http_client()
        .post(format!("{}/api/generate", ollama_url()))
        .json(&payload)
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let res_body: serde_json::Value = response.json().await.ok()?;
    let response_str = res_body.get("response").and_then(|r| r.as_str())?;

    #[derive(serde::Deserialize)]
    struct TranslationOut {
        translations: Vec<String>,
    }

    let out: TranslationOut = serde_json::from_str(response_str).ok()?;
    if out.translations.len() == texts.len() {
        Some(out.translations)
    } else {
        None
    }
}

/// Rapordaki yanlış dilde kalmış alanları toplar, toplu çevirir ve geri yazar.
async fn repair_language(report: &mut FinalReport, lang: &str) {
    // (alan_referans_indexi, metin) — index 0 = genel_sonuc, 1.. = aciklama'lar
    let mut wrong_indices: Vec<usize> = Vec::new();
    let mut wrong_texts: Vec<String> = Vec::new();

    if wrong_language(&report.genel_sonuc, lang) {
        wrong_indices.push(0);
        wrong_texts.push(report.genel_sonuc.clone());
    }

    for (i, analysis) in report.detailed_analyses.iter().enumerate() {
        if wrong_language(&analysis.aciklama, lang) {
            wrong_indices.push(i + 1);
            wrong_texts.push(analysis.aciklama.clone());
        }
    }

    if wrong_texts.is_empty() {
        return;
    }

    if let Some(translations) = translate_texts(&wrong_texts, lang).await {
        for (slot, translated) in wrong_indices.iter().zip(translations) {
            if *slot == 0 {
                report.genel_sonuc = translated;
            } else if let Some(a) = report.detailed_analyses.get_mut(*slot - 1) {
                a.aciklama = translated.clone();
                // predicted_product pazarlama ajanının aciklama'sından türediyse onu da güncelle
                if a.manipulation_type == "Pazarlama" && report.predicted_product.is_some() {
                    report.predicted_product = Some(translated);
                }
            }
        }
    }
}

/// Var olan bir raporu hedef dile çevirir. 6 ajanı ve sentezörü YENİDEN
/// ÇALIŞTIRMAZ — sadece serbest metin alanlarını (genel_sonuc + aciklama'lar)
/// TEK Ollama çağrısıyla çevirir. Arayüz dili değiştirildiğinde /v1/translate-report
/// endpoint'i tarafından kullanılır (token/performans verimliliği için).
pub async fn translate_report(mut report: FinalReport, target_lang: &str) -> FinalReport {
    let mut texts: Vec<String> = vec![report.genel_sonuc.clone()];
    for a in &report.detailed_analyses {
        texts.push(a.aciklama.clone());
    }

    if let Some(translated) = translate_texts(&texts, target_lang).await {
        let mut iter = translated.into_iter();
        if let Some(g) = iter.next() {
            report.genel_sonuc = g;
        }
        for a in report.detailed_analyses.iter_mut() {
            if let Some(new_text) = iter.next() {
                // predicted_product pazarlama ajanının aciklama'sından türetiliyorsa senkron tut
                if a.manipulation_type == "Pazarlama" && report.predicted_product.is_some() {
                    report.predicted_product = Some(new_text.clone());
                }
                a.aciklama = new_text;
            }
        }
    }
    // KALİTE KONTROLÜ: Model bazen bazı alanları eksik/karışık dilde çevirir.
    // repair_language hâlâ yanlış dilde kalan alanları tespit edip yalnızca
    // onları bir kez daha çevirir (ekstra maliyet sadece hata varsa oluşur).
    repair_language(&mut report, target_lang).await;

    // Çeviri başarısız olursa (Ollama erişilemez vb.) rapor olduğu gibi döner;
    // istemci eski görünümü korur, veri kaybı yaşanmaz.
    report
}

// Yönetici (Sentezör) ajana ulaşılamadığında, 6 uzman raporundan basit bir
// en-yüksek-güven-skoru mantığıyla yerel özet üretir (seçili dilde).
fn fallback_summary(analyses: &[AgentAnalysis], lang: &str) -> (bool, String, String) {
    let best = analyses
        .iter()
        .filter(|a| a.detected)
        .max_by(|a, b| a.confidence_score.total_cmp(&b.confidence_score));

    match best {
        Some(agent) => {
            let summary = if lang == "en" {
                format!(
                    "The manager agent could not be reached; based on the expert reports, the most significant finding is: {}",
                    agent.aciklama
                )
            } else {
                format!(
                    "Yönetici ajana ulaşılamadı; uzman ajan raporlarına göre en belirgin bulgu: {}",
                    agent.aciklama
                )
            };
            (true, agent.manipulation_type.clone(), summary)
        }
        None => {
            let summary = if lang == "en" {
                "The manager agent could not be reached; the expert agents did not detect any manipulation.".to_string()
            } else {
                "Yönetici ajana ulaşılamadı; uzman ajanlar herhangi bir manipülasyon tespit etmedi.".to_string()
            };
            (false, "Yok".to_string(), summary)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_quoted_removes_citations() {
        assert_eq!(strip_quoted(r#"metin "quoted part" devam"#), "metin  devam");
        assert_eq!(strip_quoted("alıntısız metin"), "alıntısız metin");
        // Tırnak kapanmadan biterse kalan kısım atılır, panik olmaz
        assert_eq!(strip_quoted(r#"başlangıç "kapanmayan"#), "başlangıç ");
    }

    #[test]
    fn wrong_language_detects_mixups() {
        // TR arayüzde İngilizce kalmış özet → yanlış dil
        assert!(wrong_language(
            "The text is using fear to push the reader toward a purchase.",
            "tr"
        ));
        // EN arayüzde Türkçe kalmış özet → yanlış dil
        assert!(wrong_language(
            "Bu metin korku yaratarak okuyucuyu bir ürüne yönlendiriyor.",
            "en"
        ));
        // Doğru dilde metinler işaretlenmez
        assert!(!wrong_language(
            "Bu metin korku yaratarak okuyucuyu bir ürüne yönlendiriyor.",
            "tr"
        ));
        assert!(!wrong_language(
            "The text is using fear to push the reader toward a purchase.",
            "en"
        ));
    }

    #[test]
    fn wrong_language_ignores_quotes() {
        // İngilizce kısım alıntı içinde: TR arayüzde yanlış dil SAYILMAZ
        assert!(!wrong_language(
            r#"Bu metin "only 3 left in stock, buy the product now" ifadesiyle aciliyet baskısı kuruyor ve bu bir manipülasyon."#,
            "tr"
        ));
    }

    #[test]
    fn fallback_summary_picks_highest_confidence() {
        let make = |t: &str, detected: bool, score: f32| AgentAnalysis {
            manipulation_type: t.to_string(),
            detected,
            confidence_score: score,
            aciklama: format!("{} bulgusu", t),
            target_sentences: vec![],
        };

        let analyses = vec![
            make("Dilsel", true, 0.7),
            make("Psikolojik", true, 0.9),
            make("Sosyal", false, 0.0),
        ];
        let (manipulated, dominant, summary) = fallback_summary(&analyses, "tr");
        assert!(manipulated);
        assert_eq!(dominant, "Psikolojik");
        assert!(summary.contains("Psikolojik bulgusu"));

        // Hiç tespit yoksa temiz sonuç
        let clean = vec![make("Dilsel", false, 0.0)];
        let (manipulated, dominant, _) = fallback_summary(&clean, "en");
        assert!(!manipulated);
        assert_eq!(dominant, "Yok");
    }
}
