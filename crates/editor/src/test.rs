use std::ops::{Deref, DerefMut, Range};

use indoc::indoc;

use collections::BTreeMap;
use gpui::{keymap::Keystroke, ModelHandle, ViewContext, ViewHandle};
use language::Selection;
use settings::Settings;
use util::{
    set_eq,
    test::{marked_text, marked_text_ranges, marked_text_ranges_by, SetEqError},
};

use crate::{
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
    Autoscroll, DisplayPoint, Editor, EditorMode, MultiBuffer,
};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

// Returns a snapshot from text containing '|' character markers with the markers removed, and DisplayPoints for each one.
pub fn marked_display_snapshot(
    text: &str,
    cx: &mut gpui::MutableAppContext,
) -> (DisplaySnapshot, Vec<DisplayPoint>) {
    let (unmarked_text, markers) = marked_text(text);

    let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
    let font_id = cx
        .font_cache()
        .select_font(family_id, &Default::default())
        .unwrap();
    let font_size = 14.0;

    let buffer = MultiBuffer::build_simple(&unmarked_text, cx);
    let display_map =
        cx.add_model(|cx| DisplayMap::new(buffer, font_id, font_size, None, 1, 1, cx));
    let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
    let markers = markers
        .into_iter()
        .map(|offset| offset.to_display_point(&snapshot))
        .collect();

    (snapshot, markers)
}

pub fn select_ranges(editor: &mut Editor, marked_text: &str, cx: &mut ViewContext<Editor>) {
    let (umarked_text, text_ranges) = marked_text_ranges(marked_text);
    assert_eq!(editor.text(cx), umarked_text);
    editor.change_selections(None, cx, |s| s.select_ranges(text_ranges));
}

pub fn assert_text_with_selections(
    editor: &mut Editor,
    marked_text: &str,
    cx: &mut ViewContext<Editor>,
) {
    let (unmarked_text, text_ranges) = marked_text_ranges(marked_text);

    assert_eq!(editor.text(cx), unmarked_text);
    assert_eq!(editor.selections.ranges(cx), text_ranges);
}

pub(crate) fn build_editor(
    buffer: ModelHandle<MultiBuffer>,
    cx: &mut ViewContext<Editor>,
) -> Editor {
    Editor::new(EditorMode::Full, buffer, None, None, None, cx)
}

pub struct EditorTestContext<'a> {
    pub cx: &'a mut gpui::TestAppContext,
    pub window_id: usize,
    pub editor: ViewHandle<Editor>,
}

impl<'a> EditorTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext) -> EditorTestContext<'a> {
        let (window_id, editor) = cx.update(|cx| {
            cx.set_global(Settings::test(cx));
            crate::init(cx);

            let (window_id, editor) = cx.add_window(Default::default(), |cx| {
                build_editor(MultiBuffer::build_simple("", cx), cx)
            });

            editor.update(cx, |_, cx| cx.focus_self());

            (window_id, editor)
        });

        Self {
            cx,
            window_id,
            editor,
        }
    }

    pub fn update_editor<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Editor, &mut ViewContext<Editor>) -> T,
    {
        self.editor.update(self.cx, update)
    }

    pub fn buffer_text(&mut self) -> String {
        self.editor.read_with(self.cx, |editor, cx| {
            editor.buffer.read(cx).snapshot(cx).text()
        })
    }

    pub fn simulate_keystroke(&mut self, keystroke_text: &str) {
        let keystroke = Keystroke::parse(keystroke_text).unwrap();
        let input = if keystroke.modified() {
            None
        } else {
            Some(keystroke.key.clone())
        };
        self.cx
            .dispatch_keystroke(self.window_id, keystroke, input, false);
    }

    pub fn simulate_keystrokes<const COUNT: usize>(&mut self, keystroke_texts: [&str; COUNT]) {
        for keystroke_text in keystroke_texts.into_iter() {
            self.simulate_keystroke(keystroke_text);
        }
    }

    // Sets the editor state via a marked string.
    // `|` characters represent empty selections
    // `[` to `}` represents a non empty selection with the head at `}`
    // `{` to `]` represents a non empty selection with the head at `{`
    pub fn set_state(&mut self, text: &str) {
        self.editor.update(self.cx, |editor, cx| {
            let (unmarked_text, mut selection_ranges) = marked_text_ranges_by(
                &text,
                vec!['|'.into(), ('[', '}').into(), ('{', ']').into()],
            );
            editor.set_text(unmarked_text, cx);

            let mut selections: Vec<Range<usize>> =
                selection_ranges.remove(&'|'.into()).unwrap_or_default();
            selections.extend(
                selection_ranges
                    .remove(&('{', ']').into())
                    .unwrap_or_default()
                    .into_iter()
                    .map(|range| range.end..range.start),
            );
            selections.extend(
                selection_ranges
                    .remove(&('[', '}').into())
                    .unwrap_or_default(),
            );

            editor.change_selections(Some(Autoscroll::Fit), cx, |s| s.select_ranges(selections));
        })
    }

    // Asserts the editor state via a marked string.
    // `|` characters represent empty selections
    // `[` to `}` represents a non empty selection with the head at `}`
    // `{` to `]` represents a non empty selection with the head at `{`
    pub fn assert_editor_state(&mut self, text: &str) {
        let (unmarked_text, mut selection_ranges) = marked_text_ranges_by(
            &text,
            vec!['|'.into(), ('[', '}').into(), ('{', ']').into()],
        );
        let buffer_text = self.buffer_text();
        assert_eq!(
            buffer_text, unmarked_text,
            "Unmarked text doesn't match buffer text"
        );

        let expected_empty_selections = selection_ranges.remove(&'|'.into()).unwrap_or_default();
        let expected_reverse_selections = selection_ranges
            .remove(&('{', ']').into())
            .unwrap_or_default();
        let expected_forward_selections = selection_ranges
            .remove(&('[', '}').into())
            .unwrap_or_default();

        self.assert_selections(
            expected_empty_selections,
            expected_reverse_selections,
            expected_forward_selections,
            Some(text.to_string()),
        )
    }

    pub fn assert_editor_selections(&mut self, expected_selections: Vec<Selection<usize>>) {
        let mut empty_selections = Vec::new();
        let mut reverse_selections = Vec::new();
        let mut forward_selections = Vec::new();

        for selection in expected_selections {
            let range = selection.range();
            if selection.is_empty() {
                empty_selections.push(range);
            } else if selection.reversed {
                reverse_selections.push(range);
            } else {
                forward_selections.push(range)
            }
        }

        self.assert_selections(
            empty_selections,
            reverse_selections,
            forward_selections,
            None,
        )
    }

    fn assert_selections(
        &mut self,
        expected_empty_selections: Vec<Range<usize>>,
        expected_reverse_selections: Vec<Range<usize>>,
        expected_forward_selections: Vec<Range<usize>>,
        asserted_text: Option<String>,
    ) {
        let (empty_selections, reverse_selections, forward_selections) =
            self.editor.read_with(self.cx, |editor, cx| {
                let mut empty_selections = Vec::new();
                let mut reverse_selections = Vec::new();
                let mut forward_selections = Vec::new();

                for selection in editor.selections.all::<usize>(cx) {
                    let range = selection.range();
                    if selection.is_empty() {
                        empty_selections.push(range);
                    } else if selection.reversed {
                        reverse_selections.push(range);
                    } else {
                        forward_selections.push(range)
                    }
                }

                (empty_selections, reverse_selections, forward_selections)
            });

        let asserted_selections = asserted_text.unwrap_or_else(|| {
            self.insert_markers(
                &expected_empty_selections,
                &expected_reverse_selections,
                &expected_forward_selections,
            )
        });
        let actual_selections =
            self.insert_markers(&empty_selections, &reverse_selections, &forward_selections);

        let unmarked_text = self.buffer_text();
        let all_eq: Result<(), SetEqError<String>> =
            set_eq!(expected_empty_selections, empty_selections)
                .map_err(|err| {
                    err.map(|missing| {
                        let mut error_text = unmarked_text.clone();
                        error_text.insert(missing.start, '|');
                        error_text
                    })
                })
                .and_then(|_| {
                    set_eq!(expected_reverse_selections, reverse_selections).map_err(|err| {
                        err.map(|missing| {
                            let mut error_text = unmarked_text.clone();
                            error_text.insert(missing.start, '{');
                            error_text.insert(missing.end, ']');
                            error_text
                        })
                    })
                })
                .and_then(|_| {
                    set_eq!(expected_forward_selections, forward_selections).map_err(|err| {
                        err.map(|missing| {
                            let mut error_text = unmarked_text.clone();
                            error_text.insert(missing.start, '[');
                            error_text.insert(missing.end, '}');
                            error_text
                        })
                    })
                });

        match all_eq {
            Err(SetEqError::LeftMissing(location_text)) => {
                panic!(
                    indoc! {"
                        Editor has extra selection
                        Extra Selection Location:
                        {}
                        Asserted selections:
                        {}
                        Actual selections:
                        {}"},
                    location_text, asserted_selections, actual_selections,
                );
            }
            Err(SetEqError::RightMissing(location_text)) => {
                panic!(
                    indoc! {"
                        Editor is missing empty selection
                        Missing Selection Location:
                        {}
                        Asserted selections:
                        {}
                        Actual selections:
                        {}"},
                    location_text, asserted_selections, actual_selections,
                );
            }
            _ => {}
        }
    }

    fn insert_markers(
        &mut self,
        empty_selections: &Vec<Range<usize>>,
        reverse_selections: &Vec<Range<usize>>,
        forward_selections: &Vec<Range<usize>>,
    ) -> String {
        let mut editor_text_with_selections = self.buffer_text();
        let mut selection_marks = BTreeMap::new();
        for range in empty_selections {
            selection_marks.insert(&range.start, '|');
        }
        for range in reverse_selections {
            selection_marks.insert(&range.start, '{');
            selection_marks.insert(&range.end, ']');
        }
        for range in forward_selections {
            selection_marks.insert(&range.start, '[');
            selection_marks.insert(&range.end, '}');
        }
        for (offset, mark) in selection_marks.into_iter().rev() {
            editor_text_with_selections.insert(*offset, mark);
        }

        editor_text_with_selections
    }

    pub fn assert_clipboard_content(&mut self, expected_content: Option<&str>) {
        self.cx.update(|cx| {
            let actual_content = cx.read_from_clipboard().map(|item| item.text().to_owned());
            let expected_content = expected_content.map(|content| content.to_owned());
            assert_eq!(actual_content, expected_content);
        })
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
