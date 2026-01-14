# Diff Review Implementation Improvements

This document describes **required** changes to the diff review feature. **ALL items in this document MUST be implemented. None are optional or allowed to be skipped.**

## Background

The diff review feature allows users to add review comments to diff hunks in the `ProjectDiff` view. Comments are stored locally and can be batch-sent to the Agent panel. The current implementation has several issues that need to be fixed.

---

## Issue 1: `DiffHunkKey` Uses `DisplayRow` (MUST FIX)

### Problem

The `DiffHunkKey` struct currently uses `DisplayRow` to identify hunks:

```rust
// crates/editor/src/editor.rs
pub struct DiffHunkKey {
    pub file_path: Arc<util::rel_path::RelPath>,
    pub hunk_start_row: DisplayRow,  // <-- PROBLEM: This changes when buffer is edited!
}
```

`DisplayRow` is a visual row number that changes whenever lines are added or removed above the hunk. This means:
- If you add a comment to a hunk at row 10
- Then insert 5 lines above it
- The hunk moves to row 15
- But the `DiffHunkKey` still says row 10
- The comment is now orphaned and won't display correctly

### Solution

Replace `DisplayRow` with an `Anchor`. Anchors are buffer positions that automatically track their logical location as the buffer changes.

### Implementation Steps

1. **Modify `DiffHunkKey` in `crates/editor/src/editor.rs`:**

   Find this struct (around line 1037):
   ```rust
   #[derive(Clone, Debug, PartialEq, Eq, Hash)]
   pub struct DiffHunkKey {
       pub file_path: Arc<util::rel_path::RelPath>,
       pub hunk_start_row: DisplayRow,
   }
   ```

   Change it to:
   ```rust
   #[derive(Clone, Debug)]
   pub struct DiffHunkKey {
       /// The file path (relative to worktree) this hunk belongs to.
       pub file_path: Arc<util::rel_path::RelPath>,
       /// An anchor at the start of the hunk. This tracks position as the buffer changes.
       pub hunk_start_anchor: Anchor,
   }
   ```

   **IMPORTANT:** Remove `PartialEq`, `Eq`, and `Hash` derives because `Anchor` doesn't implement these traits in a way suitable for HashMap keys.

2. **Change the storage type in `Editor` struct:**

   Find these fields (around line 1270):
   ```rust
   stored_review_comments: HashMap<DiffHunkKey, Vec<StoredReviewComment>>,
   ```

   Change to use a `Vec` instead of `HashMap`:
   ```rust
   stored_review_comments: Vec<(DiffHunkKey, Vec<StoredReviewComment>)>,
   ```

   This is necessary because we can no longer use `DiffHunkKey` as a hash key.

3. **Update `show_diff_review_overlay` method:**

   Find where `hunk_key` is created (around line 20920):
   ```rust
   let hunk_key = DiffHunkKey {
       file_path,
       hunk_start_row: display_row,
   };
   ```

   Change to:
   ```rust
   // Create an anchor at the start of the hunk
   let hunk_start_anchor = buffer_snapshot.anchor_before(Point::new(buffer_point.row, 0));
   let hunk_key = DiffHunkKey {
       file_path,
       hunk_start_anchor,
   };
   ```

4. **Update all methods that use `DiffHunkKey`:**

   The following methods need to be updated to work with the new Vec-based storage:

   - `comments_for_hunk()` - Change from `HashMap::get` to `Vec::iter().find()`
   - `hunk_comment_count()` - Same change
   - `add_review_comment()` - Change from `HashMap::entry().or_default()` to finding/pushing
   - `take_all_review_comments()` - Already returns a Vec, minimal changes needed

   For `comments_for_hunk`, you'll need to compare anchors. Two anchors represent the "same" hunk if they resolve to the same buffer position. Use:
   ```rust
   fn find_hunk_comments(&self, key: &DiffHunkKey, snapshot: &MultiBufferSnapshot) -> Option<&Vec<StoredReviewComment>> {
       let key_point = key.hunk_start_anchor.to_point(snapshot);
       self.stored_review_comments
           .iter()
           .find(|(k, _)| {
               k.file_path == key.file_path &&
               k.hunk_start_anchor.to_point(snapshot) == key_point
           })
           .map(|(_, comments)| comments)
   }
   ```

5. **Update `calculate_overlay_height` and `refresh_diff_review_overlay_height`:**

   These methods take `&DiffHunkKey` and need to use the new comparison logic. You'll need to pass a `&MultiBufferSnapshot` to compare anchors.

6. **Update tests in `crates/editor/src/editor_tests.rs`:**

   The `test_hunk_key` helper function needs to create an anchor:
   ```rust
   fn test_hunk_key(file_path: &str, anchor: Anchor) -> DiffHunkKey {
       DiffHunkKey {
           file_path: if file_path.is_empty() {
               Arc::from(util::rel_path::RelPath::empty())
           } else {
               Arc::from(util::rel_path::RelPath::unix(file_path).unwrap())
           },
           hunk_start_anchor: anchor,
       }
   }
   ```

   Tests will need to create anchors from the editor's buffer. For simple tests, you can use `Anchor::min()` as a placeholder.

---

## Issue 2: No Cleanup of Orphaned Comments (MUST FIX)

### Problem

If a hunk is reverted or deleted, the comments associated with it remain in `stored_review_comments`. These orphaned comments:
- Waste memory
- Could cause confusion if the same file path is used later
- May have invalid anchors that point to deleted text

### Solution

Add a method to validate and clean orphaned comments, and call it when the buffer changes.

### Implementation Steps

1. **Add a new method `cleanup_orphaned_review_comments` in `crates/editor/src/editor.rs`:**

   Add this method to the `Editor` impl block (after `take_all_review_comments`):

   ```rust
   /// Removes review comments whose anchors are no longer valid or whose
   /// associated diff hunks no longer exist.
   ///
   /// This should be called when the buffer changes to prevent orphaned comments
   /// from accumulating.
   pub fn cleanup_orphaned_review_comments(&mut self, cx: &mut Context<Self>) {
       let snapshot = self.buffer.read(cx).snapshot(cx);

       // Remove comments with invalid anchors
       self.stored_review_comments.retain(|(hunk_key, comments)| {
           // Check if the hunk anchor is still valid (not pointing to deleted text)
           let anchor_valid = hunk_key.hunk_start_anchor.is_valid(&snapshot);

           if !anchor_valid {
               return false; // Remove this entire hunk's comments
           }

           true
       });

       // Also clean up individual comments with invalid anchor ranges
       for (_, comments) in &mut self.stored_review_comments {
           comments.retain(|comment| {
               comment.anchor_range.start.is_valid(&snapshot) &&
               comment.anchor_range.end.is_valid(&snapshot)
           });
       }

       // Remove empty hunk entries
       self.stored_review_comments.retain(|(_, comments)| !comments.is_empty());

       cx.notify();
   }
   ```

2. **Call the cleanup method when the buffer changes:**

   Find where buffer subscriptions are set up in `Editor::new` (around line 2350). There should be a subscription to buffer events. Add a call to cleanup when relevant buffer events occur.

   Look for code like:
   ```rust
   cx.subscribe(&buffer, |editor, buffer, event, cx| {
       // ... existing event handling
   })
   ```

   Add handling for buffer edit events:
   ```rust
   MultiBufferEvent::Edited { .. } => {
       // Clean up orphaned comments after edits
       editor.cleanup_orphaned_review_comments(cx);
   }
   ```

   **Note:** Be careful not to call cleanup too frequently. You may want to debounce this or only clean up on specific events like `TransactionUndone` or when excerpts are removed.

3. **Add a test for orphaned comment cleanup:**

   Add a new test in `crates/editor/src/editor_tests.rs`:

   ```rust
   #[gpui::test]
   fn test_orphaned_comments_are_cleaned_up(cx: &mut TestAppContext) {
       init_test(cx, |_| {});

       // Create an editor with some text
       let editor = cx.add_window(|window, cx| {
           let buffer = cx.new(|cx| {
               Buffer::local("line 1\nline 2\nline 3\n", cx)
           });
           let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
           Editor::new(EditorMode::Full, multi_buffer, None, false, window, cx)
       });

       // Add a comment
       editor.update(cx, |editor, _window, cx| {
           let snapshot = editor.buffer().read(cx).snapshot(cx);
           let anchor = snapshot.anchor_after(Point::new(1, 0)); // Line 2
           let key = DiffHunkKey {
               file_path: Arc::from(util::rel_path::RelPath::empty()),
               hunk_start_anchor: anchor,
           };
           add_test_comment(editor, key, "Comment on line 2", 1, cx);
           assert_eq!(editor.total_review_comment_count(), 1);
       }).unwrap();

       // Delete line 2 (this should orphan the comment)
       editor.update(cx, |editor, window, cx| {
           editor.select_all(&SelectAll, window, cx);
           editor.insert("completely new content", window, cx);
       }).unwrap();

       // Trigger cleanup
       editor.update(cx, |editor, _window, cx| {
           editor.cleanup_orphaned_review_comments(cx);
           // Comment should be removed because its anchor is invalid
           assert_eq!(editor.total_review_comment_count(), 0);
       }).unwrap();
   }
   ```

---

## Issue 4: No User Feedback on Failure (MUST FIX)

### Problem

In `handle_send_review_to_agent` in `crates/agent_ui/src/text_thread_editor.rs`, when something goes wrong (no ProjectDiff found, no agent panel), the code only logs a warning. Users see nothing happen and don't know why.

### Solution

Show a toast notification to the user when the action fails.

### Implementation Steps

1. **Find `handle_send_review_to_agent` in `crates/agent_ui/src/text_thread_editor.rs`:**

   Look for lines like:
   ```rust
   let Some(project_diff) = workspace.items_of_type::<ProjectDiff>(cx).next() else {
       log::warn!("No ProjectDiff found when sending review");
       return;
   };
   ```

2. **Add toast notifications for each failure case:**

   First, add the import at the top of the file:
   ```rust
   use workspace::notifications::NotifyResultExt;
   ```

   Or use the workspace's toast API directly. Look at how other parts of the codebase show notifications. A common pattern is:

   ```rust
   workspace.show_toast(
       Toast::new(
           NotificationId::unique::<Self>(),
           "No changes to review. Open the Project Diff panel first.",
       ),
       cx,
   );
   ```

3. **Update each early return with a notification:**

   ```rust
   // Case 1: No ProjectDiff
   let Some(project_diff) = workspace.items_of_type::<ProjectDiff>(cx).next() else {
       workspace.show_toast(
           Toast::new(
               NotificationId::unique::<SendReviewToAgent>(),
               "No Project Diff panel found. Open it first to add review comments.",
           ),
           cx,
       );
       return;
   };

   // Case 2: No comments
   if comments.is_empty() {
       workspace.show_toast(
           Toast::new(
               NotificationId::unique::<SendReviewToAgent>(),
               "No review comments to send. Add comments using the + button in the diff view.",
           ),
           cx,
       );
       return;
   }

   // Case 3: No agent panel
   let Some(panel) = workspace.panel::<crate::AgentPanel>(cx) else {
       workspace.show_toast(
           Toast::new(
               NotificationId::unique::<SendReviewToAgent>(),
               "Agent panel is not available.",
           ),
           cx,
       );
       return;
   };
   ```

4. **Also handle the deferred failure cases:**

   Inside the `cx.defer_in` closure, there are more log warnings. These are trickier because you're inside a deferred context. You have access to `workspace`, so you can still show toasts:

   ```rust
   cx.defer_in(window, move |workspace, window, cx| {
       let Some(panel) = workspace.panel::<crate::AgentPanel>(cx) else {
           workspace.show_toast(
               Toast::new(
                   NotificationId::unique::<SendReviewToAgent>(),
                   "Agent panel closed unexpectedly.",
               ),
               cx,
           );
           return;
       };
       // ... rest of the code
   });
   ```

---

## Issue 6: Deep Chain Access in ProjectDiff (MUST FIX)

### Problem

In `crates/git_ui/src/project_diff.rs`, getting the review comment count requires traversing three layers:

```rust
self.editor.read(cx).primary_editor().read(cx).total_review_comment_count()
```

This is fragile and doesn't notify the UI when the count changes, so the toolbar button won't update reactively.

### Solution

Add an `EditorEvent` that fires when review comments change, and observe it in `ProjectDiff`.

### Implementation Steps

1. **Add a new event variant in `crates/editor/src/editor.rs`:**

   Find the `EditorEvent` enum (around line 890):
   ```rust
   pub enum EditorEvent {
       InputIgnored { ... },
       ExcerptsAdded { ... },
       // ... many other variants
   }
   ```

   Add a new variant:
   ```rust
   /// Emitted when the stored review comments change (added, removed, or updated).
   ReviewCommentsChanged {
       /// The new total count of review comments.
       total_count: usize,
   },
   ```

2. **Emit the event when comments change:**

   Find all places where comments are modified and emit the event:

   - `add_review_comment()` - after adding
   - `remove_review_comment()` - after removing (if successful)
   - `update_review_comment()` - after updating (if successful)
   - `take_all_review_comments()` - after taking all
   - `cleanup_orphaned_review_comments()` - after cleanup (if anything was removed)

   For each, add:
   ```rust
   cx.emit(EditorEvent::ReviewCommentsChanged {
       total_count: self.total_review_comment_count(),
   });
   ```

   Example for `add_review_comment`:
   ```rust
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

       let stored_comment = StoredReviewComment::new(id, comment, display_row, anchor_range);

       // ... add to storage ...

       cx.emit(EditorEvent::ReviewCommentsChanged {
           total_count: self.total_review_comment_count(),
       });
       cx.notify();
       id
   }
   ```

3. **Update `ProjectDiff` to track the count reactively:**

   In `crates/git_ui/src/project_diff.rs`, add a field to cache the count:

   Find the `ProjectDiff` struct and add:
   ```rust
   review_comment_count: usize,
   ```

   Initialize it to 0 in `ProjectDiff::new`.

4. **Subscribe to editor events in `ProjectDiff::new`:**

   Find where `ProjectDiff` is created and subscribes to editor events. Add:

   ```rust
   cx.subscribe(&editor, |this, _editor, event: &editor::EditorEvent, cx| {
       if let editor::EditorEvent::ReviewCommentsChanged { total_count } = event {
           this.review_comment_count = *total_count;
           cx.notify(); // Trigger re-render of toolbar
       }
   });
   ```

   **Note:** `SplittableEditor` wraps the actual editor. You may need to subscribe to the primary editor specifically:
   ```rust
   let primary_editor = editor.read(cx).primary_editor().clone();
   cx.subscribe(&primary_editor, |this, _editor, event: &editor::EditorEvent, cx| {
       // ... handle event
   });
   ```

5. **Update `total_review_comment_count` to use the cached value:**

   Change:
   ```rust
   pub fn total_review_comment_count(&self, cx: &App) -> usize {
       self.editor
           .read(cx)
           .primary_editor()
           .read(cx)
           .total_review_comment_count()
   }
   ```

   To:
   ```rust
   pub fn total_review_comment_count(&self) -> usize {
       self.review_comment_count
   }
   ```

6. **Update the toolbar render to use the simpler method:**

   In `ProjectDiffToolbar::render`, change:
   ```rust
   let review_count = project_diff.read(cx).total_review_comment_count(cx);
   ```

   To:
   ```rust
   let review_count = project_diff.read(cx).total_review_comment_count();
   ```

---

## Issue 7: Delete `test_diff_review_overlay_height_calculation` (MUST DO)

### Problem

This test tests an internal implementation detail (the height calculation formula). If we change the formula, the test breaks even though nothing is actually wrong from a user perspective.

### Implementation Steps

1. **Find and delete the test in `crates/editor/src/editor_tests.rs`:**

   Search for `test_diff_review_overlay_height_calculation` and delete the entire test function:

   ```rust
   #[gpui::test]
   fn test_diff_review_overlay_height_calculation(cx: &mut TestAppContext) {
       // ... delete all of this ...
   }
   ```

   This test starts around line 30700 (after the anchor changes, line numbers will shift).

---

## Verification Checklist

After implementing all changes, verify:

- [ ] `DiffHunkKey` uses `Anchor` instead of `DisplayRow`
- [ ] `stored_review_comments` is a `Vec` instead of `HashMap`
- [ ] All methods that use `DiffHunkKey` have been updated
- [ ] `cleanup_orphaned_review_comments` method exists and works
- [ ] Cleanup is called on buffer changes
- [ ] Test for orphaned comment cleanup passes
- [ ] Toast notifications show for all failure cases in `handle_send_review_to_agent`
- [ ] `EditorEvent::ReviewCommentsChanged` variant exists
- [ ] Event is emitted from all comment-modifying methods
- [ ] `ProjectDiff` subscribes to the event and caches the count
- [ ] `test_diff_review_overlay_height_calculation` has been deleted
- [ ] All existing tests still pass (run `cargo test -p editor`)
- [ ] The feature works end-to-end in the actual application

---

## Order of Implementation

Implement in this order to minimize conflicts:

1. **Issue 7** (delete test) - Do this first, it's the simplest
2. **Issue 1** (DiffHunkKey anchor) - This is the most invasive change
3. **Issue 2** (orphaned cleanup) - Builds on the anchor changes
4. **Issue 6** (EditorEvent) - Independent of the above
5. **Issue 4** (user feedback) - Independent, do last

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/editor/src/editor.rs` | `DiffHunkKey`, storage type, cleanup method, event emission |
| `crates/editor/src/editor_tests.rs` | Update `test_hunk_key` helper, delete height test, add cleanup test |
| `crates/git_ui/src/project_diff.rs` | Cache count, subscribe to event, simplify getter |
| `crates/agent_ui/src/text_thread_editor.rs` | Add toast notifications |

**Remember: ALL items in this document are REQUIRED. Do not skip any of them.**
