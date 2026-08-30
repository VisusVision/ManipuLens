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
use chrono::Local;
use std::collections::BTreeMap;

/// Profil sayaçlarının hesaplandığı en fazla geçmiş kaydı.
pub const PROFILE_HISTORY_LIMIT: i64 = 500;

/// Çıkarım katmanı kaç analizde bir tazelenir (demografi ajanı bunu kullanır).
#[allow(dead_code)]
pub const PROFILE_INFERENCE_EVERY: i64 = 5;

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
    };

    db.upsert_profile(&profile);
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
