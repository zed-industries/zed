# Persistent Terminal Logging for Zed

## Overview

This proposal adds persistent, configurable terminal logging to Zed, allowing users to retain terminal output across sessions, search through historical logs, and manage log storage efficiently.

## Goals

- **Persist terminal output** beyond the lifetime of a terminal session
- **Configurable storage** with sensible defaults
- **Efficient memory usage** through streaming and chunking
- **Searchable logs** with fast full-text search
- **Size management** with automatic rotation and compression
- **Privacy-aware** with redaction capabilities
- **Low performance impact** on terminal responsiveness

## Non-Goals

- Real-time log sharing/collaboration (future consideration)
- Cloud sync of logs (out of scope for initial implementation)
- Complex log analytics (keep it simple)

---

## Architecture

### Components

1. **TerminalLogger** - Core logging engine
2. **LogStorage** - File management, rotation, compression
3. **LogIndex** - Search index for fast queries
4. **LogManager** - UI panel for browsing logs
5. **Settings** - Configuration options

### Data Flow

```
Terminal Output → TerminalLogger → LogStorage (file)
                                    ↓
                              LogIndex (incremental)
                                    ↓
                              LogManager (UI)
```

---

## 1. Storage Design

### File Structure

```
~/.config/zed/logs/terminal/  (or project/.zed/terminal-logs/)
├── active/
│   ├── terminal-{uuid}.log      # Current terminal output (plain text)
│   └── terminal-{uuid}.meta.json # Metadata
├── archived/
│   ├── terminal-{uuid}-{timestamp}.log.gz
│   └── terminal-{uuid}-{timestamp}.meta.json
├── index/
│   ├── log-{uuid}.index         # Search index (SQLite FTS or tantivy)
│   └── global.index             # Cross-log search index
└── logs.db                      # Master database (sqlite)
```

### Metadata Format

```json
{
  "id": "uuid",
  "project_path": "/path/to/project",
  "created_at": "2025-01-15T10:30:00Z",
  "closed_at": "2025-01-15T11:45:00Z",
  "shell": "zsh",
  "cwd": "/path/to/project/src",
  "exit_code": 0,
  "task_label": "cargo build",
  "size_bytes": 1048576,
  "line_count": 15420,
  "is_compressed": false,
  "path": "active/terminal-uuid.log"
}
```

### Database Schema (logs.db)

```sql
CREATE TABLE logs (
    id TEXT PRIMARY KEY,
    project_path TEXT,
    created_at TIMESTAMP,
    closed_at TIMESTAMP,
    shell TEXT,
    cwd TEXT,
    exit_code INTEGER,
    task_label TEXT,
    size_bytes INTEGER,
    line_count INTEGER,
    is_compressed BOOLEAN,
    path TEXT
);

CREATE TABLE log_lines (
    log_id TEXT,
    line_number INTEGER,
    content TEXT,
    timestamp TIMESTAMP,
    FOREIGN KEY (log_id) REFERENCES logs(id)
);

-- Full-text search index
CREATE VIRTUAL TABLE log_lines_fts USING fts5(
    content,
    content=log_lines,
    content_rowid=rowid
);
```

---

## 2. Size Limits & Rotation

### Default Limits

| Limit Type | Default | Configurable | Action on Exceed |
|------------|---------|--------------|------------------|
| Per-terminal | 100 MB | Yes | Stop logging, notify |
| Per-project | 1 GB | Yes | Delete oldest logs |
| Global | 10 GB | Yes | Delete oldest logs |
| Retention | 90 days | Yes | Delete expired logs |

### Rotation Strategy

1. **Check size** before each write (buffered, every 100 lines or 100ms)
2. **If per-terminal limit reached**:
   - Close current log
   - Compress and move to `archived/`
   - Create new log file with incremented counter
   - Notify user
3. **Background cleanup** (daily):
   - Delete logs older than retention period
   - Enforce per-project and global limits (LRU eviction)
   - Compress logs > 10 MB that are > 7 days old

### Compression

- Use `gzip` (standard, widely supported)
- Transparent decompression when reading
- Keep original `.log` for active logs only
- Archive format: `terminal-{uuid}-{timestamp}.log.gz`

---

## 3. Memory Management

### Write Path (Low Memory)

```rust
struct TerminalLogger {
    file: BufWriter<File>,
    buffer: Vec<u8>,
    buffer_line_count: usize,
    bytes_written: u64,
}

impl TerminalLogger {
    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        // Accumulate in memory buffer
        self.buffer.extend_from_slice(data);
        self.bytes_written += data.len() as u64;

        // Flush conditions:
        // - Buffer > 4KB
        // - Buffer contains > 100 lines
        // - 100ms elapsed since last flush
        if self.should_flush() {
            self.file.write_all(&self.buffer)?;
            self.file.flush()?;
            self.buffer.clear();
        }

        // Check size limits
        if self.bytes_written > self.max_size {
            self.rotate()?;
        }

        Ok(())
    }
}
```

### Read/Search Path (Chunked)

```rust
struct LogReader {
    file: BufReader<File>,
    chunk_size: usize, // 1 MB chunks
    cache: LruCache<u64, Vec<String>>,
}

impl LogReader {
    fn read_lines(&mut self, start_line: u64, count: usize) -> io::Result<Vec<String>> {
        // Seek to approximate position (line * avg_line_len)
        // Read chunk, split lines, return requested range
        // Cache chunks for repeated access
    }

    fn search(&mut self, query: &str) -> io::Result<Vec<SearchResult>> {
        // Stream through file, yield matches
        // Don't load entire file into memory
        // Use memory-mapped files for large logs (>100 MB)
    }
}
```

### Cache Policy

- **Recent lines cache**: Last 1000 lines per terminal (in memory)
- **Chunk cache**: 10 most-recently-accessed 1 MB chunks (LRU)
- **Index cache**: Search index in memory (SQLite does this automatically)
- **Global cache limit**: 500 MB total across all terminals

---

## 4. UI/UX Design

### Logs Panel

```
┌─────────────────────────────────────────────────────────────┐
│ Terminal Logs                                    [⚙️] [🗑️] │
├─────────────────────────────────────────────────────────────┤
│ Search: [________________________] [regex] [case-sensitive]│
│ Filter: [● All] [● Project] [● Today] [● Week] [● Month]   │
│         [✓ Show successful] [✓ Show failed] [✓ Show tasks] │
├─────────────────────────────────────────────────────────────┤
│ ● active-abc123.log                     12:34 PM   45 MB   │
│   zsh • /project/src • cargo build • exit 0                 │
│                                                                           │
│ ● build-2025-01-15.log                  Yesterday  2 MB   │
│   bash • /project • make • exit 1                              │
│                                                                           │
│ ● npm-start-2025-01-14.log             2 days ago  8 MB   │
│   zsh • /project • npm start • exit 0                         │
│                                                                           │
│ ...                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Preview Pane (Split View)

```
┌──────────────────────────────┬──────────────────────────────┐
│ Log List                     │ Preview                     │
│                              │                              │
│ ● active-abc123.log          │ 12:34:45 user@host ~/project│
│   (selected)                 │ $ cargo build               │
│                              │   Compiling zed-terminal... │
│                              │   Finished dev [unoptimized]│
│                              │   target/debug/zed          │
│                              │                              │
│                              │ [Open in Terminal] [Export] │
└──────────────────────────────┴──────────────────────────────┘
```

### Context Menu (Right-click on log)

```
Show in Terminal (replay)
Copy All
Export As...
Delete
Delete All Logs
Open Log Folder
```

### Terminal Tab Menu

Right-click on terminal tab:
```
[✓] Enable Logging
Pause Logging
Show Log
Log Settings...
```

### Status Bar Indicator

```
[📋 Logging]  # Green, when actively logging
[📋 Paused]   # Yellow, when logging paused
[📋 45 MB]    # Shows size when hovering
```

---

## 5. Settings

### Global Settings (settings.json)

```json
{
  "terminal": {
    "logging": {
      "enabled": true,
      "storage_path": "~/.config/zed/logs/terminal/",
      "per_terminal_limit_mb": 100,
      "per_project_limit_mb": 1000,
      "global_limit_mb": 10000,
      "retention_days": 90,
      "auto_compress": true,
      "compress_after_mb": 10,
      "buffer_ms": 100,
      "buffer_lines": 1000,
      "enable_search_index": true,
      "redact_patterns": [
        "(?i)password\\s*=\\s*\\S+",
        "(?i)token\\s*[:=]\\s*\\S+",
        "(?i)secret\\s*[:=]\\s*\\S+",
        "\\b[A-Za-z0-9+/]{40,}={0,2}\\b"  // Base64 blobs
      ]
    }
  }
}
```

### Project-Specific Settings (.zed/terminal-logging.json)

```json
{
  "enabled": true,
  "storage_path": "./.zed/terminal-logs/",
  "per_project_limit_mb": 500,
  "redact_patterns": [
    "YOUR_API_KEY\\s*=\\s*\\S+"
  ]
}
```

### UI Settings Panel

```
Terminal Logging
────────────────────────────────────────────────
[x] Enable terminal logging
Storage location: [~/.config/zed/logs/...] [Browse]
Current usage: 2.3 GB / 10 GB

Per-terminal limit: ████████░░ 80 MB
Per-project limit: █████░░░░░ 30% used
Retention: Keep logs for [90] days

Performance:
[x] Enable search indexing
Buffer writes every [100] ms or [1000] lines

Compression:
[x] Auto-compress logs older than 30 days
[x] Compress logs larger than 10 MB

Privacy:
[x] Redact sensitive patterns
[Edit patterns...]

Advanced:
[ ] Debug logging (verbose)
[ ] Force sync to disk after each write (slower, safer)
────────────────────────────────────────────────
[Reset to Defaults] [Clear All Logs...]
```

---

## 6. Implementation Plan

### Phase 1: Core Logging (Week 1-2)

**Tasks:**
1. Create `crates/terminal-logging/` crate
2. Implement `TerminalLogger` with buffered writes
3. Add metadata tracking
4. Basic file rotation (size-based)
5. Settings integration
6. Unit tests for logger

**Deliverable:** Terminal output is written to files with rotation

### Phase 2: Storage Management (Week 3)

**Tasks:**
1. Implement `LogStorage` with cleanup policies
2. Add compression support (gzip)
3. Create `logs.db` SQLite database
4. Background cleanup task
5. Size limit enforcement

**Deliverable:** Automatic cleanup and compression

### Phase 3: Search Index (Week 4)

**Tasks:**
1. Implement `LogIndex` using SQLite FTS5 or tantivy
2. Incremental indexing during writes
3. Search API with filters
4. Performance optimization (caching)

**Deliverable:** Fast full-text search across logs

### Phase 4: UI Panel (Week 5-6)

**Tasks:**
1. Create `TerminalLogsPanel` in `crates/terminal/`
2. Implement log list with sorting/filtering
3. Add preview pane
4. Context menus and actions
5. Status bar indicator
6. Keyboard shortcuts

**Deliverable:** Full UI for browsing and managing logs

### Phase 5: Integration Polish (Week 7)

**Tasks:**
1. Terminal tab menu integration
2. Command palette commands
3. Settings UI in Settings panel
4. Error handling and user notifications
5. Performance testing and optimization
6. Documentation

**Deliverable:** Production-ready feature

### Phase 6: Advanced Features (Week 8+)

**Tasks:**
1. Redaction engine
2. Project-specific settings
3. Extension API
4. Migration tools
5. Comprehensive testing

**Deliverable:** Advanced features and polish

---

## 7. API Design

### TerminalLogger (Public API)

```rust
pub struct TerminalLogger {
    id: TerminalId,
    log_path: PathBuf,
    meta: LogMetadata,
}

impl TerminalLogger {
    pub fn new(
        config: &LoggingConfig,
        terminal_id: TerminalId,
        project_path: Option<PathBuf>,
        shell: &str,
        cwd: Option<PathBuf>,
    ) -> Result<Self>;

    pub fn write(&mut self, bytes: &[u8]) -> Result<()>;

    pub fn close(self) -> Result<LogMetadata> {
        // Finalize log, write metadata, close file
    }

    pub fn log_path(&self) -> &Path;

    pub fn bytes_written(&self) -> u64;

    pub fn should_rotate(&self) -> bool;
}
```

### LogManager (Singleton)

```rust
pub struct LogManager {
    storage: LogStorage,
    index: LogIndex,
    active_logs: HashMap<TerminalId, TerminalLogger>,
}

impl LogManager {
    pub fn global(cx: &App) -> &LogManager;

    pub fn start_logging(
        &mut self,
        terminal_id: TerminalId,
        config: LoggingConfig,
    ) -> Result<TerminalLogger>;

    pub fn stop_logging(&mut self, terminal_id: TerminalId) -> Result<()>;

    pub fn get_log(&self, log_id: &str) -> Option<LogEntry>;

    pub fn list_logs(&self, filters: LogFilters) -> Vec<LogEntry>;

    pub fn search(&self, query: &str, filters: LogFilters) -> SearchResult;

    pub fn delete_log(&mut self, log_id: &str) -> Result<()>;

    pub fn export_log(&self, log_id: &str, dest: PathBuf) -> Task<Result<()>>;

    pub fn open_log_in_terminal(&self, log_id: &str) -> Task<Result<()>>;
}
```

### Settings

```rust
#[derive(Serialize, Deserialize)]
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
    pub redact_patterns: Vec<Regex>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_path: dirs::config_dir().join("zed/logs/terminal"),
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
```

---

## 8. Error Handling

### Failure Modes & Recovery

| Error | Handling | User Impact |
|-------|----------|-------------|
| Disk full | Stop logging, show notification, keep terminal working | No new logs, existing logs safe |
| Permission denied | Fall back to temp dir, warn user | Logs in temp (cleared on reboot) |
| File corrupted | Mark as corrupted, skip in listings | Can't view that log, others OK |
| I/O error | Retry 3x with backoff, then stop | Temporary logging pause |
| Out of memory | Reduce cache, disable indexing | Slower search, logs still written |

### User Notifications

- **Toast notification** when logging stops due to size limit
- **Warning** when reaching 80% of limit
- **Error** in Logs Panel if storage unavailable
- **Status bar** indicator shows state (logging/paused/error)

---

## 9. Testing Strategy

### Unit Tests

- `TerminalLogger::write()` buffers correctly
- Rotation triggers at exact limit
- Compression produces valid gzip
- Redaction masks all patterns
- Metadata serialization/deserialization
- Size calculation accurate

### Integration Tests

- Terminal output → log file
- Search returns correct results
- Cleanup deletes expired logs
- Multi-project isolation
- Crash recovery

### Performance Tests

- 100 MB log search < 1 second
- 10 concurrent terminals: < 5% CPU overhead
- Memory usage < 50 MB per active terminal
- Disk I/O doesn't block terminal rendering

---

## 10. Migration & Compatibility

### Versioning

- Log format version in metadata: `"format_version": 1`
- Migration scripts for future versions
- Support reading logs from last 2 versions

### First Run

1. Create default config if missing
2. Create storage directories
3. Run initial cleanup (delete very old logs)
4. Migrate any existing logs from old location (if applicable)

### Settings Sync

- Sync `LoggingConfig` via Zed settings sync
- Do NOT sync actual log files (too large, privacy)
- Sync only preferences

---

## 11. Documentation

### User Documentation

- **Feature guide**: "How to use terminal logging"
- **Settings reference**: All options explained
- **Troubleshooting**: Common issues and solutions
- **Privacy guide**: How redaction works, best practices

### Developer Documentation

- **Architecture diagram**: Component interactions
- **API reference**: TerminalLogger, LogManager
- **Contributing**: How to add tests, debug issues
- **Performance tuning**: Tips for large deployments

---

## 12. Open Questions

1. **Should logs be included in project backups?** → Yes, if stored in project, but warn about size
2. **Should we support cloud sync?** → Out of scope, but design for future
3. **Should logs be searchable from global search (Cmd+Shift+F)?** → Yes, with filter option
4. **Should we support live tailing of logs?** → Yes, in preview pane with auto-refresh
5. **Should we support log sharing (copy link, upload)?** → Future, need secure sharing infrastructure
6. **What about multi-line ANSI sequences?** → Strip ANSI for search, preserve in display
7. **Should we support binary output?** → Store as-is, but mark as binary, don't index

---

## 13. Alternatives Considered

### Alternative 1: External Tools Only
- **Pros**: No code changes, user choice (script, tee, etc.)
- **Cons**: Not integrated, no UI, no management
- **Decision**: Rejected - we want first-class integration

### Alternative 2: Append to Single File per Project
- **Pros**: Simple, easy to find
- **Cons**: Hard to separate sessions, no rotation, slow to search
- **Decision**: Rejected - need per-terminal separation

### Alternative 3: SQLite Only (No Text Files)
- **Pros**: Atomic, searchable, transactional
- **Cons**: Hard to inspect manually, larger overhead
- **Decision**: Rejected - keep text files for simplicity, use SQLite for index only

### Alternative 4: Memory-Mapped Files Only
- **Pros**: Fast, OS handles caching
- **Cons**: Complex, platform differences, file locking
- **Decision**: Partially adopted - use for reading large logs, not writing

---

## 14. Success Metrics

- **Adoption**: > 50% of terminal users enable logging within 1 month
- **Performance**: < 1% CPU overhead on typical terminal workloads
- **Reliability**: > 99.9% of terminal sessions successfully write logs
- **Usability**: Users can find past terminal output in < 30 seconds
- **Storage**: Average user stays under 1 GB total logs (with rotation)

---

## 15. Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Disk space exhaustion | Medium | High | Aggressive defaults, warnings, easy cleanup |
| Privacy leak (sensitive data) | Medium | High | Redaction by default, clear warnings |
| Performance degradation | Low | Medium | Async I/O, buffering, extensive testing |
| Complex UI | Medium | Medium | Start simple, iterate based on feedback |
| Migration issues | Low | Medium | Thorough testing, backup/restore tools |

---

## Conclusion

This design provides a robust, performant, and user-friendly terminal logging system for Zed. It balances functionality with simplicity, ensures privacy and performance, and provides a solid foundation for future enhancements.

**Next Steps:**
1. Get feedback from maintainers on overall design
2. Implement Phase 1 (core logging) as proof of concept
3. Iterate based on testing and user feedback
4. Full implementation following the phased plan

---

## Appendix: Code Structure

```
crates/
├── terminal/
│   ├── src/
│   │   ├── terminal.rs          (existing)
│   │   ├── terminal_builder.rs
│   │   └── ...
│   └── Cargo.toml
├── terminal-logging/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── logger.rs            # TerminalLogger
│   │   ├── storage.rs           # LogStorage (file management)
│   │   ├── index.rs             # LogIndex (search)
│   │   ├── manager.rs           # LogManager (singleton)
│   │   ├── config.rs            # Settings structs
│   │   ├── redact.rs            # Pattern matching & masking
│   │   └── compression.rs       # Gzip utilities
│   ├── Cargo.toml
│   └── README.md
└── terminal-logs-panel/
    ├── src/
    │   ├── panel.rs             # LogsPanel UI
    │   ├── list_view.rs
    │   ├── preview.rs
    │   ├── search_ui.rs
    │   └── settings.rs
    ├── Cargo.toml
    └── README.md
```

**Dependencies to add:**
- `sqlite` (or `rusqlite`) for metadata and FTS
- `tantivy` (optional, alternative full-text search)
- `flate2` for gzip compression
- `regex` for redaction
- `chrono` for timestamps
- `directories` for config paths
- `serde`/`serde_json` for metadata
- `anyhow`/`thiserror` for error handling

---

**Author:** AI Assistant  
**Date:** 2025-01-15  
**Status:** Proposal  
**Target:** Zed v0.225+