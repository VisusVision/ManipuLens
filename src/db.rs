//! SQLite veri katmanı.
//!
//! Eski sürüm users.json / history.jsonl dosyalarına yazıyordu; her kayıtta
//! tüm dosya yeniden yazılıyor ve dosyalar sınırsız büyüyordu. Artık tüm
//! kalıcı veri (kullanıcılar, analiz geçmişi, oturumlar) tek SQLite
//! dosyasında tutulur. İlk açılışta eski JSON dosyaları varsa İÇE AKTARILIR
//! (dosyalar silinmez/değiştirilmez — geri dönüş her zaman mümkün).

use crate::types::{AgentVerdict, HistoryEntry, User, UserProfile};
use rusqlite::{params, Connection};
use std::sync::Mutex;

pub struct Db {
    conn: Mutex<Connection>,
}

/// Oturum kaydı (Authorization: Bearer <token> ile doğrulanır)
pub struct Session {
    pub user_id: String,
    pub email: String,
    pub expires_at: i64,
}

impl Db {
    pub fn open(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| e.to_string())?;
        let db = Db { conn: Mutex::new(conn) };
        db.init_schema()?;
        Ok(db)
    }

    #[cfg(test)]
    pub fn open_in_memory() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        let db = Db { conn: Mutex::new(conn) };
        db.init_schema().unwrap();
        db
    }

    fn init_schema(&self) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS users (
                id            TEXT PRIMARY KEY,
                email         TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at    TEXT NOT NULL,
                verified      INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS history (
                id                    INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp             TEXT NOT NULL,
                client_id             TEXT NOT NULL,
                text_preview          TEXT NOT NULL,
                is_manipulated        INTEGER NOT NULL,
                dominant_manipulation TEXT NOT NULL,
                genel_sonuc           TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_history_client ON history(client_id);
            "#,
        )
        .map_err(|e| e.to_string())?;

        // Şema göçü: eski kurulumlarda history tablosunda bu sütunlar yok.
        // ALTER TABLE idempotent değil; sütun zaten varsa hata döner, yok sayılır.
        let _ = conn.execute("ALTER TABLE history ADD COLUMN lang TEXT", []);
        // Veri seti katmanı: kimlik e-posta yerine UUID ile taşınsın ve 6
        // uzman ajanın kararı da saklansın (eskiden yalnızca baskın tip vardı).
        let _ = conn.execute("ALTER TABLE history ADD COLUMN user_id TEXT", []);
        let _ = conn.execute("ALTER TABLE history ADD COLUMN agents_json TEXT", []);
        let _ = conn.execute("ALTER TABLE history ADD COLUMN predicted_product TEXT", []);
        let _ = conn.execute("ALTER TABLE history ADD COLUMN text_len INTEGER", []);

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                token      TEXT PRIMARY KEY,
                user_id    TEXT NOT NULL,
                email      TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                expires_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS user_profiles (
                user_id        TEXT PRIMARY KEY,
                profile_json   TEXT NOT NULL,
                analyzed_count INTEGER NOT NULL,
                model_version  TEXT NOT NULL,
                updated_at     TEXT NOT NULL
            );
            "#,
        )
        .map_err(|e| e.to_string())
    }

    /// Eski JSON dosyalarını (varsa) bir kez içe aktarır. Tablolar boş
    /// değilse hiçbir şey yapmaz; kaynak dosyalara dokunulmaz.
    pub fn migrate_from_json_files(&self) {
        let user_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))
                .unwrap_or(0)
        };
        if user_count == 0 {
            if let Ok(content) = std::fs::read_to_string("users.json") {
                if let Ok(users) = serde_json::from_str::<Vec<User>>(&content) {
                    let mut imported = 0;
                    for u in &users {
                        if self.insert_user(u).is_ok() {
                            imported += 1;
                        }
                    }
                    tracing::info!(imported, "users.json içe aktarıldı (dosya korunuyor)");
                }
            }
        }

        let history_count: i64 = {
            let conn = self.conn.lock().unwrap();
            conn.query_row("SELECT COUNT(*) FROM history", [], |r| r.get(0))
                .unwrap_or(0)
        };
        if history_count == 0 {
            if let Ok(content) = std::fs::read_to_string("history.jsonl") {
                let mut imported = 0;
                for line in content.lines() {
                    if let Ok(entry) = serde_json::from_str::<HistoryEntry>(line) {
                        self.insert_history(&entry);
                        imported += 1;
                    }
                }
                tracing::info!(imported, "history.jsonl içe aktarıldı (dosya korunuyor)");
            }
        }
    }

    // ===== Kullanıcılar =====

    pub fn user_by_email(&self, email: &str) -> Option<User> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, email, password_hash, created_at, verified FROM users WHERE email = ?1",
            params![email],
            |r| {
                Ok(User {
                    id: r.get(0)?,
                    email: r.get(1)?,
                    password_hash: r.get(2)?,
                    created_at: r.get(3)?,
                    verified: r.get::<_, i64>(4)? != 0,
                })
            },
        )
        .ok()
    }

    /// UNIQUE(email) ihlalinde Err döner (yarış durumunda çifte kayıt imkânsız).
    pub fn insert_user(&self, user: &User) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO users (id, email, password_hash, created_at, verified) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![user.id, user.email, user.password_hash, user.created_at, user.verified as i64],
        )
        .map(|_| ())
        .map_err(|e| e.to_string())
    }

    /// Doğrulandı olarak işaretler; güncel kullanıcıyı döner.
    pub fn set_verified(&self, email: &str) -> Option<User> {
        {
            let conn = self.conn.lock().unwrap();
            conn.execute("UPDATE users SET verified = 1 WHERE email = ?1", params![email])
                .ok()?;
        }
        self.user_by_email(email)
    }

    pub fn update_password(&self, email: &str, password_hash: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        matches!(
            conn.execute(
                "UPDATE users SET password_hash = ?1 WHERE email = ?2",
                params![password_hash, email],
            ),
            Ok(n) if n > 0
        )
    }

    // ===== Geçmiş =====

    pub fn insert_history(&self, entry: &HistoryEntry) {
        let conn = self.conn.lock().unwrap();
        // Ajan kararları tek JSON sütununda: 6 satır yerine 1 satır, ve
        // ajan listesi değişirse şema göçü gerekmez.
        let agents_json = entry
            .agents
            .as_ref()
            .and_then(|a| serde_json::to_string(a).ok());
        let _ = conn.execute(
            "INSERT INTO history (timestamp, client_id, text_preview, is_manipulated, dominant_manipulation, genel_sonuc, lang, user_id, agents_json, predicted_product, text_len)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                entry.timestamp,
                entry.client_id,
                entry.text_preview,
                entry.is_manipulated as i64,
                entry.dominant_manipulation,
                entry.genel_sonuc,
                entry.lang,
                entry.user_id,
                agents_json,
                entry.predicted_product,
                entry.text_len
            ],
        );
    }

    /// history satırından HistoryEntry kurar. Sütun sırası SELECT'lerde
    /// ortaktır: timestamp, client_id, text_preview, is_manipulated,
    /// dominant_manipulation, genel_sonuc, lang, user_id, agents_json,
    /// predicted_product, text_len — `offset` ilk sütunun indeksidir.
    fn row_to_entry(r: &rusqlite::Row, offset: usize) -> rusqlite::Result<HistoryEntry> {
        let agents_json: Option<String> = r.get(offset + 8)?;
        Ok(HistoryEntry {
            timestamp: r.get(offset)?,
            client_id: r.get(offset + 1)?,
            text_preview: r.get(offset + 2)?,
            is_manipulated: r.get::<_, i64>(offset + 3)? != 0,
            dominant_manipulation: r.get(offset + 4)?,
            genel_sonuc: r.get(offset + 5)?,
            lang: r.get(offset + 6)?,
            user_id: r.get(offset + 7)?,
            agents: agents_json
                .as_deref()
                .and_then(|j| serde_json::from_str::<Vec<AgentVerdict>>(j).ok()),
            predicted_product: r.get(offset + 9)?,
            text_len: r.get(offset + 10)?,
        })
    }

    /// En yeni kayıt önce, en fazla `limit` kayıt. Satır id'leri de döner:
    /// çeviri sonrası özetin kalıcı güncellenmesi (update_history_summary) için.
    pub fn history_for_client(&self, client_id: &str, limit: i64) -> Vec<(i64, HistoryEntry)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = match conn.prepare(
            "SELECT id, timestamp, client_id, text_preview, is_manipulated, dominant_manipulation, genel_sonuc, lang, user_id, agents_json, predicted_product, text_len
             FROM history WHERE client_id = ?1 ORDER BY id DESC LIMIT ?2",
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map(params![client_id, limit], |r| {
            Ok((r.get::<_, i64>(0)?, Self::row_to_entry(r, 1)?))
        });
        match rows {
            Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Çevrilen özeti kalıcı yazar: aynı kayıt bir daha Ollama'ya gitmez.
    /// (Dil tutarlılığı düzeltmesi — geçmiş her açılışta yeniden çevrilmesin.)
    pub fn update_history_summary(&self, id: i64, genel_sonuc: &str, lang: &str) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "UPDATE history SET genel_sonuc = ?1, lang = ?2 WHERE id = ?3",
            params![genel_sonuc, lang, id],
        );
    }

    /// Veri seti dışa aktarımı için TÜM geçmiş, en eski kayıt önce.
    /// Kullanıcı ayrımı `user_id` ile yapılır; eski kayıtlarda bu alan boşsa
    /// `users` tablosundan e-postayla çözülür (çözülemezse None kalır).
    pub fn history_for_export(&self) -> Vec<HistoryEntry> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = match conn.prepare(
            "SELECT h.timestamp, h.client_id, h.text_preview, h.is_manipulated, h.dominant_manipulation, h.genel_sonuc, h.lang,
                    COALESCE(h.user_id, u.id) AS user_id, h.agents_json, h.predicted_product, h.text_len
             FROM history h LEFT JOIN users u ON u.email = h.client_id
             ORDER BY h.id ASC",
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map([], |r| Self::row_to_entry(r, 0));
        match rows {
            Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    // ===== Kullanıcı profilleri =====

    /// Profili yazar veya günceller (user_id birincil anahtar).
    pub fn upsert_profile(&self, profile: &UserProfile) {
        let Ok(profile_json) = serde_json::to_string(profile) else {
            return;
        };
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT INTO user_profiles (user_id, profile_json, analyzed_count, model_version, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(user_id) DO UPDATE SET
                profile_json = excluded.profile_json,
                analyzed_count = excluded.analyzed_count,
                model_version = excluded.model_version,
                updated_at = excluded.updated_at",
            params![
                profile.user_id,
                profile_json,
                profile.stats.total,
                profile.model_version,
                profile.updated_at
            ],
        );
    }

    pub fn profile_for_user(&self, user_id: &str) -> Option<UserProfile> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT profile_json FROM user_profiles WHERE user_id = ?1",
            params![user_id],
            |r| r.get::<_, String>(0),
        )
        .ok()
        .and_then(|j| serde_json::from_str(&j).ok())
    }

    /// Kullanıcı profilini siler (KVKK: kullanıcı kendi profilini kaldırabilir).
    pub fn delete_profile(&self, user_id: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM user_profiles WHERE user_id = ?1", params![user_id])
            .map(|n| n > 0)
            .unwrap_or(false)
    }

    // ===== Oturumlar =====

    pub fn create_session(&self, token: &str, user_id: &str, email: &str, now: i64, expires_at: i64) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute(
            "INSERT INTO sessions (token, user_id, email, created_at, expires_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![token, user_id, email, now, expires_at],
        );
        // Süresi dolan oturumları fırsattan temizle
        let _ = conn.execute("DELETE FROM sessions WHERE expires_at < ?1", params![now]);
    }

    /// Geçerli (süresi dolmamış) oturumu döner; dolmuşsa siler.
    pub fn session_by_token(&self, token: &str, now: i64) -> Option<Session> {
        let conn = self.conn.lock().unwrap();
        let session = conn
            .query_row(
                "SELECT user_id, email, expires_at FROM sessions WHERE token = ?1",
                params![token],
                |r| {
                    Ok(Session {
                        user_id: r.get(0)?,
                        email: r.get(1)?,
                        expires_at: r.get(2)?,
                    })
                },
            )
            .ok()?;
        if session.expires_at < now {
            let _ = conn.execute("DELETE FROM sessions WHERE token = ?1", params![token]);
            return None;
        }
        Some(session)
    }

    /// Şifre değişince kullanıcının tüm oturumlarını düşür (çalınmış token ölür).
    pub fn delete_sessions_for_user(&self, email: &str) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute("DELETE FROM sessions WHERE email = ?1", params![email]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_user(email: &str) -> User {
        User {
            id: format!("id-{}", email),
            email: email.to_string(),
            password_hash: "hash".to_string(),
            created_at: "2026-01-01T00:00:00+03:00".to_string(),
            verified: false,
        }
    }

    #[test]
    fn user_roundtrip_and_unique_email() {
        let db = Db::open_in_memory();
        db.insert_user(&sample_user("a@b.com")).unwrap();
        // Aynı e-posta ikinci kez eklenemez
        assert!(db.insert_user(&sample_user("a@b.com")).is_err());

        let u = db.user_by_email("a@b.com").unwrap();
        assert!(!u.verified);

        let u = db.set_verified("a@b.com").unwrap();
        assert!(u.verified);

        assert!(db.update_password("a@b.com", "newhash"));
        assert_eq!(db.user_by_email("a@b.com").unwrap().password_hash, "newhash");
        assert!(!db.update_password("yok@b.com", "x"));
    }

    #[test]
    fn history_isolated_per_client_and_ordered() {
        let db = Db::open_in_memory();
        for i in 0..3 {
            db.insert_history(&HistoryEntry {
                timestamp: format!("t{}", i),
                client_id: "user1".to_string(),
                text_preview: format!("metin {}", i),
                is_manipulated: i % 2 == 0,
                dominant_manipulation: "Dilsel".to_string(),
                genel_sonuc: "sonuç".to_string(),
                lang: Some("tr".to_string()),
                user_id: None,
                agents: None,
                predicted_product: None,
                text_len: None,
            });
        }
        db.insert_history(&HistoryEntry {
            timestamp: "tx".to_string(),
            client_id: "user2".to_string(),
            text_preview: "başka".to_string(),
            is_manipulated: false,
            dominant_manipulation: "Yok".to_string(),
            genel_sonuc: "temiz".to_string(),
            lang: None,
            user_id: None,
            agents: None,
            predicted_product: None,
            text_len: None,
        });

        let h = db.history_for_client("user1", 100);
        assert_eq!(h.len(), 3);
        // En yeni önce
        assert_eq!(h[0].1.timestamp, "t2");
        // Başka kullanıcının kaydı sızmaz
        assert!(h.iter().all(|(_, e)| e.client_id == "user1"));
        // lang alanı korunur
        assert_eq!(h[0].1.lang.as_deref(), Some("tr"));

        let h2 = db.history_for_client("user1", 2);
        assert_eq!(h2.len(), 2);

        // lang'sız (eski) kayıt None döner
        let h3 = db.history_for_client("user2", 10);
        assert_eq!(h3[0].1.lang, None);
    }

    #[test]
    fn history_summary_update_persists_translation() {
        let db = Db::open_in_memory();
        db.insert_history(&HistoryEntry {
            timestamp: "t0".to_string(),
            client_id: "user1".to_string(),
            text_preview: "önizleme".to_string(),
            is_manipulated: true,
            dominant_manipulation: "Dilsel".to_string(),
            genel_sonuc: "Türkçe özet".to_string(),
            lang: Some("tr".to_string()),
            user_id: None,
            agents: None,
            predicted_product: None,
            text_len: None,
        });

        let (id, _) = db.history_for_client("user1", 1)[0].clone();
        db.update_history_summary(id, "English summary", "en");

        let (_, updated) = db.history_for_client("user1", 1)[0].clone();
        assert_eq!(updated.genel_sonuc, "English summary");
        assert_eq!(updated.lang.as_deref(), Some("en"));
    }

    #[test]
    fn session_lifecycle() {
        let db = Db::open_in_memory();
        let now = 1_000_000;
        db.create_session("tok1", "uid", "a@b.com", now, now + 100);

        let s = db.session_by_token("tok1", now).unwrap();
        assert_eq!(s.email, "a@b.com");

        // Süresi dolunca None döner ve silinir
        assert!(db.session_by_token("tok1", now + 101).is_none());
        assert!(db.session_by_token("tok1", now).is_none());

        // Şifre sıfırlanınca tüm oturumlar düşer
        db.create_session("tok2", "uid", "a@b.com", now, now + 100);
        db.delete_sessions_for_user("a@b.com");
        assert!(db.session_by_token("tok2", now).is_none());
    }
}
