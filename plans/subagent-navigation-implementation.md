# Subagent Navigation Implementation Plan

> ⚠️ **IMPORTANT**: Commit your changes as you go. This planning file itself should **NEVER** be checked in to git.

## Current Implementation Status (2025-01-16)

### ✅ COMPLETED - Core Implementation (Phases 1-7)

All core functionality has been implemented and passes `./script/clippy`:

1. **Action**: Added `NavigateToParentThread` action in `agent_ui.rs`
2. **Data Model**: Added `SubagentBreadcrumb` struct and `subagent_navigation_stack` field to `AcpThreadView`
3. **Navigation Methods**: 
   - `navigate_to_subagent()` - Navigate into a subagent
   - `navigate_to_parent()` - Go back to parent  
   - `navigate_to_ancestor()` - Go to specific ancestor
   - `displayed_thread()` - Get the currently displayed thread
   - `is_viewing_subagent()` - Check if viewing a subagent
   - `sync_list_state_with_displayed_thread()` - Sync list state when navigating
4. **Breadcrumb Rendering**: Added `render_subagent_breadcrumbs()` method
5. **Subagent Card Button**: Added fullscreen/maximize button to subagent cards
6. **Render Changes**: Breadcrumbs shown when viewing subagent, message editor hidden
7. **Test Support**: Added `navigate_into_subagent()` and `viewing_subagent()` methods

### ❌ BLOCKED - Visual Tests (Phase 8)

Visual tests were added to `visual_test_runner.rs` but are **not rendering content properly**. The agent panel shows an empty dark area instead of thread content.

**Root Cause Analysis:**
- The `AcpThreadView` has entries (verified via debug output: 2 entries - UserMessage and ToolCall)
- The thread state is "Ready" 
- However, `list_state.item_count()` returns 0, causing `has_messages` to be false
- When `has_messages` is false, the view renders `render_recent_history()` instead of the thread list
- The `list_state` is only updated via `handle_thread_event` when `NewEntry` events are processed
- There's a timing issue where entries are added to the thread but the events aren't properly processed in the test context

**What Was Tried:**
1. `allow_parking()` / `forbid_parking()` around async operations
2. `window.dispatch_action(ToggleZoom)` to zoom panel
3. Setting `dock: DockPosition::Left` and `default_width: px(500.0)` 
4. Multiple `run_until_parked()` and `advance_clock()` calls
5. Sending messages in window context
6. Calling `cx.notify()` on thread_view

**The Problem:**
The `list_state.splice_focusable()` call happens in `handle_thread_event` which requires the subscription to be active and events to flow through. In the test context, the events from `thread.send()` / `handle_session_update()` are emitted but not being received by the thread view's subscription.

### Next Steps to Complete

1. **Implement Dynamic Status Indicators (Phase 9)** - NEW REQUIREMENT
   - Currently the subagent panel header always shows a green checkmark
   - Need to show "generating..." animation when subagent is still running
   - Show green checkmark when completed successfully
   - Show red X when failed (using `ToolCallStatus::Failed`)
   - Parent thread breadcrumb should also show status (generating/complete/error)

2. **Fix Visual Test Event Processing** (BLOCKER)
   - Option A: Add a test-support method to `AcpThreadView` that manually syncs `list_state` with thread entries
   - Option B: Investigate why `cx.subscribe_in()` subscriptions aren't receiving events in the visual test context
   - Option C: Use a different test approach that doesn't rely on the full AgentPanel (render `AcpThreadView` directly)

3. **Generate Baseline Images**
   Once tests render correctly:
   ```bash
   UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests
   ```

4. **Verify Against Mockup**
   - Parent breadcrumb shows at top with correct icon and title
   - Keyboard shortcut visible in breadcrumb
   - Current subagent header shows below breadcrumb  
   - Subagent content renders correctly
   - Message editor is hidden when viewing subagent
   - Fullscreen button visible on subagent cards
   - **NEW:** Status indicators animate while generating, show checkmark/X when done

### Files Modified

- `zed7/crates/agent_ui/src/agent_ui.rs` - Added `NavigateToParentThread` action
- `zed7/crates/agent_ui/src/acp/thread_view.rs` - Core implementation (all navigation logic, breadcrumbs, UI changes)
- `zed7/crates/zed/src/visual_test_runner.rs` - Visual test scaffolding (incomplete/broken)

---


## Overview

Currently, subagent cards have an "Expand Subagent" button that toggles inline expansion of the subagent's content within the parent thread view. We want to add a new "fullscreen" navigation mode where clicking a button switches the UI to show the subagent's thread as the active view, with a breadcrumb bar showing the ancestor threads that can be clicked to navigate back.

**Key requirements:**
- When viewing a subagent, the parent thread (and any other subagents) keep running - this is purely a UI navigation change
- Support arbitrary nesting depth (up to `MAX_SUBAGENT_DEPTH = 4`)
- Ancestor threads appear as clickable breadcrumbs at the top
- Clicking an ancestor breadcrumb navigates back to that thread's view
- All thread state management continues working exactly as before
- The existing inline expansion behavior is kept alongside the new fullscreen navigation
- Hide the message editor text box when viewing a subagent (not the root thread)
- Keyboard shortcut `Cmd-Shift-T` to navigate back to parent, shown in the breadcrumb bar

## Clarified Requirements

1. **Nested subagents**: Support arbitrary levels, not just one. Breadcrumbs should show full ancestor chain.

2. **Message editor**: Hide the message editor text box entirely when displaying a subagent as the active thread. Only the root thread should have the message editor visible.

3. **Collapse button**: Ignore for now - don't include the `—` button in the subagent header.

4. **File change stats**: Hide the files changed display in the subagent header for now.

5. **Parent thread status**: No status indicator for parent thread in breadcrumb.

6. **Architecture**: Same UI elements for parent and child - just displaying different contents. Not separate message editors.

7. **Keyboard navigation**: Use `Cmd-Shift-T` to navigate back to parent. Show this shortcut in the parent breadcrumb bar.

8. **Existing inline expansion**: Keep the current expand/collapse behavior. Add an "Expand Subagent" icon button (fullscreen icon, no text, tooltip "Expand Subagent") that appears in the corner of the subagent card - visible both when expanded and collapsed.

## Architecture Analysis

### Current State

1. **`AgentPanel`** (`agent_panel.rs`):
   - Has `active_view: ActiveView` enum that can be `ExternalAgentThread { thread_view: Entity<AcpThreadView> }`
   - `set_active_view()` manages transitions between views
   - `render_toolbar()` renders the top bar with thread title, new thread menu, etc.

2. **`AcpThreadView`** (`thread_view.rs`):
   - Renders a single `AcpThread`
   - Has `expanded_subagents: HashSet<acp::SessionId>` for inline expansion
   - `render_subagent_card()` renders subagent cards with expand/collapse disclosure
   - `expand_subagent()` toggles inline expansion
   - Has `ThreadState::Ready { thread, ... }` holding the root thread

3. **`AcpThread`** (`acp_thread.rs`):
   - Contains tool calls, which can have `ToolCallContent::SubagentThread(Entity<AcpThread>)`
   - Each subagent is a full `Entity<AcpThread>` that runs independently

### Proposed Architecture

**Navigation State in `AcpThreadView`:**

```rust
/// Stack of subagent threads we've navigated into
/// When empty, we're viewing the root thread
/// When non-empty, the last item is the currently displayed subagent
subagent_navigation_stack: Vec<SubagentBreadcrumb>,

struct SubagentBreadcrumb {
    thread: Entity<AcpThread>,
    title: SharedString,
    _subscription: Subscription,
}
```

**Key insight**: The `ThreadState::Ready { thread, ... }` always holds the ROOT thread. The `subagent_navigation_stack` tracks which subagent (if any) we're currently viewing. This way:
- Root thread is always `self.thread()`
- Currently displayed thread is `self.displayed_thread()` (last in stack, or root if empty)
- Breadcrumbs show: root → stack[0] → stack[1] → ... → stack[n-1] (current)

## Implementation Plan

### Phase 1: Data Model Changes

**File: `zed7/crates/agent_ui/src/acp/thread_view.rs`**

1. Add new struct for breadcrumb:
```rust
struct SubagentBreadcrumb {
    thread: Entity<AcpThread>,
    title: SharedString,
    _subscription: Subscription,
}
```

2. Add field to `AcpThreadView`:
```rust
/// Navigation stack for viewing nested subagents
/// Empty = viewing root thread, non-empty = last item is current view
subagent_navigation_stack: Vec<SubagentBreadcrumb>,
```

3. Add action for navigation:
```rust
// In actions or at module level
actions!(agent, [NavigateToParentThread]);
```

### Phase 2: Navigation Logic

**File: `zed7/crates/agent_ui/src/acp/thread_view.rs`**

1. Add method to navigate into a subagent:
```rust
fn navigate_to_subagent(
    &mut self,
    subagent_thread: Entity<AcpThread>,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    let title = subagent_thread.read(cx).title();
    
    let subscription = cx.subscribe(&subagent_thread, |this, _, event, cx| {
        // Handle subagent events - forward title updates, etc.
        match event {
            AcpThreadEvent::TitleUpdated => {
                // Update title in breadcrumb if needed
                cx.notify();
            }
            AcpThreadEvent::NewEntry | AcpThreadEvent::EntryUpdated(_) => {
                cx.notify();
            }
            _ => {}
        }
    });
    
    self.subagent_navigation_stack.push(SubagentBreadcrumb {
        thread: subagent_thread,
        title,
        _subscription: subscription,
    });
    cx.notify();
}
```

2. Add method to navigate back to parent (or specific ancestor):
```rust
fn navigate_to_parent(&mut self, cx: &mut Context<Self>) {
    if !self.subagent_navigation_stack.is_empty() {
        self.subagent_navigation_stack.pop();
        cx.notify();
    }
}

fn navigate_to_ancestor(&mut self, depth: usize, cx: &mut Context<Self>) {
    // depth 0 = root thread, depth 1 = first subagent, etc.
    // Truncate stack to show ancestor at given depth
    if depth == 0 {
        self.subagent_navigation_stack.clear();
    } else {
        self.subagent_navigation_stack.truncate(depth);
    }
    cx.notify();
}
```

3. Add helper to get the currently displayed thread:
```rust
fn displayed_thread(&self) -> Option<&Entity<AcpThread>> {
    if let Some(breadcrumb) = self.subagent_navigation_stack.last() {
        Some(&breadcrumb.thread)
    } else {
        self.thread()
    }
}

fn is_viewing_subagent(&self) -> bool {
    !self.subagent_navigation_stack.is_empty()
}
```

### Phase 3: Action Registration

**File: `zed7/crates/agent_ui/src/acp/thread_view.rs`**

1. Register action and keybinding:
```rust
// In module init or appropriate place
cx.bind_keys([
    KeyBinding::new("cmd-shift-t", NavigateToParentThread, Some("AcpThreadView")),
]);
```

2. Add action handler in render or appropriate place:
```rust
.on_action(cx.listener(|this, _: &NavigateToParentThread, window, cx| {
    this.navigate_to_parent(cx);
}))
```

### Phase 4: UI Changes - Breadcrumb Bar

**File: `zed7/crates/agent_ui/src/acp/thread_view.rs`**

1. Add breadcrumb rendering for subagent navigation:
```rust
fn render_subagent_breadcrumbs(
    &self,
    window: &mut Window,
    cx: &Context<Self>,
) -> Option<impl IntoElement> {
    if self.subagent_navigation_stack.is_empty() {
        return None;
    }
    
    let root_title = self.thread()
        .map(|t| t.read(cx).title())
        .unwrap_or_else(|| "Thread".into());
    
    let focus_handle = self.focus_handle.clone();
    
    Some(
        v_flex()
            .w_full()
            // Root thread breadcrumb (always shown when viewing subagent)
            .child(
                h_flex()
                    .w_full()
                    .h(px(32.0)) // Or Tab::container_height(cx)
                    .px_2()
                    .gap_1()
                    .bg(cx.theme().colors().tab_bar_background)
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .cursor_pointer()
                    .child(Icon::new(IconName::ZedAgent).color(Color::Muted).size(IconSize::Small))
                    .child(
                        Label::new(root_title)
                            .size(LabelSize::Small)
                    )
                    .child(
                        div().flex_1() // Spacer
                    )
                    .child(
                        KeyBinding::for_action_in(&NavigateToParentThread, &focus_handle, window, cx)
                            .map(|kb| kb.into_any_element())
                            .unwrap_or_else(|| Empty.into_any_element())
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.navigate_to_ancestor(0, cx);
                    }))
            )
            // Intermediate ancestors (if depth > 1)
            .children(
                self.subagent_navigation_stack.iter().enumerate()
                    .take(self.subagent_navigation_stack.len().saturating_sub(1))
                    .map(|(i, breadcrumb)| {
                        let depth = i + 1;
                        h_flex()
                            .w_full()
                            .h(px(28.0))
                            .px_2()
                            .gap_1()
                            .bg(cx.theme().colors().surface_background)
                            .border_b_1()
                            .border_color(cx.theme().colors().border)
                            .cursor_pointer()
                            .child(Icon::new(IconName::ArrowRight).color(Color::Muted).size(IconSize::XSmall))
                            .child(Label::new(breadcrumb.title.clone()).size(LabelSize::Small))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.navigate_to_ancestor(depth, cx);
                            }))
                    })
            )
            // Current subagent header (last in stack)
            .when_some(self.subagent_navigation_stack.last(), |this, current| {
                let thread = current.thread.read(cx);
                let is_running = /* check if thread has pending operations */;
                
                this.child(
                    h_flex()
                        .w_full()
                        .h(px(32.0))
                        .px_2()
                        .gap_1()
                        .bg(cx.theme().colors().surface_background)
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .child(if is_running {
                            SpinnerLabel::new().size(LabelSize::Small).into_any_element()
                        } else {
                            Icon::new(IconName::Check)
                                .color(Color::Success)
                                .size(IconSize::Small)
                                .into_any_element()
                        })
                        .child(
                            Label::new(current.title.clone())
                                .size(LabelSize::Small)
                        )
                )
            })
    )
}
```

### Phase 5: UI Changes - Subagent Card Button

**File: `zed7/crates/agent_ui/src/acp/thread_view.rs`**

Modify `render_subagent_card()` to add the fullscreen button:

```rust
// In the header section of the card, add a button:
.child(
    IconButton::new(
        SharedString::from(format!("expand-subagent-{}-{}", entry_ix, context_ix)),
        IconName::Maximize // or appropriate fullscreen icon
    )
    .icon_size(IconSize::Small)
    .tooltip(|_, cx| Tooltip::text("Expand Subagent", cx))
    .on_click(cx.listener({
        let thread = thread.clone();
        move |this, _, window, cx| {
            this.navigate_to_subagent(thread.clone(), window, cx);
        }
    }))
)
```

The button should appear in the card header, next to the disclosure chevron, visible in both expanded and collapsed states.

### Phase 6: Modify Main Render

**File: `zed7/crates/agent_ui/src/acp/thread_view.rs`**

In the `Render` implementation:

1. Add breadcrumbs at the top when viewing a subagent
2. Conditionally hide message editor when viewing subagent
3. Render the displayed thread's content (not always root)

```rust
impl Render for AcpThreadView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_viewing_subagent = self.is_viewing_subagent();
        
        // ... existing setup ...
        
        v_flex()
            .size_full()
            // Add breadcrumbs when viewing subagent
            .children(self.render_subagent_breadcrumbs(window, cx))
            // Thread content - use displayed_thread() instead of thread()
            .child(self.render_thread_content(window, cx))
            // Hide message editor when viewing subagent
            .when(!is_viewing_subagent, |this| {
                this.child(self.render_message_editor(window, cx))
            })
    }
}
```

### Phase 7: Update Thread Content Rendering

Need to update methods that render thread entries to use `displayed_thread()`:
- `render_entries()` or equivalent
- Anywhere that reads from `self.thread()` for display purposes

### Phase 8: Visual Tests

**File: `zed7/crates/zed/src/visual_test_runner.rs`**

Add new visual test function:

```rust
fn run_subagent_navigation_visual_tests(
    app_state: Arc<AppState>,
    cx: &mut VisualTestAppContext,
    update_baseline: bool,
) -> Result<TestResult> {
    // 1. Set up workspace with agent panel
    // 2. Create a thread with a subagent tool call
    // 3. Capture screenshot: "subagent_navigation_parent_view" - parent thread with subagent card
    // 4. Click the expand button to navigate into subagent
    // 5. Capture screenshot: "subagent_navigation_subagent_view" showing:
    //    - Parent breadcrumb at top with Cmd-Shift-T shortcut
    //    - Subagent header with status icon and title
    //    - Subagent content (messages)
    //    - NO message editor at bottom
    // 6. Compare against expected layout
}
```

### Phase 9: Dynamic Status Indicators

**File: `zed7/crates/agent_ui/src/acp/thread_view.rs`**

Currently, the subagent breadcrumb header shows a green checkmark at all times. This needs to change to show dynamic status:

1. **Subagent header in expanded view** (when navigated into a subagent):
   - Show `SpinnerLabel` animation while the subagent's thread is generating (`ThreadStatus::Generating`)
   - Show green checkmark (`IconName::Check` with `Color::Success`) when completed successfully
   - Show red X (`IconName::XCircle` with `Color::Error`) when failed

2. **Parent thread breadcrumb** (shown when viewing a subagent):
   - Show `SpinnerLabel` animation while the parent thread is still generating
   - Show green checkmark when parent thread is idle/completed
   - Show red X if parent thread had an error

**Determining error state:**
- For subagents: Check the `ToolCallStatus` of the subagent tool call:
  - `ToolCallStatus::Pending | ToolCallStatus::InProgress` → generating animation
  - `ToolCallStatus::Completed` → green checkmark
  - `ToolCallStatus::Failed | ToolCallStatus::Rejected | ToolCallStatus::Canceled` → red X
- For parent thread: Use `thread.status()` which returns `ThreadStatus::Generating` or `ThreadStatus::Idle`
  - Could also track if an `AcpThreadEvent::Error` was emitted

**Changes to `render_subagent_breadcrumbs`:**

```rust
fn render_subagent_breadcrumbs(
    &self,
    _window: &mut Window,
    cx: &Context<Self>,
) -> Option<impl IntoElement> {
    if self.subagent_navigation_stack.is_empty() {
        return None;
    }

    let root_thread = self.thread()?;
    let root_title = root_thread.read(cx).title();
    let root_is_generating = root_thread.read(cx).status() == ThreadStatus::Generating;
    // Note: Could also track root_has_error via thread_error field

    let focus_handle = self.focus_handle.clone();

    // Render root/parent breadcrumb with dynamic status
    let root_status_icon = if root_is_generating {
        SpinnerLabel::new()
            .size(LabelSize::Small)
            .into_any_element()
    } else {
        Icon::new(IconName::Check)
            .color(Color::Success)
            .size(IconSize::Small)
            .into_any_element()
    };

    Some(
        v_flex()
            .w_full()
            .child(
                h_flex()
                    .id("subagent-breadcrumb-root")
                    .w_full()
                    .h(px(32.0))
                    .px_2()
                    .gap_1()
                    .bg(cx.theme().colors().tab_bar_background)
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .cursor_pointer()
                    .child(root_status_icon)  // Dynamic status instead of static ZedAgent icon
                    .child(Label::new(root_title).size(LabelSize::Small))
                    .child(div().flex_1())
                    .child(KeyBinding::for_action_in(
                        &NavigateToParentThread,
                        &focus_handle,
                        cx,
                    ))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.navigate_to_ancestor(0, cx);
                    })),
            )
            // ... rest of breadcrumb rendering
    )
}
```

**Changes to `render_subagent_card`:**

The subagent card already correctly shows spinner vs checkmark based on `tool_call_in_progress`. Need to add error state:

```rust
let icon = h_flex().w_4().justify_center().child(
    if is_running {
        SpinnerLabel::new()
            .size(LabelSize::Small)
            .into_any_element()
    } else if is_failed {
        Icon::new(IconName::XCircle)
            .size(IconSize::Small)
            .color(Color::Error)
            .into_any_element()
    } else {
        Icon::new(IconName::Check)
            .size(IconSize::Small)
            .color(Color::Success)
            .into_any_element()
    }
);
```

To determine `is_failed`, pass the `ToolCallStatus` to `render_subagent_card` and check for `Failed | Rejected | Canceled`.

### Test Verification Loop

After implementing, run:

```bash
cargo run -p zed --bin zed_visual_test_runner --features visual-tests
```

Then:
1. Open `target/visual_tests/subagent_navigation_*.png`
2. Compare visually to the mockup
3. Key things to verify:
   - Parent breadcrumb shows at top with correct icon and title
   - Keyboard shortcut (Cmd-Shift-T) visible in breadcrumb
   - Current subagent header shows below breadcrumb
   - Subagent content renders correctly
   - Message editor is hidden
   - Fullscreen button visible on subagent cards (both expanded and collapsed)
4. If not matching, iterate on the UI code
5. Repeat until visual match is achieved

## Detailed File Changes

### `zed7/crates/agent_ui/src/acp/thread_view.rs`

1. Add `SubagentBreadcrumb` struct (~line 90)
2. Add `subagent_navigation_stack: Vec<SubagentBreadcrumb>` field to `AcpThreadView` (~line 345)
3. Initialize `subagent_navigation_stack: Vec::new()` in `AcpThreadView::new()` (~line 530)
4. Add `navigate_to_subagent()` method
5. Add `navigate_to_parent()` method
6. Add `navigate_to_ancestor()` method
7. Add `displayed_thread()` helper
8. Add `is_viewing_subagent()` helper
9. Add `render_subagent_breadcrumbs()` method
10. Modify `render_subagent_card()` (~line 3575) to add fullscreen button
11. Modify `Render::render()` to:
    - Include breadcrumbs when viewing subagent
    - Hide message editor when viewing subagent
    - Use `displayed_thread()` for content rendering
12. Register `NavigateToParentThread` action with `cmd-shift-t` keybinding
13. Add action handler for `NavigateToParentThread`

### `zed7/crates/agent_ui/src/lib.rs` (or actions file)

1. Add `NavigateToParentThread` action definition

### `zed7/crates/zed/src/visual_test_runner.rs`

1. Add `run_subagent_navigation_visual_tests()` function
2. Register test in `run_visual_tests()` main test loop

## Implementation Order

1. [x] Add `SubagentBreadcrumb` struct and `subagent_navigation_stack` field
2. [x] Add navigation methods (`navigate_to_subagent`, `navigate_to_parent`, `navigate_to_ancestor`)
3. [x] Add `displayed_thread()` and `is_viewing_subagent()` helpers
4. [x] Add `NavigateToParentThread` action with keybinding
5. [x] Add action handler
6. [x] Add `render_subagent_breadcrumbs()` method
7. [x] Modify `render_subagent_card()` to add fullscreen icon button
8. [x] Update main render to show breadcrumbs and hide message editor conditionally
9. [x] Update thread content rendering to use `displayed_thread()`
10. [ ] **BLOCKED** Add visual tests - test scaffolding exists but content not rendering
11. [ ] Run visual tests: `cargo run -p zed --bin zed_visual_test_runner --features visual-tests`
12. [ ] Inspect PNGs in `target/visual_tests/`
13. [ ] Iterate until visual output matches mockup
14. [ ] Clean up, remove any debug code
15. [ ] Final test run to ensure all visual tests pass
16. [x] Run `./script/clippy` to check for issues - PASSES

## Icon Names Reference

Confirmed available in `zed7/crates/icons/src/icons.rs`:
- `IconName::Maximize` ✓ - Use this for the fullscreen/expand button
- `IconName::ArrowRight` ✓ - For intermediate breadcrumb arrows
- `IconName::ZedAgent` ✓ - For root thread icon in breadcrumb
- `IconName::Check` ✓ - For completed subagent status
- `IconName::ChevronDown` / `IconName::ChevronUp` ✓ - For existing expand/collapse

Also available that might be useful:
- `IconName::Exit` - Could be alternative for "go back"
- `IconName::ArrowLeft` - Could be used in breadcrumb navigation