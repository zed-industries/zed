# How to Submit This PR to Zed

This guide walks you through submitting the terminal logging feature to the Zed repository.

## Prerequisites

1. **GitHub Account** - You need a GitHub account with write access to fork zed-industries/zed
2. **Git** - Installed and configured
3. **Rust Toolchain** - Nightly recommended, with required components
4. ** Zed Development Environment** - Built from source at least once
5. **Time** - This is a large PR; expect review cycles

---

## Step 1: Fork and Clone Zed

```bash
# Fork the repository on GitHub first (via web UI)
# Then clone your fork:
git clone https://github.com/YOUR_USERNAME/zed.git
cd zed

# Add upstream remote:
git remote add upstream https://github.com/zed-industries/zed.git
```

---

## Step 2: Create Feature Branch

```bash
# From main branch:
git checkout main
git pull upstream main

# Create feature branch:
git checkout -b feat/terminal-logging
```

---

## Step 3: Copy Implementation Files

The design/implementation docs are in `OkOci/zed-terminal-logging-proposal/`. You need to implement them:

### Option A: Copy from Proposal (if implementations exist)
If the proposal contains complete implementations:
```bash
# Copy new crates
cp -r OkOci/zed-terminal-logging-proposal/crates/terminal-logging crates/
cp -r OkOci/zed-terminal-logging-proposal/crates/terminal-logs-panel crates/

# Copy modified files (merge carefully)
# ... you'll need to manually integrate changes
```

### Option B: Implement from Scratch (Recommended)
Use the `IMPLEMENTATION.md` guide to build the feature incrementally:

1. **Week 1-2**: Create `crates/terminal-logging/` and implement `TerminalLogger`
2. **Week 3**: Add `LogStorage` with rotation/compression
3. **Week 4**: Implement `LogIndex` with SQLite FTS
4. **Week 5-6**: Build `TerminalLogsPanel` UI
5. **Week 7**: Integrate with terminal, add commands, polish
6. **Week 8+**: Testing, optimization, documentation

**Commit frequently** with descriptive messages:
```bash
git add .
git commit -m "feat(terminal-logging): implement TerminalLogger with buffered writes"
git commit -m "feat(terminal-logging): add LogStorage with rotation"
# etc.
```

---

## Step 4: Update Dependencies

In relevant `Cargo.toml` files, add:

```toml
# crates/terminal-logging/Cargo.toml
[dependencies]
flate2 = "1.0"
regex = "1.10"
chrono = "0.4"
directories = "5.0"
sqlite = "0.36"
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
log = { workspace = true }
gpui = { workspace = true }
collections = { workspace = true }
util = { workspace = true }

# crates/terminal/Cargo.toml
[dependencies]
terminal-logging = { path = "../terminal-logging" }

# crates/terminal-logs-panel/Cargo.toml
[dependencies]
terminal-logging = { path = "../terminal-logging" }
gpui = { workspace = true }
```

---

## Step 5: Build and Test

```bash
# Build all crates
cargo build --all

# Run tests for terminal-logging
cargo test -p terminal-logging

# Run Zed and test manually:
cargo run --bin zed

# In Zed:
# - Open terminal, run commands
# - Close terminal
# - Open Logs Panel (Cmd+Shift+P → "Terminal: Open Logs")
# - Verify logs appear
# - Test search
# - Test settings
```

---

## Step 6: Write Tests

Add comprehensive tests:

```bash
# Unit tests in each crate
cargo test -p terminal-logging
cargo test -p terminal-logs-panel

# Integration test
cargo test --test terminal_logging_integration
```

**Minimum coverage:**
- Logger: buffering, rotation, compression, redaction
- Storage: cleanup, metadata management
- Index: search correctness, filters
- UI: panel rendering, interactions

---

## Step 7: Update Documentation

### User Documentation
Create `docs/terminal-logging.md`:

```markdown
# Terminal Logging

Zed can automatically save all terminal output to disk, allowing you to review past sessions, search through history, and manage logs.

## Enabling/Disabling

Terminal logging is enabled by default. To disable:

1. Open Settings (`Cmd+,`)
2. Search for "terminal logging"
3. Uncheck "Enable terminal logging"

## Viewing Logs

Open the Logs Panel:
- `Cmd+Shift+P` → "Terminal: Open Logs"
- Or click the 📋 icon in the status bar

## Searching

Use the search bar in the Logs Panel to find text across all logs. Supports:
- Plain text search
- Regex (enable regex mode)
- Filters by date, project, exit status

## Storage

Default location: `~/.config/zed/logs/terminal/`

You can change this in settings. Project-specific logs can be stored in `.zed/terminal-logs/` within your project.

## Privacy

Sensitive data (passwords, tokens, API keys) are automatically redacted. Custom patterns can be added in settings.

## Managing Storage

Logs are automatically:
- Rotated at 100 MB (configurable)
- Compressed after 30 days (configurable)
- Deleted after 90 days (configurable)

Use "Clear All Logs" in the Logs Panel settings to manually free space.

## Exporting

Right-click any log → Export As... → Choose format (Plain Text, Compressed, JSON)
```

### Settings Reference
Add to `docs/settings/terminal.md`:

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
      "redact_patterns": ["(?i)password\\s*=\\s*\\S+", ...]
    }
  }
}
```

---

## Step 8: Prepare PR Description

Copy the content from `PR_TEMPLATE.md` and customize:

1. Update issue numbers
2. Add actual screenshots (if available)
3. Update performance metrics with actual numbers from your testing
4. List any known limitations
5. Mention any breaking changes (likely none)

---

## Step 9: Push and Create PR

```bash
# Push your branch to your fork:
git push -u origin feat/terminal-logging
```

Then on GitHub:

1. Go to your fork: https://github.com/YOUR_USERNAME/zed
2. You should see a "Compare & pull request" button for `feat/terminal-logging`
3. Click it
4. Ensure:
   - **base**: `main`
   - **head**: `YOUR_USERNAME:feat/terminal-logging`
   - **title**: `feat(terminal): add persistent terminal logging`
5. Paste the PR template content
6. Add reviewers (if you know who maintains terminal)
7. Submit

---

## Step 10: Address Review Feedback

Be prepared for:

- **Code style feedback** - Zed has specific conventions
- **Performance concerns** - May need benchmarking
- **API design** - Might need to adjust interfaces
- **Testing** - May need more tests
- **Documentation** - May need more detail

**Respond promptly** to reviews. Make requested changes on new commits:

```bash
# Make changes based on feedback
git add .
git commit -m "review: address feedback on TerminalLogger API"
git push
```

---

## Step 11: Iterate

Large PRs often need multiple review rounds:

1. **Initial review** - Architecture, API design
2. **Detailed review** - Code quality, error handling
3. **Testing review** - Ensure coverage and performance
4. **Final review** - Merge readiness

**Be patient**. This is a substantial feature. It may take weeks or months to get merged.

---

## Step 12: Merge

Once approved:

1. **Rebase** on latest main (if needed):
   ```bash
   git fetch upstream
   git rebase upstream/main
   git push -f origin feat/terminal-logging
   ```

2. **Squash commits** (if requested):
   ```bash
   git rebase -i HEAD~N  # N = number of commits
   # Squash into logical commits
   git push -f
   ```

3. **Merge** - Maintainer will merge (or you can if you have write access):
   - Prefer "Squash and merge" for clean history
   - Or "Rebase and merge" to preserve individual commits

4. **Delete branch** - Both on GitHub and locally

---

## Alternative: Submit Issue First

If you're unsure about the design, consider:

1. **Open an issue** first with the design proposal (DESIGN.md)
2. **Gather feedback** from maintainers and community
3. **Revise design** based on feedback
4. **Then implement** and submit PR

This can save time if there are fundamental design concerns.

---

## Common Pitfalls

### 1. **Feature Too Large**
- **Problem**: PR is 50+ files, overwhelming reviewers
- **Solution**: Break into smaller PRs (logger first, then UI, then integration)

### 2. **Missing Tests**
- **Problem**: Reviewers ask "are there tests?"
- **Solution**: Write tests alongside implementation, not after

### 3. **Performance Regressions**
- **Problem**: Terminal becomes sluggish
- **Solution**: Benchmark continuously, use async I/O, profile with Instruments/perf

### 4. **Platform-Specific Code**
- **Problem**: Works on macOS but not Linux/Windows
- **Solution**: Test on all platforms, use cfg attributes appropriately

### 5. **Memory Leaks**
- **Problem**: Logs accumulate in memory
- **Solution**: Implement Drop correctly, use weak references, profile

---

## Success Criteria

Your PR will likely be accepted if:

✅ **Solves a real problem** - Users want persistent logs
✅ **Well-designed** - Clean architecture, separation of concerns
✅ **Well-tested** - High coverage, performance tests
✅ **Well-documented** - User and developer docs
✅ **Performant** - <1% overhead, <50 MB memory
✅ **Privacy-aware** - Redaction, user control
✅ **Maintainable** - Clean code, no hacks
✅ **Cross-platform** - Works on macOS, Linux, Windows
✅ **Follows Zed conventions** - Code style, error handling, async patterns

---

## Getting Help

- **Zed Discord**: #development channel
- **GitHub Discussions**: https://github.com/zed-industries/zed/discussions
- **Existing contributors**: Tag @maintainers in issues/PRs
- **This proposal**: Reference `OkOci/zed-terminal-logging-proposal/DESIGN.md` for design rationale

---

## Timeline Estimate

| Phase | Duration | Notes |
|-------|----------|-------|
| Implementation | 2-4 weeks | Depends on your familiarity with Zed codebase |
| Testing & Polish | 1 week | Bug fixes, performance tuning |
| Review Cycles | 2-4 weeks | Multiple rounds of feedback |
| **Total** | **4-8 weeks** | From first commit to merge |

---

## After Merge

1. **Celebrate** 🎉 - You contributed to a major open-source project!
2. **Monitor** - Watch for bug reports, be ready to fix
3. **Document** - Update any external docs, blog posts, tutorials
4. **Share** - Tweet, blog, tell friends about your contribution
5. **Continue contributing** - You're now a known contributor!

---

## Appendix: Quick Reference

### Useful Commands

```bash
# Build
cargo build --all

# Test
cargo test --all

# Format
cargo fmt --all

# Lint
cargo clippy --all

# Run Zed
cargo run --bin zed

# Check for outdated dependencies
cargo outdated

# Generate coverage report
cargo tarpaulin --out Xml
```

### Zed Codebase Tips

- **GPUI**: Zed's UI framework - see `crates/gpui/`
- **Settings**: Use `settings::Settings` trait, register in `SettingsStore`
- **Async**: Use `cx.spawn()`, `cx.background_spawn()`, avoid blocking
- **Error handling**: Use `anyhow::Result`, `thiserror::Error`, log with `log::error!`
- **Testing**: Use `gpui::TestAppContext`, `cx.executor().allow_parking()`

### File Locations

- Terminal code: `crates/terminal/src/terminal.rs`
- Settings: `crates/settings/src/`
- Workspace panels: `crates/workspace/src/`
- Command palette: `crates/command-palette/src/`

---

**Good luck with your PR!** 🚀

*May your code be bug-free and your reviews be swift.*