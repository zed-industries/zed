//! Convergio Database Access
//!
//! This module provides direct access to Convergio's SQLite database
//! for reading conversation history and synchronization with CLI.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use sqlez::connection::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

type ArcStr = Arc<str>;

/// Path to convergio database
const CONVERGIO_DB_PATH: &str =
    "Library/Containers/com.convergio.app/Data/data/convergio.db";

/// Message type enum matching convergio's schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum MessageType {
    System = 0,
    User = 1,
    Assistant = 2,
    Tool = 3,
}

impl TryFrom<i32> for MessageType {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(MessageType::System),
            1 => Ok(MessageType::User),
            2 => Ok(MessageType::Assistant),
            3 => Ok(MessageType::Tool),
            _ => Err(anyhow!("Invalid message type: {}", value)),
        }
    }
}

/// A chat message from convergio database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: i64,
    pub session_id: String,
    pub message_type: MessageType,
    pub sender_name: Option<String>,
    pub content: String,
    pub metadata_json: Option<String>,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cost_usd: f64,
    pub created_at: DateTime<Utc>,
}

/// A session from convergio database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_name: Option<String>,
    pub total_cost: f64,
    pub total_messages: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

/// Session metadata with agent info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub session: Session,
    pub agent_name: Option<String>,
    pub last_message_preview: Option<String>,
}

/// Convergio database connection
pub struct ConvergioDb {
    connection: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl ConvergioDb {
    /// Open the convergio database
    pub fn open() -> Result<Self> {
        let home = std::env::var("HOME")
            .map_err(|_| anyhow!("HOME environment variable not set"))?;
        let db_path = PathBuf::from(home).join(CONVERGIO_DB_PATH);

        if !db_path.exists() {
            return Err(anyhow!(
                "Convergio database not found at: {}",
                db_path.display()
            ));
        }

        let connection = Connection::open_file(&db_path.to_string_lossy());

        // Enable WAL mode for better concurrent access
        let _ = connection.exec("PRAGMA journal_mode=WAL;");

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            db_path,
        })
    }

    /// Get database path
    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    /// Get all sessions, optionally filtered by agent name
    pub fn sessions(&self, agent_name: Option<&str>) -> Result<Vec<SessionMetadata>> {
        let conn = self.connection.lock()
            .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

        // Query sessions with the latest message info
        let query = if let Some(agent) = agent_name {
            format!(
                r#"
                SELECT DISTINCT s.id, s.user_name, s.total_cost, s.total_messages,
                       s.started_at, s.ended_at,
                       m.sender_name,
                       (SELECT content FROM messages WHERE session_id = s.id ORDER BY id DESC LIMIT 1) as last_content
                FROM sessions s
                LEFT JOIN messages m ON m.session_id = s.id
                    AND m.sender_name LIKE '%{}%'
                WHERE m.id IS NOT NULL
                GROUP BY s.id
                ORDER BY s.started_at DESC
                LIMIT 100
                "#,
                agent
            )
        } else {
            r#"
            SELECT s.id, s.user_name, s.total_cost, s.total_messages,
                   s.started_at, s.ended_at,
                   (SELECT sender_name FROM messages WHERE session_id = s.id AND sender_name IS NOT NULL LIMIT 1) as agent_name,
                   (SELECT content FROM messages WHERE session_id = s.id ORDER BY id DESC LIMIT 1) as last_content
            FROM sessions s
            ORDER BY s.started_at DESC
            LIMIT 100
            "#.to_string()
        };

        let mut stmt = conn.select_bound::<(), (
            String,           // id
            Option<String>,   // user_name
            f64,              // total_cost
            i64,              // total_messages
            String,           // started_at
            Option<String>,   // ended_at
            Option<String>,   // agent_name
            Option<String>,   // last_content
        )>(&query)?;

        let rows = stmt(())?;
        let mut sessions = Vec::new();

        for (id, user_name, total_cost, total_messages, started_at, ended_at, agent_name, last_content) in rows {
            let started_at = parse_datetime(&started_at)?;
            let ended_at = ended_at.map(|s| parse_datetime(&s)).transpose()?;

            sessions.push(SessionMetadata {
                session: Session {
                    id,
                    user_name,
                    total_cost,
                    total_messages,
                    started_at,
                    ended_at,
                },
                agent_name,
                last_message_preview: last_content.map(|c| truncate_string(&c, 100)),
            });
        }

        Ok(sessions)
    }

    /// Get messages for a specific session
    pub fn messages_for_session(&self, session_id: &str) -> Result<Vec<ChatMessage>> {
        let conn = self.connection.lock()
            .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

        let query = r#"
            SELECT id, session_id, type, sender_name, content,
                   metadata_json, input_tokens, output_tokens, cost_usd, created_at
            FROM messages
            WHERE session_id = ?
            ORDER BY id ASC
        "#;

        let mut stmt = conn.select_bound::<ArcStr, (
            i64,              // id
            String,           // session_id
            i32,              // type
            Option<String>,   // sender_name
            String,           // content
            Option<String>,   // metadata_json
            i64,              // input_tokens
            i64,              // output_tokens
            f64,              // cost_usd
            String,           // created_at
        )>(query)?;

        let rows = stmt(ArcStr::from(session_id))?;
        let mut messages = Vec::new();

        for (id, session_id, msg_type, sender_name, content, metadata_json, input_tokens, output_tokens, cost_usd, created_at) in rows {
            messages.push(ChatMessage {
                id,
                session_id,
                message_type: MessageType::try_from(msg_type)?,
                sender_name,
                content,
                metadata_json,
                input_tokens,
                output_tokens,
                cost_usd,
                created_at: parse_datetime(&created_at)?,
            });
        }

        Ok(messages)
    }

    /// Get the most recent session for a specific agent
    pub fn latest_session_for_agent(&self, agent_name: &str) -> Result<Option<SessionMetadata>> {
        let sessions = self.sessions(Some(agent_name))?;
        Ok(sessions.into_iter().next())
    }

    /// Get message count for a session (useful for detecting changes)
    pub fn message_count(&self, session_id: &str) -> Result<i64> {
        let conn = self.connection.lock()
            .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

        let query = "SELECT COUNT(*) FROM messages WHERE session_id = ?";
        let mut stmt = conn.select_bound::<ArcStr, i64>(query)?;
        let rows = stmt(ArcStr::from(session_id))?;

        Ok(rows.into_iter().next().unwrap_or(0))
    }

    /// Get latest message timestamp for a session
    pub fn latest_message_time(&self, session_id: &str) -> Result<Option<DateTime<Utc>>> {
        let conn = self.connection.lock()
            .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

        let query = "SELECT created_at FROM messages WHERE session_id = ? ORDER BY id DESC LIMIT 1";
        let mut stmt = conn.select_bound::<ArcStr, String>(query)?;
        let rows = stmt(ArcStr::from(session_id))?;

        if let Some(created_at) = rows.into_iter().next() {
            Ok(Some(parse_datetime(&created_at)?))
        } else {
            Ok(None)
        }
    }

    /// Create a new session for an agent
    pub fn create_session(&self, agent_name: &str) -> Result<String> {
        let conn = self.connection.lock()
            .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

        let session_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let query = r#"
            INSERT INTO sessions (id, user_name, total_cost, total_messages, started_at)
            VALUES (?, ?, 0.0, 0, ?)
        "#;

        conn.exec_bound::<(ArcStr, ArcStr, ArcStr)>(query)?((
            ArcStr::from(session_id.as_str()),
            ArcStr::from(agent_name),
            ArcStr::from(now.as_str()),
        ))?;

        Ok(session_id)
    }

    /// Insert a user message into a session
    pub fn insert_user_message(&self, session_id: &str, content: &str) -> Result<i64> {
        let conn = self.connection.lock()
            .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let query = r#"
            INSERT INTO messages (session_id, type, sender_name, content, input_tokens, output_tokens, cost_usd, created_at)
            VALUES (?, 1, 'You', ?, 0, 0, 0.0, ?)
        "#;

        conn.exec_bound::<(ArcStr, ArcStr, ArcStr)>(query)?((
            ArcStr::from(session_id),
            ArcStr::from(content),
            ArcStr::from(now.as_str()),
        ))?;

        // Get the last inserted row ID
        let id_query = "SELECT last_insert_rowid()";
        let mut stmt = conn.select_bound::<(), i64>(id_query)?;
        let rows = stmt(())?;

        Ok(rows.into_iter().next().unwrap_or(0))
    }

    /// Get or create session for an agent
    pub fn get_or_create_session(&self, agent_name: &str) -> Result<String> {
        // Try to get existing session
        if let Some(session) = self.latest_session_for_agent(agent_name)? {
            return Ok(session.session.id);
        }

        // Create new session
        self.create_session(agent_name)
    }

    /// Insert an assistant message into a session
    pub fn insert_assistant_message(
        &self,
        session_id: &str,
        sender_name: &str,
        content: &str,
        input_tokens: i64,
        output_tokens: i64,
        cost_usd: f64,
    ) -> Result<i64> {
        let conn = self.connection.lock()
            .map_err(|e| anyhow!("Failed to lock connection: {}", e))?;

        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let query = r#"
            INSERT INTO messages (session_id, type, sender_name, content, input_tokens, output_tokens, cost_usd, created_at)
            VALUES (?, 2, ?, ?, ?, ?, ?, ?)
        "#;

        conn.exec_bound::<(ArcStr, ArcStr, ArcStr, i64, i64, f64, ArcStr)>(query)?((
            ArcStr::from(session_id),
            ArcStr::from(sender_name),
            ArcStr::from(content),
            input_tokens,
            output_tokens,
            cost_usd,
            ArcStr::from(now.as_str()),
        ))?;

        // Get the last inserted row ID
        let id_query = "SELECT last_insert_rowid()";
        let mut stmt = conn.select_bound::<(), i64>(id_query)?;
        let rows = stmt(())?;

        // Update session stats
        let update_query = r#"
            UPDATE sessions
            SET total_messages = total_messages + 1,
                total_cost = total_cost + ?
            WHERE id = ?
        "#;
        let _ = conn.exec_bound::<(f64, ArcStr)>(update_query)?((
            cost_usd,
            ArcStr::from(session_id),
        ));

        Ok(rows.into_iter().next().unwrap_or(0))
    }
}

/// Parse datetime string from SQLite
fn parse_datetime(s: &str) -> Result<DateTime<Utc>> {
    // Try various formats that SQLite might use
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // SQLite default format: "YYYY-MM-DD HH:MM:SS"
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Ok(Utc.from_utc_datetime(&naive));
    }

    // Try with microseconds
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
        return Ok(Utc.from_utc_datetime(&naive));
    }

    Err(anyhow!("Failed to parse datetime: {}", s))
}

/// Truncate string to max length with ellipsis
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_message_type_conversion() {
        assert_eq!(MessageType::try_from(0).unwrap(), MessageType::System);
        assert_eq!(MessageType::try_from(1).unwrap(), MessageType::User);
        assert_eq!(MessageType::try_from(2).unwrap(), MessageType::Assistant);
        assert_eq!(MessageType::try_from(3).unwrap(), MessageType::Tool);
        assert!(MessageType::try_from(4).is_err());
    }

    #[test]
    fn test_parse_datetime() {
        let dt = parse_datetime("2025-12-21 14:30:00").unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 21);
    }
}
