use std::cmp;

use editor::{
    display_map::ToDisplayPoint, movement, scroll::Autoscroll, ClipboardSelection, DisplayPoint,
};
use gpui::{impl_actions, AppContext, ViewContext};
use language::{Bias, SelectionGoal};
use serde::Deserialize;
use settings::Settings;
use workspace::Workspace;

use crate::{state::Mode, utils::copy_selections_content, UseSystemClipboard, Vim, VimSettings};

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Paste {
    #[serde(default)]
    before: bool,
    #[serde(default)]
    preserve_clipboard: bool,
}

impl_actions!(vim, [Paste]);

pub(crate) fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(paste);
}

fn system_clipboard_is_newer(vim: &Vim, cx: &mut AppContext) -> bool {
    cx.read_from_clipboard().is_some_and(|item| {
        if let Some(last_state) = vim.workspace_state.registers.get(".system.") {
            last_state != item.text()
        } else {
            true
        }
    })
}

fn paste(_: &mut Workspace, action: &Paste, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.record_current_action(cx);
        vim.update_active_editor(cx, |vim, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let (clipboard_text, clipboard_selections): (String, Option<_>) =
                    if VimSettings::get_global(cx).use_system_clipboard == UseSystemClipboard::Never
                        || VimSettings::get_global(cx).use_system_clipboard
                            == UseSystemClipboard::OnYank
                            && !system_clipboard_is_newer(vim, cx)
                    {
                        (
                            vim.workspace_state
                                .registers
                                .get("\"")
                                .cloned()
                                .unwrap_or_else(|| "".to_string()),
                            None,
                        )
                    } else {
                        if let Some(item) = cx.read_from_clipboard() {
                            let clipboard_selections = item
                                .metadata::<Vec<ClipboardSelection>>()
                                .filter(|clipboard_selections| {
                                    clipboard_selections.len() > 1
                                        && vim.state().mode != Mode::VisualLine
                                });
                            (item.text().clone(), clipboard_selections)
                        } else {
                            ("".into(), None)
                        }
                    };

                if clipboard_text.is_empty() {
                    return;
                }

                if !action.preserve_clipboard && vim.state().mode.is_visual() {
                    copy_selections_content(vim, editor, vim.state().mode == Mode::VisualLine, cx);
                }

                let (display_map, current_selections) = editor.selections.all_adjusted_display(cx);

                // unlike zed, if you have a multi-cursor selection from vim block mode,
                // pasting it will paste it on subsequent lines, even if you don't yet
                // have a cursor there.
                let mut selections_to_process = Vec::new();
                let mut i = 0;
                while i < current_selections.len() {
                    selections_to_process
                        .push((current_selections[i].start..current_selections[i].end, true));
                    i += 1;
                }
                if let Some(clipboard_selections) = clipboard_selections.as_ref() {
                    let left = current_selections
                        .iter()
                        .map(|selection| cmp::min(selection.start.column(), selection.end.column()))
                        .min()
                        .unwrap();
                    let mut row = current_selections.last().unwrap().end.row() + 1;
                    while i < clipboard_selections.len() {
                        let cursor =
                            display_map.clip_point(DisplayPoint::new(row, left), Bias::Left);
                        selections_to_process.push((cursor..cursor, false));
                        i += 1;
                        row += 1;
                    }
                }

                let first_selection_indent_column =
                    clipboard_selections.as_ref().and_then(|zed_selections| {
                        zed_selections
                            .first()
                            .map(|selection| selection.first_line_indent)
                    });
                let before = action.before || vim.state().mode == Mode::VisualLine;

                let mut edits = Vec::new();
                let mut new_selections = Vec::new();
                let mut original_indent_columns = Vec::new();
                let mut start_offset = 0;

                for (ix, (selection, preserve)) in selections_to_process.iter().enumerate() {
                    let (mut to_insert, original_indent_column) =
                        if let Some(clipboard_selections) = &clipboard_selections {
                            if let Some(clipboard_selection) = clipboard_selections.get(ix) {
                                let end_offset = start_offset + clipboard_selection.len;
                                let text = clipboard_text[start_offset..end_offset].to_string();
                                start_offset = end_offset + 1;
                                (text, Some(clipboard_selection.first_line_indent))
                            } else {
                                ("".to_string(), first_selection_indent_column)
                            }
                        } else {
                            (clipboard_text.to_string(), first_selection_indent_column)
                        };
                    let line_mode = to_insert.ends_with('\n');
                    let is_multiline = to_insert.contains('\n');

                    if line_mode && !before {
                        if selection.is_empty() {
                            to_insert =
                                "\n".to_owned() + &to_insert[..to_insert.len() - "\n".len()];
                        } else {
                            to_insert = "\n".to_owned() + &to_insert;
                        }
                    } else if !line_mode && vim.state().mode == Mode::VisualLine {
                        to_insert = to_insert + "\n";
                    }

                    let display_range = if !selection.is_empty() {
                        selection.start..selection.end
                    } else if line_mode {
                        let point = if before {
                            movement::line_beginning(&display_map, selection.start, false)
                        } else {
                            movement::line_end(&display_map, selection.start, false)
                        };
                        point..point
                    } else {
                        let point = if before {
                            selection.start
                        } else {
                            movement::saturating_right(&display_map, selection.start)
                        };
                        point..point
                    };

                    let point_range = display_range.start.to_point(&display_map)
                        ..display_range.end.to_point(&display_map);
                    let anchor = if is_multiline || vim.state().mode == Mode::VisualLine {
                        display_map.buffer_snapshot.anchor_before(point_range.start)
                    } else {
                        display_map.buffer_snapshot.anchor_after(point_range.end)
                    };

                    if *preserve {
                        new_selections.push((anchor, line_mode, is_multiline));
                    }
                    edits.push((point_range, to_insert));
                    original_indent_columns.extend(original_indent_column);
                }

                editor.edit_with_block_indent(edits, original_indent_columns, cx);

                // in line_mode vim will insert the new text on the next (or previous if before) line
                // and put the cursor on the first non-blank character of the first inserted line (or at the end if the first line is blank).
                // otherwise vim will insert the next text at (or before) the current cursor position,
                // the cursor will go to the last (or first, if is_multiline) inserted character.
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.replace_cursors_with(|map| {
                        let mut cursors = Vec::new();
                        for (anchor, line_mode, is_multiline) in &new_selections {
                            let mut cursor = anchor.to_display_point(map);
                            if *line_mode {
                                if !before {
                                    cursor = movement::down(
                                        map,
                                        cursor,
                                        SelectionGoal::None,
                                        false,
                                        &text_layout_details,
                                    )
                                    .0;
                                }
                                cursor = movement::indented_line_beginning(map, cursor, true);
                            } else if !is_multiline {
                                cursor = movement::saturating_left(map, cursor)
                            }
                            cursors.push(cursor);
                            if vim.state().mode == Mode::VisualBlock {
                                break;
                            }
                        }

                        cursors
                    });
                })
            });
        });
        vim.switch_mode(Mode::Normal, true, cx);
    });
}

#[cfg(test)]
mod test {
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
        UseSystemClipboard, VimSettings,
    };
    use gpui::ClipboardItem;
    use indoc::indoc;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // single line
        cx.set_shared_state(indoc! {"
            The quick brown
            fox ˇjumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "w", "y"]).await;
        cx.assert_shared_clipboard("jumps o").await;
        cx.set_shared_state(indoc! {"
            The quick brown
            fox jumps oveˇr
            the lazy dog"})
            .await;
        cx.simulate_shared_keystroke("p").await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            fox jumps overjumps ˇo
            the lazy dog"})
            .await;

        cx.set_shared_state(indoc! {"
            The quick brown
            fox jumps oveˇr
            the lazy dog"})
            .await;
        cx.simulate_shared_keystroke("shift-p").await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            fox jumps ovejumps ˇor
            the lazy dog"})
            .await;

        // line mode
        cx.set_shared_state(indoc! {"
            The quick brown
            fox juˇmps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["d", "d"]).await;
        cx.assert_shared_clipboard("fox jumps over\n").await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            the laˇzy dog"})
            .await;
        cx.simulate_shared_keystroke("p").await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            the lazy dog
            ˇfox jumps over"})
            .await;
        cx.simulate_shared_keystrokes(["k", "shift-p"]).await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            ˇfox jumps over
            the lazy dog
            fox jumps over"})
            .await;

        // multiline, cursor to first character of pasted text.
        cx.set_shared_state(indoc! {"
            The quick brown
            fox jumps ˇover
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "j", "y"]).await;
        cx.assert_shared_clipboard("over\nthe lazy do").await;

        cx.simulate_shared_keystroke("p").await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            fox jumps oˇover
            the lazy dover
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["u", "shift-p"]).await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            fox jumps ˇover
            the lazy doover
            the lazy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_yank_system_clipboard_never(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<VimSettings>(cx, |s| {
                s.use_system_clipboard = Some(UseSystemClipboard::Never)
            });
        });

        cx.set_state(
            indoc! {"
                The quick brown
                fox jˇumps over
                the lazy dog"},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["v", "i", "w", "y"]);
        cx.assert_state(
            indoc! {"
                The quick brown
                fox ˇjumps over
                the lazy dog"},
            Mode::Normal,
        );
        cx.simulate_keystroke("p");
        cx.assert_state(
            indoc! {"
                The quick brown
                fox jjumpˇsumps over
                the lazy dog"},
            Mode::Normal,
        );
        assert_eq!(cx.read_from_clipboard(), None);
    }

    #[gpui::test]
    async fn test_yank_system_clipboard_on_yank(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<VimSettings>(cx, |s| {
                s.use_system_clipboard = Some(UseSystemClipboard::OnYank)
            });
        });

        // copy in visual mode
        cx.set_state(
            indoc! {"
                The quick brown
                fox jˇumps over
                the lazy dog"},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["v", "i", "w", "y"]);
        cx.assert_state(
            indoc! {"
                The quick brown
                fox ˇjumps over
                the lazy dog"},
            Mode::Normal,
        );
        cx.simulate_keystroke("p");
        cx.assert_state(
            indoc! {"
                The quick brown
                fox jjumpˇsumps over
                the lazy dog"},
            Mode::Normal,
        );
        assert_eq!(
            cx.read_from_clipboard().map(|item| item.text().clone()),
            Some("jumps".into())
        );
        cx.simulate_keystrokes(["d", "d", "p"]);
        cx.assert_state(
            indoc! {"
                The quick brown
                the lazy dog
                ˇfox jjumpsumps over"},
            Mode::Normal,
        );
        assert_eq!(
            cx.read_from_clipboard().map(|item| item.text().clone()),
            Some("jumps".into())
        );
        cx.write_to_clipboard(ClipboardItem::new("test-copy".to_string()));
        cx.simulate_keystroke("shift-p");
        cx.assert_state(
            indoc! {"
                The quick brown
                the lazy dog
                test-copˇyfox jjumpsumps over"},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_paste_visual(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // copy in visual mode
        cx.set_shared_state(indoc! {"
                The quick brown
                fox jˇumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "i", "w", "y"]).await;
        cx.assert_shared_state(indoc! {"
                The quick brown
                fox ˇjumps over
                the lazy dog"})
            .await;
        // paste in visual mode
        cx.simulate_shared_keystrokes(["w", "v", "i", "w", "p"])
            .await;
        cx.assert_shared_state(indoc! {"
                The quick brown
                fox jumps jumpˇs
                the lazy dog"})
            .await;
        cx.assert_shared_clipboard("over").await;
        // paste in visual line mode
        cx.simulate_shared_keystrokes(["up", "shift-v", "shift-p"])
            .await;
        cx.assert_shared_state(indoc! {"
            ˇover
            fox jumps jumps
            the lazy dog"})
            .await;
        cx.assert_shared_clipboard("over").await;
        // paste in visual block mode
        cx.simulate_shared_keystrokes(["ctrl-v", "down", "down", "p"])
            .await;
        cx.assert_shared_state(indoc! {"
            oveˇrver
            overox jumps jumps
            overhe lazy dog"})
            .await;

        // copy in visual line mode
        cx.set_shared_state(indoc! {"
                The quick brown
                fox juˇmps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["shift-v", "d"]).await;
        cx.assert_shared_state(indoc! {"
                The quick brown
                the laˇzy dog"})
            .await;
        // paste in visual mode
        cx.simulate_shared_keystrokes(["v", "i", "w", "p"]).await;
        cx.assert_shared_state(
            &indoc! {"
                The quick brown
                the_
                ˇfox jumps over
                _dog"}
            .replace('_', " "), // Hack for trailing whitespace
        )
        .await;
        cx.assert_shared_clipboard("lazy").await;
        cx.set_shared_state(indoc! {"
            The quick brown
            fox juˇmps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["shift-v", "d"]).await;
        cx.assert_shared_state(indoc! {"
            The quick brown
            the laˇzy dog"})
            .await;
        // paste in visual line mode
        cx.simulate_shared_keystrokes(["k", "shift-v", "p"]).await;
        cx.assert_shared_state(indoc! {"
            ˇfox jumps over
            the lazy dog"})
            .await;
        cx.assert_shared_clipboard("The quick brown\n").await;
    }

    #[gpui::test]
    async fn test_paste_visual_block(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // copy in visual block mode
        cx.set_shared_state(indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["ctrl-v", "2", "j", "y"])
            .await;
        cx.assert_shared_clipboard("q\nj\nl").await;
        cx.simulate_shared_keystrokes(["p"]).await;
        cx.assert_shared_state(indoc! {"
            The qˇquick brown
            fox jjumps over
            the llazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "i", "w", "shift-p"])
            .await;
        cx.assert_shared_state(indoc! {"
            The ˇq brown
            fox jjjumps over
            the lllazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "i", "w", "shift-p"])
            .await;

        cx.set_shared_state(indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["ctrl-v", "j", "y"]).await;
        cx.assert_shared_clipboard("q\nj").await;
        cx.simulate_shared_keystrokes(["l", "ctrl-v", "2", "j", "shift-p"])
            .await;
        cx.assert_shared_state(indoc! {"
            The qˇqick brown
            fox jjmps over
            the lzy dog"})
            .await;

        cx.simulate_shared_keystrokes(["shift-v", "p"]).await;
        cx.assert_shared_state(indoc! {"
            ˇq
            j
            fox jjmps over
            the lzy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_paste_indent(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new_typescript(cx).await;

        cx.set_state(
            indoc! {"
            class A {ˇ
            }
        "},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["o", "a", "(", ")", "{", "escape"]);
        cx.assert_state(
            indoc! {"
            class A {
                a()ˇ{}
            }
            "},
            Mode::Normal,
        );
        // cursor goes to the first non-blank character in the line;
        cx.simulate_keystrokes(["y", "y", "p"]);
        cx.assert_state(
            indoc! {"
            class A {
                a(){}
                ˇa(){}
            }
            "},
            Mode::Normal,
        );
        // indentation is preserved when pasting
        cx.simulate_keystrokes(["u", "shift-v", "up", "y", "shift-p"]);
        cx.assert_state(
            indoc! {"
                ˇclass A {
                    a(){}
                class A {
                    a(){}
                }
                "},
            Mode::Normal,
        );
    }
}
