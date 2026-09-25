//! Reklam hedefleme ajanı.
//!
//! İki katman vardır ve ayrım bilinçlidir:
//!
//! 1. **Kural katmanı** (`score_candidates`): saf fonksiyon, LLM çağrısı yok.
//!    Envanterdeki kampanyaları profille eşleştirir, skorlar, hariç tutma
//!    kurallarını uygular. Deterministik olduğu için test edilebilir ve
//!    "bu reklam neden gösterildi?" sorusuna kesin cevap verilebilir.
//! 2. **Gerekçe katmanı** (`explain_top`): yalnız kısa listeye tek bir Azure OpenAI
//!    çağrısı yapar ve gerekçeyi insan cümlesine çevirir. Hedefleme KARARINI
//!    vermez — nondeterministik bir modelin reklam kararını denetlemek mümkün
//!    olmaz, üstelik model çöktüğünde hedefleme de çökerdi.
//!
//! Rıza: bu modülün çağrılabilmesi için kullanıcının `ads_consent` alanı true
//! olmalıdır. Çağıran taraf bunu doğrular; `score_candidates` profil almadan
//! zaten aday üretemez.

use crate::agents::call_llm_json;
use crate::types::{DemographicInference, UserProfile};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Kural katmanının sürüm etiketi; skorlama değişirse artırılır.
pub const TARGETING_MODEL_VERSION: &str = "targeting-v1";

/// Bir kampanyanın gösterilmeye değer sayılması için gereken en düşük skor.
/// Altında kalan eşleşme "zorlama" olur: elde ilgili kampanya yoksa hiç
/// reklam göstermemek, alakasızını göstermekten iyidir.
pub const MIN_SCORE: f32 = 0.25;

/// Kullanıcıya bir seferde en fazla kaç kampanya döner.
pub const MAX_ADS: usize = 3;

/// Demografi çıkarımında bir özelliğin kullanılabilmesi için gereken güven.
/// Demografi ajanının kendi eşiğiyle aynı: altında kalan tahmin "bilinmiyor".
const RELIABLE_CONFIDENCE: f32 = 0.60;

/// Envanterdeki tek kampanya.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ad {
    pub id: String,
    pub brand: String,
    pub category: String,
    pub title: String,
    pub body: String,
    /// Hedef kriterleri; boş bırakılan alan "fark etmez" demektir.
    #[serde(default)]
    pub target: AdTarget,
    /// Yaş doğrulaması isteyen kategori (kumar, alkol, kredi).
    #[serde(default)]
    pub sensitive: bool,
    /// Aciliyet/kıtlık kurgusu kullanıyor.
    #[serde(default)]
    pub urgency: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AdTarget {
    #[serde(default)]
    pub ilgi_alanlari: Vec<String>,
    #[serde(default)]
    pub tuketici_egilimi: Vec<String>,
    #[serde(default)]
    pub yas_araliklari: Vec<String>,
    #[serde(default)]
    pub egitim_seviyeleri: Vec<String>,
    #[serde(default)]
    pub diller: Vec<String>,
}

/// Skorlanmış aday: kampanya + skor + hangi kuralların eşleştiği.
#[derive(Serialize, Debug, Clone)]
pub struct ScoredAd {
    pub ad: Ad,
    pub score: f32,
    /// Eşleşen kuralların kısa etiketleri; gerekçe cümlesi bunlardan üretilir.
    pub reasons: Vec<String>,
}

/// Kullanıcıya dönen nihai karar.
#[derive(Serialize, Debug, Clone)]
pub struct AdDecision {
    pub ad_id: String,
    pub brand: String,
    pub title: String,
    pub body: String,
    pub score: f32,
    /// "Neden bu reklam?" satırı — kullanıcıya her zaman gösterilir.
    pub reason: String,
}

/// Profilin çıkarım katmanını tipli hale getirir. Çıkarım yoksa None döner —
/// o durumda yalnız sayaç katmanına dayanan kurallar çalışır.
fn inference_of(profile: &UserProfile) -> Option<DemographicInference> {
    profile
        .inference
        .clone()
        .and_then(|v| serde_json::from_value(v).ok())
}

/// Türkçe metni karşılaştırmaya uygun sade biçime indirger.
fn normalize(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| match c {
            'ı' | 'i' | 'İ' | '\u{0307}' => 'i',
            'ş' => 's',
            'ğ' => 'g',
            'ü' => 'u',
            'ö' => 'o',
            'ç' => 'c',
            'â' => 'a',
            other => other,
        })
        .collect()
}

fn contains_either(a: &str, b: &str) -> bool {
    let (a, b) = (normalize(a), normalize(b));
    !a.is_empty() && !b.is_empty() && (a.contains(&b) || b.contains(&a))
}

/// Güvenilir bir özellik değeri; zayıf ya da "bilinmiyor" ise None.
fn reliable_value(inference: Option<&DemographicInference>, pick: fn(&DemographicInference) -> &crate::types::DemographicTrait) -> Option<String> {
    let inference = inference?;
    let trait_ = pick(inference);
    if trait_.guven < RELIABLE_CONFIDENCE || normalize(&trait_.deger) == "bilinmiyor" {
        return None;
    }
    Some(trait_.deger.clone())
}

/// Kullanıcının baskın manipülasyon tipi (en çok baskın çıkan).
fn dominant_type(profile: &UserProfile) -> Option<&str> {
    profile
        .stats
        .dominant_counts
        .iter()
        .filter(|(tip, _)| normalize(tip) != "yok")
        .max_by_key(|(_, n)| **n)
        .map(|(tip, _)| tip.as_str())
}

/// Kampanyaları profile göre skorlar. Saf fonksiyon: aynı girdi hep aynı
/// sıralamayı verir, bu yüzden doğrudan test edilir.
///
/// Hariç tutma kuralları (skorlamadan önce, kesin):
/// - Kampanya pasifse.
/// - Kullanıcının hiç kullanmadığı bir dile kurulmuş kampanya.
/// - Hassas kategori (kumar/alkol/kredi) ve güvenilir bir yetişkin yaş
///   sinyali yok. Yaş bilinmiyorsa da gösterilmez — "bilinmiyor" yetişkin
///   sayılmaz.
/// - Aciliyet kurgusu kullanan kampanya, davranışsal manipülasyona (FOMO,
///   yapay aciliyet) en çok maruz kalan kullanıcıya gösterilmez. ManipuLens
///   bu tuzağı gösteren araç; aynı tuzağı kendi panelinde kurmaz.
pub fn score_candidates(profile: &UserProfile, inventory: &[Ad]) -> Vec<ScoredAd> {
    let inference = inference_of(profile);
    let inference_ref = inference.as_ref();

    let yas = reliable_value(inference_ref, |i| &i.yas_araligi);
    let egitim = reliable_value(inference_ref, |i| &i.egitim_seviyesi);
    let egilim = reliable_value(inference_ref, |i| &i.tuketici_egilimi);
    let ilgiler: Vec<String> = inference_ref
        .map(|i| i.ilgi_alanlari.clone())
        .unwrap_or_default();
    let dominant = dominant_type(profile).map(normalize);

    let mut scored: Vec<ScoredAd> = inventory
        .iter()
        .filter_map(|ad| {
            // --- Hariç tutma ---
            if !ad.target.diller.is_empty()
                && !ad
                    .target
                    .diller
                    .iter()
                    .any(|d| profile.stats.lang_counts.contains_key(d))
            {
                return None;
            }

            if ad.sensitive && !is_adult(yas.as_deref()) {
                return None;
            }

            if ad.urgency && dominant.as_deref() == Some("davranissal") {
                return None;
            }

            // --- Skorlama ---
            let mut score = 0.0_f32;
            let mut reasons: Vec<String> = Vec::new();

            let ilgi_eslesme = ad
                .target
                .ilgi_alanlari
                .iter()
                .filter(|hedef| ilgiler.iter().any(|i| contains_either(i, hedef)))
                .count();
            if ilgi_eslesme > 0 {
                score += 0.20 * ilgi_eslesme.min(2) as f32;
                reasons.push("ilgi alanı".to_string());
            }

            if let Some(egilim) = egilim.as_deref() {
                if ad
                    .target
                    .tuketici_egilimi
                    .iter()
                    .any(|hedef| contains_either(egilim, hedef))
                {
                    score += 0.25;
                    reasons.push("tüketici eğilimi".to_string());
                }
            }

            if let Some(yas) = yas.as_deref() {
                if ad.target.yas_araliklari.iter().any(|h| h == yas) {
                    score += 0.15;
                    reasons.push("yaş aralığı".to_string());
                }
            }

            if let Some(egitim) = egitim.as_deref() {
                if ad
                    .target
                    .egitim_seviyeleri
                    .iter()
                    .any(|h| contains_either(egitim, h))
                {
                    score += 0.10;
                    reasons.push("eğitim seviyesi".to_string());
                }
            }

            // Pazarlama ajanının çıkardığı ürün cümleleri: kullanıcının zaten
            // maruz kaldığı sektör. Sayaç katmanından gelir, çıkarım gerekmez.
            if profile
                .stats
                .top_products
                .iter()
                .any(|p| contains_either(p, &ad.category) || contains_either(p, &ad.brand))
            {
                score += 0.20;
                reasons.push("geçmiş ürün ilgisi".to_string());
            }

            if score < MIN_SCORE {
                return None;
            }

            Some(ScoredAd {
                ad: ad.clone(),
                score,
                reasons,
            })
        })
        .collect();

    // Skora göre azalan; eşitlikte id'ye göre — sıralama kararlı olsun.
    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.ad.id.cmp(&b.ad.id))
    });
    scored.truncate(MAX_ADS);
    scored
}

/// Yaş aralığı yetişkin mi? Bilinmiyorsa (None) yetişkin SAYILMAZ.
fn is_adult(yas: Option<&str>) -> bool {
    let Some(yas) = yas else { return false };
    // "25-34", "18-24", "35-44", "65+" gibi etiketlerden ilk sayıyı al.
    let first: String = yas.chars().take_while(|c| c.is_ascii_digit()).collect();
    first.parse::<u32>().map(|n| n >= 18).unwrap_or(false)
}

/// Kural katmanının etiketlerinden gerekçe cümlesi üretir. LLM çağrısı
/// başarısız olduğunda da kullanıcı "neden bu reklam?" cevabını görür.
pub fn local_reason(scored: &ScoredAd, lang: &str) -> String {
    if scored.reasons.is_empty() {
        return if lang == "en" {
            "Shown as a general suggestion; your profile was not used.".to_string()
        } else {
            "Genel öneri olarak gösteriliyor; profilin kullanılmadı.".to_string()
        };
    }
    if lang == "en" {
        format!("Matched on: {}.", scored.reasons.join(", "))
    } else {
        format!("Eşleşme: {}.", scored.reasons.join(", "))
    }
}

/// Kısa listeye tek bir Azure OpenAI çağrısı yapıp gerekçeleri insan cümlesine
/// çevirir. Çağrı başarısız olursa kural katmanının etiketleri kullanılır —
/// hedefleme LLM'e bağımlı hale gelmez.
pub async fn explain_top(candidates: &[ScoredAd], profile: &UserProfile, lang: &str) -> Vec<AdDecision> {
    let mut decisions: Vec<AdDecision> = candidates
        .iter()
        .map(|c| AdDecision {
            ad_id: c.ad.id.clone(),
            brand: c.ad.brand.clone(),
            title: c.ad.title.clone(),
            body: c.ad.body.clone(),
            score: c.score,
            reason: local_reason(c, lang),
        })
        .collect();

    if decisions.is_empty() {
        return decisions;
    }

    let out_lang = if lang == "en" { "English" } else { "Turkish" };
    let ozet = inference_of(profile).map(|i| i.ozet).unwrap_or_default();

    let system = format!(
        r#"You write the "why am I seeing this ad?" line shown under each suggestion.

INPUT: JSON with "profile_summary" and "ads" (each with id, brand, category and the matched rule labels).
For every ad write ONE short sentence naming the concrete reason it matched - the interest, tendency or past exposure - in plain words.
NEVER invent a reason that is not in the matched labels. NEVER mention age, education or any demographic guess explicitly; say what the person is interested in, not who the model thinks they are.
LANGUAGE: write every sentence in {out_lang}.

Output ONLY one valid JSON object, no markdown:
{{"reasons":[{{"id":"<ad id>","reason":"..."}}]}}"#
    );

    let input = json!({
        "profile_summary": ozet,
        "ads": candidates.iter().map(|c| json!({
            "id": c.ad.id,
            "brand": c.ad.brand,
            "category": c.ad.category,
            "matched": c.reasons,
        })).collect::<Vec<_>>(),
    })
    .to_string();

    let llm: Option<Vec<(String, String)>> = async {
        let text = call_llm_json(&system, &input).await.ok()?;

        #[derive(Deserialize)]
        struct Reason {
            id: String,
            reason: String,
        }
        #[derive(Deserialize)]
        struct Out {
            reasons: Vec<Reason>,
        }

        let out: Out = serde_json::from_str(&text).ok()?;
        Some(out.reasons.into_iter().map(|r| (r.id, r.reason)).collect())
    }
    .await;

    if let Some(pairs) = llm {
        for (id, reason) in pairs {
            let reason = reason.trim();
            if reason.is_empty() {
                continue;
            }
            if let Some(d) = decisions.iter_mut().find(|d| d.ad_id == id) {
                d.reason = reason.to_string();
            }
        }
    } else {
        tracing::warn!("reklam gerekçesi üretilemedi; kural etiketleri kullanılıyor");
    }

    decisions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DemographicTrait, ProfileStats};
    use std::collections::BTreeMap;

    fn trait_of(deger: &str, guven: f32) -> DemographicTrait {
        DemographicTrait {
            deger: deger.to_string(),
            guven,
            dayanak: "dayanak".to_string(),
        }
    }

    fn inference(yas: &str, egitim: &str, egilim: &str, ilgiler: &[&str]) -> serde_json::Value {
        serde_json::to_value(DemographicInference {
            yas_araligi: trait_of(yas, 0.8),
            cinsiyet: trait_of("bilinmiyor", 0.0),
            egitim_seviyesi: trait_of(egitim, 0.7),
            tuketici_egilimi: trait_of(egilim, 0.9),
            ilgi_alanlari: ilgiler.iter().map(|s| s.to_string()).collect(),
            ozet: "özet".to_string(),
        })
        .unwrap()
    }

    fn profile(inference: Option<serde_json::Value>) -> UserProfile {
        let mut lang_counts = BTreeMap::new();
        lang_counts.insert("tr".to_string(), 10);
        UserProfile {
            user_id: "uid-1".to_string(),
            stats: ProfileStats {
                total: 10,
                manipulated: 5,
                lang_counts,
                ..Default::default()
            },
            inference,
            model_version: "demographic-v1".to_string(),
            updated_at: "2026-09-12T00:00:00+03:00".to_string(),
            inference_at: None,
            inference_count: None,
        }
    }

    fn ad(id: &str, category: &str, ilgi: &[&str]) -> Ad {
        Ad {
            id: id.to_string(),
            brand: format!("Marka-{id}"),
            category: category.to_string(),
            title: "başlık".to_string(),
            body: "gövde".to_string(),
            target: AdTarget {
                ilgi_alanlari: ilgi.iter().map(|s| s.to_string()).collect(),
                diller: vec!["tr".to_string()],
                ..Default::default()
            },
            sensitive: false,
            urgency: false,
        }
    }

    #[test]
    fn interest_and_tendency_matches_rank_first() {
        let p = profile(Some(inference("25-34", "üniversite", "teknoloji ürünleri", &["kripto", "yazılım"])));
        let mut hedefli = ad("a1", "finans", &["kripto"]);
        hedefli.target.tuketici_egilimi = vec!["teknoloji".to_string()];
        let zayif = ad("a2", "kozmetik", &["cilt bakımı"]);

        let sonuc = score_candidates(&p, &[zayif, hedefli]);
        assert_eq!(sonuc.len(), 1, "eşiği geçmeyen kampanya elenir");
        assert_eq!(sonuc[0].ad.id, "a1");
        assert!(sonuc[0].reasons.contains(&"ilgi alanı".to_string()));
        assert!(sonuc[0].reasons.contains(&"tüketici eğilimi".to_string()));
    }

    #[test]
    fn sensitive_ads_need_a_reliable_adult_age() {
        let cocuk = profile(Some(inference("13-17", "lise", "oyun", &["kumar"])));
        let mut hassas = ad("a1", "kumar", &["kumar"]);
        hassas.sensitive = true;
        hassas.target.tuketici_egilimi = vec!["oyun".to_string()];
        assert!(score_candidates(&cocuk, std::slice::from_ref(&hassas)).is_empty());

        // Yaş bilinmiyorsa da gösterilmez
        let bilinmiyor = profile(Some(inference("bilinmiyor", "lise", "oyun", &["kumar"])));
        assert!(score_candidates(&bilinmiyor, std::slice::from_ref(&hassas)).is_empty());

        let yetiskin = profile(Some(inference("25-34", "lise", "oyun", &["kumar"])));
        assert_eq!(score_candidates(&yetiskin, &[hassas]).len(), 1);
    }

    #[test]
    fn urgency_ads_are_withheld_from_fomo_prone_users() {
        let mut p = profile(Some(inference("25-34", "lise", "teknoloji", &["kripto"])));
        p.stats
            .dominant_counts
            .insert("Davranışsal".to_string(), 7);
        p.stats.dominant_counts.insert("Dilsel".to_string(), 2);

        let mut aciliyet = ad("a1", "finans", &["kripto"]);
        aciliyet.urgency = true;
        aciliyet.target.tuketici_egilimi = vec!["teknoloji".to_string()];
        assert!(
            score_candidates(&p, std::slice::from_ref(&aciliyet)).is_empty(),
            "FOMO'ya açık kullanıcıya aciliyet kurgusu gösterilmez"
        );

        // Aynı kampanya, aciliyet kurgusu olmadan gösterilebilir
        aciliyet.urgency = false;
        assert_eq!(score_candidates(&p, &[aciliyet]).len(), 1);
    }

    #[test]
    fn language_mismatch_excludes_the_campaign() {
        let p = profile(Some(inference("25-34", "lise", "teknoloji", &["kripto"])));
        let mut yabanci = ad("a1", "finans", &["kripto"]);
        yabanci.target.diller = vec!["en".to_string()];
        yabanci.target.tuketici_egilimi = vec!["teknoloji".to_string()];
        assert!(score_candidates(&p, &[yabanci]).is_empty());
    }

    #[test]
    fn weak_inference_is_not_used() {
        // Güveni düşük özellikler "bilinmiyor" gibi davranır: yalnız ilgi
        // alanı eşleşmesi kalır ve tek başına eşiği geçmez.
        let mut zayif = inference("25-34", "üniversite", "teknoloji", &["kripto"]);
        zayif["tuketici_egilimi"]["guven"] = json!(0.3);
        zayif["yas_araligi"]["guven"] = json!(0.3);
        zayif["egitim_seviyesi"]["guven"] = json!(0.3);
        let p = profile(Some(zayif));

        let mut kampanya = ad("a1", "finans", &["kripto"]);
        kampanya.target.tuketici_egilimi = vec!["teknoloji".to_string()];
        kampanya.target.yas_araliklari = vec!["25-34".to_string()];
        kampanya.target.egitim_seviyeleri = vec!["üniversite".to_string()];

        assert!(
            score_candidates(&p, std::slice::from_ref(&kampanya)).is_empty(),
            "zayıf çıkarımla geriye yalnız 0.20'lik ilgi eşleşmesi kalır, eşiği geçmez"
        );

        // Aynı kampanya, güvenilir çıkarımla eşiği geçer: eleyen şey güven,
        // kampanyanın kendisi değil.
        let guvenli = profile(Some(inference("25-34", "üniversite", "teknoloji", &["kripto"])));
        assert_eq!(score_candidates(&guvenli, &[kampanya]).len(), 1);
    }

    #[test]
    fn profile_without_inference_can_still_match_on_history() {
        let mut p = profile(None);
        p.stats.top_products =
            vec!["Kişi Kripto-X satın almaya veya yönelmeye meyilli olabilir.".to_string()];

        let mut kampanya = ad("a1", "Kripto-X", &["kripto"]);
        kampanya.brand = "Kripto-X".to_string();
        let sonuc = score_candidates(&p, &[kampanya]);
        assert_eq!(sonuc.len(), 0, "tek başına 0.20 eşiğin altında kalır");

        // Ürün hem markayla hem kategoriyle eşleşirse yine tek kural sayılır;
        // ikinci bir sinyal gerekir.
        let mut ikinci = ad("a2", "kripto", &["kripto"]);
        ikinci.brand = "Kripto-X".to_string();
        ikinci.target.diller = vec![];
        let sonuc = score_candidates(&p, &[ikinci]);
        assert!(sonuc.is_empty());
    }

    #[test]
    fn results_are_capped_and_stable() {
        let p = profile(Some(inference("25-34", "lise", "teknoloji", &["kripto"])));
        let inventory: Vec<Ad> = (0..6)
            .map(|i| {
                let mut a = ad(&format!("a{i}"), "finans", &["kripto"]);
                a.target.tuketici_egilimi = vec!["teknoloji".to_string()];
                a
            })
            .collect();

        let sonuc = score_candidates(&p, &inventory);
        assert_eq!(sonuc.len(), MAX_ADS);
        // Eşit skorda id sırası kararlıdır
        assert_eq!(sonuc[0].ad.id, "a0");
        assert_eq!(sonuc[2].ad.id, "a2");
    }
}
