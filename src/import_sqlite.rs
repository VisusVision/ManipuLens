//! Tek seferlik göç aracı: eski `manipulens.db` (SQLite) dosyasını okuyup
//! PostgreSQL'e aktarır. Kaynak dosyaya dokunulmaz — geri dönüş her zaman
//! mümkün kalsın.
//!
//! Kullanım:
//! ```text
//! cargo run -- --import-sqlite manipulens.db
//! ```
//!
//! Hedef tablo doluysa o tablo atlanır: komut iki kez çalıştırılırsa geçmiş
//! ikiye katlanmaz. Oturumlar da taşınır, böylece göçten sonra kimse yeniden
//! giriş yapmak zorunda kalmaz.

use crate::db::Db;
use crate::types::{HistoryEntry, User, UserProfile};
use rusqlite::Connection;

/// Göç sonucunun satır sayıları.
pub struct ImportReport {
    pub users: usize,
    pub history: usize,
    pub profiles: usize,
    pub sessions: usize,
    pub skipped_tables: Vec<&'static str>,
}

struct SessionRow {
    token: String,
    user_id: String,
    email: String,
    created_at: i64,
    expires_at: i64,
}

pub async fn import_from_sqlite(db: &Db, path: &str) -> Result<ImportReport, String> {
    let conn = Connection::open(path).map_err(|e| format!("{path} açılamadı: {e}"))?;

    let users = read_users(&conn)?;
    let history = read_history(&conn)?;
    let profiles = read_profiles(&conn)?;
    let sessions = read_sessions(&conn)?;

    let mut report = ImportReport {
        users: 0,
        history: 0,
        profiles: 0,
        sessions: 0,
        skipped_tables: Vec::new(),
    };

    if db.is_table_empty("users").await {
        for u in &users {
            if db.insert_user(u).await.is_ok() {
                report.users += 1;
            }
        }
    } else {
        report.skipped_tables.push("users");
    }

    if db.is_table_empty("history").await {
        for h in &history {
            db.insert_history(h).await;
            report.history += 1;
        }
    } else {
        report.skipped_tables.push("history");
    }

    if db.is_table_empty("user_profiles").await {
        for p in &profiles {
            db.upsert_profile(p).await;
            report.profiles += 1;
        }
    } else {
        report.skipped_tables.push("user_profiles");
    }

    if db.is_table_empty("sessions").await {
        for s in &sessions {
            db.create_session(&s.token, &s.user_id, &s.email, s.created_at, s.expires_at)
                .await;
            report.sessions += 1;
        }
    } else {
        report.skipped_tables.push("sessions");
    }

    Ok(report)
}

fn read_users(conn: &Connection) -> Result<Vec<User>, String> {
    let mut stmt = conn
        .prepare("SELECT id, email, password_hash, created_at, verified FROM users")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(User {
                id: r.get(0)?,
                email: r.get(1)?,
                password_hash: r.get(2)?,
                created_at: r.get(3)?,
                verified: r.get::<_, i64>(4)? != 0,
            })
        })
        .map_err(|e| e.to_string())?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

fn read_history(conn: &Connection) -> Result<Vec<HistoryEntry>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT timestamp, client_id, text_preview, is_manipulated, dominant_manipulation,
                    genel_sonuc, lang, user_id, agents_json, predicted_product, text_len
             FROM history ORDER BY id ASC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            let agents_json: Option<String> = r.get(8)?;
            Ok(HistoryEntry {
                timestamp: r.get(0)?,
                client_id: r.get(1)?,
                text_preview: r.get(2)?,
                is_manipulated: r.get::<_, i64>(3)? != 0,
                dominant_manipulation: r.get(4)?,
                genel_sonuc: r.get(5)?,
                lang: r.get(6)?,
                user_id: r.get(7)?,
                agents: agents_json.as_deref().and_then(|j| serde_json::from_str(j).ok()),
                predicted_product: r.get(9)?,
                text_len: r.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

fn read_profiles(conn: &Connection) -> Result<Vec<UserProfile>, String> {
    let mut stmt = conn
        .prepare("SELECT profile_json FROM user_profiles")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(|e| e.to_string())?;
    Ok(rows
        .filter_map(|r| r.ok())
        .filter_map(|j| serde_json::from_str::<UserProfile>(&j).ok())
        .collect())
}

fn read_sessions(conn: &Connection) -> Result<Vec<SessionRow>, String> {
    let mut stmt = conn
        .prepare("SELECT token, user_id, email, created_at, expires_at FROM sessions")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(SessionRow {
                token: r.get(0)?,
                user_id: r.get(1)?,
                email: r.get(2)?,
                created_at: r.get(3)?,
                expires_at: r.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}
