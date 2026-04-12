# Smoke testing pull requests

You are helping the user manually smoke-test a set of related pull requests in `/Users/nathan/src/zed`. The user will switch branches, you will run `cargo run 2>&1` (with a long timeout, at least 600000ms), and guide them through what to test for each PR. When they report results or stop the command, move to the next PR.

**Important:** The user's local SQLite DB may have schema from other branches (e.g. `thread_id` as PK). This causes harmless `ON CONFLICT clause does not match` errors in logs. Ignore these — they don't affect the features being tested.

## PR status

- **#53732 (`retained-workspaces`)** — Already tested ✅
- **#53733 (`agent-panel-overlay-split`)** — Next
- **#53734 (`introduce-thread-id`)** — After that
- **#53736 (`integrate-project-group-refactor`)** — Final integration test

## What each PR does and what to test

### PR #53733 — `agent-panel-overlay-split`

Branch: `agent-panel-overlay-split`

This splits `ActiveView` into `BaseView` + `OverlayView`. History and Settings are now overlays on top of the conversation instead of replacing it. The `previous_view` stashing is gone.

**Test plan:**
1. Open the agent panel, start a conversation (type something, send it)
2. Open History (click the history icon) — the conversation should be hidden behind the history view
3. Press Back or click History again — conversation should reappear exactly as you left it (same scroll position, same editor content)
4. Open Settings (gear icon) while in a conversation — same overlay behavior
5. Press Back — conversation returns
6. Open History, then open Settings from History — Settings should replace History as the overlay
7. Press Back — should return to the conversation (not to History)
8. Start a new thread while History is open — should dismiss the overlay and show the new thread
9. No crashes, no panics in the terminal output

### PR #53734 — `introduce-thread-id`

Branch: `introduce-thread-id`

This introduces `ThreadId` as the primary thread identity. `session_id` becomes optional. The sidebar now tracks threads by `ThreadActivation` instead of bare `SessionId`.

**Test plan:**
1. Open the agent panel, create a new thread, send a message — thread should appear in the sidebar
2. Create another thread — both should be listed
3. Click between threads in the sidebar — should switch correctly
4. Archive a thread (right-click → archive) — should move to archived
5. Open archived threads view, unarchive one — should restore
6. Delete an archived thread — should disappear
7. Close and reopen Zed — threads should persist and load correctly
8. Check sidebar thread list shows correct titles, timestamps, agent icons
9. No crashes, no panics

### PR #53736 — `integrate-project-group-refactor`

Branch: `integrate-project-group-refactor`

This merges all three PRs above. Test that everything works together.

**Test plan:**
1. All tests from #53733 (overlay behavior)
2. All tests from #53732 (workspace grouping, sidebar open/close, group collapse persistence)
3. All tests from #53734 (thread identity, sidebar thread list, archive/unarchive)
4. Combined scenario: open sidebar with multiple project groups, switch between projects, open agent panel, start threads in different projects, open History, switch projects while History is open, go back
5. No crashes, no panics

## How to run

For each PR, switch branches and run:
```
git checkout <branch>
cargo run 2>&1
```

Use `timeout_ms: 600000` (10 minutes). The user will interact with Zed and report back, then stop the command when done.
