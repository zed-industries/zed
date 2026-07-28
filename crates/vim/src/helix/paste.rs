use std::ops::Range;

use editor::{
    Anchor, DisplayPoint, MultiBufferOffset, MultiBufferSnapshot, SelectionEffects, ToOffset,
    display_map::DisplaySnapshot, movement,
};
use gpui::{Action, Context, Window};
use language::{Bias, Selection};
use schemars::JsonSchema;
use serde::Deserialize;
use text::{LineEnding, SelectionGoal};

use crate::{
    Vim,
    helix::HelixReplaceWithYanked,
    state::{Mode, Register},
};

/// Pastes text from the specified register at the cursor position.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub struct HelixPaste {
    #[serde(default)]
    before: bool,
}

struct ReplacementTarget {
    edit_range: Range<MultiBufferOffset>,
    start_anchor: Anchor,
    replacement_len: usize,
    id: usize,
    reversed: bool,
    goal: SelectionGoal,
}

impl ReplacementTarget {
    fn is_replaceable(&self) -> bool {
        !self.edit_range.is_empty()
    }

    fn into_selection(self, snapshot: &MultiBufferSnapshot) -> Selection<MultiBufferOffset> {
        let start = self.start_anchor.to_offset(snapshot);
        Selection {
            id: self.id,
            start,
            end: start + self.replacement_len,
            reversed: self.reversed,
            goal: self.goal,
        }
    }
}

fn register_texts_for_selections(
    register: Register,
    selection_count: usize,
    count: usize,
) -> Option<Vec<String>> {
    let Register {
        text,
        clipboard_selections,
    } = register;
    let mut start_offset: usize = 0;
    let mut replacement_texts: Vec<String> = Vec::with_capacity(selection_count);

    for index in 0..selection_count {
        let replacement_text = if let Some(clipboard_selection) = clipboard_selections
            .as_ref()
            .and_then(|items| items.get(index))
        {
            let end_offset = start_offset.checked_add(clipboard_selection.len)?;
            let replacement_text = text.get(start_offset..end_offset)?.to_string();
            start_offset = if clipboard_selection.is_entire_line {
                end_offset
            } else {
                end_offset.checked_add(1)?
            };
            replacement_text
        } else if let Some(last_text) = replacement_texts.last() {
            last_text.clone()
        } else {
            text.to_string()
        };

        replacement_texts.push(replacement_text);
    }

    for replacement_text in &mut replacement_texts {
        // Clipboard metadata describes the original bytes, but `Editor::edit` normalizes line
        // endings. Normalize after splitting and before callers measure lengths, otherwise CRLF
        // input can produce selections beyond the inserted text.
        LineEnding::normalize(replacement_text);
        *replacement_text = replacement_text.repeat(count);
    }

    Some(replacement_texts)
}

fn replacement_targets(
    display_map: &DisplaySnapshot,
    selections: Vec<Selection<DisplayPoint>>,
) -> Vec<ReplacementTarget> {
    selections
        .into_iter()
        .map(|selection| {
            let mut range = selection.range();
            if range.is_empty() {
                range.end = movement::saturating_right(display_map, range.start);
            }

            let edit_range = range.start.to_offset(display_map, Bias::Left)
                ..range.end.to_offset(display_map, Bias::Left);
            ReplacementTarget {
                start_anchor: display_map
                    .buffer_snapshot()
                    .anchor_before(edit_range.start),
                edit_range,
                replacement_len: 0,
                id: selection.id,
                reversed: selection.reversed,
                goal: selection.goal,
            }
        })
        .collect()
}

fn prepare_replacement_edits(
    targets: &mut [ReplacementTarget],
    replacement_texts: Vec<String>,
) -> Option<Vec<(Range<MultiBufferOffset>, String)>> {
    let mut edits = Vec::with_capacity(replacement_texts.len());
    let mut replacement_texts = replacement_texts.into_iter();

    for target in targets {
        if !target.is_replaceable() {
            continue;
        }

        let replacement_text = replacement_texts.next()?;
        target.replacement_len = replacement_text.len();
        edits.push((target.edit_range.clone(), replacement_text));
    }

    Some(edits)
}

impl Vim {
    pub fn helix_replace_with_yanked(
        &mut self,
        _: &HelixReplaceWithYanked,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.record_current_action(cx);
        self.store_visual_marks(window, cx);
        let count = Vim::take_count(cx).unwrap_or(1);

        self.update_editor(cx, |vim, editor, cx| {
            if editor.read_only(cx) {
                return;
            }

            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let selected_register = vim.selected_register.take();
                let Some(register) = Vim::update_globals(cx, |globals, cx| {
                    globals.read_register(selected_register, Some(editor), cx)
                }) else {
                    return;
                };

                let display_map = editor.display_snapshot(cx);
                let current_selections = editor.selections.all_display(&display_map);
                let mut targets = replacement_targets(&display_map, current_selections);
                let replacement_count = targets
                    .iter()
                    .filter(|target| target.is_replaceable())
                    .count();

                let Some(replacement_texts) =
                    register_texts_for_selections(register, replacement_count, count)
                else {
                    return;
                };
                let Some(edits) = prepare_replacement_edits(&mut targets, replacement_texts) else {
                    return;
                };

                editor.edit(edits, cx);

                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select(
                        targets
                            .into_iter()
                            .map(|target| target.into_selection(&snapshot))
                            .collect(),
                    );
                });
            });
        });

        self.switch_mode(Mode::HelixNormal, true, window, cx);
    }

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
            if editor.read_only(cx) {
                return;
            }

            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let selected_register = vim.selected_register.take();

                let Some(register) = Vim::update_globals(cx, |globals, cx| {
                    globals.read_register(selected_register, Some(editor), cx)
                })
                .filter(|reg| !reg.text.is_empty()) else {
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
                let Some(replacement_texts) =
                    register_texts_for_selections(register, current_selections.len(), count)
                else {
                    return;
                };

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
                        // In Helix, a single-point cursor is "on top" of a
                        // character, and pasting after means after that character.
                        // At line end this means the next line. But on an empty
                        // line there is no character, so paste at the cursor.
                        let right = movement::right(&display_map, sel.end);
                        if right.row() != sel.end.row() && sel.end.column() == 0 {
                            sel.end
                        } else {
                            right
                        }
                    } else {
                        sel.end
                    };
                    let point = display_point.to_point(&display_map);
                    let anchor = if action.before {
                        display_map.buffer_snapshot().anchor_after(point)
                    } else {
                        display_map.buffer_snapshot().anchor_before(point)
                    };
                    new_selections.push((anchor, to_insert.len()));
                    edits.push((point..point, to_insert));
                }

                editor.edit(edits, cx);

                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges(new_selections.into_iter().map(|(anchor, len)| {
                        let offset = anchor.to_offset(&snapshot);
                        if action.before {
                            offset.saturating_sub_usize(len)..offset
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

    use gpui::ClipboardItem;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_system_clipboard_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state(
            indoc! {"
            The quiˇck brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.write_to_clipboard(ClipboardItem::new_string("clipboard".to_string()));
        cx.simulate_keystrokes("p");
        cx.assert_state(
            indoc! {"
            The quic«clipboardˇ»k brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Multiple cursors with system clipboard (no metadata) pastes
        // the same text at each cursor.
        cx.set_state(
            indoc! {"
            ˇThe quick brown
            fox ˇjumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
        cx.write_to_clipboard(ClipboardItem::new_string("hi".to_string()));
        cx.simulate_keystrokes("p");
        cx.assert_state(
            indoc! {"
            T«hiˇ»he quick brown
            fox j«hiˇ»umps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // Multiple cursors on empty lines should paste on those same lines.
        cx.set_state("ˇ\nˇ\nˇ\nend", Mode::HelixNormal);
        cx.write_to_clipboard(ClipboardItem::new_string("X".to_string()));
        cx.simulate_keystrokes("p");
        cx.assert_state("«Xˇ»\n«Xˇ»\n«Xˇ»\nend", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_system_clipboard_crlf_paste_at_end_of_buffer(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇ", Mode::HelixNormal);

        cx.write_to_clipboard(ClipboardItem::new_string("a\r\nb".to_string()));
        cx.simulate_keystrokes("p");

        cx.assert_state("«a\nbˇ»", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_read_only_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("aˇb", Mode::HelixNormal);
        cx.write_to_clipboard(ClipboardItem::new_string("clipboard".to_string()));
        cx.update_editor(|editor, _window, _cx| editor.set_read_only(true));

        cx.simulate_keystrokes("p");

        cx.assert_state("aˇb", Mode::HelixNormal);
    }

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
