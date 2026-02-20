# Summary: Persistent Terminal Logging for Zed

## Overview

This proposal adds **persistent, searchable terminal logging** to the Zed code editor. Currently, terminal output is stored only in memory (`last_content`) and is lost when terminals close or Zed restarts. This feature would automatically save all terminal output to disk with comprehensive management, search, and privacy features.

---

## The Problem

- ❌ Terminal output disappears when closing terminals
- ❌ No way to search past command output
- ❌ Cannot retain build logs, debug sessions, or command history
- ❌ No audit trail of what was run

---

## The Solution

A complete terminal logging system with:

### Core Features
- ✅ **Automatic logging** - All terminal output saved to disk
- ✅ **Full-text search** - SQLite FTS5 across all logs
- ✅ **Size management** - Configurable limits with rotation
- ✅ **Compression** - gzip for old logs
- ✅ **Privacy** - Pattern-based redaction (passwords, tokens, API keys)
- ✅ **Low overhead** - <1% CPU, <50 MB memory per terminal

### User Interface
- **Logs Panel** - Browse all logs with search and filters
- **Preview Pane** - Read log content without leaving Zed
- **Status Bar** - Indicator showing logging status
- **Context Menus** - Right-click actions (export, delete, replay)
- **Command Palette** - Quick access to logs

---

## Architecture

### Components

1. **TerminalLogger** - Buffered writes, size tracking, rotation
2. **LogStorage** - File management, compression, cleanup
3. **LogIndex** - SQLite FTS5 search index
4. **LogManager** - Singleton managing all logs
5. **TerminalLogsPanel** - UI for browsing logs

### Data Flow

```
Terminal Output → TerminalLogger → LogStorage (file)
                                    ↓
                              LogIndex (incremental)
                                    ↓
                              LogManager (UI)
```

### Storage Structure

```
~/.config/zed/logs/terminal/
├── active/
│   ├── terminal-{uuid}.log
│   └── terminal-{uuid}.meta.json
├── archived/
│   ├── terminal-{uuid}-{timestamp}.log.gz
│   └── terminal-{uuid}-{timestamp}.meta.json
├── index/
│   └── log-{uuid}.index
└── logs.db (SQLite metadata + FTS)
```

---

## Configuration

### Default Settings

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

---

## Implementation Plan

### Phase 1: Core Logging (Week 1-2)
- Create `crates/terminal-logging/`
- Implement `TerminalLogger` with buffered writes
- Add metadata tracking
- Basic file rotation (size-based)
- Settings integration
- Unit tests

**Deliverable:** Terminal output written to files

### Phase 2: Storage Management (Week 3)
- Implement `LogStorage` with cleanup policies
- Add gzip compression
- Create SQLite `logs.db`
- Background cleanup task
- Size limit enforcement

**Deliverable:** Automatic cleanup and compression

### Phase 3: Search Index (Week 4)
- Implement `LogIndex` with SQLite FTS5
- Incremental indexing during writes
- Search API with filters
- Performance optimization

**Deliverable:** Fast full-text search

### Phase 4: UI Panel (Week 5-6)
- Create `TerminalLogsPanel`
- Log list with sorting/filtering
- Preview pane
- Context menus and actions
- Status bar indicator

**Deliverable:** Full browsing UI

### Phase 5: Integration Polish (Week 7)
- Command palette commands
- Terminal tab menu
- Settings UI
- Error handling and notifications
- Performance testing

**Deliverable:** Production-ready feature

### Phase 6: Advanced Features (Week 8+)
- Redaction engine
- Project-specific settings
- Extension API
- Migration tools

**Deliverable:** Advanced polish

---

## New Crates

- `crates/terminal-logging/` - Core logging engine
- `crates/terminal-logs-panel/` - UI panel

### Modified Crates

- `crates/terminal/` - Logger integration
- `crates/settings/` - Configuration
- `crates/workspace/` - Panel registration
- `crates/command-palette/` - Commands

---

## Key Design Decisions

1. **Plain text logs + sidecar metadata** - Human-readable, easy to inspect
2. **SQLite for index only** - Fast FTS, logs remain plain text
3. **Per-terminal files** - Easy isolation and management
4. **Buffered async writes** - Minimal performance impact
5. **Redaction by default** - Privacy-first approach

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Write throughput | >10,000 lines/sec |
| Search latency (100 MB) | <1 second |
| Memory per terminal | <50 MB |
| CPU overhead (10 terminals) | <5% |
| Disk I/O latency | <10 ms |

---

## Testing Strategy

### Unit Tests
- Logger buffering and flushing
- Size limit enforcement
- Rotation triggers
- Compression/decompression
- Redaction patterns
- Metadata serialization
- Storage cleanup
- Search correctness

### Integration Tests
- Terminal → log file
- Search returns correct results
- Multi-project isolation
- Crash recovery
- Settings changes

### Performance Tests
- Large log search speed
- Concurrent terminal overhead
- Memory usage
- I/O blocking

---

## Privacy & Security

- **Automatic redaction** of passwords, tokens, API keys
- **Custom patterns** - Users can add their own regex
- **Warnings** when logging sensitive environments
- **No cloud sync** - All logs stored locally
- **User control** - Can disable per-terminal or globally

---

## Success Criteria

- ✅ >50% of terminal users enable logging within 1 month
- ✅ <1% CPU overhead on typical workloads
- ✅ >99.9% of sessions successfully write logs
- ✅ Users find past output in <30 seconds
- ✅ Average user stays under 1 GB total logs

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Disk space exhaustion | Medium | High | Aggressive defaults, warnings, easy cleanup |
| Privacy leak | Medium | High | Redaction by default, clear warnings |
| Performance degradation | Low | Medium | Async I/O, buffering, extensive testing |
| Complex UI | Medium | Medium | Start simple, iterate on feedback |
| Migration issues | Low | Medium | Thorough testing, backup tools |

---

## Documentation

### User-Facing
- Feature guide
- Settings reference
- Troubleshooting
- Privacy guide

### Developer
- Architecture diagram
- API reference
- Contributing guide
- Performance tuning

---

## Open Questions

1. Should logs be included in project backups? → Yes, with warnings
2. Support cloud sync? → Out of scope for v1
3. Searchable from global search (Cmd+Shift+F)? → Yes, with filter
4. Live tail mode? → Yes, in preview pane
5. Log sharing? → Future enhancement
6. Multi-line ANSI sequences? → Strip for search, preserve display
7. Binary output? → Store as-is, mark as binary

---

## Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| External tools only (tee, script) | Not integrated, no UI |
| Single file per project | Hard to separate sessions, no rotation |
| SQLite only (no text files) | Hard to inspect manually |
| Memory-mapped only | Complex, platform differences |

---

## Next Steps

1. **Get feedback** on design from maintainers
2. **Implement Phase 1** as proof of concept
3. **Iterate** based on testing and feedback
4. **Full implementation** following phased plan
5. **Submit PR** with complete feature

---

## Files in This Proposal

- `DESIGN.md` - Complete technical design (762 lines)
- `IMPLEMENTATION.md` - Step-by-step implementation guide (1576 lines)
- `README.md` - User-facing feature overview (465 lines)
- `PR_TEMPLATE.md` - Pull request template (218 lines)
- `SUBMISSION_GUIDE.md` - How to submit the PR (458 lines)
- `SUMMARY.md` - This file

**Total:** ~3500 lines of comprehensive documentation

---

## Status

📋 **Proposal** - Seeking feedback from Zed maintainers

**Not yet implemented** - This is a design document with implementation plan.

---

## Contact

- **Discussion:** Open issue in zed-industries/zed
- **Questions:** Tag @maintainers
- **Updates:** Watch this proposal

---

**Author:** AI Assistant  
**Date:** 2025-01-15  
**Version:** 1.0  
**Target:** Zed v0.225+

---

*This proposal represents a complete, production-ready design for persistent terminal logging in Zed, with detailed implementation guidance, testing strategy, and documentation.*