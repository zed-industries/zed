use crate::{
    AnchorRangeExt, DisplayPoint, Editor, MultiBuffer, RowExt,
    display_map::{HighlightKey, ToDisplayPoint},
};
use buffer_diff::DiffHunkStatusKind;
use collections::BTreeMap;
use futures::Future;

use gpui::{
    AnyWindowHandle, App, Context, Entity, Focusable as _, Keystroke, Pixels, Point,
    VisualTestContext, Window, WindowHandle, prelude::*,
};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot, LanguageRegistry};
use multi_buffer::{Anchor, ExcerptRange, MultiBufferRow};
use parking_lot::RwLock;
use project::{FakeFs, Project};
use std::{
    any::TypeId,
    ops::{Deref, DerefMut, Range},
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use util::{
    assert_set_eq,
    test::{generate_marked_text, marked_text_ranges},
};

use super::{build_editor, build_editor_with_project};

pub struct EditorTestContext {
    pub cx: gpui::VisualTestContext,
    pub window: AnyWindowHandle,
    pub editor: Entity<Editor>,
    pub assertion_cx: AssertionContextManager,
}

impl EditorTestContext {
    pub async fn new(cx: &mut gpui::TestAppContext) -> EditorTestContext {
        let fs = FakeFs::new(cx.executor());
        let root = Self::root_path();
        fs.insert_tree(
            root,
            serde_json::json!({
                ".git": {},
                "file": "",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [root], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(root.join("file"), cx)
            })
            .await
            .unwrap();
        let editor = cx.add_window(|window, cx| {
            let editor = build_editor_with_project(
                project,
                MultiBuffer::build_from_buffer(buffer, cx),
                window,
                cx,
            );

            window.focus(&editor.focus_handle(cx));
            editor
        });
        let editor_view = editor.root(cx).unwrap();

        cx.run_until_parked();
        Self {
            cx: VisualTestContext::from_window(*editor.deref(), cx),
            window: editor.into(),
            editor: editor_view,
            assertion_cx: AssertionContextManager::new(),
        }
    }

    #[cfg(target_os = "windows")]
    fn root_path() -> &'static Path {
        Path::new("C:\\root")
    }

    #[cfg(not(target_os = "windows"))]
    fn root_path() -> &'static Path {
        Path::new("/root")
    }

    pub async fn for_editor_in(editor: Entity<Editor>, cx: &mut gpui::VisualTestContext) -> Self {
        cx.focus(&editor);
        Self {
            window: cx.windows()[0],
            cx: cx.clone(),
            editor,
            assertion_cx: AssertionContextManager::new(),
        }
    }

    pub async fn for_editor(editor: WindowHandle<Editor>, cx: &mut gpui::TestAppContext) -> Self {
        let editor_view = editor.root(cx).unwrap();
        Self {
            cx: VisualTestContext::from_window(*editor.deref(), cx),
            window: editor.into(),
            editor: editor_view,
            assertion_cx: AssertionContextManager::new(),
        }
    }

    #[track_caller]
    pub fn new_multibuffer<const COUNT: usize>(
        cx: &mut gpui::TestAppContext,
        excerpts: [&str; COUNT],
    ) -> EditorTestContext {
        let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
        let buffer = cx.new(|cx| {
            for excerpt in excerpts.into_iter() {
                let (text, ranges) = marked_text_ranges(excerpt, false);
                let buffer = cx.new(|cx| Buffer::local(text, cx));
                multibuffer.push_excerpts(buffer, ranges.into_iter().map(ExcerptRange::new), cx);
            }
            multibuffer
        });

        let editor = cx.add_window(|window, cx| {
            let editor = build_editor(buffer, window, cx);
            window.focus(&editor.focus_handle(cx));

            editor
        });

        let editor_view = editor.root(cx).unwrap();
        Self {
            cx: VisualTestContext::from_window(*editor.deref(), cx),
            window: editor.into(),
            editor: editor_view,
            assertion_cx: AssertionContextManager::new(),
        }
    }

    pub fn condition(
        &self,
        predicate: impl FnMut(&Editor, &App) -> bool,
    ) -> impl Future<Output = ()> {
        self.editor
            .condition::<crate::EditorEvent>(&self.cx, predicate)
    }

    #[track_caller]
    pub fn editor<F, T>(&mut self, read: F) -> T
    where
        F: FnOnce(&Editor, &Window, &mut Context<Editor>) -> T,
    {
        self.editor
            .update_in(&mut self.cx, |this, window, cx| read(this, window, cx))
    }

    #[track_caller]
    pub fn update_editor<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) -> T,
    {
        self.editor.update_in(&mut self.cx, update)
    }

    pub fn multibuffer<F, T>(&mut self, read: F) -> T
    where
        F: FnOnce(&MultiBuffer, &App) -> T,
    {
        self.editor(|editor, _, cx| read(editor.buffer().read(cx), cx))
    }

    pub fn update_multibuffer<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut MultiBuffer, &mut Context<MultiBuffer>) -> T,
    {
        self.update_editor(|editor, _, cx| editor.buffer().update(cx, update))
    }

    pub fn buffer_text(&mut self) -> String {
        self.multibuffer(|buffer, cx| buffer.snapshot(cx).text())
    }

    pub fn display_text(&mut self) -> String {
        self.update_editor(|editor, _, cx| editor.display_text(cx))
    }

    pub fn buffer<F, T>(&mut self, read: F) -> T
    where
        F: FnOnce(&Buffer, &App) -> T,
    {
        self.multibuffer(|multibuffer, cx| {
            let buffer = multibuffer.as_singleton().unwrap().read(cx);
            read(buffer, cx)
        })
    }

    pub fn language_registry(&mut self) -> Arc<LanguageRegistry> {
        self.editor(|editor, _, cx| {
            editor
                .project
                .as_ref()
                .unwrap()
                .read(cx)
                .languages()
                .clone()
        })
    }

    pub fn update_buffer<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Buffer, &mut Context<Buffer>) -> T,
    {
        self.update_multibuffer(|multibuffer, cx| {
            let buffer = multibuffer.as_singleton().unwrap();
            buffer.update(cx, update)
        })
    }

    pub fn buffer_snapshot(&mut self) -> BufferSnapshot {
        self.buffer(|buffer, _| buffer.snapshot())
    }

    pub fn add_assertion_context(&self, context: String) -> ContextHandle {
        self.assertion_cx.add_context(context)
    }

    pub fn assertion_context(&self) -> String {
        self.assertion_cx.context()
    }

    // unlike cx.simulate_keystrokes(), this does not run_until_parked
    // so you can use it to test detailed timing
    pub fn simulate_keystroke(&mut self, keystroke_text: &str) {
        let keystroke = Keystroke::parse(keystroke_text).unwrap();
        self.cx.dispatch_keystroke(self.window, keystroke);
    }

    pub fn run_until_parked(&mut self) {
        self.cx.background_executor.run_until_parked();
    }

    #[track_caller]
    pub fn ranges(&mut self, marked_text: &str) -> Vec<Range<usize>> {
        let (unmarked_text, ranges) = marked_text_ranges(marked_text, false);
        assert_eq!(self.buffer_text(), unmarked_text);
        ranges
    }

    pub fn display_point(&mut self, marked_text: &str) -> DisplayPoint {
        let ranges = self.ranges(marked_text);
        let snapshot = self.editor.update_in(&mut self.cx, |editor, window, cx| {
            editor.snapshot(window, cx)
        });
        ranges[0].start.to_display_point(&snapshot)
    }

    pub fn pixel_position(&mut self, marked_text: &str) -> Point<Pixels> {
        let display_point = self.display_point(marked_text);
        self.pixel_position_for(display_point)
    }

    pub fn pixel_position_for(&mut self, display_point: DisplayPoint) -> Point<Pixels> {
        self.update_editor(|editor, window, cx| {
            let newest_point = editor.selections.newest_display(cx).head();
            let pixel_position = editor.pixel_position_of_newest_cursor.unwrap();
            let line_height = editor
                .style()
                .unwrap()
                .text
                .line_height_in_pixels(window.rem_size());
            let snapshot = editor.snapshot(window, cx);
            let details = editor.text_layout_details(window);

            let y = pixel_position.y
                + line_height * (display_point.row().as_f32() - newest_point.row().as_f32());
            let x = pixel_position.x + snapshot.x_for_display_point(display_point, &details)
                - snapshot.x_for_display_point(newest_point, &details);
            Point::new(x, y)
        })
    }

    // Returns anchors for the current buffer using `«` and `»`
    pub fn text_anchor_range(&mut self, marked_text: &str) -> Range<language::Anchor> {
        let ranges = self.ranges(marked_text);
        let snapshot = self.buffer_snapshot();
        snapshot.anchor_before(ranges[0].start)..snapshot.anchor_after(ranges[0].end)
    }

    pub fn set_head_text(&mut self, diff_base: &str) {
        self.cx.run_until_parked();
        let fs =
            self.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).fs().as_fake());
        let path = self.update_buffer(|buffer, _| buffer.file().unwrap().path().clone());
        fs.set_head_for_repo(
            &Self::root_path().join(".git"),
            &[(path.as_unix_str(), diff_base.to_string())],
            "deadbeef",
        );
        self.cx.run_until_parked();
    }

    pub fn clear_index_text(&mut self) {
        self.cx.run_until_parked();
        let fs =
            self.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).fs().as_fake());
        fs.set_index_for_repo(&Self::root_path().join(".git"), &[]);
        self.cx.run_until_parked();
    }

    pub fn set_index_text(&mut self, diff_base: &str) {
        self.cx.run_until_parked();
        let fs =
            self.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).fs().as_fake());
        let path = self.update_buffer(|buffer, _| buffer.file().unwrap().path().clone());
        fs.set_index_for_repo(
            &Self::root_path().join(".git"),
            &[(path.as_unix_str(), diff_base.to_string())],
        );
        self.cx.run_until_parked();
    }

    #[track_caller]
    pub fn assert_index_text(&mut self, expected: Option<&str>) {
        let fs =
            self.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).fs().as_fake());
        let path = self.update_buffer(|buffer, _| buffer.file().unwrap().path().clone());
        let mut found = None;
        fs.with_git_state(&Self::root_path().join(".git"), false, |git_state| {
            found = git_state.index_contents.get(&path.into()).cloned();
        })
        .unwrap();
        assert_eq!(expected, found.as_deref());
    }

    /// Change the editor's text and selections using a string containing
    /// embedded range markers that represent the ranges and directions of
    /// each selection.
    ///
    /// Returns a context handle so that assertion failures can print what
    /// editor state was needed to cause the failure.
    ///
    /// See the `util::test::marked_text_ranges` function for more information.
    #[track_caller]
    pub fn set_state(&mut self, marked_text: &str) -> ContextHandle {
        let state_context = self.add_assertion_context(format!(
            "Initial Editor State: \"{}\"",
            marked_text.escape_debug()
        ));
        let (unmarked_text, selection_ranges) = marked_text_ranges(marked_text, true);
        self.editor.update_in(&mut self.cx, |editor, window, cx| {
            editor.set_text(unmarked_text, window, cx);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges(selection_ranges)
            })
        });
        state_context
    }

    /// Only change the editor's selections
    #[track_caller]
    pub fn set_selections_state(&mut self, marked_text: &str) -> ContextHandle {
        let state_context = self.add_assertion_context(format!(
            "Initial Editor State: \"{}\"",
            marked_text.escape_debug()
        ));
        let (unmarked_text, selection_ranges) = marked_text_ranges(marked_text, true);
        self.editor.update_in(&mut self.cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), unmarked_text);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges(selection_ranges)
            })
        });
        state_context
    }

    /// Assert about the text of the editor, the selections, and the expanded
    /// diff hunks.
    ///
    /// Diff hunks are indicated by lines starting with `+` and `-`.
    #[track_caller]
    pub fn assert_state_with_diff(&mut self, expected_diff_text: String) {
        assert_state_with_diff(&self.editor, &mut self.cx, &expected_diff_text);
    }

    #[track_caller]
    pub fn assert_excerpts_with_selections(&mut self, marked_text: &str) {
        let expected_excerpts = marked_text
            .strip_prefix("[EXCERPT]\n")
            .unwrap()
            .split("[EXCERPT]\n")
            .collect::<Vec<_>>();

        let (multibuffer_snapshot, selections, excerpts) = self.update_editor(|editor, _, cx| {
            let multibuffer_snapshot = editor.buffer.read(cx).snapshot(cx);

            let selections = editor.selections.disjoint_anchors_arc();
            let excerpts = multibuffer_snapshot
                .excerpts()
                .map(|(e_id, snapshot, range)| (e_id, snapshot.clone(), range))
                .collect::<Vec<_>>();

            (multibuffer_snapshot, selections, excerpts)
        });

        assert!(
            excerpts.len() == expected_excerpts.len(),
            "should have {} excerpts, got {}",
            expected_excerpts.len(),
            excerpts.len()
        );

        for (ix, (excerpt_id, snapshot, range)) in excerpts.into_iter().enumerate() {
            let is_folded = self
                .update_editor(|editor, _, cx| editor.is_buffer_folded(snapshot.remote_id(), cx));
            let (expected_text, expected_selections) =
                marked_text_ranges(expected_excerpts[ix], true);
            if expected_text == "[FOLDED]\n" {
                assert!(is_folded, "excerpt {} should be folded", ix);
                let is_selected = selections.iter().any(|s| s.head().excerpt_id == excerpt_id);
                if !expected_selections.is_empty() {
                    assert!(
                        is_selected,
                        "excerpt {ix} should be selected. got {:?}",
                        self.editor_state(),
                    );
                } else {
                    assert!(
                        !is_selected,
                        "excerpt {ix} should not be selected, got: {selections:?}",
                    );
                }
                continue;
            }
            assert!(!is_folded, "excerpt {} should not be folded", ix);
            assert_eq!(
                multibuffer_snapshot
                    .text_for_range(Anchor::range_in_buffer(
                        excerpt_id,
                        snapshot.remote_id(),
                        range.context.clone()
                    ))
                    .collect::<String>(),
                expected_text
            );

            let selections = selections
                .iter()
                .filter(|s| s.head().excerpt_id == excerpt_id)
                .map(|s| {
                    let head = text::ToOffset::to_offset(&s.head().text_anchor, &snapshot)
                        - text::ToOffset::to_offset(&range.context.start, &snapshot);
                    let tail = text::ToOffset::to_offset(&s.head().text_anchor, &snapshot)
                        - text::ToOffset::to_offset(&range.context.start, &snapshot);
                    tail..head
                })
                .collect::<Vec<_>>();
            // todo: selections that cross excerpt boundaries..
            assert_eq!(
                selections, expected_selections,
                "excerpt {} has incorrect selections",
                ix,
            );
        }
    }

    /// Make an assertion about the editor's text and the ranges and directions
    /// of its selections using a string containing embedded range markers.
    ///
    /// See the `util::test::marked_text_ranges` function for more information.
    #[track_caller]
    pub fn assert_editor_state(&mut self, marked_text: &str) {
        let (expected_text, expected_selections) = marked_text_ranges(marked_text, true);
        pretty_assertions::assert_eq!(self.buffer_text(), expected_text, "unexpected buffer text");
        self.assert_selections(expected_selections, marked_text.to_string())
    }

    /// Make an assertion about the editor's text and the ranges and directions
    /// of its selections using a string containing embedded range markers.
    ///
    /// See the `util::test::marked_text_ranges` function for more information.
    #[track_caller]
    pub fn assert_display_state(&mut self, marked_text: &str) {
        let (expected_text, expected_selections) = marked_text_ranges(marked_text, true);
        pretty_assertions::assert_eq!(self.display_text(), expected_text, "unexpected buffer text");
        self.assert_selections(expected_selections, marked_text.to_string())
    }

    pub fn editor_state(&mut self) -> String {
        generate_marked_text(self.buffer_text().as_str(), &self.editor_selections(), true)
    }

    #[track_caller]
    pub fn assert_editor_background_highlights<Tag: 'static>(&mut self, marked_text: &str) {
        let expected_ranges = self.ranges(marked_text);
        let actual_ranges: Vec<Range<usize>> = self.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            editor
                .background_highlights
                .get(&HighlightKey::Type(TypeId::of::<Tag>()))
                .map(|h| h.1.clone())
                .unwrap_or_default()
                .iter()
                .map(|range| range.to_offset(&snapshot.buffer_snapshot))
                .collect()
        });
        assert_set_eq!(actual_ranges, expected_ranges);
    }

    #[track_caller]
    pub fn assert_editor_text_highlights<Tag: ?Sized + 'static>(&mut self, marked_text: &str) {
        let expected_ranges = self.ranges(marked_text);
        let snapshot = self.update_editor(|editor, window, cx| editor.snapshot(window, cx));
        let actual_ranges: Vec<Range<usize>> = snapshot
            .text_highlight_ranges::<Tag>()
            .map(|ranges| ranges.as_ref().clone().1)
            .unwrap_or_default()
            .into_iter()
            .map(|range| range.to_offset(&snapshot.buffer_snapshot))
            .collect();
        assert_set_eq!(actual_ranges, expected_ranges);
    }

    #[track_caller]
    pub fn assert_editor_selections(&mut self, expected_selections: Vec<Range<usize>>) {
        let expected_marked_text =
            generate_marked_text(&self.buffer_text(), &expected_selections, true)
                .replace(" \n", "•\n");

        self.assert_selections(expected_selections, expected_marked_text)
    }

    #[track_caller]
    fn editor_selections(&mut self) -> Vec<Range<usize>> {
        self.editor
            .update(&mut self.cx, |editor, cx| {
                editor.selections.all::<usize>(cx)
            })
            .into_iter()
            .map(|s| {
                if s.reversed {
                    s.end..s.start
                } else {
                    s.start..s.end
                }
            })
            .collect::<Vec<_>>()
    }

    #[track_caller]
    fn assert_selections(
        &mut self,
        expected_selections: Vec<Range<usize>>,
        expected_marked_text: String,
    ) {
        let actual_selections = self.editor_selections();
        let actual_marked_text =
            generate_marked_text(&self.buffer_text(), &actual_selections, true)
                .replace(" \n", "•\n");
        if expected_selections != actual_selections {
            pretty_assertions::assert_eq!(
                actual_marked_text,
                expected_marked_text,
                "{}Editor has unexpected selections",
                self.assertion_context(),
            );
        }
    }
}

#[track_caller]
pub fn assert_state_with_diff(
    editor: &Entity<Editor>,
    cx: &mut VisualTestContext,
    expected_diff_text: &str,
) {
    let (snapshot, selections) = editor.update_in(cx, |editor, window, cx| {
        (
            editor.snapshot(window, cx).buffer_snapshot.clone(),
            editor.selections.ranges::<usize>(cx),
        )
    });

    let actual_marked_text = generate_marked_text(&snapshot.text(), &selections, true);

    // Read the actual diff.
    let line_infos = snapshot.row_infos(MultiBufferRow(0)).collect::<Vec<_>>();
    let has_diff = line_infos.iter().any(|info| info.diff_status.is_some());
    let actual_diff = actual_marked_text
        .split('\n')
        .zip(line_infos)
        .map(|(line, info)| {
            let mut marker = match info.diff_status.map(|status| status.kind) {
                Some(DiffHunkStatusKind::Added) => "+ ",
                Some(DiffHunkStatusKind::Deleted) => "- ",
                Some(DiffHunkStatusKind::Modified) => unreachable!(),
                None => {
                    if has_diff {
                        "  "
                    } else {
                        ""
                    }
                }
            };
            if line.is_empty() {
                marker = marker.trim();
            }
            format!("{marker}{line}")
        })
        .collect::<Vec<_>>()
        .join("\n");

    pretty_assertions::assert_eq!(actual_diff, expected_diff_text, "unexpected diff state");
}

impl Deref for EditorTestContext {
    type Target = gpui::VisualTestContext;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl DerefMut for EditorTestContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

/// Tracks string context to be printed when assertions fail.
/// Often this is done by storing a context string in the manager and returning the handle.
#[derive(Clone)]
pub struct AssertionContextManager {
    id: Arc<AtomicUsize>,
    contexts: Arc<RwLock<BTreeMap<usize, String>>>,
}

impl Default for AssertionContextManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertionContextManager {
    pub fn new() -> Self {
        Self {
            id: Arc::new(AtomicUsize::new(0)),
            contexts: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add_context(&self, context: String) -> ContextHandle {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let mut contexts = self.contexts.write();
        contexts.insert(id, context);
        ContextHandle {
            id,
            manager: self.clone(),
        }
    }

    pub fn context(&self) -> String {
        let contexts = self.contexts.read();
        format!("\n{}\n", contexts.values().join("\n"))
    }
}

/// Used to track the lifetime of a piece of context so that it can be provided when an assertion fails.
/// For example, in the EditorTestContext, `set_state` returns a context handle so that if an assertion fails,
/// the state that was set initially for the failure can be printed in the error message
pub struct ContextHandle {
    id: usize,
    manager: AssertionContextManager,
}

impl Drop for ContextHandle {
    fn drop(&mut self) {
        let mut contexts = self.manager.contexts.write();
        contexts.remove(&self.id);
    }
}
