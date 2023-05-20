use std::{
    any::TypeId,
    ops::{Deref, DerefMut, Range},
};

use futures::Future;
use indoc::indoc;

use crate::{
    display_map::ToDisplayPoint, AnchorRangeExt, Autoscroll, DisplayPoint, Editor, MultiBuffer,
};
use gpui::{
    keymap_matcher::Keystroke, AppContext, ContextHandle, ModelContext, ViewContext, ViewHandle,
};
use language::{Buffer, BufferSnapshot};
use settings::Settings;
use util::{
    assert_set_eq,
    test::{generate_marked_text, marked_text_ranges},
};

use super::build_editor;

pub struct EditorTestContext<'a> {
    pub cx: &'a mut gpui::TestAppContext,
    pub window_id: usize,
    pub editor: ViewHandle<Editor>,
}

impl<'a> EditorTestContext<'a> {
    pub fn new(cx: &'a mut gpui::TestAppContext) -> EditorTestContext<'a> {
        let (window_id, editor) = cx.update(|cx| {
            cx.set_global(Settings::test(cx));
            crate::init(cx);

            let (window_id, editor) = cx.add_window(Default::default(), |cx| {
                cx.focus_self();
                build_editor(MultiBuffer::build_simple("", cx), cx)
            });

            (window_id, editor)
        });

        Self {
            cx,
            window_id,
            editor,
        }
    }

    pub fn condition(
        &self,
        predicate: impl FnMut(&Editor, &AppContext) -> bool,
    ) -> impl Future<Output = ()> {
        self.editor.condition(self.cx, predicate)
    }

    pub fn editor<F, T>(&self, read: F) -> T
    where
        F: FnOnce(&Editor, &ViewContext<Editor>) -> T,
    {
        self.editor.read_with(self.cx, read)
    }

    pub fn update_editor<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Editor, &mut ViewContext<Editor>) -> T,
    {
        self.editor.update(self.cx, update)
    }

    pub fn multibuffer<F, T>(&self, read: F) -> T
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

    pub fn buffer_text(&self) -> String {
        self.multibuffer(|buffer, cx| buffer.snapshot(cx).text())
    }

    pub fn buffer<F, T>(&self, read: F) -> T
    where
        F: FnOnce(&Buffer, &AppContext) -> T,
    {
        self.multibuffer(|multibuffer, cx| {
            let buffer = multibuffer.as_singleton().unwrap().read(cx);
            read(buffer, cx)
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

    pub fn buffer_snapshot(&self) -> BufferSnapshot {
        self.buffer(|buffer, _| buffer.snapshot())
    }

    pub fn simulate_keystroke(&mut self, keystroke_text: &str) -> ContextHandle {
        let keystroke_under_test_handle =
            self.add_assertion_context(format!("Simulated Keystroke: {:?}", keystroke_text));
        let keystroke = Keystroke::parse(keystroke_text).unwrap();
        self.cx.dispatch_keystroke(self.window_id, keystroke, false);
        keystroke_under_test_handle
    }

    pub fn simulate_keystrokes<const COUNT: usize>(
        &mut self,
        keystroke_texts: [&str; COUNT],
    ) -> ContextHandle {
        let keystrokes_under_test_handle =
            self.add_assertion_context(format!("Simulated Keystrokes: {:?}", keystroke_texts));
        for keystroke_text in keystroke_texts.into_iter() {
            self.simulate_keystroke(keystroke_text);
        }
        keystrokes_under_test_handle
    }

    pub fn ranges(&self, marked_text: &str) -> Vec<Range<usize>> {
        let (unmarked_text, ranges) = marked_text_ranges(marked_text, false);
        assert_eq!(self.buffer_text(), unmarked_text);
        ranges
    }

    pub fn display_point(&mut self, marked_text: &str) -> DisplayPoint {
        let ranges = self.ranges(marked_text);
        let snapshot = self
            .editor
            .update(self.cx, |editor, cx| editor.snapshot(cx));
        ranges[0].start.to_display_point(&snapshot)
    }

    // Returns anchors for the current buffer using `«` and `»`
    pub fn text_anchor_range(&self, marked_text: &str) -> Range<language::Anchor> {
        let ranges = self.ranges(marked_text);
        let snapshot = self.buffer_snapshot();
        snapshot.anchor_before(ranges[0].start)..snapshot.anchor_after(ranges[0].end)
    }

    pub fn set_diff_base(&mut self, diff_base: Option<&str>) {
        let diff_base = diff_base.map(String::from);
        self.update_buffer(|buffer, cx| buffer.set_diff_base(diff_base, cx));
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
            marked_text.escape_debug().to_string()
        ));
        let (unmarked_text, selection_ranges) = marked_text_ranges(marked_text, true);
        self.editor.update(self.cx, |editor, cx| {
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
            marked_text.escape_debug().to_string()
        ));
        let (unmarked_text, selection_ranges) = marked_text_ranges(marked_text, true);
        self.editor.update(self.cx, |editor, cx| {
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
            panic!("Unmarked text doesn't match buffer text\nBuffer text: {buffer_text:?}\nUnmarked text: {unmarked_text:?}\nRaw buffer text\n{buffer_text}Raw unmarked text\n{unmarked_text}");
        }

        self.assert_selections(expected_selections, marked_text.to_string())
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
                .unwrap_or_default()
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
            .highlight_ranges::<Tag>()
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
    fn assert_selections(
        &mut self,
        expected_selections: Vec<Range<usize>>,
        expected_marked_text: String,
    ) {
        let actual_selections = self
            .editor
            .read_with(self.cx, |editor, cx| editor.selections.all::<usize>(cx))
            .into_iter()
            .map(|s| {
                if s.reversed {
                    s.end..s.start
                } else {
                    s.start..s.end
                }
            })
            .collect::<Vec<_>>();
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

impl<'a> Deref for EditorTestContext<'a> {
    type Target = gpui::TestAppContext;

    fn deref(&self) -> &Self::Target {
        self.cx
    }
}

impl<'a> DerefMut for EditorTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}
