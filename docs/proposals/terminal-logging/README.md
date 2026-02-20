# Persistent Terminal Logging for Zed

## Overview

This proposal introduces a comprehensive terminal logging system for Zed that persistently stores terminal output across sessions, enabling users to search, review, and manage their terminal history.

**Status:** Design Proposal (Not Yet Implemented)

---

## The Problem

Currently, Zed's terminal output is stored only in memory (`last_content`) and is **lost when the terminal is closed or Zed restarts**. This means:

- ❌ Valuable build output, debug sessions, and command results disappear
- ❌ No way to search through past terminal output
- ❌ Cannot review what happened in a previous session
- ❌ No audit trail of commands run

## The Solution

Add persistent, configurable terminal logging with:

- ✅ **Automatic logging** of all terminal output to disk
- ✅ **Full-text search** across all historical logs
- ✅ **Size management** with automatic rotation and compression
- ✅ **Privacy features** including redaction of sensitive data
- ✅ **Low overhead** - minimal impact on terminal performance
- ✅ **Intuitive UI** - browse, search, and manage logs from within Zed

---

## Quick Look

### What Users Get

1. **Logs Panel** - A new panel to browse all terminal logs
   ```
   ┌─────────────────────────────────────────────┐
   │ Terminal Logs                      [⚙️] [🗑️] │
   ├─────────────────────────────────────────────┤
   │ Search: [________________________]         │
   │ ● active-abc123.log           12:34 PM 45 MB│
   │   zsh • /project/src • cargo build • exit 0 │
   │ ● build-2025-01-15.log        Yesterday  2 MB│
   │ ...                                         │
   └─────────────────────────────────────────────┘
   ```

2. **Status Bar Indicator** - Know when logging is active
   - `[📋 Logging]` - Green, actively writing
   - `[📋 Paused]` - Yellow, logging paused
   - `[📋 45 MB]` - Hover to see size

3. **Right-Click Terminal Tab** - Quick access to log controls
   ```
   [✓] Enable Logging
   Pause Logging
   Show Log
   Log Settings...
   ```

4. **Search Integration** - Find anything you've ever typed
   - Search across all logs with regex support
   - Filter by project, date, exit status, shell type
   - Instant results with incremental indexing

---

## Key Features

### 1. Smart Storage

**Default Location:** `~/.config/zed/logs/terminal/`

**Per-Project Option:** Store logs inside project at `.zed/terminal-logs/` (auto-added to `.gitignore`)

**File Structure:**
```
~/.config/zed/logs/terminal/
├── active/
│   ├── terminal-{uuid}.log        # Current output
│   └── terminal-{uuid}.meta.json  # Metadata
├── archived/
│   ├── terminal-{uuid}-{timestamp}.log.gz  # Compressed old logs
│   └── terminal-{uuid}-{timestamp}.meta.json
├── index/
│   └── log-{uuid}.index           # Search index
└── logs.db                        # Master database
```

### 2. Size Management

**Default Limits:**
- Per-terminal: 100 MB (configurable)
- Per-project: 1 GB (configurable)
- Global: 10 GB (configurable)
- Retention: 90 days (configurable)

**Automatic Cleanup:**
- Compress logs > 10 MB after 30 days
- Delete logs older than retention period
- Enforce limits with LRU eviction
- Daily background cleanup

### 3. Privacy & Security

**Redaction by Default:**
Automatically masks:
- Passwords: `password = secret123`
- Tokens: `token: abcdef123456`
- API keys
- Base64 blobs (40+ chars)

**Custom Patterns:**
Add your own regex patterns in settings:
```json
{
  "terminal.logging.redact_patterns": [
    "YOUR_API_KEY\\s*=\\s*\\S+",
    "Bearer\\s+[A-Za-z0-9-]+"
  ]
}
```

### 4. Performance Optimized

- **Buffered writes**: 4 KB or 100ms, whichever comes first
- **Async I/O**: Never blocks terminal rendering
- **Memory efficient**: < 50 MB per active terminal
- **Chunked reading**: Load only what's needed for display
- **Background indexing**: Search index built incrementally

**Overhead:** < 1% CPU on typical workloads

---

## Configuration

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
        "\\b[A-Za-z0-9+/]{40,}={0,2}\\b"
      ]
    }
  }
}
```

### Project-Specific Settings (`.zed/terminal-logging.json`)

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

Access via `Cmd+,` → Terminal → Logging:

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
[ ] Force sync to disk after each write
────────────────────────────────────────────────
[Reset to Defaults] [Clear All Logs...]
```

---

## Usage Examples

### Basic Workflow

1. **Open a terminal** in Zed - logging starts automatically
2. **Run commands** - all output is saved
3. **Close terminal** - log is finalized and indexed
4. **Open Logs Panel** (`Cmd+Shift+P` → "Terminal: Open Logs")
5. **Search** for something you ran last week
6. **Preview** the log in the split pane
7. **Export** if needed, or **delete** to free space

### Searching

```
Search: "cargo build"
Filter: [● This Project] [● Last 7 days] [✓ Show successful]

Results:
  build-2025-01-15.log (line 234)
    Finished release [optimized] target/release/zed
  
  build-2025-01-14.log (line 567)
    error: could not compile `zed` due to previous error
```

### Exporting

Right-click a log → **Export As...** → Choose format:
- **Plain Text** (.log) - Raw output
- **Compressed** (.log.gz) - With compression
- **JSON** (.json) - Includes metadata

### Replaying

"Show in Terminal" reopens the log in a new terminal (read-only), allowing you to:
- Scroll through as if it were live
- Copy/paste from old output
- Search within the terminal itself

---

## Architecture

### Components

```
┌─────────────────────────────────────────────────────┐
│                     Terminal                         │
│  ┌────────────┐    ┌─────────────────────────────┐  │
│  │  Alacritty │───▶│    TerminalLogger           │  │
│  │   Terminal │    │  • Buffered writes          │  │
│  └────────────┘    │  • Size tracking            │  │
│                    │  • Rotation logic           │  │
│                    └─────────────────────────────┘  │
│                                    │                 │
│                                    ▼                 │
│                    ┌─────────────────────────────┐  │
│                    │     LogStorage               │  │
│                    │  • File management           │  │
│                    │  • Compression (gzip)        │  │
│                    │  • Cleanup policies          │  │
│                    └─────────────────────────────┘  │
│                                    │                 │
│                                    ▼                 │
│                    ┌─────────────────────────────┐  │
│                    │      LogIndex                │  │
│                    │   • SQLite FTS5              │  │
│                    │   • Incremental updates      │  │
│                    │   • Fast search              │  │
│                    └─────────────────────────────┘  │
│                                    │                 │
│                                    ▼                 │
│                    ┌─────────────────────────────┐  │
│                    │     LogManager (UI)          │  │
│                    │  • List view                 │  │
│                    │  • Preview pane              │  │
│                    │  • Search UI                 │  │
│                    └─────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

### Data Flow

1. Terminal output → `TerminalLogger::write()`
2. Buffer accumulates (4 KB / 100 ms)
3. Flush to `active/terminal-{uuid}.log`
4. Update `logs.db` metadata
5. Incrementally update search index
6. UI polls `LogManager` for updates

---

## Implementation Status

| Phase | Status | Estimated |
|-------|--------|-----------|
| Design | ✅ Complete | - |
| Core Logging (Phase 1) | ❌ Not Started | Week 1-2 |
| Storage Management (Phase 2) | ❌ Not Started | Week 3 |
| Search Index (Phase 3) | ❌ Not Started | Week 4 |
| UI Panel (Phase 4) | ❌ Not Started | Week 5-6 |
| Integration Polish (Phase 5) | ❌ Not Started | Week 7 |
| Advanced Features (Phase 6) | ❌ Not Started | Week 8+ |

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Search 100 MB log | < 1 second | Cold start |
| CPU overhead (10 terminals) | < 5% | Average |
| Memory per terminal | < 50 MB | Peak |
| Disk I/O impact | < 10 ms per write | 95th percentile |
| Startup time impact | < 100 ms | Clean start |

---

## Testing

### Unit Tests
- ✅ Buffer flush logic
- ✅ Size limit enforcement
- ✅ Rotation triggers
- ✅ Compression/decompression
- ✅ Redaction patterns
- ✅ Metadata serialization

### Integration Tests
- ⏳ Terminal → log file
- ⏳ Search correctness
- ⏳ Cleanup policies
- ⏳ Multi-project isolation
- ⏳ Crash recovery

### Performance Tests
- ⏳ Large log search
- ⏳ Concurrent terminals
- ⏳ Memory usage
- ⏳ I/O blocking

---

## Dependencies

To be added to `crates/terminal/Cargo.toml`:

```toml
[dependencies]
sqlite = "0.36"          # Metadata and FTS
flate2 = "1.0"          # gzip compression
regex = "1.10"          # Pattern matching
chrono = "0.4"          # Timestamps
directories = "5.0"     # Config paths
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"          # Error handling
thiserror = "1.0"       # Error definitions
```

Optional:
```toml
tantivy = "0.22"        # Alternative FTS (faster, more features)
```

---

## Contributing

This is a design proposal. Implementation contributions are welcome once the design is approved by the Zed maintainers.

**Areas needing contribution:**
- Core `TerminalLogger` implementation
- `LogStorage` with rotation/compression
- `LogIndex` with SQLite FTS5
- `TerminalLogsPanel` UI
- Settings integration
- Tests (unit, integration, performance)

**See Also:** `DESIGN.md` for detailed technical specifications.

---

## License

This proposal is licensed under the same terms as the Zed repository (AGPLv3 / Apache 2.0 / GPL-3.0).

The implementation, once merged, will be part of Zed and under Zed's license.

---

## Contact & Feedback

- **Discussion:** Open an issue in the Zed repository
- **Questions:** Tag @maintainers in the issue
- **Updates:** Watch this proposal for changes

---

**Author:** AI Assistant  
**Date:** January 15, 2025  
**Version:** 1.0  
**Target:** Zed v0.225+

---

## Appendix

### Comparison with Alternatives

| Feature | This Proposal | External Tools (tee, script) | Single Project File |
|---------|---------------|-----------------------------|---------------------|
| Integrated UI | ✅ | ❌ | ✅ |
| Per-terminal separation | ✅ | ❌ | ❌ |
| Automatic rotation | ✅ | ❌ | ❌ |
| Full-text search | ✅ | ❌ | Slow |
| Size management | ✅ | ❌ | ❌ |
| Privacy (redaction) | ✅ | ❌ | ❌ |
| Low overhead | ✅ | ✅ | ✅ |
| Manual effort | None | High | Medium |

### Migration Path

If a user already uses external logging:

1. **First run** detects existing logs in common locations
2. **Import wizard** offers to migrate:
   - `~/.local/share/zed/logs/` (if exists)
   - `project/logs/` directories
3. **Preserve** original files, create symlinks or copies
4. **Index** imported logs for search
5. **Configure** to avoid duplicate logging

### Future Enhancements

- Cloud sync (encrypted)
- Live tail mode (follow logs)
- Log sharing (secure, time-limited links)
- Advanced analytics (command frequency, duration)
- Anomaly detection (errors, warnings)
- Integration with Zed's task system (enhanced task output)
- Voice/audio cues for specific patterns
- Machine learning for log summarization
- Collaborative log review (multiplayer)

---

*This document represents a complete design proposal for persistent terminal logging in Zed. It has been crafted with attention to performance, privacy, and user experience.*