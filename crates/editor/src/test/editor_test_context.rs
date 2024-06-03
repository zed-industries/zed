use crate::{
    display_map::ToDisplayPoint, AnchorRangeExt, Autoscroll, DisplayPoint, Editor, MultiBuffer,
    RowExt,
};
use collections::BTreeMap;
use futures::Future;
use gpui::{
    AnyWindowHandle, AppContext, Keystroke, ModelContext, Pixels, Point, View, ViewContext,
    VisualTestContext,
};
use indoc::indoc;
use itertools::Itertools;
use language::{Buffer, BufferSnapshot, LanguageRegistry};
use multi_buffer::ExcerptRange;
use parking_lot::RwLock;
use project::{FakeFs, Project};
use std::{
    any::TypeId,
    ops::{Deref, DerefMut, Range},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use ui::Context;
use util::{
    assert_set_eq,
    test::{generate_marked_text, marked_text_ranges},
};

use super::{build_editor, build_editor_with_project};

pub struct EditorTestContext {
    pub cx: gpui::VisualTestContext,
    pub window: AnyWindowHandle,
    pub editor: View<Editor>,
    pub assertion_cx: AssertionContextManager,
}

impl EditorTestContext {
    pub async fn new(cx: &mut gpui::TestAppContext) -> EditorTestContext {
        let fs = FakeFs::new(cx.executor());
        // fs.insert_file("/file", "".to_owned()).await;
        fs.insert_tree(
            "/root",
            serde_json::json!({
                "file": "",
            }),
        )
        .await;
        let project = Project::test(fs, ["/root".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/root/file", cx)
            })
            .await
            .unwrap();
        let editor = cx.add_window(|cx| {
            let editor =
                build_editor_with_project(project, MultiBuffer::build_from_buffer(buffer, cx), cx);
            editor.focus(cx);
            editor
        });
        let editor_view = editor.root_view(cx).unwrap();
        Self {
            cx: VisualTestContext::from_window(*editor.deref(), cx),
            window: editor.into(),
            editor: editor_view,
            assertion_cx: AssertionContextManager::new(),
        }
    }

    pub fn new_multibuffer<const COUNT: usize>(
        cx: &mut gpui::TestAppContext,
        excerpts: [&str; COUNT],
    ) -> EditorTestContext {
        let mut multibuffer = MultiBuffer::new(0, language::Capability::ReadWrite);
        let buffer = cx.new_model(|cx| {
            for excerpt in excerpts.into_iter() {
                let (text, ranges) = marked_text_ranges(excerpt, false);
                let buffer = cx.new_model(|cx| Buffer::local(text, cx));
                multibuffer.push_excerpts(
                    buffer,
                    ranges.into_iter().map(|range| ExcerptRange {
                        context: range,
                        primary: None,
                    }),
                    cx,
                );
            }
            multibuffer
        });

        let editor = cx.add_window(|cx| {
            let editor = build_editor(buffer, cx);
            editor.focus(cx);
            editor
        });

        let editor_view = editor.root_view(cx).unwrap();
        Self {
            cx: VisualTestContext::from_window(*editor.deref(), cx),
            window: editor.into(),
            editor: editor_view,
            assertion_cx: AssertionContextManager::new(),
        }
    }

    pub fn condition(
        &self,
        predicate: impl FnMut(&Editor, &AppContext) -> bool,
    ) -> impl Future<Output = ()> {
        self.editor
            .condition::<crate::EditorEvent>(&self.cx, predicate)
    }

    #[track_caller]
    pub fn editor<F, T>(&mut self, read: F) -> T
    where
        F: FnOnce(&Editor, &ViewContext<Editor>) -> T,
    {
        self.editor
            .update(&mut self.cx, |this, cx| read(&this, &cx))
    }

    #[track_caller]
    pub fn update_editor<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Editor, &mut ViewContext<Editor>) -> T,
    {
        self.editor.update(&mut self.cx, update)
    }

    pub fn multibuffer<F, T>(&mut self, read: F) -> T
    where
        F: FnOnce(&MultiBuffer, &AppContext) -> T,
    {
        self.editor(|editor, cx| read(editor.buffer().read(cx), cx))
    }

    pub fn update_multibuffer<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut MultiBuffer, &mut ModelContext<MultiBuffer>) -> T,
    {
        self.update_editor(|editor, cx| editor.buffer().update(cx, update))
    }

    pub fn buffer_text(&mut self) -> String {
        self.multibuffer(|buffer, cx| buffer.snapshot(cx).text())
    }

    pub fn display_text(&mut self) -> String {
        self.update_editor(|editor, cx| editor.display_text(cx))
    }

    pub fn buffer<F, T>(&mut self, read: F) -> T
    where
        F: FnOnce(&Buffer, &AppContext) -> T,
    {
        self.multibuffer(|multibuffer, cx| {
            let buffer = multibuffer.as_singleton().unwrap().read(cx);
            read(buffer, cx)
        })
    }

    pub fn language_registry(&mut self) -> Arc<LanguageRegistry> {
        self.editor(|editor, cx| {
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
        F: FnOnce(&mut Buffer, &mut ModelContext<Buffer>) -> T,
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

    pub fn ranges(&mut self, marked_text: &str) -> Vec<Range<usize>> {
        let (unmarked_text, ranges) = marked_text_ranges(marked_text, false);
        assert_eq!(self.buffer_text(), unmarked_text);
        ranges
    }

    pub fn display_point(&mut self, marked_text: &str) -> DisplayPoint {
        let ranges = self.ranges(marked_text);
        let snapshot = self
            .editor
            .update(&mut self.cx, |editor, cx| editor.snapshot(cx));
        ranges[0].start.to_display_point(&snapshot)
    }

    pub fn pixel_position(&mut self, marked_text: &str) -> Point<Pixels> {
        let display_point = self.display_point(marked_text);
        self.pixel_position_for(display_point)
    }

    pub fn pixel_position_for(&mut self, display_point: DisplayPoint) -> Point<Pixels> {
        self.update_editor(|editor, cx| {
            let newest_point = editor.selections.newest_display(cx).head();
            let pixel_position = editor.pixel_position_of_newest_cursor.unwrap();
            let line_height = editor
                .style()
                .unwrap()
                .text
                .line_height_in_pixels(cx.rem_size());
            let snapshot = editor.snapshot(cx);
            let details = editor.text_layout_details(cx);

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

    pub fn set_diff_base(&mut self, diff_base: Option<&str>) {
        self.update_buffer(|buffer, cx| buffer.set_diff_base(diff_base.map(ToOwned::to_owned), cx));
    }

    /// Change the editor's text and selections using a string containing
    /// embedded range markers that represent the ranges and directions of
    /// each selection.
    ///
    /// Returns a context handle so that assertion failures can print what
    /// editor state was needed to cause the failure.
    ///
    /// See the `util::test::marked_text_ranges` function for more information.
    pub fn set_state(&mut self, marked_text: &str) -> ContextHandle {
        let state_context = self.add_assertion_context(format!(
            "Initial Editor State: \"{}\"",
            marked_text.escape_debug()
        ));
        let (unmarked_text, selection_ranges) = marked_text_ranges(marked_text, true);
        self.editor.update(&mut self.cx, |editor, cx| {
            editor.set_text(unmarked_text, cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_ranges(selection_ranges)
            })
        });
        state_context
    }

    /// Only change the editor's selections
    pub fn set_selections_state(&mut self, marked_text: &str) -> ContextHandle {
        let state_context = self.add_assertion_context(format!(
            "Initial Editor State: \"{}\"",
            marked_text.escape_debug()
        ));
        let (unmarked_text, selection_ranges) = marked_text_ranges(marked_text, true);
        self.editor.update(&mut self.cx, |editor, cx| {
            assert_eq!(editor.text(cx), unmarked_text);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_ranges(selection_ranges)
            })
        });
        state_context
    }

    /// Make an assertion about the editor's text and the ranges and directions
    /// of its selections using a string containing embedded range markers.
    ///
    /// See the `util::test::marked_text_ranges` function for more information.
    #[track_caller]
    pub fn assert_editor_state(&mut self, marked_text: &str) {
        let (unmarked_text, expected_selections) = marked_text_ranges(marked_text, true);
        let buffer_text = self.buffer_text();

        if buffer_text != unmarked_text {
            panic!("Unmarked text doesn't match buffer text\nBuffer text: {buffer_text:?}\nUnmarked text: {unmarked_text:?}\nRaw buffer text\n{buffer_text}\nRaw unmarked text\n{unmarked_text}");
        }

        self.assert_selections(expected_selections, marked_text.to_string())
    }

    pub fn editor_state(&mut self) -> String {
        generate_marked_text(self.buffer_text().as_str(), &self.editor_selections(), true)
    }

    #[track_caller]
    pub fn assert_editor_background_highlights<Tag: 'static>(&mut self, marked_text: &str) {
        let expected_ranges = self.ranges(marked_text);
        let actual_ranges: Vec<Range<usize>> = self.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            editor
                .background_highlights
                .get(&TypeId::of::<Tag>())
                .map(|h| h.1.clone())
                .unwrap_or_else(|| Arc::from([]))
                .into_iter()
                .map(|range| range.to_offset(&snapshot.buffer_snapshot))
                .collect()
        });
        assert_set_eq!(actual_ranges, expected_ranges);
    }

    #[track_caller]
    pub fn assert_editor_text_highlights<Tag: ?Sized + 'static>(&mut self, marked_text: &str) {
        let expected_ranges = self.ranges(marked_text);
        let snapshot = self.update_editor(|editor, cx| editor.snapshot(cx));
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
            generate_marked_text(&self.buffer_text(), &expected_selections, true);
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
            generate_marked_text(&self.buffer_text(), &actual_selections, true);
        if expected_selections != actual_selections {
            panic!(
                indoc! {"

                {}Editor has unexpected selections.

                Expected selections:
                {}

                Actual selections:
                {}
            "},
                self.assertion_context(),
                expected_marked_text,
                actual_marked_text,
            );
        }
    }
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
