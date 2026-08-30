//! Kullanıcı profili ve veri seti katmanı.
//!
//! İki katman vardır ve maliyetleri kasten farklıdır:
//!
//! 1. **Sayaç katmanı** (`compute_stats`): kullanıcının geçmişinden doğrudan
//!    sayılan büyüklükler. LLM çağrısı yoktur, her analizden sonra çalışır ve
//!    milisaniye sürer. Tahmin değil ölçümdür.
//! 2. **Çıkarım katmanı** (Kart 2, demografi ajanı): yorum üretir, LLM ister.
//!    Her analizde değil, `PROFILE_INFERENCE_EVERY` analizde bir tetiklenir —
//!    çünkü bir analiz zaten 7 Ollama çağrısı yapıyor ve 8.'si kullanıcının
//!    bekleme süresine biner.
//!
//! Her iki katman da isteğin DIŞINDA (`tokio::spawn`) çalışır: kullanıcı
//! raporunu alır, bağlantı kapanır, profil sonra sessizce tazelenir.

use crate::db::Db;
use crate::types::{HistoryEntry, ProfileStats, UserProfile};
use chrono::{DateTime, Local};
use std::collections::BTreeMap;

/// Profil sayaçlarının hesaplandığı en fazla geçmiş kaydı.
pub const PROFILE_HISTORY_LIMIT: i64 = 500;

/// Çıkarım katmanı kaç analizde bir tazelenir. Her analizde çalıştırmak
/// aynı yorumu tekrar tekrar üretmek olurdu; 5'te bir, LLM yükünü ~%80
/// azaltıp profilin güncelliğini gözle görülür biçimde bozmuyor.
pub const PROFILE_INFERENCE_EVERY: i64 = 5;

/// Hiç yeni analiz gelmese bile çıkarımın bayat sayıldığı süre (saat).
pub const PROFILE_INFERENCE_MAX_AGE_HOURS: i64 = 24;

/// Demografi ajanına verilen en fazla metin önizlemesi. Prompt'u şişirmemek
/// için sınırlı: 30 önizleme davranış örüntüsü için yeterli.
const EVIDENCE_PREVIEW_LIMIT: usize = 30;

/// Çıkarım katmanının sürüm etiketi. Prompt veya şema değişirse artırılır.
const INFERENCE_MODEL_VERSION: &str = "demographic-v1";

/// Profil üretmek için gereken en az analiz sayısı: daha azında istatistik de
/// çıkarım da gürültüden ibaret olur.
pub const PROFILE_MIN_ANALYSES: i64 = 5;

/// `top_products` listesinde tutulan en fazla öğe.
const TOP_PRODUCTS_LIMIT: usize = 10;

/// Geçmiş kayıtlarından deterministik sayaçları üretir. LLM kullanmaz.
pub fn compute_stats(entries: &[HistoryEntry]) -> ProfileStats {
    let mut stats = ProfileStats {
        total: entries.len() as i64,
        ..Default::default()
    };

    let mut product_counts: BTreeMap<String, i64> = BTreeMap::new();
    let mut text_len_sum: i64 = 0;
    let mut text_len_n: i64 = 0;

    for entry in entries {
        if entry.is_manipulated {
            stats.manipulated += 1;
        }

        // "Yok" baskın tip değil, temiz sonucun etiketidir — sayaca girmez.
        if entry.dominant_manipulation != "Yok" && !entry.dominant_manipulation.is_empty() {
            *stats
                .dominant_counts
                .entry(entry.dominant_manipulation.clone())
                .or_insert(0) += 1;
        }

        if let Some(agents) = &entry.agents {
            for verdict in agents {
                if verdict.d {
                    *stats.agent_detect_counts.entry(verdict.t.clone()).or_insert(0) += 1;
                }
            }
        }

        if let Some(lang) = &entry.lang {
            *stats.lang_counts.entry(lang.clone()).or_insert(0) += 1;
        }

        if let Some(product) = &entry.predicted_product {
            let trimmed = product.trim();
            if !trimmed.is_empty() {
                *product_counts.entry(trimmed.to_string()).or_insert(0) += 1;
            }
        }

        if let Some(len) = entry.text_len {
            text_len_sum += len;
            text_len_n += 1;
        }

        // Kayıtlar en yeni önce geldiği için ilk/son sınırlarını karşılaştırarak
        // buluruz; sıralamaya bağımlı kalmayalım (ISO 8601 sözlüksel sıralanır).
        match &stats.first_seen {
            Some(first) if first.as_str() <= entry.timestamp.as_str() => {}
            _ => stats.first_seen = Some(entry.timestamp.clone()),
        }
        match &stats.last_seen {
            Some(last) if last.as_str() >= entry.timestamp.as_str() => {}
            _ => stats.last_seen = Some(entry.timestamp.clone()),
        }
    }

    if text_len_n > 0 {
        stats.avg_text_len = text_len_sum / text_len_n;
    }

    // En sık geçen ürün/sektör cümleleri: önce sayıya, eşitlikte alfabetik
    // (deterministik çıktı — aynı veri her zaman aynı profili üretir).
    let mut products: Vec<(String, i64)> = product_counts.into_iter().collect();
    products.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    stats.top_products = products
        .into_iter()
        .take(TOP_PRODUCTS_LIMIT)
        .map(|(p, _)| p)
        .collect();

    stats
}

/// Kullanıcının profilindeki sayaç katmanını tazeler ve kaydeder.
///
/// `client_id` geçmiş tablosundaki kimliktir (bugün e-posta), `user_id` ise
/// profilin anahtarıdır (UUID). Yeterli kayıt yoksa profil yazılmaz —
/// yarım veriyle üretilmiş profil, profilin olmamasından kötüdür.
pub fn refresh_stats(db: &Db, user_id: &str, client_id: &str) -> Option<UserProfile> {
    let entries: Vec<HistoryEntry> = db
        .history_for_client(client_id, PROFILE_HISTORY_LIMIT)
        .into_iter()
        .map(|(_, e)| e)
        .collect();

    if (entries.len() as i64) < PROFILE_MIN_ANALYSES {
        return None;
    }

    let stats = compute_stats(&entries);

    // Var olan çıkarım (demografi ajanı çıktısı) korunur; bu fonksiyon
    // yalnızca sayaç katmanını tazeler.
    let previous = db.profile_for_user(user_id);
    let profile = UserProfile {
        user_id: user_id.to_string(),
        stats,
        inference: previous.as_ref().and_then(|p| p.inference.clone()),
        model_version: previous
            .as_ref()
            .map(|p| p.model_version.clone())
            .unwrap_or_else(|| "stats-v1".to_string()),
        updated_at: Local::now().to_rfc3339(),
        // Çıkarım meta bilgisi sayaç tazelemesinde korunur: bayatlık ölçümü
        // yalnızca çıkarımın kendisi yenilendiğinde sıfırlanmalı.
        inference_at: previous.as_ref().and_then(|p| p.inference_at.clone()),
        inference_count: previous.as_ref().and_then(|p| p.inference_count),
    };

    db.upsert_profile(&profile);
    Some(profile)
}

/// Çıkarım katmanının tazelenmesi gerekiyor mu?
///
/// Üç tetikleyici var: hiç çıkarım yok, son çıkarımdan bu yana
/// `PROFILE_INFERENCE_EVERY` yeni analiz birikti, ya da çıkarım
/// `PROFILE_INFERENCE_MAX_AGE_HOURS` saatten eski.
pub fn needs_inference(profile: &UserProfile) -> bool {
    if profile.inference.is_none() {
        return true;
    }

    match profile.inference_count {
        Some(count) if profile.stats.total - count >= PROFILE_INFERENCE_EVERY => return true,
        None => return true,
        _ => {}
    }

    match profile
        .inference_at
        .as_deref()
        .and_then(|ts| DateTime::parse_from_rfc3339(ts).ok())
    {
        Some(at) => {
            let age = Local::now().signed_duration_since(at);
            age.num_hours() >= PROFILE_INFERENCE_MAX_AGE_HOURS
        }
        // Zaman damgası okunamıyorsa bayat kabul et: tazelemek, yanlış
        // güncel sanmaktan ucuz.
        None => true,
    }
}

/// Demografi ajanına verilecek kanıt paketi: sayaçlar + son taranan
/// metinlerin önizlemeleri. Tam metin zaten hiç saklanmıyor.
fn build_evidence(stats: &ProfileStats, entries: &[HistoryEntry]) -> String {
    let previews: Vec<&str> = entries
        .iter()
        .take(EVIDENCE_PREVIEW_LIMIT)
        .map(|e| e.text_preview.as_str())
        .collect();

    serde_json::json!({
        "stats": stats,
        "recent_previews": previews,
    })
    .to_string()
}

/// Çıkarım katmanını tazeler: demografi ajanını çağırır ve sonucu profile
/// yazar. Ajan hata verirse profil OLDUĞU GİBİ bırakılır — bozuk çıkarım
/// yazmaktansa eski çıkarımı korumak daha doğru.
///
/// Bu fonksiyon isteğin dışında çağrılmak üzere tasarlandı; kullanıcı bunu
/// beklemez.
pub async fn refresh_inference(
    db: &Db,
    user_id: &str,
    client_id: &str,
    lang: &str,
) -> Option<UserProfile> {
    let entries: Vec<HistoryEntry> = db
        .history_for_client(client_id, PROFILE_HISTORY_LIMIT)
        .into_iter()
        .map(|(_, e)| e)
        .collect();

    if (entries.len() as i64) < PROFILE_MIN_ANALYSES {
        return None;
    }

    let stats = compute_stats(&entries);
    let evidence = build_evidence(&stats, &entries);

    let inference = match crate::agents::analyze_demographic(&evidence, lang).await {
        Ok(inference) => inference,
        Err(e) => {
            tracing::warn!(user_id, error = %e, "demografi ajanı başarısız; profil korunuyor");
            return None;
        }
    };

    let now = Local::now().to_rfc3339();
    let total = stats.total;
    let profile = UserProfile {
        user_id: user_id.to_string(),
        stats,
        inference: serde_json::to_value(&inference).ok(),
        model_version: INFERENCE_MODEL_VERSION.to_string(),
        updated_at: now.clone(),
        inference_at: Some(now),
        inference_count: Some(total),
    };

    db.upsert_profile(&profile);
    tracing::info!(user_id, analyses = total, "kullanıcı profili çıkarımı tazelendi");
    Some(profile)
}

/// Veri setini JSONL olarak dışa aktarır: satır başına bir analiz.
///
/// Gizlilik: e-posta ASLA yazılmaz — kullanıcı ayrımı `user_id` (UUID) ile
/// yapılır, çözülemeyen eski kayıtlar `null` kalır. Metnin tamamı zaten
/// saklanmıyor; yalnızca 120 karakterlik önizleme dışa aktarılır.
pub fn export_dataset(db: &Db, path: &str) -> Result<usize, String> {
    use std::io::Write;

    let entries = db.history_for_export();
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut writer = std::io::BufWriter::new(file);
    let mut written = 0usize;

    for entry in &entries {
        let line = serde_json::json!({
            "ts": entry.timestamp,
            "user_id": entry.user_id,
            "lang": entry.lang,
            "text_preview": entry.text_preview,
            "text_len": entry.text_len,
            "is_manipulated": entry.is_manipulated,
            "dominant": entry.dominant_manipulation,
            "agents": entry.agents,
            "predicted_product": entry.predicted_product,
            "summary": entry.genel_sonuc,
        });
        writeln!(writer, "{}", line).map_err(|e| e.to_string())?;
        written += 1;
    }

    writer.flush().map_err(|e| e.to_string())?;
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AgentVerdict;

    fn entry(ts: &str, manipulated: bool, dominant: &str, lang: &str) -> HistoryEntry {
        HistoryEntry {
            timestamp: ts.to_string(),
            client_id: "a@b.com".to_string(),
            text_preview: "metin".to_string(),
            is_manipulated: manipulated,
            dominant_manipulation: dominant.to_string(),
            genel_sonuc: "özet".to_string(),
            lang: Some(lang.to_string()),
            user_id: Some("uid-1".to_string()),
            agents: Some(vec![
                AgentVerdict { t: "Dilsel".into(), d: manipulated, c: 0.8 },
                AgentVerdict { t: "Pazarlama".into(), d: false, c: 0.1 },
            ]),
            predicted_product: if manipulated { Some("Kişi X almaya meyilli".into()) } else { None },
            text_len: Some(100),
        }
    }

    #[test]
    fn stats_count_dominant_and_agents() {
        let entries = vec![
            entry("2026-01-01T00:00:00+03:00", true, "Pazarlama", "tr"),
            entry("2026-01-02T00:00:00+03:00", true, "Pazarlama", "tr"),
            entry("2026-01-03T00:00:00+03:00", false, "Yok", "en"),
        ];
        let stats = compute_stats(&entries);

        assert_eq!(stats.total, 3);
        assert_eq!(stats.manipulated, 2);
        // "Yok" baskın tip sayacına girmez
        assert_eq!(stats.dominant_counts.get("Pazarlama"), Some(&2));
        assert!(stats.dominant_counts.get("Yok").is_none());
        // Ajan tespitleri baskın tipten bağımsız sayılır
        assert_eq!(stats.agent_detect_counts.get("Dilsel"), Some(&2));
        assert_eq!(stats.lang_counts.get("tr"), Some(&2));
        assert_eq!(stats.lang_counts.get("en"), Some(&1));
        assert_eq!(stats.avg_text_len, 100);
        assert_eq!(stats.first_seen.as_deref(), Some("2026-01-01T00:00:00+03:00"));
        assert_eq!(stats.last_seen.as_deref(), Some("2026-01-03T00:00:00+03:00"));
        assert_eq!(stats.top_products.len(), 1);
    }

    #[test]
    fn stats_tolerate_old_entries_without_new_fields() {
        // Şema göçünden önceki kayıtlar: agents/text_len/predicted_product yok
        let mut old = entry("2026-01-01T00:00:00+03:00", true, "Dilsel", "tr");
        old.agents = None;
        old.text_len = None;
        old.predicted_product = None;

        let stats = compute_stats(&[old]);
        assert_eq!(stats.total, 1);
        assert_eq!(stats.avg_text_len, 0);
        assert!(stats.agent_detect_counts.is_empty());
        assert!(stats.top_products.is_empty());
    }

    #[test]
    fn refresh_stats_requires_minimum_history() {
        let db = Db::open_in_memory();
        for i in 0..(PROFILE_MIN_ANALYSES - 1) {
            db.insert_history(&entry(
                &format!("2026-01-0{}T00:00:00+03:00", i + 1),
                true,
                "Pazarlama",
                "tr",
            ));
        }
        assert!(refresh_stats(&db, "uid-1", "a@b.com").is_none());

        db.insert_history(&entry("2026-01-09T00:00:00+03:00", true, "Pazarlama", "tr"));
        let profile = refresh_stats(&db, "uid-1", "a@b.com").expect("profil üretilmeli");
        assert_eq!(profile.stats.total, PROFILE_MIN_ANALYSES);
        assert_eq!(db.profile_for_user("uid-1").unwrap().stats.total, PROFILE_MIN_ANALYSES);
    }

    fn profile_with(total: i64, inference_count: Option<i64>, inference_at: Option<String>) -> UserProfile {
        UserProfile {
            user_id: "uid-1".into(),
            stats: ProfileStats { total, ..Default::default() },
            inference: Some(serde_json::json!({"ozet": "x"})),
            model_version: "demographic-v1".into(),
            updated_at: Local::now().to_rfc3339(),
            inference_at,
            inference_count,
        }
    }

    #[test]
    fn inference_needed_when_missing() {
        let mut p = profile_with(10, Some(10), Some(Local::now().to_rfc3339()));
        assert!(!needs_inference(&p));
        p.inference = None;
        assert!(needs_inference(&p));
    }

    #[test]
    fn inference_needed_after_n_new_analyses() {
        let now = Local::now().to_rfc3339();
        // 4 yeni analiz: henüz eşiğe gelmedi
        let p = profile_with(14, Some(10), Some(now.clone()));
        assert!(!needs_inference(&p));
        // 5 yeni analiz: eşik doldu
        let p = profile_with(15, Some(10), Some(now));
        assert!(needs_inference(&p));
    }

    #[test]
    fn inference_needed_when_stale_by_time() {
        let old = (Local::now() - chrono::Duration::hours(PROFILE_INFERENCE_MAX_AGE_HOURS + 1))
            .to_rfc3339();
        let p = profile_with(10, Some(10), Some(old));
        assert!(needs_inference(&p));
    }

    #[test]
    fn inference_needed_when_timestamp_unreadable() {
        let p = profile_with(10, Some(10), Some("bozuk-zaman".into()));
        assert!(needs_inference(&p));
    }

    #[test]
    fn evidence_caps_previews_and_keeps_stats() {
        let entries: Vec<HistoryEntry> = (0..40)
            .map(|i| entry(&format!("2026-01-01T00:00:{:02}+03:00", i), true, "Pazarlama", "tr"))
            .collect();
        let stats = compute_stats(&entries);
        let evidence: serde_json::Value =
            serde_json::from_str(&build_evidence(&stats, &entries)).unwrap();

        assert_eq!(
            evidence["recent_previews"].as_array().unwrap().len(),
            EVIDENCE_PREVIEW_LIMIT
        );
        assert_eq!(evidence["stats"]["total"], 40);
    }

    #[test]
    fn refresh_stats_preserves_existing_inference() {
        let db = Db::open_in_memory();
        for i in 0..PROFILE_MIN_ANALYSES {
            db.insert_history(&entry(
                &format!("2026-01-{:02}T00:00:00+03:00", i + 1),
                true,
                "Pazarlama",
                "tr",
            ));
        }
        db.upsert_profile(&profile_with(
            PROFILE_MIN_ANALYSES,
            Some(PROFILE_MIN_ANALYSES),
            Some(Local::now().to_rfc3339()),
        ));

        let refreshed = refresh_stats(&db, "uid-1", "a@b.com").expect("profil üretilmeli");
        // Sayaç tazelemesi çıkarımı silmez
        assert!(refreshed.inference.is_some());
        assert_eq!(refreshed.inference_count, Some(PROFILE_MIN_ANALYSES));
    }

    #[test]
    fn agents_survive_database_roundtrip() {
        let db = Db::open_in_memory();
        db.insert_history(&entry("2026-01-01T00:00:00+03:00", true, "Pazarlama", "tr"));

        let rows = db.history_for_client("a@b.com", 10);
        let stored = &rows[0].1;
        let agents = stored.agents.as_ref().expect("ajan kararları saklanmalı");
        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0].t, "Dilsel");
        assert!(agents[0].d);
        assert_eq!(stored.user_id.as_deref(), Some("uid-1"));
        assert_eq!(stored.text_len, Some(100));
    }
}
