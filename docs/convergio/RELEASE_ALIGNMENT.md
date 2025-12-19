# Convergio Studio Release Alignment

This document describes how Convergio Studio releases align with upstream Zed releases.

## Versioning Strategy

Convergio Studio uses **independent semantic versioning** that tracks Convergio-specific features:

- **CONVERGIO_VERSION**: Our feature version (e.g., `1.0.0`)
- **Zed Base**: The upstream Zed version we're based on (tracked in `Cargo.toml`)

### Version Format

```
Convergio v{CONVERGIO_VERSION} (based on Zed v{ZED_VERSION})
```

Example: `Convergio v1.0.0 (based on Zed v0.61)`

## Upstream Sync Process

### Step 1: Track Upstream

```bash
# Add upstream remote (one-time)
git remote add upstream https://github.com/zed-industries/zed.git

# Fetch latest
git fetch upstream
```

### Step 2: Review Changes

```bash
# See what's new
git log --oneline main..upstream/main | head -50

# Check for conflicts with convergio crates
git diff main..upstream/main -- crates/
```

### Step 3: Merge Upstream

```bash
# Create sync branch
git checkout -b sync/zed-v0.62

# Merge with strategy
git merge upstream/main --no-commit

# Resolve conflicts, prioritizing:
# 1. Upstream changes to core Zed
# 2. Convergio changes to convergio-specific crates
```

### Step 4: Test & Validate

```bash
# Build
cargo build --release -p zed

# Run tests
cargo test -p agent_ui -p acp_tools -p git_graph

# Manual test Convergio features
```

### Step 5: Update Tracking

1. Update `CONVERGIO_CHANGELOG.md` with new Zed version
2. Bump `CONVERGIO_VERSION` if needed
3. Commit and create PR

## Convergio-Specific Crates

These crates are Convergio additions and should NOT be overwritten during upstream sync:

| Crate | Purpose |
|-------|---------|
| `crates/acp_tools` | Agent Communication Protocol tools |
| `crates/acp_thread` | Thread management for ACP |
| `crates/git_graph` | Git graph visualization |

## Files Modified from Upstream

These files have Convergio modifications and need careful merge:

| File | Modification |
|------|-------------|
| `Cargo.toml` | Added convergio crates to workspace |
| `crates/zed/Cargo.toml` | Added convergio dependencies |
| `crates/zed/src/main.rs` | Added convergio panel init |
| `crates/git_ui/src/lib.rs` | Added git_graph_view module |
| `crates/git_ui/src/git_ui.rs` | Added git_graph_view init |
| `README.md` | Added Convergio section |

## Release Checklist

### Minor Release (feature additions)

- [ ] All convergio crates compile
- [ ] Existing tests pass
- [ ] New features documented in CONVERGIO_CHANGELOG.md
- [ ] CONVERGIO_VERSION bumped
- [ ] README.md updated if needed

### Major Release (breaking changes or major upstream sync)

- [ ] All items from minor release checklist
- [ ] Version mapping updated in CONVERGIO_CHANGELOG.md
- [ ] Migration guide written if breaking changes
- [ ] Full manual test of all Convergio features

### Upstream Sync Release

- [ ] Sync branch created and tested
- [ ] Conflicts resolved properly
- [ ] Convergio features still work
- [ ] CONVERGIO_CHANGELOG.md updated with base version

## CI/CD (Planned)

Future GitHub Actions workflow will:

1. Build on macOS, Linux, Windows
2. Run convergio-specific tests
3. Create releases with proper naming
4. Generate release notes from CONVERGIO_CHANGELOG.md

## Support Matrix

| Platform | Status | Notes |
|----------|--------|-------|
| macOS (Apple Silicon) | Supported | Primary development platform |
| macOS (Intel) | Supported | |
| Linux | Supported | |
| Windows | Supported | |
