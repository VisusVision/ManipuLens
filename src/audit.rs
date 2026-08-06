//! Yapılandırılmış denetim (audit) logu.
//!
//! Eski sistem: `analiz_log.txt` içine serbest formatta, analiz edilen METNİN
//! TAMAMI ve kullanıcı e-postası düz metin yazılıyordu — hem ayrıştırılamıyor
//! hem gizlilik riski taşıyordu.
//!
//! Yeni sistem:
//! - `logs/audit-YYYY-MM-DD.jsonl`: günlük dönen, satır başına bir JSON olay.
//!   Her olayda ISO zaman damgası + olay tipi + yapılandırılmış alanlar var;
//!   `jq` veya herhangi bir araçla sorgulanabilir.
//! - Metnin tamamı ASLA loglanmaz; yalnızca ilk 120 karakterlik önizleme ve
//!   uzunluk yazılır (analiz sonuçlarının tam kaydı zaten SQLite `history`
//!   tablosunda kullanıcı bazında tutuluyor).
//! - Auth olayları (giriş, hatalı giriş, kilitleme, kayıt, kod, sıfırlama)
//!   da aynı formatta loglanır — kim, ne zaman, ne yaptı takip edilebilir.
//! - Aynı olay `tracing` ile konsola da düşer (canlı izleme).

use chrono::Local;
use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::Write;

/// Metin önizlemesi: ilk 120 karakter (log ve geçmiş kayıtlarında tam metin tutulmaz)
pub const PREVIEW_CHARS: usize = 120;

pub fn preview(text: &str) -> String {
    text.chars().take(PREVIEW_CHARS).collect()
}

/// Olayı günlük JSONL dosyasına yazar ve konsola düşürür.
/// `fields` bir JSON nesnesi olmalı; içine ts/event otomatik eklenir.
pub fn audit(event: &str, mut fields: Value) {
    let now = Local::now();
    if let Some(obj) = fields.as_object_mut() {
        obj.insert("ts".to_string(), json!(now.to_rfc3339()));
        obj.insert("event".to_string(), json!(event));
    }

    tracing::info!(target: "audit", event, detay = %fields);

    let _ = std::fs::create_dir_all("logs");
    let path = format!("logs/audit-{}.jsonl", now.format("%Y-%m-%d"));
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(file, "{}", fields);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_truncates_at_char_boundary() {
        let long: String = "ğ".repeat(300);
        let p = preview(&long);
        assert_eq!(p.chars().count(), PREVIEW_CHARS);
        // Çok baytlı karakterde panik olmamalı (byte değil char sınırı)
        assert!(p.chars().all(|c| c == 'ğ'));
    }

    #[test]
    fn preview_short_text_unchanged() {
        assert_eq!(preview("kısa metin"), "kısa metin");
    }
}
