use std::ops::{Deref, Range};

use collections::BTreeMap;
use itertools::{Either, Itertools};

use editor::{display_map::ToDisplayPoint, Autoscroll};
use gpui::{json::json, keymap::Keystroke, ViewHandle};
use indoc::indoc;
use language::Selection;
use util::{
    set_eq,
    test::{marked_text, marked_text_ranges_by, SetEqError},
};
use workspace::{WorkspaceHandle, WorkspaceParams};

use crate::{state::Operator, *};

pub struct VimTestContext<'a> {
    cx: &'a mut gpui::TestAppContext,
    window_id: usize,
    editor: ViewHandle<Editor>,
}

impl<'a> VimTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext, enabled: bool) -> VimTestContext<'a> {
        cx.update(|cx| {
            editor::init(cx);
            crate::init(cx);

            settings::KeymapFileContent::load("keymaps/vim.json", cx).unwrap();
        });

        let params = cx.update(WorkspaceParams::test);

        cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = enabled;
            });
        });

        params
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "dir": { "test.txt": "" } }))
            .await;

        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let item = workspace
            .update(cx, |workspace, cx| workspace.open_path(file, true, cx))
            .await
            .expect("Could not open test file");

        let editor = cx.update(|cx| {
            item.act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });
        editor.update(cx, |_, cx| cx.focus_self());

        Self {
            cx,
            window_id,
            editor,
        }
    }

    pub fn enable_vim(&mut self) {
        self.cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = true;
            });
        })
    }

    pub fn disable_vim(&mut self) {
        self.cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = false;
            });
        })
    }

    pub fn mode(&mut self) -> Mode {
        self.cx.read(|cx| cx.global::<Vim>().state.mode)
    }

    pub fn active_operator(&mut self) -> Option<Operator> {
        self.cx
            .read(|cx| cx.global::<Vim>().state.operator_stack.last().copied())
    }

    pub fn editor_text(&mut self) -> String {
        self.editor
            .update(self.cx, |editor, cx| editor.snapshot(cx).text())
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

    pub fn set_state(&mut self, text: &str, mode: Mode) {
        self.cx
            .update(|cx| Vim::update(cx, |vim, cx| vim.switch_mode(mode, cx)));
        self.editor.update(self.cx, |editor, cx| {
            let (unmarked_text, markers) = marked_text(&text);
            editor.set_text(unmarked_text, cx);
            let cursor_offset = markers[0];
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.replace_cursors_with(|map| vec![cursor_offset.to_display_point(map)])
            });
        })
    }

    // Asserts the editor state via a marked string.
    // `|` characters represent empty selections
    // `[` to `}` represents a non empty selection with the head at `}`
    // `{` to `]` represents a non empty selection with the head at `{`
    pub fn assert_editor_state(&mut self, text: &str) {
        let (text_with_ranges, expected_empty_selections) = marked_text(&text);
        let (unmarked_text, mut selection_ranges) =
            marked_text_ranges_by(&text_with_ranges, vec![('[', '}'), ('{', ']')]);
        let editor_text = self.editor_text();
        assert_eq!(
            editor_text, unmarked_text,
            "Unmarked text doesn't match editor text"
        );

        let expected_reverse_selections = selection_ranges.remove(&('{', ']')).unwrap_or_default();
        let expected_forward_selections = selection_ranges.remove(&('[', '}')).unwrap_or_default();

        self.assert_selections(
            expected_empty_selections,
            expected_reverse_selections,
            expected_forward_selections,
            Some(text.to_string()),
        )
    }

    pub fn assert_editor_selections(&mut self, expected_selections: Vec<Selection<usize>>) {
        let (expected_empty_selections, expected_non_empty_selections): (Vec<_>, Vec<_>) =
            expected_selections.into_iter().partition_map(|selection| {
                if selection.is_empty() {
                    Either::Left(selection.head())
                } else {
                    Either::Right(selection)
                }
            });

        let (expected_reverse_selections, expected_forward_selections): (Vec<_>, Vec<_>) =
            expected_non_empty_selections
                .into_iter()
                .partition_map(|selection| {
                    let range = selection.start..selection.end;
                    if selection.reversed {
                        Either::Left(range)
                    } else {
                        Either::Right(range)
                    }
                });

        self.assert_selections(
            expected_empty_selections,
            expected_reverse_selections,
            expected_forward_selections,
            None,
        )
    }

    fn assert_selections(
        &mut self,
        expected_empty_selections: Vec<usize>,
        expected_reverse_selections: Vec<Range<usize>>,
        expected_forward_selections: Vec<Range<usize>>,
        asserted_text: Option<String>,
    ) {
        let (empty_selections, reverse_selections, forward_selections) =
            self.editor.read_with(self.cx, |editor, cx| {
                let (empty_selections, non_empty_selections): (Vec<_>, Vec<_>) = editor
                    .selections
                    .interleaved::<usize>(cx)
                    .into_iter()
                    .partition_map(|selection| {
                        if selection.is_empty() {
                            Either::Left(selection.head())
                        } else {
                            Either::Right(selection)
                        }
                    });

                let (reverse_selections, forward_selections): (Vec<_>, Vec<_>) =
                    non_empty_selections.into_iter().partition_map(|selection| {
                        let range = selection.start..selection.end;
                        if selection.reversed {
                            Either::Left(range)
                        } else {
                            Either::Right(range)
                        }
                    });
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

        let unmarked_text = self.editor_text();
        let all_eq: Result<(), SetEqError<String>> =
            set_eq!(expected_empty_selections, empty_selections)
                .map_err(|err| {
                    err.map(|missing| {
                        let mut error_text = unmarked_text.clone();
                        error_text.insert(missing, '|');
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
                        Extra Selection Location: {}
                        Asserted selections: {}
                        Actual selections: {}"},
                    location_text, asserted_selections, actual_selections,
                );
            }
            Err(SetEqError::RightMissing(location_text)) => {
                panic!(
                    indoc! {"
                        Editor is missing empty selection
                        Missing Selection Location: {}
                        Asserted selections: {}
                        Actual selections: {}"},
                    location_text, asserted_selections, actual_selections,
                );
            }
            _ => {}
        }
    }

    fn insert_markers(
        &mut self,
        empty_selections: &Vec<usize>,
        reverse_selections: &Vec<Range<usize>>,
        forward_selections: &Vec<Range<usize>>,
    ) -> String {
        let mut editor_text_with_selections = self.editor_text();
        let mut selection_marks = BTreeMap::new();
        for offset in empty_selections {
            selection_marks.insert(offset, '|');
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

    pub fn assert_binding<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        initial_state: &str,
        initial_mode: Mode,
        state_after: &str,
        mode_after: Mode,
    ) {
        self.set_state(initial_state, initial_mode);
        self.simulate_keystrokes(keystrokes);
        self.assert_editor_state(state_after);
        assert_eq!(self.mode(), mode_after);
        assert_eq!(self.active_operator(), None);
    }

    pub fn binding<const COUNT: usize>(
        mut self,
        keystrokes: [&'static str; COUNT],
    ) -> VimBindingTestContext<'a, COUNT> {
        let mode = self.mode();
        VimBindingTestContext::new(keystrokes, mode, mode, self)
    }
}

impl<'a> Deref for VimTestContext<'a> {
    type Target = gpui::TestAppContext;

    fn deref(&self) -> &Self::Target {
        self.cx
    }
}

pub struct VimBindingTestContext<'a, const COUNT: usize> {
    cx: VimTestContext<'a>,
    keystrokes_under_test: [&'static str; COUNT],
    mode_before: Mode,
    mode_after: Mode,
}

impl<'a, const COUNT: usize> VimBindingTestContext<'a, COUNT> {
    pub fn new(
        keystrokes_under_test: [&'static str; COUNT],
        mode_before: Mode,
        mode_after: Mode,
        cx: VimTestContext<'a>,
    ) -> Self {
        Self {
            cx,
            keystrokes_under_test,
            mode_before,
            mode_after,
        }
    }

    pub fn binding<const NEW_COUNT: usize>(
        self,
        keystrokes_under_test: [&'static str; NEW_COUNT],
    ) -> VimBindingTestContext<'a, NEW_COUNT> {
        VimBindingTestContext {
            keystrokes_under_test,
            cx: self.cx,
            mode_before: self.mode_before,
            mode_after: self.mode_after,
        }
    }

    pub fn mode_after(mut self, mode_after: Mode) -> Self {
        self.mode_after = mode_after;
        self
    }

    pub fn assert(&mut self, initial_state: &str, state_after: &str) {
        self.cx.assert_binding(
            self.keystrokes_under_test,
            initial_state,
            self.mode_before,
            state_after,
            self.mode_after,
        )
    }
}

impl<'a, const COUNT: usize> Deref for VimBindingTestContext<'a, COUNT> {
    type Target = VimTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}
