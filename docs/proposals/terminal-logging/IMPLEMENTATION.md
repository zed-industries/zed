# Implementation Guide: Persistent Terminal Logging for Zed

This guide provides step-by-step instructions for implementing the terminal logging feature described in DESIGN.md.

## Prerequisites

- Familiarity with Rust and the Zed codebase
- Understanding of async programming in Rust (smol, futures)
- Knowledge of GPUI (Zed's UI framework)
- Rust toolchain installed (nightly recommended)
- Git for version control

---

## Phase 1: Core Logging (Week 1-2)

### 1.1 Create the Crate

```bash
cd crates
cargo new terminal-logging
```

Edit `crates/terminal-logging/Cargo.toml`:

```toml
[package]
name = "terminal-logging"
version = "0.1.0"
edition.workspace = true
publish.workspace = true

[dependencies]
anyhow.workspace = true
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
directories.workspace = true
log.workspace = true
regex.workspace = true
collections.workspace = true
gpui.workspace = true
util.workspace = true
flate2 = "1.0"
```

### 1.2 Implement TerminalLogger

Create `crates/terminal-logging/src/logger.rs`:

```rust
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
    u64,
};

use anyhow::{Context, Result};
use chrono::Utc;
use collections::HashMap;
use gpui::AsyncAppContext;
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::{
    config::LoggingConfig,
    storage::LogMetadata,
    redact::{Redactor, RedactionConfig},
};

#[derive(Debug, Clone)]
pub struct TerminalId(pub uuid::Uuid);

impl TerminalId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMetadata {
    pub id: String,
    pub terminal_id: TerminalId,
    pub project_path: Option<PathBuf>,
    pub created_at: chrono::DateTime<Utc>,
    pub closed_at: Option<chrono::DateTime<Utc>>,
    pub shell: String,
    pub cwd: Option<PathBuf>,
    pub exit_code: Option<i32>,
    pub task_label: Option<String>,
    pub size_bytes: u64,
    pub line_count: u64,
    pub is_compressed: bool,
    pub path: PathBuf,
}

pub struct TerminalLogger {
    id: TerminalId,
    config: LoggingConfig,
    metadata: LogMetadata,
    file: BufWriter<File>,
    buffer: Vec<u8>,
    buffer_line_count: usize,
    bytes_written: u64,
    last_flush: Instant,
    redactor: Redactor,
    start_time: chrono::DateTime<Utc>,
}

impl TerminalLogger {
    pub fn new(
        config: LoggingConfig,
        terminal_id: TerminalId,
        project_path: Option<PathBuf>,
        shell: &str,
        cwd: Option<PathBuf>,
    ) -> Result<Self> {
        let storage_dir = config.storage_path.clone();
        std::fs::create_dir_all(&storage_dir).context("Failed to create storage directory")?;

        let active_dir = storage_dir.join("active");
        std::fs::create_dir_all(&active_dir).context("Failed to create active directory")?;

        let filename = format!("terminal-{}.log", terminal_id.0);
        let log_path = active_dir.join(filename);

        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&log_path)
            .context("Failed to create log file")?;

        let metadata = LogMetadata {
            id: terminal_id.0.to_string(),
            terminal_id: terminal_id.clone(),
            project_path: project_path.clone(),
            created_at: Utc::now(),
            closed_at: None,
            shell: shell.to_string(),
            cwd: cwd.clone(),
            exit_code: None,
            task_label: None,
            size_bytes: 0,
            line_count: 0,
            is_compressed: false,
            path: log_path.clone(),
        };

        // Write initial metadata as JSON sidecar
        let meta_path = log_path.with_extension("meta.json");
        let meta_json = serde_json::to_string_pretty(&metadata)?;
        std::fs::write(meta_path, meta_json).context("Failed to write metadata")?;

        Ok(Self {
            id: terminal_id,
            config,
            metadata,
            file: BufWriter::new(file),
            buffer: Vec::new(),
            buffer_line_count: 0,
            bytes_written: 0,
            last_flush: Instant::now(),
            redactor: Redactor::new(&config.redact_patterns),
            start_time: Utc::now(),
        })
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
        // Apply redaction if patterns are configured
        let mut data_to_write = if self.config.redact_patterns.is_empty() {
            bytes.to_vec()
        } else {
            let text = String::from_utf8_lossy(bytes);
            let redacted = self.redactor.redact(&text);
            redacted.into_bytes()
        };

        // Accumulate in buffer
        self.buffer.append(&mut data_to_write);
        self.bytes_written += data_to_write.len() as u64;

        // Count lines (approximate)
        self.buffer_line_count += data_to_write.iter().filter(|&&b| b == b'\n').count();

        // Check if we should flush
        let should_flush = self.buffer.len() >= 4096
            || self.buffer_line_count >= self.config.buffer_lines
            || self.last_flush.elapsed() >= Duration::from_millis(self.config.buffer_ms);

        if should_flush {
            self.flush()?;
        }

        // Check size limits
        if self.bytes_written >= self.config.per_terminal_limit_bytes() {
            // Signal that rotation is needed
            return Err(anyhow::anyhow!("Terminal log size limit reached"));
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        if !self.buffer.is_empty() {
            self.file.write_all(&self.buffer)?;
            self.file.flush()?;
            self.buffer.clear();
            self.buffer_line_count = 0;
            self.last_flush = Instant::now();

            // Update metadata
            self.metadata.size_bytes = self.bytes_written;
            self.metadata.line_count += 1; // Approximate

            // Periodically update metadata file (every 100 flushes or 10 MB)
            if self.bytes_written % (100 * 4096) == 0 || self.bytes_written > 10_000_000 {
                self.update_metadata_file()?;
            }
        }

        Ok(())
    }

    fn update_metadata_file(&self) -> Result<()> {
        let meta_path = self.metadata.path.with_extension("meta.json");
        let meta_json = serde_json::to_string_pretty(&self.metadata)?;
        std::fs::write(meta_path, meta_json).context("Failed to update metadata file")?;
        Ok(())
    }

    pub fn close(mut self) -> Result<LogMetadata> {
        // Flush any remaining buffer
        self.flush()?;

        // Finalize metadata
        self.metadata.closed_at = Some(Utc::now());
        self.metadata.size_bytes = self.bytes_written;

        // Write final metadata
        let meta_path = self.metadata.path.with_extension("meta.json");
        let meta_json = serde_json::to_string_pretty(&self.metadata)?;
        std::fs::write(meta_path, meta_json).context("Failed to write final metadata")?;

        // Close file
        self.file.flush()?;

        Ok(self.metadata.clone())
    }

    pub fn log_path(&self) -> &PathBuf {
        &self.metadata.path
    }

    pub fn metadata(&self) -> &LogMetadata {
        &self.metadata
    }

    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    pub fn terminal_id(&self) -> &TerminalId {
        &self.id
    }

    pub fn should_rotate(&self) -> bool {
        self.bytes_written >= self.config.per_terminal_limit_bytes()
    }
}

impl Drop for TerminalLogger {
    fn drop(&mut self) {
        // Best effort to flush on drop
        let _ = self.flush();
    }
}
```

### 1.3 Implement Config

Create `crates/terminal-logging/src/config.rs`:

```rust
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub storage_path: PathBuf,
    pub per_terminal_limit_mb: u64,
    pub per_project_limit_mb: u64,
    pub global_limit_mb: u64,
    pub retention_days: u32,
    pub auto_compress: bool,
    pub compress_after_mb: u64,
    pub buffer_ms: u64,
    pub buffer_lines: usize,
    pub enable_search_index: bool,
    pub redact_patterns: Vec<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        let config_dir = directories::ProjectDirs::from("", "zed", "zed")
            .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("~/.config/zed/logs/terminal"))
            .canonicalize()
            .unwrap_or_else(|_| PathBuf::from("~/.config/zed/logs/terminal"));

        Self {
            enabled: true,
            storage_path: config_dir,
            per_terminal_limit_mb: 100,
            per_project_limit_mb: 1000,
            global_limit_mb: 10000,
            retention_days: 90,
            auto_compress: true,
            compress_after_mb: 10,
            buffer_ms: 100,
            buffer_lines: 1000,
            enable_search_index: true,
            redact_patterns: default_redact_patterns(),
        }
    }
}

impl LoggingConfig {
    pub fn per_terminal_limit_bytes(&self) -> u64 {
        self.per_terminal_limit_mb * 1024 * 1024
    }

    pub fn per_project_limit_bytes(&self) -> u64 {
        self.per_project_limit_mb * 1024 * 1024
    }

    pub fn global_limit_bytes(&self) -> u64 {
        self.global_limit_mb * 1024 * 1024
    }

    pub fn compress_after_bytes(&self) -> u64 {
        self.compress_after_mb * 1024 * 1024
    }
}

fn default_redact_patterns() -> Vec<String> {
    vec![
        "(?i)password\\s*=\\s*\\S+".to_string(),
        "(?i)token\\s*[:=]\\s*\\S+".to_string(),
        "(?i)secret\\s*[:=]\\s*\\S+".to_string(),
        "(?i)api[_-]?key\\s*[:=]\\s*\\S+".to_string(),
        "\\b[A-Za-z0-9+/]{40,}={0,2}\\b".to_string(), // Base64 blobs
        "\\b(?:sk|pk)_[A-Za-z0-9]{48,}\\b".to_string(), // Stripe-like keys
    ]
}
```

### 1.4 Implement Redaction

Create `crates/terminal-logging/src/redact.rs`:

```rust
use regex::Regex;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Redactor {
    patterns: Vec<Regex>,
}

impl Redactor {
    pub fn new(patterns: &[String]) -> Self {
        let patterns = patterns
            .iter()
            .filter_map(|pattern| {
                Regex::new(pattern)
                    .log_err()
                    .map(|re| re.to_owned())
            })
            .collect();

        Self { patterns }
    }

    pub fn redact(&self, text: &str) -> String {
        let mut result = text.to_string();

        for pattern in &self.patterns {
            let replacement = "[REDACTED]";
            result = pattern.replace_all(&result, replacement).to_string();
        }

        result
    }

    pub fn redact_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let text = String::from_utf8_lossy(bytes);
        let redacted = self.redact(&text);
        redacted.into_bytes()
    }
}
```

### 1.5 Add to Terminal Integration

Modify `crates/terminal/src/terminal.rs`:

1. Add terminal-logging dependency in `Cargo.toml`:
```toml
terminal-logging = { path = "../terminal-logging" }
```

2. In `Terminal` struct, add:
```rust
pub struct Terminal {
    // ... existing fields ...
    logger: Option<terminal_logging::TerminalLogger>,
    logger_errors: Vec<anyhow::Error>,
}
```

3. In `Terminal::new()` and `TerminalBuilder::new()`, initialize logger:
```rust
// After terminal is created
if let Ok(Some(logger)) = Self::init_logger(cx) {
    terminal.logger = Some(logger);
}
```

4. Add method:
```rust
impl Terminal {
    fn init_logger(cx: &mut Context<Self>) -> Result<Option<terminal_logging::TerminalLogger>> {
        let config = settings::SettingsStore::global(cx)
            .get::<LoggingSettings>()?
            .terminal_logging
            .clone();

        if !config.enabled {
            return Ok(None);
        }

        let terminal_id = terminal_logging::TerminalId::new();
        let project_path = cx.project_path().map(|p| p.to_path_buf());
        let shell = "unknown"; // TODO: get from shell settings
        let cwd = None; // TODO: get from terminal

        match terminal_logging::TerminalLogger::new(
            config,
            terminal_id,
            project_path,
            shell,
            cwd,
        ) {
            Ok(logger) => Ok(Some(logger)),
            Err(e) => {
                log::warn!("Failed to initialize terminal logger: {}", e);
                Ok(None)
            }
        }
    }

    pub fn write_to_pty(&self, input: impl Into<Cow<'static, [u8]>>) {
        let input = input.into();

        // Write to logger if enabled
        if let Some(logger) = &self.logger {
            if let Err(e) = logger.write(&input) {
                self.logger_errors.push(e.clone());
                log::warn!("Terminal logging error: {}", e);
            }
        }

        // ... existing PTY write logic ...
    }
}
```

### 1.6 Add Settings

Create `crates/terminal-logging/src/settings.rs`:

```rust
use gpui::{AppContext, Settings};
use serde::{Deserialize, Serialize};

use crate::config::LoggingConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub terminal_logging: LoggingConfig,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            terminal_logging: LoggingConfig::default(),
        }
    }
}

impl Settings for LoggingSettings {
    const NAME: &'static str = "terminal-logging";

    fn read(_: &AppContext) -> anyhow::Result<Self> {
        Ok(Self::default())
    }

    fn write(&self, _: &AppContext) -> anyhow::Result<()> {
        Ok(())
    }
}
```

### 1.7 Write Tests

Create `crates/terminal-logging/tests/integration_test.rs`:

```rust
use std::fs;

use tempfile::tempdir;
use terminal_logging::{LoggingConfig, TerminalLogger, TerminalId};

#[test]
fn test_terminal_logger_basic() {
    let temp_dir = tempdir().unwrap();
    let config = LoggingConfig {
        storage_path: temp_dir.path().to_path_buf(),
        per_terminal_limit_mb: 1,
        ..Default::default()
    };

    let terminal_id = TerminalId::new();
    let mut logger = TerminalLogger::new(
        config.clone(),
        terminal_id.clone(),
        None,
        "bash",
        None,
    ).unwrap();

    // Write some data
    logger.write(b"Hello, world!\n").unwrap();
    logger.write(b"Second line\n").unwrap();

    // Close logger
    let metadata = logger.close().unwrap();

    // Verify log file exists
    assert!(metadata.path.exists());
    assert!(metadata.path.with_extension("meta.json").exists());

    // Verify content
    let content = fs::read_to_string(&metadata.path).unwrap();
    assert!(content.contains("Hello, world!"));
    assert!(content.contains("Second line"));

    // Verify metadata
    let meta_path = metadata.path.with_extension("meta.json");
    let meta_content = fs::read_to_string(meta_path).unwrap();
    let meta: serde_json::Value = serde_json::from_str(&meta_content).unwrap();
    assert_eq!(meta["id"], terminal_id.0.to_string());
    assert_eq!(meta["shell"], "bash");
}
```

---

## Phase 2: Storage Management (Week 3)

### 2.1 Implement LogStorage

Create `crates/terminal-logging/src/storage.rs`:

```rust
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use sqlite::{Connection, OpenFlags};

use crate::{
    config::LoggingConfig,
    logger::{LogMetadata, TerminalId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredLog {
    pub metadata: LogMetadata,
    pub is_active: bool,
}

pub struct LogStorage {
    config: LoggingConfig,
    db_path: PathBuf,
    db: Connection,
}

impl LogStorage {
    pub fn new(config: LoggingConfig) -> Result<Self> {
        let storage_dir = config.storage_path.clone();
        fs::create_dir_all(&storage_dir).context("Failed to create storage directory")?;

        let db_path = storage_dir.join("logs.db");
        let db = Self::open_database(&db_path)?;

        Ok(Self {
            config,
            db_path,
            db,
        })
    }

    fn open_database(path: &Path) -> Result<Connection> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE;
        let conn = Connection::open_with_flags(path, flags)
            .context("Failed to open SQLite database")?;

        // Initialize schema
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS logs (
                id TEXT PRIMARY KEY,
                terminal_id TEXT NOT NULL,
                project_path TEXT,
                created_at TEXT NOT NULL,
                closed_at TEXT,
                shell TEXT NOT NULL,
                cwd TEXT,
                exit_code INTEGER,
                task_label TEXT,
                size_bytes INTEGER NOT NULL,
                line_count INTEGER NOT NULL,
                is_compressed BOOLEAN NOT NULL DEFAULT 0,
                path TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_logs_created_at ON logs(created_at);
            CREATE INDEX IF NOT EXISTS idx_logs_project_path ON logs(project_path);
            CREATE INDEX IF NOT EXISTS idx_logs_shell ON logs(shell);

            CREATE VIRTUAL TABLE IF NOT EXISTS log_lines_fts USING fts5(
                content,
                content=log_lines,
                content_rowid=rowid
            );

            CREATE TABLE IF NOT EXISTS log_lines (
                log_id TEXT NOT NULL,
                line_number INTEGER NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                FOREIGN KEY (log_id) REFERENCES logs(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_log_lines_log_id ON log_lines(log_id);
            "#,
        ).context("Failed to initialize database schema")?;

        Ok(conn)
    }

    pub fn register_log(&mut self, metadata: LogMetadata) -> Result<()> {
        self.db.execute(
            "INSERT INTO logs (id, terminal_id, project_path, created_at, closed_at, shell, cwd, exit_code, task_label, size_bytes, line_count, is_compressed, path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            sqlite::params![
                metadata.id,
                metadata.terminal_id.0.to_string(),
                metadata.project_path.as_ref().and_then(|p| p.to_str()),
                metadata.created_at.to_rfc3339(),
                metadata.closed_at.map(|t| t.to_rfc3339()),
                metadata.shell,
                metadata.cwd.as_ref().and_then(|p| p.to_str()),
                metadata.exit_code,
                metadata.task_label,
                metadata.size_bytes as i64,
                metadata.line_count as i64,
                metadata.is_compressed,
                metadata.path.to_str()
            ],
        ).context("Failed to register log in database")?;

        Ok(())
    }

    pub fn update_log(&mut self, metadata: &LogMetadata) -> Result<()> {
        self.db.execute(
            "UPDATE logs SET closed_at = ?1, size_bytes = ?2, line_count = ?3, is_compressed = ?4 WHERE id = ?5",
            sqlite::params![
                metadata.closed_at.map(|t| t.to_rfc3339()),
                metadata.size_bytes as i64,
                metadata.line_count as i64,
                metadata.is_compressed,
                metadata.id
            ],
        ).context("Failed to update log in database")?;

        Ok(())
    }

    pub fn list_logs(&self, include_active: bool) -> Result<Vec<StoredLog>> {
        let mut stmt = self.db.prepare(
            "SELECT id, terminal_id, project_path, created_at, closed_at, shell, cwd, exit_code, task_label, size_bytes, line_count, is_compressed, path FROM logs ORDER BY created_at DESC"
        ).context("Failed to prepare list statement")?;

        let mut logs = Vec::new();
        let rows = stmt.query([]).context("Failed to query logs")?;

        for row in rows {
            let row = row?;
            let is_active = row.get::<_, Option<String>>("closed_at")?.is_none();

            if !include_active && is_active {
                continue;
            }

            logs.push(StoredLog {
                metadata: LogMetadata {
                    id: row.get("id")?,
                    terminal_id: TerminalId(row.get::<_, String>("terminal_id")?.parse().unwrap_or_else(|_| uuid::Uuid::nil())),
                    project_path: row.get::<_, Option<String>>("project_path")?.map(PathBuf::from),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>("created_at")?).unwrap_or_else(|_| Utc::now()).with_timezone(&Utc),
                    closed_at: row.get::<_, Option<String>>("closed_at")?.and_then(|s| DateTime::parse_from_rfc3339(&s).ok()).map(|t| t.with_timezone(&Utc)),
                    shell: row.get("shell")?,
                    cwd: row.get::<_, Option<String>>("cwd")?.map(PathBuf::from),
                    exit_code: row.get::<_, Option<i64>>("exit_code")?.map(|i| i as i32),
                    task_label: row.get::<_, Option<String>>("task_label")?,
                    size_bytes: row.get::<_, i64>("size_bytes")? as u64,
                    line_count: row.get::<_, i64>("line_count")? as u64,
                    is_compressed: row.get("is_compressed")?,
                    path: PathBuf::from(row.get::<_, String>("path")?),
                },
                is_active,
            });
        }

        Ok(logs)
    }

    pub fn cleanup_old_logs(&mut self) -> Result<usize> {
        let retention_cutoff = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
        let cutoff_str = retention_cutoff.to_rfc3339();

        // Find logs to delete (old and closed)
        let mut stmt = self.db.prepare(
            "SELECT id, path FROM logs WHERE closed_at IS NOT NULL AND created_at < ?1"
        ).context("Failed to prepare cleanup statement")?;

        let rows = stmt.query([&cutoff_str]).context("Failed to query old logs")?;

        let mut deleted_count = 0;
        for row in rows {
            let row = row?;
            let log_id: String = row.get("id")?;
            let path: String = row.get("path")?;

            // Delete file
            if let Err(e) = fs::remove_file(&path) {
                log::warn!("Failed to delete log file {}: {}", path, e);
                continue;
            }

            // Delete metadata file
            let meta_path = Path::new(&path).with_extension("meta.json");
            let _ = fs::remove_file(meta_path);

            // Delete from database
            self.db.execute(
                "DELETE FROM logs WHERE id = ?1",
                sqlite::params![log_id],
            ).ok();

            deleted_count += 1;
        }

        Ok(deleted_count)
    }

    pub fn enforce_size_limits(&mut self) -> Result<()> {
        // TODO: Implement per-project and global limit enforcement
        // This would involve:
        // 1. Calculate current usage per project and globally
        // 2. If over limit, delete oldest logs until under limit
        // 3. Consider log importance (task logs vs regular shells)
        Ok(())
    }

    pub fn compress_log(&mut self, log_id: &str) -> Result<()> {
        // TODO: Implement compression
        Ok(())
    }
}
```

### 1.8 Add to TerminalBuilder

Modify `TerminalBuilder::subscribe()` to start logging:

```rust
impl TerminalBuilder {
    pub fn subscribe(mut self, cx: &Context<Terminal>) -> Terminal {
        // Start logging if configured
        if let Ok(Some(mut logger)) = Terminal::init_logger(cx) {
            // Store logger in terminal
            // We'll need to modify Terminal struct to hold logger
        }

        // ... existing event loop code ...
    }
}
```

---

## Phase 3: Search Index (Week 4)

### 3.1 Implement LogIndex

Create `crates/terminal-logging/src/index.rs`:

```rust
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use sqlite::{Connection, OpenFlags};
use tokio::sync::RwLock;

use crate::{config::LoggingConfig, storage::LogStorage};

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub log_id: String,
    pub line_number: u64,
    pub line_content: String,
    pub snippet: String,
}

pub struct LogIndex {
    config: LoggingConfig,
    db_path: PathBuf,
    db: Arc<RwLock<Connection>>,
}

impl LogIndex {
    pub fn new(config: LoggingConfig, storage: &LogStorage) -> Result<Self> {
        let db_path = config.storage_path.join("index.db");
        let db = Self::open_index_db(&db_path)?;

        Ok(Self {
            config,
            db_path,
            db: Arc::new(RwLock::new(db)),
        })
    }

    fn open_index_db(path: &Path) -> Result<Connection> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE;
        let conn = Connection::open_with_flags(path, flags)
            .context("Failed to open index database")?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS log_lines (
                log_id TEXT NOT NULL,
                line_number INTEGER NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS log_lines_fts USING fts5(
                content,
                content=log_lines,
                content_rowid=rowid
            );

            CREATE INDEX IF NOT EXISTS idx_log_lines_log_id ON log_lines(log_id);
            "#,
        ).context("Failed to initialize index schema")?;

        Ok(conn)
    }

    pub async fn index_log(&self, log_id: &str, lines: Vec<(u64, String)>) -> Result<()> {
        let db = self.db.write().await;

        let tx = db.transaction()?;

        for (line_num, content) in lines {
            tx.execute(
                "INSERT INTO log_lines (log_id, line_number, content, timestamp) VALUES (?1, ?2, ?3, ?4)",
                sqlite::params![log_id, line_num, content, chrono::Utc::now().to_rfc3339()],
            ).context("Failed to insert line into index")?;
        }

        tx.commit().context("Failed to commit index transaction")?;

        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<Vec<SearchResult>> {
        let db = self.db.read().await;

        let limit = limit.unwrap_or(100);

        let mut stmt = db.prepare(
            "SELECT log_id, line_number, content FROM log_lines_fts WHERE log_lines_fts MATCH ?1 ORDER BY rank LIMIT ?2"
        ).context("Failed to prepare search statement")?;

        let rows = stmt.query([query, &limit.to_string()])
            .context("Failed to execute search")?;

        let mut results = Vec::new();
        for row in rows {
            let row = row?;
            results.push(SearchResult {
                log_id: row.get("log_id")?,
                line_number: row.get::<_, i64>("line_number")? as u64,
                line_content: row.get("content")?,
                snippet: row.get("content")?, // Could enhance with highlighting
            });
        }

        Ok(results)
    }

    pub async fn search_with_filters(
        &self,
        query: &str,
        log_ids: Option<&[String]>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<SearchResult>> {
        // Build query with filters
        let mut sql = String::from(
            "SELECT l.log_id, l.line_number, l.content FROM log_lines l \
             JOIN log_lines_fts fts ON l.rowid = fts.rowid \
             WHERE log_lines_fts MATCH ?1"
        );

        let mut params: Vec<Box<dyn sqlite::ToSql>> = vec![Box::new(query)];

        if let Some(log_ids) = log_ids {
            if !log_ids.is_empty() {
                let placeholders = (0..log_ids.len())
                    .map(|i| format!("?{}", i + 2))
                    .collect::<Vec<_>>()
                    .join(",");
                sql.push_str(&format!(" AND log_id IN ({})", placeholders));
                for id in log_ids {
                    params.push(Box::new(id.as_str()));
                }
            }
        }

        if let Some(start) = start_date {
            sql.push_str(" AND timestamp >= ?");
            params.push(Box::new(start.to_rfc3339()));
        }

        if let Some(end) = end_date {
            sql.push_str(" AND timestamp <= ?");
            params.push(Box::new(end.to_rfc3339()));
        }

        sql.push_str(" ORDER BY log_id, line_number LIMIT ?");
        params.push(Box::new(limit.unwrap_or(100).to_string()));

        let db = self.db.read().await;
        let mut stmt = db.prepare(&sql).context("Failed to prepare filtered search")?;

        let rows = stmt.query(params.iter().map(|p| p.as_ref()))
            .context("Failed to execute filtered search")?;

        let mut results = Vec::new();
        for row in rows {
            let row = row?;
            results.push(SearchResult {
                log_id: row.get("log_id")?,
                line_number: row.get::<_, i64>("line_number")? as u64,
                line_content: row.get("content")?,
                snippet: row.get("content")?,
            });
        }

        Ok(results)
    }

    pub async fn delete_log_index(&self, log_id: &str) -> Result<()> {
        let db = self.db.write().await;
        db.execute(
            "DELETE FROM log_lines WHERE log_id = ?1",
            sqlite::params![log_id],
        ).context("Failed to delete log from index")?;
        Ok(())
    }
}
```

---

## Phase 4: UI Panel (Week 5-6)

### 4.1 Create TerminalLogsPanel

Create `crates/terminal-logs-panel/src/panel.rs`:

```rust
use std::path::PathBuf;
use std::sync::Arc;

use gpui::{
    AppContext, AsyncAppContext, Div, EventEmitter, Focusable, Interactive, ParentElement,
    Render, Scroll, Size, StatefulInteractiveElement, Styled, Subscription, Task,
    UniformList, UniformListHandle, View, ViewContext, ViewHandler,
};

use crate::{
    logger::TerminalLogger,
    storage::{LogStorage, StoredLog},
    index::{LogIndex, SearchResult},
    config::LoggingConfig,
};

pub struct TerminalLogsPanel {
    storage: Arc<LogStorage>,
    index: Arc<LogIndex>,
    config: LoggingConfig,
    logs: Vec<StoredLog>,
    selected_log: Option<StoredLog>,
    search_query: String,
    search_results: Vec<SearchResult>,
    is_searching: bool,
    list_handle: UniformListHandle,
    scroll: Scroll,
}

impl TerminalLogsPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let config = LoggingConfig::default();
        let storage = Arc::new(LogStorage::new(config.clone()).unwrap());
        let index = Arc::new(LogIndex::new(config.clone(), &storage).unwrap());

        let mut panel = Self {
            storage,
            index,
            config,
            logs: Vec::new(),
            selected_log: None,
            search_query: String::new(),
            search_results: Vec::new(),
            is_searching: false,
            list_handle: UniformListHandle::new(),
            scroll: Scroll::new(),
        };

        panel.refresh_logs(cx);
        panel
    }

    fn refresh_logs(&mut self, cx: &mut ViewContext<Self>) {
        match self.storage.list_logs(true) {
            Ok(logs) => {
                self.logs = logs;
                self.list_handle.reset();
            }
            Err(e) => {
                log::error!("Failed to list logs: {}", e);
            }
        }
        cx.notify();
    }

    fn select_log(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if let Some(log) = self.logs.get(index) {
            self.selected_log = Some(log.clone());
            cx.notify();
        }
    }

    fn delete_log(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if let Some(log) = self.logs.get(index).cloned() {
            if let Err(e) = self.storage.delete_log(&log.metadata.id) {
                log::error!("Failed to delete log: {}", e);
                // Show error notification
            } else {
                self.refresh_logs(cx);
            }
        }
    }

    fn perform_search(&mut self, cx: &mut ViewContext<Self>) {
        if self.search_query.is_empty() {
            self.search_results.clear();
            self.is_searching = false;
            cx.notify();
            return;
        }

        self.is_searching = true;
        let query = self.search_query.clone();
        let index = self.index.clone();

        cx.spawn(|this, cx| async move {
            match index.search(&query, None).await {
                Ok(results) => {
                    this.update(cx, |this, cx| {
                        this.search_results = results;
                        this.is_searching = false;
                        cx.notify();
                    }).ok();
                }
                Err(e) => {
                    log::error!("Search failed: {}", e);
                    this.update(cx, |this, cx| {
                        this.is_searching = false;
                        cx.notify();
                    }).ok();
                }
            }
        });
    }
}

impl Render for TerminalLogsPanel {
    fn render(&self, cx: &mut gpui::RenderContext) -> impl gpui::Element {
        Div::new()
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(
                Div::new()
                    .size(Size::new(px(300.0), px(100.0)))
                    .child(
                        // Search bar
                        Div::new()
                            .flex()
                            .flex_row()
                            .p_2()
                            .child(
                                gpui::TextInput::new(
                                    &self.search_query,
                                    "Search logs...",
                                    cx,
                                )
                                .on_input(|query, cx| {
                                    // Debounce search
                                })
                            )
                    )
            )
            .child(
                Div::new()
                    .flex()
                    .flex_row()
                    .size_full()
                    .child(
                        // Log list
                        self.render_log_list(cx)
                    )
                    .child(
                        // Preview pane
                        self.render_preview(cx)
                    )
            )
    }
}

impl TerminalLogsPanel {
    fn render_log_list(&self, cx: &mut gpui::RenderContext) -> impl gpui::Element {
        Scroll::new(
            UniformList::new(
                self.logs.len(),
                self.list_handle.clone(),
                move |index, cx| {
                    if let Some(log) = self.logs.get(index) {
                        self.render_log_item(log, index, cx)
                    } else {
                        Div::new().into_element()
                    }
                },
            )
            .size(Size::new(px(300.0), px(100.0)))
            .on_click(move |index, cx| self.select_log(index, cx))
        )
    }

    fn render_log_item(&self, log: &StoredLog, index: usize, cx: &mut gpui::RenderContext) -> impl gpui::Element {
        let is_selected = self.selected_log.as_ref().map(|s| s.metadata.id == log.metadata.id).unwrap_or(false);

        Div::new()
            .flex()
            .flex_col()
            .p_2()
            .bg(if is_selected {
                cx.theme().colors().selection
            } else {
                cx.theme().colors().transparent
            })
            .hover_style(|style| style.bg(cx.theme().colors().element_hover))
            .on_click(move |_, cx| self.select_log(index, cx))
            .child(
                gpui::Label::new(format!("{}", log.metadata.shell))
                    .size(gpui::TextSize::Small)
                    .color(cx.theme().colors().text)
            )
            .child(
                gpui::Label::new(
                    log.metadata
                        .cwd
                        .as_ref()
                        .and_then(|p| p.to_str())
                        .unwrap_or("unknown")
                        .to_string()
                )
                .size(gpui::TextSize::Small)
                .color(cx.theme().colors().text_muted)
            )
            .child(
                gpui::Label::new(format!(
                    "{} • {}",
                    format_file_size(log.metadata.size_bytes),
                    format_datetime(log.metadata.created_at)
                ))
                .size(gpui::TextSize::Small)
                .color(cx.theme().colors().text_muted)
            )
    }

    fn render_preview(&self, cx: &mut gpui::RenderContext) -> impl gpui::Element {
        if let Some(log) = &self.selected_log {
            // Read and display log content
            Div::new()
                .flex()
                .flex_col()
                .size(Size::new(px(400.0), px(100.0)))
                .child(
                    gpui::Label::new(format!("Log: {}", log.metadata.id))
                        .size(gpui::TextSize::Medium)
                        .color(cx.theme().colors().text)
                )
                .child(
                    Scroll::new(
                        Div::new()
                            .child(gpui::Label::new("Log content preview..."))
                    )
                )
        } else {
            Div::new()
                .flex()
                .flex_col()
                .size(Size::new(px(400.0), px(100.0)))
                .child(
                    gpui::Label::new("Select a log to preview")
                        .color(cx.theme().colors().text_muted)
                )
        }
    }
}

fn format_file_size(bytes: u64) -> String {
    let mb = bytes as f64 / 1024.0 / 1024.0;
    if mb < 1.0 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", mb)
    }
}

fn format_datetime(dt: chrono::DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M").to_string()
}
```

---

## Phase 5: Integration Polish (Week 7)

### 5.1 Command Palette Commands

Add to `crates/command-palette/src/commands.rs`:

```rust
use gpui::actions;

actions!(terminal_logging, [OpenLogsPanel, ClearAllLogs]);

pub struct OpenLogsPanel;
impl gpui::Command for OpenLogsPanel {
    fn execute(
        &self,
        _: &gpui::AppContext,
        _: Option<gpui::Target>,
        _: &gpui::Action,
    ) {
        // Show TerminalLogsPanel
    }
}

pub struct ClearAllLogs;
impl gpui::Command for ClearAllLogs {
    fn execute(
        &self,
        cx: &gpui::AppContext,
        _: Option<gpui::Target>,
        _: &gpui::Action,
    ) {
        // Show confirmation dialog, then clear all logs
    }
}
```

### 5.2 Status Bar Indicator

Add to `crates/status-bar/src/status_bar.rs`:

```rust
use gpui::{
    actions, div, svg, svg::Svg, IconName, Keystroke, MouseButton, Tooltip,
};

use crate::terminal_logging::{TerminalLogger, LogManager};

actions!(terminal_logging, [ToggleLogging, OpenLogs]);

pub struct TerminalLoggingStatus {
    active_loggers: usize,
}

impl TerminalLoggingStatus {
    pub fn new(cx: &mut gpui::AppContext) -> Self {
        let manager = LogManager::global(cx);
        Self {
            active_loggers: manager.active_logs().len(),
        }
    }
}

impl gpui::Render for TerminalLoggingStatus {
    fn render(&self, cx: &mut gpui::RenderContext) -> impl gpui::Element {
        let icon = if self.active_loggers > 0 {
            IconName::Document
        } else {
            IconName::DocumentOff
        };

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                svg()
                    .size(px(16.0))
                    .path(icon)
                    .color(if self.active_loggers > 0 {
                        cx.theme().colors().success
                    } else {
                        cx.theme().colors().text_muted
                    })
            )
            .child(
                gpui::Label::new(format!("{}", self.active_loggers))
                    .size(gpui::TextSize::Small)
            )
            .tooltip(move || {
                Tooltip::text(
                    if self.active_loggers > 0 {
                        format!("{} active terminal logs", self.active_loggers)
                    } else {
                        "Terminal logging disabled".to_string()
                    },
                )
            })
            .on_click(|_, cx| {
                cx.dispatch_action(OpenLogs.boxed_clone());
            })
    }
}
```

---

## Phase 6: Testing & Optimization

### 6.1 Performance Testing

Create `crates/terminal-logging/tests/performance_test.rs`:

```rust
use std::time::Instant;
use tempfile::tempdir;
use terminal_logging::{LoggingConfig, TerminalLogger, TerminalId};

#[test]
fn test_write_performance() {
    let temp_dir = tempdir().unwrap();
    let config = LoggingConfig {
        storage_path: temp_dir.path().to_path_buf(),
        per_terminal_limit_mb: 100,
        buffer_ms: 100,
        buffer_lines: 1000,
        ..Default::default()
    };

    let terminal_id = TerminalId::new();
    let mut logger = TerminalLogger::new(
        config.clone(),
        terminal_id,
        None,
        "bash",
        None,
    ).unwrap();

    let start = Instant::now();
    let iterations = 100_000;

    for i in 0..iterations {
        logger.write(format!("Line {}\n", i).as_bytes()).unwrap();
    }

    let elapsed = start.elapsed();
    let throughput = iterations as f64 / elapsed.as_secs_f64();

    println!("Wrote {} lines in {:?} ({:.2} lines/sec)", iterations, elapsed, throughput);

    // Should achieve > 10,000 lines/sec
    assert!(throughput > 10_000.0, "Write throughput too low: {:.2} lines/sec", throughput);
}
```

### 6.2 Memory Usage Testing

```rust
#[test]
fn test_memory_usage() {
    use memory_stats::memory_stats;

    let temp_dir = tempdir().unwrap();
    let config = LoggingConfig {
        storage_path: temp_dir.path().to_path_buf(),
        per_terminal_limit_mb: 100,
        ..Default::default()
    };

    let before = memory_stats().unwrap().physical_mem;
    let terminal_id = TerminalId::new();
    let mut logger = TerminalLogger::new(
        config.clone(),
        terminal_id,
        None,
        "bash",
        None,
    ).unwrap();

    // Write 10 MB
    for _ in 0..10_000 {
        logger.write(&[b'a'; 1024]).unwrap();
    }

    let after = memory_stats().unwrap().physical_mem;
    let delta = after - before;

    // Should use less than 50 MB additional memory
    assert!(delta < 50 * 1024 * 1024, "Memory usage too high: {} bytes", delta);
}
```

---

## Common Pitfalls & Solutions

### 1. **Deadlocks with Async**
- **Problem**: Using blocking I/O in async context
- **Solution**: Use `cx.background_spawn()` for all file operations

### 2. **Race Conditions**
- **Problem**: Multiple terminals writing to same log file
- **Solution**: Use per-terminal UUIDs, never share loggers

### 3. **Memory Leaks**
- **Problem**: Forgetting to close loggers on terminal drop
- **Solution**: Implement `Drop` for `TerminalLogger`

### 4. **Database Corruption**
- **Problem**: SQLite corruption on crash
- **Solution**: Use WAL mode, regular backups, recovery on startup

### 5. **Performance Degradation**
- **Problem**: Indexing blocks UI
- **Solution**: Incremental indexing, background tasks, rate limiting

---

## Debugging Tips

1. **Enable debug logging**:
   ```rust
   env_logger::Builder::from_default_env()
       .filter_module("terminal_logging", log::LevelFilter::Debug)
       .init();
   ```

2. **Check storage location**:
   ```bash
   ls -la ~/.config/zed/logs/terminal/active/
   ```

3. **Verify database**:
   ```bash
   sqlite3 ~/.config/zed/logs/terminal/logs.db ".tables"
   ```

4. **Monitor file sizes**:
   ```bash
   du -sh ~/.config/zed/logs/terminal/active/*
   ```

5. **Test redaction**:
   ```rust
   let redactor = Redactor::new(&config.redact_patterns);
   let test = "password = secret123";
   assert!(redactor.redact(test).contains("[REDACTED]"));
   ```

---

## Checklist for PR

- [ ] All Phase 1 tasks complete (core logging)
- [ ] All Phase 2 tasks complete (storage management)
- [ ] All Phase 3 tasks complete (search index)
- [ ] All Phase 4 tasks complete (UI panel)
- [ ] All Phase 5 tasks complete (integration)
- [ ] Unit tests passing (> 80% coverage)
- [ ] Integration tests passing
- [ ] Performance tests meeting targets
- [ ] Documentation updated (user + dev)
- [ ] Settings UI complete
- [ ] Error handling comprehensive
- [ ] No unwrap() in production code
- [ ] All I/O is async
- [ ] No memory leaks
- [ ] Works on macOS, Linux, Windows

---

## Next Steps After Implementation

1. **Internal testing** with Zed team
2. **Beta release** to early adopters
3. **Gather feedback** and iterate
4. **Performance tuning** based on real usage
5. **Documentation polish**
6. **Production release**

---

**Good luck with the implementation!** 🚀