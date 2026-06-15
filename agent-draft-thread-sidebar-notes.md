# Agent sidebar draft thread notes

This file summarizes the branch context, fixes attempted, product/behavior debates, and remaining open questions so the branch can be restarted with a cleaner implementation.

## Original problem

The branch was addressing agent sidebar behavior for draft threads in linked worktree project groups.

A linked worktree project group can have multiple workspaces/panels, for example:

- Workspace A: main worktree
- Workspace B: linked worktree

The sidebar groups them together, but each workspace can have its own `AgentPanel` state. This created several edge cases around empty draft thread rows.

## Key behavior decisions

### Empty draft visibility

Desired behavior:

- The sidebar should show an empty draft placeholder only for the currently active workspace's active agent panel.
- Empty draft placeholders from inactive linked worktree panels should not be visible.
- Navigating away from an empty draft should effectively remove that placeholder from the sidebar.
- Drafts with content should still be preserved and shown because they contain user-entered state.

Rejected behavior:

- Showing one empty draft placeholder per workspace panel in the same project group.

Why:

That caused stale rows such as an empty draft from Workspace B remaining visible while the user was focused on a thread from Workspace A. This felt incorrect because the sidebar was surfacing a draft the user was not currently viewing.

Implementation direction:

Use only the currently active workspace's `AgentPanel` active thread id for empty-draft visibility:

```rust
let active_panel_thread_id = active_workspace
    .as_ref()
    .and_then(|ws| ws.read(cx).panel::<AgentPanel>(cx))
    .and_then(|panel| panel.read(cx).active_thread_id(cx));
```

Then retain empty draft rows only when their `thread_id` matches this active panel thread id, unless a pending thread activation is in progress.

### Drafts with content

Desired behavior:

- Drafts with content should remain visible even when inactive.
- They hold user-typed state, so they should not be discarded simply because the user navigated elsewhere.

Important distinction:

- Empty drafts are ephemeral UI placeholders.
- Drafts with content represent user data.

### Replacement draft creation when removing drafts

There was a collateral issue:

1. Start a new thread in the main worktree workspace.
2. Type something, creating a draft with content.
3. Switch to a linked worktree workspace and create a new empty draft.
4. While focused in the linked worktree workspace, discard the draft from the main workspace.

Previously, discarding the inactive draft could auto-create a new empty replacement draft in the main workspace, even though the user did not explicitly start one there.

Desired behavior:

- Only activate/create a replacement draft when the removed draft was actually the sidebar's active entry.
- Removing an inactive draft should remove it without synthesizing a new empty draft in that inactive workspace.

This means passing `was_active` into the remove path for whether the panel should activate a replacement draft.

## Sorting debate

A later edge case appeared after rolling back the multi-panel active draft filter:

1. User is in Workspace A.
2. User switches to an active thread from Workspace B.
3. User creates a new empty draft in Workspace B.
4. The empty draft correctly appears at the top.
5. User types into it.
6. The draft becomes `DraftKind::WithContent` and drops below another thread, even though it is the current active draft.

Root cause:

- Empty drafts were pinned to the top via `DateTime::<Utc>::MAX_UTC`.
- Once typed into, the draft becomes `DraftKind::WithContent`.
- Content drafts sort by `metadata.interacted_at.unwrap_or(metadata.updated_at)`.
- Typing into a draft does not update metadata timestamps.

Important performance decision:

Do not update `ThreadMetadata.updated_at` or `interacted_at` on every keystroke. That would create noisy metadata writes and sidebar rebuilds while typing.

Preferred sorting behavior:

- Empty draft rows should appear at the top.
- The active draft row should appear at the top, whether empty or with content.
- Parked with-content drafts should sort normally by metadata time.
- Real threads should sort normally by metadata time.

Implementation direction:

Thread the active panel thread id into `push_entries_by_display_time`, and treat draft rows matching that active id as `DateTime::<Utc>::MAX_UTC`.

Conceptually:

```rust
ListEntry::Thread(thread)
    if thread.draft.is_some()
        && Some(thread.metadata.thread_id) == active_panel_thread_id =>
{
    DateTime::<Utc>::MAX_UTC
}
ListEntry::Thread(thread) if thread.draft == Some(DraftKind::Empty) => {
    DateTime::<Utc>::MAX_UTC
}
```

This is cheap: compute the active thread id once per rebuild, then do an `Option<ThreadId>` equality check during sorting.

## Timestamp debate

Another QA issue appeared:

- Create a new empty draft.
- Type into it.
- The now-with-content draft showed a timestamp like `8m` immediately.

Root cause:

- Sorting had been fixed to pin the active draft.
- Rendering still used `format_history_entry_timestamp(Self::thread_display_time(&thread.metadata))` for non-empty drafts.
- Since typing does not update metadata timestamps, the visible timestamp could be stale.

Initial proposed timestamp rule:

- Empty drafts: no timestamp.
- Active drafts, empty or with content: no timestamp.
- Parked with-content drafts: show normal historical timestamp.
- Real threads: show normal timestamp.

Then another QA issue appeared:

- Navigate away from a with-content draft.
- It still shows the stale timestamp.

Current open question / likely preferred behavior:

All draft thread rows should probably show no timestamp.

Suggested final split:

Sorting:

- Empty draft: top.
- Active draft: top.
- Parked content draft: normal sort position.
- Real thread: normal sort position.

Timestamp display:

- Any draft: no timestamp.
- Real thread: timestamp.

Potential implementation:

```rust
fn thread_timestamp(thread: &ThreadEntry) -> SharedString {
    if thread.draft.is_some() {
        SharedString::default()
    } else {
        format_history_entry_timestamp(Self::thread_display_time(&thread.metadata)).into()
    }
}
```

If keeping an `is_active` parameter for call-site symmetry, it would be unused unless the rule changes again.

## Tests discussed / added during the branch

### Empty draft visibility regression

Test intent:

- Main workspace and linked worktree workspace in the same project group.
- Both panels can internally have active empty drafts.
- Only the active workspace panel's empty draft should be visible.
- If the active workspace panel navigates away to a real thread, no inactive workspace empty draft should leak into the sidebar.
- Switching to the linked worktree workspace makes that workspace's empty draft visible.

Suggested/used name:

```rust
test_only_actively_viewed_empty_draft_is_visible_in_sidebar
```

### Inactive draft removal regression

Test intent:

- Discarding a draft from an inactive workspace should not create a replacement empty draft there.
- This still matters even after rolling back the filter because the draft being discarded has content, so it is visible/selectable.

Suggested better name:

```rust
test_discarding_inactive_content_draft_does_not_create_empty_replacement
```

The existing name was:

```rust
test_discarding_inactive_workspace_draft_does_not_create_empty_replacement
```

Clarification:

This test is not about selecting an inactive empty draft from the sidebar. That would no longer be visible. It is about selecting an inactive draft with content.

### Active draft sorting/timestamp regression

Test intent:

- Create a future-dated competing real thread.
- Open a new empty draft.
- Assert the empty draft is above the future-dated thread.
- Type into the draft.
- Assert it becomes `DraftKind::WithContent`.
- Assert it remains above the future-dated thread.
- Assert draft timestamp behavior according to the final desired rule.

Suggested/used name:

```rust
test_active_draft_with_content_stays_above_newer_thread
```

If the timestamp rule becomes “all drafts have no timestamp,” extend this test or add another one:

1. Type into active draft.
2. Navigate away to a real thread.
3. Assert the parked content draft still has no timestamp.
4. Assert it is no longer pinned unless it is still the active draft.

## PR description direction

The earlier PR description no longer matched the final behavior because it implied the fix was about preserving empty draft rows from inactive linked worktree panels.

Better short description:

```md
Follow up to #57692.

This PR refines agent draft thread behavior in linked worktree project groups.

The sidebar now only shows an empty draft placeholder for the currently active workspace's agent panel. Empty drafts from inactive linked worktree panels are hidden, while drafts with content remain visible so user-entered text is preserved.

It also keeps the active draft pinned at the top after the user starts typing. Previously, typing into an empty draft changed it into a content draft, causing it to sort by its unchanged metadata timestamp and potentially drop below another newer thread.

Finally, discarding a draft from an inactive workspace no longer creates a replacement empty draft there. Replacement drafts are only activated when the removed draft was the sidebar's active entry.

Release Notes:

- Fixed stale or incorrectly sorted agent draft rows in linked worktree project groups.
```

If the timestamp fix lands too, update the middle section to mention timestamps:

```md
Draft rows also no longer show stale timestamps while they represent unsent draft state. Empty and active draft rows stay at the top without a timestamp; parked content drafts preserve user-entered text without implying a misleading last-updated time.
```

If the final timestamp rule is “all drafts have no timestamp,” prefer:

```md
Draft rows no longer show timestamps, because draft prompt edits do not update thread metadata timestamps. This avoids showing stale times for unsent draft state while preserving normal timestamps for real threads.
```

## Validation run during the branch

Commands that passed after the active-draft sorting work:

```sh
cargo -q test -p sidebar test_active_draft_with_content_stays_above_newer_thread -- --nocapture
cargo -q test -p sidebar test_only_actively_viewed_empty_draft_is_visible_in_sidebar -- --nocapture
cargo -q test -p sidebar test_discarding_inactive_workspace_draft_does_not_create_empty_replacement -- --nocapture
cargo -q test -p sidebar test_plus_button_parks_nonempty_draft -- --nocapture
```

Diagnostics were clean for:

```sh
crates/sidebar/src/sidebar.rs
crates/sidebar/src/sidebar_tests.rs
```

## Files touched during the branch

Primary files:

- `crates/sidebar/src/sidebar.rs`
- `crates/sidebar/src/sidebar_tests.rs`

Relevant related files read during investigation:

- `crates/agent_ui/src/draft_prompt_store.rs`
- `crates/agent_ui/src/thread_metadata_store.rs`
- `crates/agent_ui/src/conversation_view.rs`
- `crates/agent_ui/src/threads_archive_view.rs`

## Important implementation cautions

- Avoid updating thread metadata on every draft keystroke.
  - This would be wasteful and could cause excessive DB writes/sidebar rebuilds.
- Treat empty drafts as ephemeral placeholders.
- Treat with-content drafts as user state that must not be silently discarded.
- Be precise about “visible in sidebar” vs. “exists internally in an inactive panel.”
- The sidebar should derive empty-draft visibility from the active workspace panel, not all workspace panels in the project group.
- Replacement draft activation should happen only when removing the active draft, not inactive drafts.
