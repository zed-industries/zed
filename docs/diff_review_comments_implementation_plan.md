# Diff Review Comments: Implementation Plan

> **Status: IMPLEMENTED** - This feature has been implemented. This document now serves as documentation of the implementation and tracks future enhancements.

This document describes the "stored review comments" feature in the diff review overlay. Comments are stored locally first, displayed in the overlay, and only sent to the Agent panel when the user explicitly clicks "Send Review to Agent."

## Table of Contents

1. [Background & Current State](#background--current-state)
2. [Goals & Requirements](#goals--requirements)
3. [Architecture Overview](#architecture-overview)
4. [Implementation Steps](#implementation-steps)
   - [Step 1: Define the Stored Comment Data Structure](#step-1-define-the-stored-comment-data-structure)
   - [Step 2: Add Comment Storage to Editor State](#step-2-add-comment-storage-to-editor-state)
   - [Step 3: Modify Submit Behavior to Store Comments Locally](#step-3-modify-submit-behavior-to-store-comments-locally)
   - [Step 4: Update Overlay Rendering to Show Stored Comments](#step-4-update-overlay-rendering-to-show-stored-comments)
   - [Step 5: Add Expandable Comments Section](#step-5-add-expandable-comments-section)
   - [Step 6: Add "Send Review to Agent" Button to Toolbar](#step-6-add-send-review-to-agent-button-to-toolbar)
   - [Step 7: Implement Batch Submission to Agent Panel](#step-7-implement-batch-submission-to-agent-panel)
   - [Step 8: Add Inline Edit Functionality](#step-8-add-inline-edit-functionality)
   - [Step 9: Add Delete Functionality](#step-9-add-delete-functionality)
   - [Step 10: Integrate User Avatar](#step-10-integrate-user-avatar)
5. [Testing Strategy](#testing-strategy)
   - [Visual Tests](#visual-tests)
   - [Unit Tests](#unit-tests)
6. [File Reference](#file-reference)
7. [Glossary](#glossary)

---

## Background & Current State

### What is the Diff Review Overlay?

The diff review overlay is a UI component that appears in the editor when viewing git diffs (via `ProjectDiff`). Users can click a "+" button in the gutter next to changed lines to open this overlay, which contains:

1. A text input field where users can type review comments
2. Hardcoded sample comments (currently for demonstration purposes)
3. Close and Submit buttons

### Current Behavior

When the user types a comment and presses Enter (or clicks the submit button):

1. The comment text and code location are captured
2. The comment is stored locally in `Editor.stored_review_comments` (keyed by hunk)
3. The overlay remains open and shows the new comment in an expandable list
4. The prompt editor is cleared for additional comments
5. Later, clicking "Send Review to Agent" in the toolbar:
   - Collects ALL comments from ALL hunks
   - Opens/focuses the Agent panel
   - Creates a new agent thread if needed
   - Inserts all comments + code as "creases" in the message editor
   - Clears the stored comments

### Key Files

| File                                        | Purpose                                                                                                          |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `crates/editor/src/editor.rs`               | Contains `DiffReviewOverlay`, `DiffHunkKey`, `StoredReviewComment`, comment storage, and overlay rendering logic |
| `crates/editor/src/actions.rs`              | Defines review-related actions (`SubmitDiffReviewComment`, `EditReviewComment`, etc.)                            |
| `crates/editor/src/element.rs`              | Registers action handlers for the editor element                                                                 |
| `crates/agent_ui/src/text_thread_editor.rs` | Handles `SendReviewToAgent` action for batch submission                                                          |
| `crates/agent_ui/src/acp/message_editor.rs` | Contains `insert_code_creases()` for inserting review comments as creases                                        |
| `crates/git_ui/src/project_diff.rs`         | `ProjectDiff` view and `ProjectDiffToolbar` with "Send Review to Agent" button                                   |
| `crates/zed/src/visual_test_runner.rs`      | Visual test infrastructure with diff review overlay tests                                                        |
| `crates/editor/src/editor_tests.rs`         | Unit tests for comment storage and overlay functionality                                                         |

---

## Goals & Requirements

Based on clarified requirements:

1. **Ephemeral storage**: Comments are stored in memory only—they are lost when the diff view is closed (no persistence across sessions)

2. **Per-hunk scoping**: Comments are scoped to the specific hunk/location where the overlay was opened. Each overlay shows only the comments for that hunk.

3. **Expandable comments section**: The overlay has a "N Comments" header that can expand/collapse to show/hide comments for that hunk

4. **User avatars**: Comments show the user's actual Zed account avatar (not generic icons)

5. **Chronological ordering**: Comments within a hunk are ordered by creation time (oldest first)

6. **Inline editing**: When editing a comment, the comment row itself becomes an editable text field (not moved to the prompt editor)

7. **Cross-file batch submission**: The "Send Review to Agent" button in the toolbar sends ALL comments across ALL files/hunks in the diff buffer to the Agent panel at once

8. **Toolbar placement**: The "Send Review to Agent" button lives in `ProjectDiffToolbar`, with a badge showing the total comment count

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         ProjectDiff                                  │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                  ProjectDiffToolbar                             │ │
│  │  [Stage] [Unstage] [↑] [↓] | [Stage All] [Commit]              │ │
│  │                                    [Send Review to Agent (5)]   │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                    SplittableEditor                           │   │
│  │  ┌────────────────────────────────────────────────────────┐  │   │
│  │  │                      Editor                             │  │   │
│  │  │                                                         │  │   │
│  │  │  stored_review_comments: HashMap<HunkKey, Vec<Comment>> │  │   │
│  │  │                                                         │  │   │
│  │  │  ┌─────────────────────────────────────────────────┐   │  │   │
│  │  │  │     DiffReviewOverlay (block at hunk row)        │   │  │   │
│  │  │  │                                                  │   │  │   │
│  │  │  │  [Avatar] [___prompt editor___] [X] [↵]          │   │  │   │
│  │  │  │                                                  │   │  │   │
│  │  │  │  ▼ 3 Comments                                    │   │  │   │
│  │  │  │  ┌────────────────────────────────────────────┐ │   │  │   │
│  │  │  │  │ [Avatar] "Comment text..." [✎] [🗑]        │ │   │  │   │
│  │  │  │  │ [Avatar] "Another comment" [✎] [🗑]        │ │   │  │   │
│  │  │  │  │ [Avatar] "Third comment"   [✎] [🗑]        │ │   │  │   │
│  │  │  │  └────────────────────────────────────────────┘ │   │  │   │
│  │  │  └─────────────────────────────────────────────────┘   │  │   │
│  │  └────────────────────────────────────────────────────────┘  │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

### Data Flow

```
User types comment → Press Enter → Store in Editor.stored_review_comments[hunk_key]
                                          ↓
                              Overlay re-renders showing new comment
                                          ↓
User clicks "Send Review to Agent" → Collect ALL comments from ALL hunks
                                          ↓
                              Send to Agent panel as creases
                                          ↓
                              Clear all stored comments
```

---

## Implementation Steps

### Step 1: Define the Stored Comment Data Structure

**File**: `crates/editor/src/editor.rs`

Create a struct to identify which hunk a comment belongs to, and a struct for the comment itself:

```rust
use std::time::Instant;

/// Identifies a specific hunk in the diff buffer.
/// Used as a key to group comments by their location.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiffHunkKey {
    /// The file path (relative to worktree) this hunk belongs to.
    pub file_path: Arc<Path>,
    /// The starting row of the hunk in the display.
    pub hunk_start_row: DisplayRow,
}

/// A review comment stored locally before being sent to the Agent panel.
#[derive(Clone)]
pub struct StoredReviewComment {
    /// Unique identifier for this comment (for edit/delete operations).
    pub id: usize,
    /// The comment text entered by the user.
    pub comment: String,
    /// The display row where this comment was added (within the hunk).
    pub display_row: DisplayRow,
    /// Anchors for the code range being reviewed.
    pub anchor_range: Range<Anchor>,
    /// Timestamp when the comment was created (for chronological ordering).
    pub created_at: Instant,
    /// Whether this comment is currently being edited inline.
    pub is_editing: bool,
}

impl StoredReviewComment {
    pub fn new(
        id: usize,
        comment: String,
        display_row: DisplayRow,
        anchor_range: Range<Anchor>,
    ) -> Self {
        Self {
            id,
            comment,
            display_row,
            anchor_range,
            created_at: Instant::now(),
            is_editing: false,
        }
    }
}
```

**Why `DiffHunkKey`?** Comments are scoped per-hunk, so we need a way to identify which hunk a comment belongs to. The combination of file path and hunk start row uniquely identifies a hunk.

**Why `Instant` for `created_at`?** Since comments are ephemeral (not persisted), we only need monotonic ordering within a session. `Instant` is simpler than `SystemTime`.

### Step 2: Add Comment Storage to Editor State

**File**: `crates/editor/src/editor.rs`

Add fields to the `Editor` struct:

```rust
use collections::HashMap;

pub struct Editor {
    // ... existing fields ...

    /// Stored review comments grouped by hunk.
    /// Key: DiffHunkKey identifying the hunk
    /// Value: Vec of comments for that hunk (ordered by creation time)
    stored_review_comments: HashMap<DiffHunkKey, Vec<StoredReviewComment>>,

    /// Counter for generating unique comment IDs.
    next_review_comment_id: usize,

    // ... existing fields ...
}
```

Initialize in `Editor::new_internal()`:

```rust
stored_review_comments: HashMap::default(),
next_review_comment_id: 0,
```

Add accessor and mutation methods:

```rust
impl Editor {
    /// Returns comments for a specific hunk, ordered by creation time.
    pub fn comments_for_hunk(&self, key: &DiffHunkKey) -> &[StoredReviewComment] {
        self.stored_review_comments
            .get(key)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Returns the total count of stored review comments across all hunks.
    pub fn total_review_comment_count(&self) -> usize {
        self.stored_review_comments.values().map(|v| v.len()).sum()
    }

    /// Returns the count of comments for a specific hunk.
    pub fn hunk_comment_count(&self, key: &DiffHunkKey) -> usize {
        self.stored_review_comments
            .get(key)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Adds a new review comment to a specific hunk.
    pub fn add_review_comment(
        &mut self,
        hunk_key: DiffHunkKey,
        comment: String,
        display_row: DisplayRow,
        anchor_range: Range<Anchor>,
        cx: &mut Context<Self>,
    ) -> usize {
        let id = self.next_review_comment_id;
        self.next_review_comment_id += 1;

        let stored_comment = StoredReviewComment::new(
            id,
            comment,
            display_row,
            anchor_range,
        );

        self.stored_review_comments
            .entry(hunk_key)
            .or_default()
            .push(stored_comment);

        cx.notify();
        id
    }

    /// Removes a review comment by ID from any hunk.
    pub fn remove_review_comment(&mut self, id: usize, cx: &mut Context<Self>) -> bool {
        for comments in self.stored_review_comments.values_mut() {
            if let Some(index) = comments.iter().position(|c| c.id == id) {
                comments.remove(index);
                cx.notify();
                return true;
            }
        }
        false
    }

    /// Updates a review comment's text by ID.
    pub fn update_review_comment(
        &mut self,
        id: usize,
        new_comment: String,
        cx: &mut Context<Self>,
    ) -> bool {
        for comments in self.stored_review_comments.values_mut() {
            if let Some(comment) = comments.iter_mut().find(|c| c.id == id) {
                comment.comment = new_comment;
                comment.is_editing = false;
                cx.notify();
                return true;
            }
        }
        false
    }

    /// Sets a comment's editing state.
    pub fn set_comment_editing(&mut self, id: usize, is_editing: bool, cx: &mut Context<Self>) {
        for comments in self.stored_review_comments.values_mut() {
            if let Some(comment) = comments.iter_mut().find(|c| c.id == id) {
                comment.is_editing = is_editing;
                cx.notify();
                return;
            }
        }
    }

    /// Takes all stored comments from all hunks, clearing the storage.
    /// Returns a Vec of (hunk_key, comments) pairs.
    pub fn take_all_review_comments(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Vec<(DiffHunkKey, Vec<StoredReviewComment>)> {
        cx.notify();
        std::mem::take(&mut self.stored_review_comments)
            .into_iter()
            .collect()
    }
}
```

### Step 3: Modify Submit Behavior to Store Comments Locally

**File**: `crates/editor/src/editor.rs`

Update the `DiffReviewOverlay` struct to track which hunk it belongs to:

```rust
pub(crate) struct DiffReviewOverlay {
    /// The display row where the overlay is anchored.
    pub display_row: DisplayRow,
    /// The anchor position for the block.
    pub anchor: Anchor,
    /// The block ID for the overlay.
    pub block_id: CustomBlockId,
    /// The editor entity for the review input.
    pub prompt_editor: Entity<Editor>,
    /// The hunk key this overlay belongs to.
    pub hunk_key: DiffHunkKey,
    /// Whether the comments section is expanded.
    pub comments_expanded: bool,
    /// Subscription to keep the action handler alive.
    _subscription: Subscription,
}
```

Update `show_diff_review_overlay()` to compute and store the hunk key:

```rust
pub fn show_diff_review_overlay(
    &mut self,
    display_row: DisplayRow,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    // Dismiss any existing overlay first
    self.dismiss_diff_review_overlay(cx);

    // Compute the hunk key for this overlay
    let hunk_key = self.compute_hunk_key(display_row, window, cx);

    // ... rest of existing setup code ...

    self.diff_review_overlay = Some(DiffReviewOverlay {
        display_row,
        anchor,
        block_id,
        prompt_editor: prompt_editor.clone(),
        hunk_key,
        comments_expanded: true, // Start expanded
        _subscription: subscription,
    });

    // ... rest of function ...
}

/// Computes the DiffHunkKey for a given display row.
fn compute_hunk_key(
    &self,
    display_row: DisplayRow,
    window: &mut Window,
    cx: &Context<Self>,
) -> DiffHunkKey {
    let snapshot = self.snapshot(window, cx);

    // Get the file path from the buffer at this row
    let display_point = DisplayPoint::new(display_row, 0);
    let buffer_point = snapshot
        .display_snapshot
        .display_point_to_point(display_point, Bias::Left);

    let file_path = self.buffer.read(cx)
        .snapshot(cx)
        .file_at_row(MultiBufferRow(buffer_point.row))
        .map(|file| file.path().clone())
        .unwrap_or_else(|| Arc::from(Path::new("")));

    // Find the start of the hunk containing this row
    // For now, use the display row itself; could be refined to find actual hunk boundaries
    let hunk_start_row = display_row; // TODO: Find actual hunk start

    DiffHunkKey {
        file_path,
        hunk_start_row,
    }
}
```

Modify `submit_diff_review_comment()` to store locally:

```rust
pub fn submit_diff_review_comment(&mut self, window: &mut Window, cx: &mut Context<Self>) {
    let Some(overlay) = self.diff_review_overlay.as_ref() else {
        return;
    };

    // Get the comment text from the prompt editor
    let comment_text = overlay.prompt_editor.read(cx).text(cx).trim().to_string();

    // Don't submit if the comment is empty
    if comment_text.is_empty() {
        return;
    }

    // Get the display row and create anchors
    let display_row = overlay.display_row;
    let hunk_key = overlay.hunk_key.clone();

    let snapshot = self.snapshot(window, cx);
    let display_point = DisplayPoint::new(display_row, 0);
    let buffer_point = snapshot
        .display_snapshot
        .display_point_to_point(display_point, Bias::Left);

    let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
    let line_start = Point::new(buffer_point.row, 0);
    let line_end = Point::new(
        buffer_point.row,
        buffer_snapshot.line_len(MultiBufferRow(buffer_point.row)),
    );

    let anchor_start = buffer_snapshot.anchor_after(line_start);
    let anchor_end = buffer_snapshot.anchor_before(line_end);

    // Store the comment locally
    self.add_review_comment(
        hunk_key,
        comment_text,
        display_row,
        anchor_start..anchor_end,
        cx,
    );

    // Clear the prompt editor but keep the overlay open
    if let Some(overlay) = self.diff_review_overlay.as_ref() {
        overlay.prompt_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
    }

    cx.notify();
}
```

### Step 4: Update Overlay Rendering to Show Stored Comments

**File**: `crates/editor/src/editor.rs`

Update `render_diff_review_overlay` signature and implementation:

```rust
fn render_diff_review_overlay(
    prompt_editor: &Entity<Editor>,
    hunk_key: &DiffHunkKey,
    comments: Vec<StoredReviewComment>,
    comments_expanded: bool,
    user_avatar: Option<Arc<RenderImage>>, // From user's Zed account
    cx: &mut BlockContext,
) -> AnyElement {
    let theme = cx.theme();
    let colors = theme.colors();
    let comment_count = comments.len();

    let avatar_size = px(20.);
    let action_icon_size = IconSize::XSmall;

    v_flex()
        .w_full()
        .bg(colors.editor_background)
        .border_b_1()
        .border_color(colors.border)
        .px_2()
        .pb_2()
        .gap_2()
        // Top row: prompt editor for new comments
        .child(render_prompt_row(
            prompt_editor,
            &user_avatar,
            avatar_size,
            action_icon_size,
            colors,
        ))
        // Expandable comments section
        .when(comment_count > 0, |el| {
            el.child(render_comments_section(
                comments,
                comments_expanded,
                &user_avatar,
                avatar_size,
                action_icon_size,
                colors,
            ))
        })
        .into_any_element()
}

fn render_prompt_row(
    prompt_editor: &Entity<Editor>,
    user_avatar: &Option<Arc<RenderImage>>,
    avatar_size: Pixels,
    action_icon_size: IconSize,
    colors: &ThemeColors,
) -> impl IntoElement {
    h_flex()
        .w_full()
        .items_center()
        .gap_2()
        .px_2()
        .py_1p5()
        .rounded_md()
        .bg(colors.surface_background)
        .child(render_avatar(user_avatar, avatar_size))
        .child(
            div()
                .flex_1()
                .border_1()
                .border_color(colors.border)
                .rounded_md()
                .bg(colors.editor_background)
                .px_2()
                .py_1()
                .child(prompt_editor.clone()),
        )
        .child(
            h_flex()
                .flex_shrink_0()
                .gap_1()
                .child(
                    IconButton::new("diff-review-close", IconName::Close)
                        .icon_color(ui::Color::Muted)
                        .icon_size(action_icon_size)
                        .tooltip(Tooltip::text("Close"))
                        .on_click(|_, window, cx| {
                            window.dispatch_action(Box::new(crate::actions::Cancel), cx);
                        }),
                )
                .child(
                    IconButton::new("diff-review-add", IconName::Return)
                        .icon_color(ui::Color::Muted)
                        .icon_size(action_icon_size)
                        .tooltip(Tooltip::text("Add comment"))
                        .on_click(|_, window, cx| {
                            window.dispatch_action(
                                Box::new(crate::actions::SubmitDiffReviewComment),
                                cx,
                            );
                        }),
                ),
        )
}

fn render_avatar(user_avatar: &Option<Arc<RenderImage>>, size: Pixels) -> impl IntoElement {
    div().size(size).flex_shrink_0().child(
        if let Some(avatar) = user_avatar {
            img(avatar.clone())
                .size(size)
                .rounded_full()
                .into_any_element()
        } else {
            Icon::new(IconName::Person)
                .size(IconSize::Small)
                .color(ui::Color::Muted)
                .into_any_element()
        }
    )
}
```

### Step 5: Add Expandable Comments Section

**File**: `crates/editor/src/editor.rs`

```rust
fn render_comments_section(
    comments: Vec<StoredReviewComment>,
    expanded: bool,
    user_avatar: &Option<Arc<RenderImage>>,
    avatar_size: Pixels,
    action_icon_size: IconSize,
    colors: &ThemeColors,
) -> impl IntoElement {
    let comment_count = comments.len();

    v_flex()
        .w_full()
        .gap_1()
        // Header with expand/collapse toggle
        .child(
            h_flex()
                .w_full()
                .items_center()
                .gap_1()
                .px_2()
                .py_1()
                .cursor_pointer()
                .rounded_md()
                .hover(|style| style.bg(colors.ghost_element_hover))
                .on_click(|_, window, cx| {
                    window.dispatch_action(
                        Box::new(crate::actions::ToggleReviewCommentsExpanded),
                        cx,
                    );
                })
                .child(
                    Icon::new(if expanded {
                        IconName::ChevronDown
                    } else {
                        IconName::ChevronRight
                    })
                    .size(IconSize::Small)
                    .color(ui::Color::Muted),
                )
                .child(
                    Label::new(format!(
                        "{} Comment{}",
                        comment_count,
                        if comment_count == 1 { "" } else { "s" }
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                ),
        )
        // Comments list (when expanded)
        .when(expanded, |el| {
            el.children(comments.into_iter().map(|comment| {
                render_comment_row(
                    comment,
                    user_avatar,
                    avatar_size,
                    action_icon_size,
                    colors,
                )
            }))
        })
}

fn render_comment_row(
    comment: StoredReviewComment,
    user_avatar: &Option<Arc<RenderImage>>,
    avatar_size: Pixels,
    action_icon_size: IconSize,
    colors: &ThemeColors,
) -> impl IntoElement {
    let comment_id = comment.id;
    let is_editing = comment.is_editing;

    h_flex()
        .w_full()
        .items_center()
        .gap_2()
        .px_2()
        .py_1p5()
        .rounded_md()
        .bg(colors.surface_background)
        .child(render_avatar(user_avatar, avatar_size))
        .child(
            if is_editing {
                // Inline edit mode: show an editable text field
                render_inline_edit_field(comment_id, &comment.comment, colors)
            } else {
                // Display mode: show the comment text
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(colors.text)
                    .child(comment.comment.clone())
                    .into_any_element()
            }
        )
        .when(!is_editing, |el| {
            el.child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new(
                            format!("diff-review-edit-{comment_id}"),
                            IconName::Pencil,
                        )
                        .icon_color(ui::Color::Muted)
                        .icon_size(action_icon_size)
                        .tooltip(Tooltip::text("Edit"))
                        .on_click(move |_, window, cx| {
                            window.dispatch_action(
                                Box::new(EditReviewComment { id: comment_id }),
                                cx,
                            );
                        }),
                    )
                    .child(
                        IconButton::new(
                            format!("diff-review-delete-{comment_id}"),
                            IconName::Trash,
                        )
                        .icon_color(ui::Color::Muted)
                        .icon_size(action_icon_size)
                        .tooltip(Tooltip::text("Delete"))
                        .on_click(move |_, window, cx| {
                            window.dispatch_action(
                                Box::new(DeleteReviewComment { id: comment_id }),
                                cx,
                            );
                        }),
                    ),
            )
        })
}

fn render_inline_edit_field(
    comment_id: usize,
    current_text: &str,
    colors: &ThemeColors,
) -> AnyElement {
    // This would need to create an inline editor entity
    // For simplicity, showing the structure - actual implementation
    // would need to manage an Entity<Editor> per editing comment
    div()
        .flex_1()
        .border_1()
        .border_color(colors.border)
        .rounded_md()
        .bg(colors.editor_background)
        .px_2()
        .py_1()
        .child(current_text.to_string()) // Placeholder - needs actual editor
        .into_any_element()
}
```

### Step 6: Add "Send Review to Agent" Button to Toolbar

**File**: `crates/editor/src/actions.rs`

Add new actions:

```rust
actions!(
    editor,
    [
        // ... existing actions ...

        /// Sends all stored review comments to the Agent panel.
        SendReviewToAgent,

        /// Toggles the expanded state of the comments section in the overlay.
        ToggleReviewCommentsExpanded,
    ]
);

/// Edits a stored review comment inline.
#[derive(Clone, PartialEq, Deserialize)]
pub struct EditReviewComment {
    pub id: usize,
}

impl_actions!(editor, [EditReviewComment]);

/// Deletes a stored review comment.
#[derive(Clone, PartialEq, Deserialize)]
pub struct DeleteReviewComment {
    pub id: usize,
}

impl_actions!(editor, [DeleteReviewComment]);

/// Confirms an inline edit of a review comment.
#[derive(Clone, PartialEq, Deserialize)]
pub struct ConfirmEditReviewComment {
    pub id: usize,
    pub new_text: String,
}

impl_actions!(editor, [ConfirmEditReviewComment]);

/// Cancels an inline edit of a review comment.
#[derive(Clone, PartialEq, Deserialize)]
pub struct CancelEditReviewComment {
    pub id: usize,
}

impl_actions!(editor, [CancelEditReviewComment]);
```

**File**: `crates/git_ui/src/project_diff.rs`

Add a method to get the total comment count:

```rust
impl ProjectDiff {
    /// Returns the total count of review comments across all hunks/files.
    pub fn total_review_comment_count(&self, cx: &App) -> usize {
        self.editor
            .read(cx)
            .primary_editor()
            .read(cx)
            .total_review_comment_count()
    }
}
```

Update `ProjectDiffToolbar::render()` to include the button:

```rust
impl Render for ProjectDiffToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(project_diff) = self.project_diff(cx) else {
            return div();
        };

        let focus_handle = project_diff.focus_handle(cx);
        let button_states = project_diff.read(cx).button_states(cx);
        let review_count = project_diff.read(cx).total_review_comment_count(cx);

        h_group_xl()
            .my_neg_1()
            .py_1()
            .items_center()
            .flex_wrap()
            .justify_between()
            // Left side: existing stage/unstage buttons
            .child(
                h_group_sm()
                    // ... existing stage/unstage children ...
            )
            // ... existing navigation arrows ...
            // ... existing stage all / commit buttons ...

            // Right side: "Send Review to Agent" button (only when there are comments)
            .when(review_count > 0, |el| {
                el.child(vertical_divider())
                    .child(
                        Button::new("send-review", "Send Review to Agent")
                            .icon(IconName::ZedAssistant)
                            .icon_position(IconPosition::Start)
                            .tooltip(Tooltip::text("Send all review comments to the Agent panel"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&SendReviewToAgent, window, cx)
                            }))
                            // Badge showing comment count
                            .child(
                                div()
                                    .ml_1()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded_md()
                                    .bg(cx.theme().colors().element_background)
                                    .text_xs()
                                    .child(format!("{}", review_count)),
                            ),
                    )
            })
    }
}
```

### Step 7: Implement Batch Submission to Agent Panel

**File**: `crates/agent_ui/src/text_thread_editor.rs`

Register and implement the handler:

```rust
impl TextThreadEditor {
    pub fn init(cx: &mut App) {
        // ... existing registrations ...
        cx.observe_new::<Workspace>(|workspace, _window, cx| {
            workspace
                .register_action(TextThreadEditor::handle_submit_diff_review_comment)
                .register_action(TextThreadEditor::handle_send_review_to_agent);
        })
        .detach();
    }

    /// Handles the SendReviewToAgent action from the ProjectDiff toolbar.
    /// Collects ALL stored review comments from ALL hunks and sends them
    /// to the Agent panel as creases.
    pub fn handle_send_review_to_agent(
        workspace: &mut Workspace,
        _: &SendReviewToAgent,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        use crate::acp::AcpThreadView;
        use git_ui::ProjectDiff;

        // Find the ProjectDiff item
        let Some(project_diff) = workspace.items_of_type::<ProjectDiff>(cx).next() else {
            log::warn!("No ProjectDiff found when sending review");
            return;
        };

        // Extract all stored comments from all hunks
        let (all_comments, buffer) = project_diff.update(cx, |project_diff, cx| {
            let editor = project_diff.editor().read(cx).primary_editor();
            let comments = editor.update(cx, |editor, cx| {
                editor.take_all_review_comments(cx)
            });
            let buffer = editor.read(cx).buffer().clone();
            (comments, buffer)
        });

        // Flatten: we have Vec<(DiffHunkKey, Vec<StoredReviewComment>)>
        // Convert to Vec<StoredReviewComment> for processing
        let comments: Vec<_> = all_comments
            .into_iter()
            .flat_map(|(_, comments)| comments)
            .collect();

        if comments.is_empty() {
            log::info!("No review comments to send");
            return;
        }

        log::info!("Sending {} review comments to Agent", comments.len());

        // Focus the agent panel
        workspace.focus_panel::<crate::AgentPanel>(window, cx);

        // Defer to ensure panel is focused and ready
        cx.defer_in(window, move |workspace, window, cx| {
            let Some(panel) = workspace.panel::<crate::AgentPanel>(cx) else {
                log::warn!("No agent panel found");
                return;
            };

            // Check if there's an active thread
            let has_active_thread =
                panel.update(cx, |panel, _cx| panel.active_thread_view().is_some());

            if !has_active_thread {
                // Create a new thread
                window.dispatch_action(Box::new(crate::NewThread), cx);
            }

            // Defer multiple times to ensure thread creation completes
            cx.defer_in(window, move |workspace, window, cx| {
                cx.defer_in(window, move |workspace, window, cx| {
                    let Some(panel) = workspace.panel::<crate::AgentPanel>(cx) else {
                        return;
                    };

                    panel.update(cx, |panel, cx| {
                        if let Some(thread_view) = panel.active_thread_view().cloned() {
                            // Build creases for all comments
                            let snapshot = buffer.read(cx).snapshot(cx);
                            let mut all_creases = Vec::new();

                            for comment in comments {
                                let point_range =
                                    comment.anchor_range.start.to_point(&snapshot)
                                    ..comment.anchor_range.end.to_point(&snapshot);

                                let mut creases = selections_creases(
                                    vec![point_range.clone()],
                                    snapshot.clone(),
                                    cx,
                                );

                                // Prepend user's comment to the code
                                for (code_text, crease_title) in &mut creases {
                                    *code_text = format!("{}\n\n{}", comment.comment, code_text);
                                    *crease_title = format!("Review: {}", crease_title);
                                }

                                all_creases.extend(creases);
                            }

                            // Insert all creases into the message editor
                            thread_view.update(cx, |thread_view, cx| {
                                thread_view.insert_code_crease(all_creases, window, cx);
                            });
                        }
                    });
                });
            });
        });
    }
}
```

### Step 8: Add Inline Edit Functionality

**File**: `crates/editor/src/editor.rs`

The inline edit functionality requires managing an editor per comment being edited. Add state tracking:

```rust
pub(crate) struct DiffReviewOverlay {
    // ... existing fields ...

    /// Editors for comments currently being edited inline.
    /// Key: comment ID, Value: Editor entity for inline editing
    pub inline_edit_editors: HashMap<usize, Entity<Editor>>,
}
```

Handle the edit action:

```rust
impl Editor {
    fn handle_edit_review_comment(
        &mut self,
        action: &EditReviewComment,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let comment_id = action.id;

        // Set the comment to editing mode
        self.set_comment_editing(comment_id, true, cx);

        // Create an inline editor for this comment if needed
        if let Some(overlay) = &mut self.diff_review_overlay {
            if !overlay.inline_edit_editors.contains_key(&comment_id) {
                // Find the comment text
                let comment_text = self.stored_review_comments
                    .values()
                    .flatten()
                    .find(|c| c.id == comment_id)
                    .map(|c| c.comment.clone())
                    .unwrap_or_default();

                // Create inline editor
                let inline_editor = cx.new(|cx| {
                    let mut editor = Editor::single_line(window, cx);
                    editor.set_text(&comment_text, window, cx);
                    editor
                });

                overlay.inline_edit_editors.insert(comment_id, inline_editor);
            }
        }

        cx.notify();
    }

    fn handle_confirm_edit_review_comment(
        &mut self,
        action: &ConfirmEditReviewComment,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_review_comment(action.id, action.new_text.clone(), cx);

        // Remove the inline editor
        if let Some(overlay) = &mut self.diff_review_overlay {
            overlay.inline_edit_editors.remove(&action.id);
        }
    }

    fn handle_cancel_edit_review_comment(
        &mut self,
        action: &CancelEditReviewComment,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_comment_editing(action.id, false, cx);

        // Remove the inline editor
        if let Some(overlay) = &mut self.diff_review_overlay {
            overlay.inline_edit_editors.remove(&action.id);
        }
    }
}
```

### Step 9: Add Delete Functionality

**File**: `crates/editor/src/editor.rs`

```rust
impl Editor {
    fn handle_delete_review_comment(
        &mut self,
        action: &DeleteReviewComment,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.remove_review_comment(action.id, cx);
    }
}
```

Register all action handlers:

```rust
// In Editor initialization or action registration
fn register_diff_review_actions(&mut self, cx: &mut Context<Self>) {
    self.register_action(Self::handle_edit_review_comment);
    self.register_action(Self::handle_delete_review_comment);
    self.register_action(Self::handle_confirm_edit_review_comment);
    self.register_action(Self::handle_cancel_edit_review_comment);
    self.register_action(Self::handle_toggle_review_comments_expanded);
}

fn handle_toggle_review_comments_expanded(
    &mut self,
    _: &ToggleReviewCommentsExpanded,
    _window: &mut Window,
    cx: &mut Context<Self>,
) {
    if let Some(overlay) = &mut self.diff_review_overlay {
        overlay.comments_expanded = !overlay.comments_expanded;
        cx.notify();
    }
}
```

### Step 10: Integrate User Avatar

> ⚠️ **EXTREMELY NOT OPTIONAL, CRUCIAL, ESSENTIAL, MUST BE DONE NO MATTER WHAT, MAY NOT BE SKIPPED FOR ANY REASON UNDER ANY CIRCUMSTANCES** ⚠️

The user's avatar MUST be displayed in the diff review overlay. The mockup clearly shows circular user avatars next to each comment and the prompt editor. Using a generic `Person` icon is NOT acceptable for the final implementation.

**Files to modify**:

- `crates/editor/src/editor.rs`
- `crates/git_ui/src/project_diff.rs`

#### Background: How Avatars Work in Zed

The user's avatar is accessed through the `UserStore`, which is part of the `AppState`. The pattern used throughout the codebase is:

```rust
// Get the UserStore from workspace or app_state
let user_store = workspace.user_store(); // or app_state.user_store

// Get the current user
let user = user_store.read(cx).current_user();

// Get the avatar URI
let avatar_uri = user.as_ref().map(|u| u.avatar_uri.clone());

// Render with the Avatar component
Avatar::new(avatar_uri.unwrap_or_default()).size(px(20.))
```

The `Avatar` component from `ui` handles loading and rendering the image from the URI.

#### Implementation Strategy

Since the diff review overlay is rendered via a block in the Editor, and the Editor doesn't have direct access to `UserStore`, we need to pass the avatar URI through. There are several approaches:

**Option A: Pass avatar URI when creating the overlay (Recommended)**

1. Modify `show_diff_review_overlay` to accept an optional avatar URI parameter
2. Have `ProjectDiff` (which has access to `Workspace` and thus `UserStore`) pass the avatar URI when showing the overlay
3. Store the avatar URI in `DiffReviewOverlay` and pass it to the render function

**Option B: Use a Global**

1. Create a global that stores the current user's avatar URI
2. Update it when the user logs in/out
3. Read from the global in the render function

**Option C: Look up through Workspace from BlockContext**

The `BlockContext` provides access to `&mut App`. We can potentially traverse to find the workspace and user store, but this is fragile.

#### Detailed Implementation (Option A - Recommended)

**Step 10.1: Add avatar_uri field to DiffReviewOverlay**

```rust
// In crates/editor/src/editor.rs

pub(crate) struct DiffReviewOverlay {
    pub display_row: DisplayRow,
    pub anchor: Anchor,
    pub block_id: CustomBlockId,
    pub prompt_editor: Entity<Editor>,
    pub hunk_key: DiffHunkKey,
    pub comments_expanded: bool,
    pub inline_edit_editors: HashMap<usize, Entity<Editor>>,
    /// The current user's avatar URI for display in comment rows.
    pub user_avatar_uri: Option<SharedString>,
    _subscription: Subscription,
}
```

**Step 10.2: Update show_diff_review_overlay signature**

```rust
// In crates/editor/src/editor.rs

pub fn show_diff_review_overlay(
    &mut self,
    display_row: DisplayRow,
    user_avatar_uri: Option<SharedString>,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    // ... existing code ...

    self.diff_review_overlay = Some(DiffReviewOverlay {
        display_row,
        anchor,
        block_id,
        prompt_editor: prompt_editor.clone(),
        hunk_key,
        comments_expanded: true,
        inline_edit_editors: HashMap::default(),
        user_avatar_uri,  // Store the avatar URI
        _subscription: subscription,
    });

    // ... rest of function ...
}
```

**Step 10.3: Update the render function to use Avatar component**

```rust
// In crates/editor/src/editor.rs

fn render_diff_review_overlay(
    prompt_editor: &Entity<Editor>,
    hunk_key: &DiffHunkKey,
    editor_handle: &WeakEntity<Editor>,
    cx: &mut BlockContext,
) -> AnyElement {
    // ... existing code ...

    // Get the avatar URI from the overlay
    let user_avatar_uri = editor_handle
        .upgrade()
        .and_then(|editor| {
            editor.read(cx).diff_review_overlay
                .as_ref()
                .and_then(|o| o.user_avatar_uri.clone())
        });

    // ... in the render, replace Icon::new(IconName::Person) with:
    .child(
        if let Some(avatar_uri) = &user_avatar_uri {
            Avatar::new(avatar_uri.clone())
                .size(avatar_size)
                .into_any_element()
        } else {
            Icon::new(IconName::Person)
                .size(IconSize::Small)
                .color(ui::Color::Muted)
                .into_any_element()
        }
    )
}
```

**Step 10.4: Update render_comment_row similarly**

```rust
fn render_comment_row(
    comment: StoredReviewComment,
    inline_editor: Option<Entity<Editor>>,
    user_avatar_uri: Option<SharedString>,
    avatar_size: Pixels,
    action_icon_size: IconSize,
    colors: &theme::ThemeColors,
) -> impl IntoElement {
    // ... replace the Person icon with Avatar ...
}
```

**Step 10.5: Update ProjectDiff to pass the avatar URI**

```rust
// In crates/git_ui/src/project_diff.rs

// When calling show_diff_review_overlay from ProjectDiff or its handlers,
// get the avatar URI from the workspace:

fn show_review_overlay_at_row(
    &mut self,
    display_row: DisplayRow,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    // Get the current user's avatar from the workspace
    let user_avatar_uri = self.workspace
        .upgrade()
        .and_then(|workspace| {
            let user_store = workspace.read(cx).user_store();
            user_store.read(cx).current_user()
                .map(|user| user.avatar_uri.clone())
        });

    self.editor.update(cx, |editor, cx| {
        editor.primary_editor().update(cx, |editor, cx| {
            editor.show_diff_review_overlay(display_row, user_avatar_uri, window, cx);
        });
    });
}
```

**Step 10.6: Add Avatar import**

Make sure to add the Avatar import in editor.rs:

```rust
use ui::{Avatar, /* other imports */};
```

#### Alternative: Fetch Avatar in Render via Global Lookup

If modifying the signature chain is too invasive, another approach is to look up the user store from a global or through the workspace. However, this requires the workspace to be available, which may not always be the case during rendering.

#### Testing

After implementing:

1. Sign in to Zed with a user account that has an avatar
2. Open the diff view (Uncommitted Changes)
3. Click the "+" button to add a review comment
4. Verify your avatar appears next to the prompt editor
5. Add a comment and verify your avatar appears next to the stored comment
6. Sign out and verify the fallback Person icon appears

---

## Testing Strategy

### Visual Tests

Add to `crates/zed/src/visual_test_runner.rs` in `run_diff_review_visual_tests()`:

```rust
// Test 6: Empty overlay (no comments, no hardcoded placeholders)
regular_window
    .update(cx, |workspace, window, cx| {
        let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
        if let Some(editor) = editors.into_iter().next() {
            editor.update(cx, |editor, cx| {
                // Dismiss any existing overlay
                editor.dismiss_diff_review_overlay(cx);
                // Show fresh overlay
                editor.show_diff_review_overlay(DisplayRow(1), window, cx);
            });
        }
    })
    .ok();

for _ in 0..3 {
    cx.advance_clock(Duration::from_millis(100));
    cx.run_until_parked();
}

let test6_result = run_visual_test(
    "diff_review_empty_overlay",
    regular_window.into(),
    cx,
    update_baseline,
)?;

// Test 7: Overlay with one stored comment
regular_window
    .update(cx, |workspace, window, cx| {
        let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
        if let Some(editor) = editors.into_iter().next() {
            editor.update(cx, |editor, cx| {
                // Type and submit a comment
                if let Some(prompt_editor) = editor.diff_review_prompt_editor().cloned() {
                    prompt_editor.update(cx, |pe, cx| {
                        pe.insert("This needs better error handling", window, cx);
                    });
                }
                editor.submit_diff_review_comment(window, cx);
            });
        }
    })
    .ok();

for _ in 0..3 {
    cx.advance_clock(Duration::from_millis(100));
    cx.run_until_parked();
}

let test7_result = run_visual_test(
    "diff_review_one_comment",
    regular_window.into(),
    cx,
    update_baseline,
)?;

// Test 8: Overlay with multiple comments + expanded
regular_window
    .update(cx, |workspace, window, cx| {
        let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
        if let Some(editor) = editors.into_iter().next() {
            editor.update(cx, |editor, cx| {
                // Add more comments
                if let Some(prompt_editor) = editor.diff_review_prompt_editor().cloned() {
                    prompt_editor.update(cx, |pe, cx| {
                        pe.insert("Second comment about imports", window, cx);
                    });
                }
                editor.submit_diff_review_comment(window, cx);

                if let Some(prompt_editor) = editor.diff_review_prompt_editor().cloned() {
                    prompt_editor.update(cx, |pe, cx| {
                        pe.insert("Third comment about naming", window, cx);
                    });
                }
                editor.submit_diff_review_comment(window, cx);
            });
        }
    })
    .ok();

for _ in 0..3 {
    cx.advance_clock(Duration::from_millis(100));
    cx.run_until_parked();
}

let test8_result = run_visual_test(
    "diff_review_multiple_comments_expanded",
    regular_window.into(),
    cx,
    update_baseline,
)?;

// Test 9: Comments collapsed
regular_window
    .update(cx, |workspace, window, cx| {
        let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
        if let Some(editor) = editors.into_iter().next() {
            editor.update(cx, |editor, cx| {
                // Toggle collapse
                if let Some(overlay) = &mut editor.diff_review_overlay {
                    overlay.comments_expanded = false;
                }
                cx.notify();
            });
        }
    })
    .ok();

for _ in 0..3 {
    cx.advance_clock(Duration::from_millis(100));
    cx.run_until_parked();
}

let test9_result = run_visual_test(
    "diff_review_comments_collapsed",
    regular_window.into(),
    cx,
    update_baseline,
)?;
```

### Unit Tests

Add to `crates/editor/src/editor.rs`:

```rust
#[cfg(test)]
mod review_comment_tests {
    use super::*;
    use gpui::TestAppContext;
    use std::sync::Arc;
    use std::path::Path;

    fn test_hunk_key() -> DiffHunkKey {
        DiffHunkKey {
            file_path: Arc::from(Path::new("test.rs")),
            hunk_start_row: DisplayRow(0),
        }
    }

    #[gpui::test]
    fn test_add_comment_to_hunk(cx: &mut TestAppContext) {
        let (editor, _window) = cx.add_window(|window, cx| {
            Editor::single_line(window, cx)
        });

        editor.update(cx, |editor, cx| {
            let key = test_hunk_key();

            let id = editor.add_review_comment(
                key.clone(),
                "Test comment".to_string(),
                DisplayRow(0),
                Anchor::min()..Anchor::max(),
                cx,
            );

            assert_eq!(editor.total_review_comment_count(), 1);
            assert_eq!(editor.hunk_comment_count(&key), 1);

            let comments = editor.comments_for_hunk(&key);
            assert_eq!(comments.len(), 1);
            assert_eq!(comments[0].comment, "Test comment");
            assert_eq!(comments[0].id, id);
        });
    }

    #[gpui::test]
    fn test_comments_are_per_hunk(cx: &mut TestAppContext) {
        let (editor, _window) = cx.add_window(|window, cx| {
            Editor::single_line(window, cx)
        });

        editor.update(cx, |editor, cx| {
            let key1 = DiffHunkKey {
                file_path: Arc::from(Path::new("file1.rs")),
                hunk_start_row: DisplayRow(0),
            };
            let key2 = DiffHunkKey {
                file_path: Arc::from(Path::new("file2.rs")),
                hunk_start_row: DisplayRow(10),
            };

            editor.add_review_comment(key1.clone(), "Comment for file1".to_string(), DisplayRow(0), Anchor::min()..Anchor::max(), cx);
            editor.add_review_comment(key2.clone(), "Comment for file2".to_string(), DisplayRow(10), Anchor::min()..Anchor::max(), cx);

            assert_eq!(editor.total_review_comment_count(), 2);
            assert_eq!(editor.hunk_comment_count(&key1), 1);
            assert_eq!(editor.hunk_comment_count(&key2), 1);

            assert_eq!(editor.comments_for_hunk(&key1)[0].comment, "Comment for file1");
            assert_eq!(editor.comments_for_hunk(&key2)[0].comment, "Comment for file2");
        });
    }

    #[gpui::test]
    fn test_remove_comment(cx: &mut TestAppContext) {
        let (editor, _window) = cx.add_window(|window, cx| {
            Editor::single_line(window, cx)
        });

        editor.update(cx, |editor, cx| {
            let key = test_hunk_key();

            let id = editor.add_review_comment(
                key.clone(),
                "To be removed".to_string(),
                DisplayRow(0),
                Anchor::min()..Anchor::max(),
                cx,
            );

            assert_eq!(editor.total_review_comment_count(), 1);

            let removed = editor.remove_review_comment(id, cx);
            assert!(removed);
            assert_eq!(editor.total_review_comment_count(), 0);

            // Try to remove again
            let removed_again = editor.remove_review_comment(id, cx);
            assert!(!removed_again);
        });
    }

    #[gpui::test]
    fn test_update_comment(cx: &mut TestAppContext) {
        let (editor, _window) = cx.add_window(|window, cx| {
            Editor::single_line(window, cx)
        });

        editor.update(cx, |editor, cx| {
            let key = test_hunk_key();

            let id = editor.add_review_comment(
                key.clone(),
                "Original text".to_string(),
                DisplayRow(0),
                Anchor::min()..Anchor::max(),
                cx,
            );

            let updated = editor.update_review_comment(id, "Updated text".to_string(), cx);
            assert!(updated);

            let comments = editor.comments_for_hunk(&key);
            assert_eq!(comments[0].comment, "Updated text");
            assert!(!comments[0].is_editing); // Should clear editing flag
        });
    }

    #[gpui::test]
    fn test_take_all_comments(cx: &mut TestAppContext) {
        let (editor, _window) = cx.add_window(|window, cx| {
            Editor::single_line(window, cx)
        });

        editor.update(cx, |editor, cx| {
            let key1 = DiffHunkKey {
                file_path: Arc::from(Path::new("file1.rs")),
                hunk_start_row: DisplayRow(0),
            };
            let key2 = DiffHunkKey {
                file_path: Arc::from(Path::new("file2.rs")),
                hunk_start_row: DisplayRow(10),
            };

            editor.add_review_comment(key1.clone(), "Comment 1".to_string(), DisplayRow(0), Anchor::min()..Anchor::max(), cx);
            editor.add_review_comment(key1.clone(), "Comment 2".to_string(), DisplayRow(1), Anchor::min()..Anchor::max(), cx);
            editor.add_review_comment(key2.clone(), "Comment 3".to_string(), DisplayRow(10), Anchor::min()..Anchor::max(), cx);

            assert_eq!(editor.total_review_comment_count(), 3);

            let taken = editor.take_all_review_comments(cx);

            // Should have 2 entries (one per hunk)
            assert_eq!(taken.len(), 2);

            // Total comments should be 3
            let total: usize = taken.iter().map(|(_, comments)| comments.len()).sum();
            assert_eq!(total, 3);

            // Storage should be empty
            assert_eq!(editor.total_review_comment_count(), 0);
        });
    }

    #[gpui::test]
    fn test_chronological_ordering(cx: &mut TestAppContext) {
        let (editor, _window) = cx.add_window(|window, cx| {
            Editor::single_line(window, cx)
        });

        editor.update(cx, |editor, cx| {
            let key = test_hunk_key();

            editor.add_review_comment(key.clone(), "First".to_string(), DisplayRow(0), Anchor::min()..Anchor::max(), cx);
            editor.add_review_comment(key.clone(), "Second".to_string(), DisplayRow(0), Anchor::min()..Anchor::max(), cx);
            editor.add_review_comment(key.clone(), "Third".to_string(), DisplayRow(0), Anchor::min()..Anchor::max(), cx);

            let comments = editor.comments_for_hunk(&key);

            // Comments should be in insertion order (chronological)
            assert_eq!(comments[0].comment, "First");
            assert_eq!(comments[1].comment, "Second");
            assert_eq!(comments[2].comment, "Third");

            // created_at should be monotonically increasing
            assert!(comments[0].created_at <= comments[1].created_at);
            assert!(comments[1].created_at <= comments[2].created_at);
        });
    }

    #[gpui::test]
    fn test_comment_ids_are_unique_across_hunks(cx: &mut TestAppContext) {
        let (editor, _window) = cx.add_window(|window, cx| {
            Editor::single_line(window, cx)
        });

        editor.update(cx, |editor, cx| {
            let key1 = DiffHunkKey {
                file_path: Arc::from(Path::new("file1.rs")),
                hunk_start_row: DisplayRow(0),
            };
            let key2 = DiffHunkKey {
                file_path: Arc::from(Path::new("file2.rs")),
                hunk_start_row: DisplayRow(10),
            };

            let id1 = editor.add_review_comment(key1, "Comment 1".to_string(), DisplayRow(0), Anchor::min()..Anchor::max(), cx);
            let id2 = editor.add_review_comment(key2, "Comment 2".to_string(), DisplayRow(10), Anchor::min()..Anchor::max(), cx);

            assert_ne!(id1, id2);
        });
    }
}
```

---

## File Reference

| File                                        | Changes                                                                                                                                                                                                                                                                                                   |
| ------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/editor/src/editor.rs`               | Add `DiffHunkKey`, `StoredReviewComment` structs; add `stored_review_comments: HashMap<DiffHunkKey, Vec<StoredReviewComment>>` field; add accessor/mutation methods; update `DiffReviewOverlay` struct; modify `submit_diff_review_comment()`; update `render_diff_review_overlay()`; add action handlers |
| `crates/editor/src/actions.rs`              | Add `SendReviewToAgent`, `ToggleReviewCommentsExpanded`, `EditReviewComment`, `DeleteReviewComment`, `ConfirmEditReviewComment`, `CancelEditReviewComment` actions                                                                                                                                        |
| `crates/git_ui/src/project_diff.rs`         | Add `total_review_comment_count()` method; update `ProjectDiffToolbar::render()` to include "Send Review to Agent" button with badge                                                                                                                                                                      |
| `crates/agent_ui/src/text_thread_editor.rs` | Add `handle_send_review_to_agent()` handler; register action                                                                                                                                                                                                                                              |
| `crates/zed/src/visual_test_runner.rs`      | Add visual tests for empty overlay, single comment, multiple comments, collapsed state, toolbar with badge                                                                                                                                                                                                |

---

## Glossary

| Term                 | Definition                                                                                                                          |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| **Anchor**           | A position in a buffer that automatically adjusts as text is inserted or deleted. Ensures comments track the correct code location. |
| **Block**            | A UI element inserted into the editor at a specific line. The diff review overlay is rendered as a sticky block.                    |
| **Crease**           | A collapsible region in the Agent panel's message editor that contains code snippets with context.                                  |
| **DiffHunkKey**      | A unique identifier for a hunk in the diff, combining file path and starting row. Used to scope comments.                           |
| **DisplayRow**       | A row number as displayed on screen (may differ from buffer row due to folding/wrapping).                                           |
| **Entity**           | GPUI's handle to a stateful object. `Entity<Editor>` is a handle to an `Editor` instance.                                           |
| **Hunk**             | A contiguous region of changes in a diff (added, modified, or deleted lines).                                                       |
| **MultiBuffer**      | A buffer that can contain excerpts from multiple source files. Used in diff views.                                                  |
| **ProjectDiff**      | The view showing git diffs in Zed. Contains the `SplittableEditor` with the diff content.                                           |
| **SplittableEditor** | A wrapper around `Editor` that supports side-by-side split views.                                                                   |

---

## Implementation Checklist

- [x] **Step 1**: Define `DiffHunkKey` and `StoredReviewComment` structs
- [x] **Step 2**: Add `stored_review_comments` HashMap to `Editor`; implement accessor methods
- [x] **Step 3**: Update `DiffReviewOverlay` with `hunk_key` and `comments_expanded`; modify `submit_diff_review_comment()`
- [x] **Step 4**: Update `render_diff_review_overlay()` to show stored comments
- [x] **Step 5**: Add expandable "N Comments" section with collapse/expand
- [x] **Step 6**: Add `SendReviewToAgent` action; add toolbar button with badge
- [x] **Step 7**: Implement `handle_send_review_to_agent()` for batch submission
- [x] **Step 8**: Add inline edit functionality with `EditReviewComment` action
- [x] **Step 9**: Add delete functionality with `DeleteReviewComment` action
- [x] **Step 10**: Integrate user avatar from `UserStore`
- [x] Remove hardcoded dummy comments from `render_diff_review_overlay()`
- [x] Add visual tests for stored comments UI states
- [x] Add unit tests for comment storage logic
- [x] Fix subscription leak in inline edit editors
- [x] Use `resize_blocks`/`replace_blocks` to avoid visual flicker on height updates

---

## Future Work

The following enhancements are planned for future iterations:

### User Feedback for Failure Cases

Currently, several failure modes in `handle_send_review_to_agent()` silently return without notifying the user:

1. **No ProjectDiff found** - Should show a toast/notification explaining the issue
2. **No agent panel available** - Should show an error or offer to open the panel
3. **Thread creation failure** - Should notify user and preserve their comments

**Recommendation**: Add toast notifications using `workspace.show_notification()` or similar for each failure case.

### Confirmation Dialog for Unsaved Comments

When the user dismisses the overlay (via Escape or clicking outside) while there is text typed in the prompt editor, the text is silently lost.

**Recommendation**: Add a confirmation dialog asking "Discard unsaved comment?" with options to:

- Discard and close
- Keep editing
- Submit comment and close

### Respect User's Agent Type Preference

The `handle_send_review_to_agent()` function always creates a `NativeAgent` thread:

```rust
panel.new_agent_thread(AgentType::NativeAgent, window, cx);
```

**Recommendation**: Check user settings or use the most recently used agent type instead of hardcoding `NativeAgent`.

### Preserve Comments on Send Failure

Currently, `take_all_review_comments()` clears the storage before the send is confirmed successful. If anything fails after this point, comments are lost.

**Recommendation**:

- Keep comments until send is confirmed successful
- Or implement an "undo" mechanism
- Or warn user before clearing if there are many comments

### Integration Test for Full Flow

There is no test that exercises the complete workflow:

1. Open ProjectDiff
2. Click "+" button to open overlay
3. Type and submit multiple comments
4. Click "Send Review to Agent"
5. Verify comments appear in agent panel

**Recommendation**: Add an integration test that verifies the full flow works correctly.

### Test Coverage Gaps

- No test for button click behavior (the `on_click` handlers)
- No test for anchor tracking after buffer edits
- No test for `SendReviewToAgent` action handler
- No error handling tests (e.g., showing overlay when no buffer exists)

### Edge Cases to Consider

- **Very long comments**: Does the overlay handle multiline/wrapped comments correctly?
- **Multiple overlays**: Behavior when clicking review button on another hunk (currently dismisses first)
- **Anchor validity**: Do anchor ranges remain valid after external file modifications?
- **Thread creation failure**: What if `new_agent_thread()` fails?

---

## Running Tests

```bash
# Run visual tests
cargo run --release -p zed --bin visual-tests

# Update visual baselines (when changes are intentional)
UPDATE_BASELINE=1 cargo run --release -p zed --bin visual-tests

# View output screenshots
open target/visual_tests/

# Run unit tests for editor (review comment tests)
cargo test -q -p editor review_comment

# Run all editor tests
cargo test -q -p editor

# Run clippy checks
./script/clippy
```

---

## Revision History

| Date    | Changes                                                                             |
| ------- | ----------------------------------------------------------------------------------- |
| Initial | Original implementation plan created                                                |
| Current | Implementation complete; added Future Work section documenting planned enhancements |
