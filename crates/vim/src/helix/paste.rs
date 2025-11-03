use editor::{ToOffset, movement};
use gpui::{Action, Context, Window};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Vim, state::Mode};

/// Pastes text from the specified register at the cursor position.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub struct HelixPaste {
    #[serde(default)]
    before: bool,
}

impl Vim {
    pub fn helix_paste(
        &mut self,
        action: &HelixPaste,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.record_current_action(cx);
        self.store_visual_marks(window, cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        // TODO: vim paste calls take_forced_motion here, but I don't know what that does
        // (none of the other helix_ methods call it)

        self.update_editor(cx, |vim, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let selected_register = vim.selected_register.take();

                let Some((text, clipboard_selections)) = Vim::update_globals(cx, |globals, cx| {
                    globals.read_register(selected_register, Some(editor), cx)
                })
                .and_then(|reg| {
                    (!reg.text.is_empty())
                        .then_some(reg.text)
                        .zip(reg.clipboard_selections)
                }) else {
                    return;
                };

                let display_map = editor.display_snapshot(cx);
                let current_selections = editor.selections.all_adjusted_display(&display_map);

                // The clipboard can have multiple selections, and there can
                // be multiple selections. Helix zips them together, so the first
                // clipboard entry gets pasted at the first selection, the second
                // entry gets pasted at the second selection, and so on. If there
                // are more clipboard selections than selections, the extra ones
                // don't get pasted anywhere. If there are more selections than
                // clipboard selections, the last clipboard selection gets
                // pasted at all remaining selections.

                let mut edits = Vec::new();
                let mut new_selections = Vec::new();
                let mut start_offset = 0;

                let mut replacement_texts: Vec<String> = Vec::new();

                for ix in 0..current_selections.len() {
                    let to_insert = if let Some(clip_sel) = clipboard_selections.get(ix) {
                        let end_offset = start_offset + clip_sel.len;
                        let text = text[start_offset..end_offset].to_string();
                        start_offset = end_offset + 1;
                        text
                    } else if let Some(last_text) = replacement_texts.last() {
                        // We have more current selections than clipboard selections: repeat the last one.
                        last_text.to_owned()
                    } else {
                        text.to_string()
                    };
                    replacement_texts.push(to_insert);
                }

                let line_mode = replacement_texts.iter().any(|text| text.ends_with('\n'));

                for (to_insert, sel) in replacement_texts.into_iter().zip(current_selections) {
                    // Helix doesn't care about the head/tail of the selection.
                    // Pasting before means pasting before the whole selection.
                    let display_point = if line_mode {
                        if action.before {
                            movement::line_beginning(&display_map, sel.start, false)
                        } else {
                            if sel.start == sel.end {
                                movement::right(
                                    &display_map,
                                    movement::line_end(&display_map, sel.end, false),
                                )
                            } else {
                                sel.end
                            }
                        }
                    } else if action.before {
                        sel.start
                    } else if sel.start == sel.end {
                        // Helix and Zed differ in how they understand
                        // single-point cursors. In Helix, a single-point cursor
                        // is "on top" of some character, and pasting after that
                        // cursor means that the pasted content should go after
                        // that character. (If the cursor is at the end of a
                        // line, the pasted content goes on the next line.)
                        movement::right(&display_map, sel.end)
                    } else {
                        sel.end
                    };
                    let point = display_point.to_point(&display_map);
                    let anchor = if action.before {
                        display_map.buffer_snapshot().anchor_after(point)
                    } else {
                        display_map.buffer_snapshot().anchor_before(point)
                    };
                    edits.push((point..point, to_insert.repeat(count)));
                    new_selections.push((anchor, to_insert.len() * count));
                }

                editor.edit(edits, cx);

                editor.change_selections(Default::default(), window, cx, |s| {
                    let snapshot = s.buffer().clone();
                    s.select_ranges(new_selections.into_iter().map(|(anchor, len)| {
                        let offset = anchor.to_offset(&snapshot);
                        if action.before {
                            offset.saturating_sub(len)..offset
                        } else {
                            offset..(offset + len)
                        }
                    }));
                })
            });
        });

        self.switch_mode(Mode::HelixNormal, true, window, cx);
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state(
            indoc! {"
            The «quiˇ»ck brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("y w p");

        cx.assert_state(
            indoc! {"
            The quick «quiˇ»brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Pasting before the selection:
        cx.set_state(
            indoc! {"
            The quick brown
            fox «jumpsˇ» over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("shift-p");
        cx.assert_state(
            indoc! {"
            The quick brown
            fox «quiˇ»jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_point_selection_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state(
            indoc! {"
            The quiˇck brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("y");

        // Pasting before the selection:
        cx.set_state(
            indoc! {"
            The quick brown
            fox jumpsˇ over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("shift-p");
        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps«cˇ» over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Pasting after the selection:
        cx.set_state(
            indoc! {"
            The quick brown
            fox jumpsˇ over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("p");
        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps «cˇ»over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Pasting after the selection at the end of a line:
        cx.set_state(
            indoc! {"
            The quick brown
            fox jumps overˇ
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("p");
        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            «cˇ»the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_multi_cursor_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        // Select two blocks of text.
        cx.set_state(
            indoc! {"
            The «quiˇ»ck brown
            fox ju«mpsˇ» over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("y");

        // Only one cursor: only the first block gets pasted.
        cx.set_state(
            indoc! {"
            ˇThe quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("shift-p");
        cx.assert_state(
            indoc! {"
            «quiˇ»The quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Two cursors: both get pasted.
        cx.set_state(
            indoc! {"
            ˇThe ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("shift-p");
        cx.assert_state(
            indoc! {"
            «quiˇ»The «mpsˇ»quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Three cursors: the second yanked block is duplicated.
        cx.set_state(
            indoc! {"
            ˇThe ˇquick brown
            fox jumpsˇ over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("shift-p");
        cx.assert_state(
            indoc! {"
            «quiˇ»The «mpsˇ»quick brown
            fox jumps«mpsˇ» over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Again with three cursors. All three should be pasted twice.
        cx.set_state(
            indoc! {"
            ˇThe ˇquick brown
            fox jumpsˇ over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("2 shift-p");
        cx.assert_state(
            indoc! {"
            «quiquiˇ»The «mpsmpsˇ»quick brown
            fox jumps«mpsmpsˇ» over
            the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_line_mode_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state(
            indoc! {"
            The quick brow«n
            ˇ»fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("y shift-p");

        cx.assert_state(
            indoc! {"
            «n
            ˇ»The quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // In line mode, if we're in the middle of a line then pasting before pastes on
        // the line before.
        cx.set_state(
            indoc! {"
            The quick brown
            fox jumpsˇ over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("shift-p");
        cx.assert_state(
            indoc! {"
            The quick brown
            «n
            ˇ»fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // In line mode, if we're in the middle of a line then pasting after pastes on
        // the line after.
        cx.set_state(
            indoc! {"
            The quick brown
            fox jumpsˇ over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("p");
        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            «n
            ˇ»the lazy dog."},
            Mode::HelixNormal,
        );

        // If we're currently at the end of a line, "the line after"
        // means right after the cursor.
        cx.set_state(
            indoc! {"
            The quick brown
            fox jumps overˇ
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("p");
        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            «n
            ˇ»the lazy dog."},
            Mode::HelixNormal,
        );

        cx.set_state(
            indoc! {"

            The quick brown
            fox jumps overˇ
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x y up up p");
        cx.assert_state(
            indoc! {"

            «fox jumps over
            ˇ»The quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.set_state(
            indoc! {"
            «The quick brown
            fox jumps over
            ˇ»the lazy dog."},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("y p p");
        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            The quick brown
            fox jumps over
            «The quick brown
            fox jumps over
            ˇ»the lazy dog."},
            Mode::HelixNormal,
        );
    }
}
