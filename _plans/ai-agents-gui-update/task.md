# AI Agents GUI Update - Tasks

## Current Status: Ready to implement Step 1

## Completed (Prior Work)
- [x] ThreadContentEditor — `.jsonl` file viewer/editor with truncation-point selection
- [x] Codex CLI tab in history panel (left dock)
- [x] Codex resume command fix (`codex resume <id>` not `--resume`)
- [x] Terminal improvements (regular terminal instead of task terminal for Codex sessions)
- [x] ThreadContentEditor only shows USER/ASSISTANT entries (hides noise)
- [x] Codex session project scoping via `session_meta.payload.cwd`
- [x] Research complete — understood full edit/rewind/truncate chain
- [x] Plan approved

## In Progress
- [ ] **CURRENT:** Step 1 — Enable pencil button
  - [ ] Add `AcpSessionTruncator` struct to `crates/agent_servers/src/acp.rs`
  - [ ] Add `AgentSessionTruncate` impl for `AcpSessionTruncator` (no-op `run()`)
  - [ ] Add `truncate()` method to `impl AgentConnection for AcpConnection`
  - [ ] Add import: `use acp_thread::{AgentSessionTruncate, UserMessageId};`
  - [ ] `cargo check -p agent_servers` — verify compiles
  - [ ] `cargo test -p acp_thread` — verify existing tests pass
  - [ ] ← RESUME HERE

## Pending
- [ ] Step 2 — Background file truncation
  - [ ] Add `session_file_path()` default method to `AgentConnection` trait in `connection.rs`
  - [ ] Implement `session_file_path()` on `AcpConnection` in `acp.rs`
  - [ ] Make `sessions_dir_for_project()` public in `claude_code_sessions.rs`
  - [ ] Add `truncate_session_file()` async function in `acp_thread.rs`
  - [ ] Add background file truncation call in `rewind()` after `entries.truncate(ix)`
  - [ ] `cargo check -p acp_thread -p agent_servers -p agent`
  - [ ] `cargo test -p acp_thread`

- [ ] Step 3 — Manual testing
  - [ ] Build dev Zed: `env -u CLAUDECODE cargo run`
  - [ ] Test with Claude Code session: send messages → edit → verify
  - [ ] Test with Codex CLI session (if file path derivation works)
  - [ ] Verify `.jsonl` file truncated on disk
  - [ ] Verify session reopens correctly from history

- [ ] Step 4 — Codex file path support (if not done in Step 2)
  - [ ] Implement Codex session file path derivation or storage
  - [ ] Test with Codex sessions

## Notes

- The `env -u CLAUDECODE` is needed when launching dev Zed from a Claude Code session (see memory: `feedback_dev_build_claudecode_env.md`)
- Step 1 alone enables the pencil button and full edit flow (rewind + resend). File truncation (Step 2) is for persistence only.
- The plan file is at: `/Users/alesloas/.claude/plans/jiggly-sparking-sutton.md`
