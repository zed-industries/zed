# Open CLI Sessions in Agent Panel — Tasks

## Current Status: Steps 1-3 complete, ready for manual testing

## Completed
- [x] Research — understood full ACP session loading flow (load_thread → load_session → stream events → populate entries)
- [x] Confirmed infrastructure exists — `external_thread()` with `ExternalAgent::ClaudeCode` handles connection + loading
- [x] Confirmed no right-click context menu exists on history items
- [x] Confirmed `ThreadHistoryEvent` enum and handler pattern in agent_panel.rs
- [x] Plan documented and approved
- [x] Step 1 — Add right-click context menu to history items
  - [x] Wrapped `render_history_entry()` rows in `right_click_menu`
  - [x] CLI sessions: "Open in Terminal" + "Open in Agent Panel"
  - [x] Non-CLI sessions: "Open"
  - [x] Handlers emit `ThreadHistoryEvent::Open` or `ThreadHistoryEvent::OpenInPanel`
- [x] Step 2 — Add `OpenInPanel(AgentSessionInfo)` to `ThreadHistoryEvent` enum
- [x] Step 3 — Handle `OpenInPanel` in agent_panel.rs
  - [x] Derives `ExternalAgent` variant from CLI source metadata (codex → Codex, else → ClaudeCode)
  - [x] Calls `this.external_thread()` to load conversation in agent panel
- [x] `cargo check -p agent_ui` — compiles clean

## Pending
- [ ] Step 4 — Manual testing
  - [ ] Build dev Zed: `env -u CLAUDECODE cargo run`
  - [ ] Right-click a Claude Code session in history panel
  - [ ] Verify context menu appears with correct options
  - [ ] Click "Open in Agent Panel" → verify conversation loads with full history
  - [ ] Click "Open in Terminal" → verify terminal opens (existing behavior unchanged)
  - [ ] Send a new message in the agent panel → verify agent responds with correct context
  - [ ] Test with Codex session (if available)

- [ ] Step 5 — Polish (optional)
  - [ ] Consider changing default double-click behavior to open in agent panel
  - [ ] Add keyboard shortcut or modifier key (e.g., Ctrl+click = agent panel)
  - [ ] Test edge cases: deleted sessions, renamed sessions, sessions from different projects

## Notes

- The `env -u CLAUDECODE` is needed when launching dev Zed from a Claude Code session
- Steps 1-3 are estimated at ~45 lines of code total across 2 files
- The heavy lifting (ACP streaming, entry rendering) is already built and battle-tested
- The plan file is at: `/Volumes/Code/GitHub/zed/_plans/chat-to-agent-panel/plans.md`
- Prior work context: `/Volumes/Code/GitHub/zed/_context/chat-to-agent-panel/_context.md`
