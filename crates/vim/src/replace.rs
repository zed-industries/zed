use crate::{
    motion::{self},
    state::Mode,
    Vim,
};
use editor::{display_map::ToDisplayPoint, Bias, Editor, ToPoint};
use gpui::{actions, ViewContext};
use language::{AutoindentMode, Point};
use std::ops::Range;
use std::sync::Arc;

actions!(vim, [ToggleReplace, UndoReplace]);

pub fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, |vim, _: &ToggleReplace, cx| {
        vim.replacements = vec![];
        vim.start_recording(cx);
        vim.switch_mode(Mode::Replace, false, cx);
    });

    Vim::action(editor, cx, |vim, _: &UndoReplace, cx| {
        if vim.mode != Mode::Replace {
            return;
        }
        let count = vim.take_count(cx);
        vim.undo_replace(count, cx)
    });
}

impl Vim {
    pub(crate) fn multi_replace(&mut self, text: Arc<str>, cx: &mut ViewContext<Self>) {
        self.update_editor(cx, |vim, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let map = editor.snapshot(cx);
                let display_selections = editor.selections.all::<Point>(cx);

                // Handles all string that require manipulation, including inserts and replaces
                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let is_new_line = text.as_ref() == "\n";
                        let mut range = selection.range();
                        // "\n" need to be handled separately, because when a "\n" is typing,
                        // we don't do a replace, we need insert a "\n"
                        if !is_new_line {
                            range.end.column += 1;
                            range.end = map.buffer_snapshot.clip_point(range.end, Bias::Right);
                        }
                        let replace_range = map.buffer_snapshot.anchor_before(range.start)
                            ..map.buffer_snapshot.anchor_after(range.end);
                        let current_text = map
                            .buffer_snapshot
                            .text_for_range(replace_range.clone())
                            .collect();
                        vim.replacements.push((replace_range.clone(), current_text));
                        (replace_range, text.clone())
                    })
                    .collect::<Vec<_>>();

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(
                        edits.clone(),
                        Some(AutoindentMode::Block {
                            original_indent_columns: Vec::new(),
                        }),
                        cx,
                    );
                });

                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(edits.iter().map(|(range, _)| range.end..range.end));
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }

    fn undo_replace(&mut self, maybe_times: Option<usize>, cx: &mut ViewContext<Self>) {
        self.update_editor(cx, |vim, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let map = editor.snapshot(cx);
                let selections = editor.selections.all::<Point>(cx);
                let mut new_selections = vec![];
                let edits: Vec<(Range<Point>, String)> = selections
                    .into_iter()
                    .filter_map(|selection| {
                        let end = selection.head();
                        let start = motion::backspace(
                            &map,
                            end.to_display_point(&map),
                            maybe_times.unwrap_or(1),
                        )
                        .to_point(&map);
                        new_selections.push(
                            map.buffer_snapshot.anchor_before(start)
                                ..map.buffer_snapshot.anchor_before(start),
                        );

                        let mut undo = None;
                        let edit_range = start..end;
                        for (i, (range, inverse)) in vim.replacements.iter().rev().enumerate() {
                            if range.start.to_point(&map.buffer_snapshot) <= edit_range.start
                                && range.end.to_point(&map.buffer_snapshot) >= edit_range.end
                            {
                                undo = Some(inverse.clone());
                                vim.replacements.remove(vim.replacements.len() - i - 1);
                                break;
                            }
                        }
                        Some((edit_range, undo?))
                    })
                    .collect::<Vec<_>>();

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });

                editor.change_selections(None, cx, |s| {
                    s.select_ranges(new_selections);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_enter_and_exit_replace_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.simulate_keystrokes("shift-r");
        assert_eq!(cx.mode(), Mode::Replace);
        cx.simulate_keystrokes("escape");
        assert_eq!(cx.mode(), Mode::Normal);
    }

    #[gpui::test]
    async fn test_replace_mode(cx: &mut gpui::TestAppContext) {
        let mut cx: NeovimBackedTestContext = NeovimBackedTestContext::new(cx).await;

        // test normal replace
        cx.set_shared_state(indoc! {"
            ˇThe quick brown
            fox jumps over
            the lazy dog."})
            .await;
        cx.simulate_shared_keystrokes("shift-r O n e").await;
        cx.shared_state().await.assert_eq(indoc! {"
            Oneˇ quick brown
            fox jumps over
            the lazy dog."});

        // test replace with line ending
        cx.set_shared_state(indoc! {"
            The quick browˇn
            fox jumps over
            the lazy dog."})
            .await;
        cx.simulate_shared_keystrokes("shift-r O n e").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick browOneˇ
            fox jumps over
            the lazy dog."});

        // test replace with blank line
        cx.set_shared_state(indoc! {"
        The quick brown
        ˇ
        fox jumps over
        the lazy dog."})
            .await;
        cx.simulate_shared_keystrokes("shift-r O n e").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            Oneˇ
            fox jumps over
            the lazy dog."});

        // test replace with newline
        cx.set_shared_state(indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."})
            .await;
        cx.simulate_shared_keystrokes("shift-r enter O n e").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The qu
            Oneˇ brown
            fox jumps over
            the lazy dog."});

        // test replace with multi cursor and newline
        cx.set_state(
            indoc! {"
            ˇThe quick brown
            fox jumps over
            the lazy ˇdog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("shift-r O n e");
        cx.assert_state(
            indoc! {"
            Oneˇ quick brown
            fox jumps over
            the lazy Oneˇ."},
            Mode::Replace,
        );
        cx.simulate_keystrokes("enter T w o");
        cx.assert_state(
            indoc! {"
            One
            Twoˇck brown
            fox jumps over
            the lazy One
            Twoˇ"},
            Mode::Replace,
        );
    }

    #[gpui::test]
    async fn test_replace_mode_with_counts(cx: &mut gpui::TestAppContext) {
        let mut cx: NeovimBackedTestContext = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 shift-r - escape").await;
        cx.shared_state().await.assert_eq("--ˇ-lo\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 shift-r a b c escape")
            .await;
        cx.shared_state().await.assert_eq("abcabcabˇc\n");
    }

    #[gpui::test]
    async fn test_replace_mode_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx: NeovimBackedTestContext = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello world\n").await;
        cx.simulate_shared_keystrokes("shift-r - - - escape 4 l .")
            .await;
        cx.shared_state().await.assert_eq("---lo --ˇ-ld\n");
    }

    #[gpui::test]
    async fn test_replace_mode_undo(cx: &mut gpui::TestAppContext) {
        let mut cx: NeovimBackedTestContext = NeovimBackedTestContext::new(cx).await;

        const UNDO_REPLACE_EXAMPLES: &[&'static str] = &[
            // replace undo with single line
            "ˇThe quick brown fox jumps over the lazy dog.",
            // replace undo with ending line
            indoc! {"
                The quick browˇn
                fox jumps over
                the lazy dog."
            },
            // replace undo with empty line
            indoc! {"
                The quick brown
                ˇ
                fox jumps over
                the lazy dog."
            },
        ];

        for example in UNDO_REPLACE_EXAMPLES {
            // normal undo
            cx.simulate("shift-r O n e backspace backspace backspace", example)
                .await
                .assert_matches();
            // undo with new line
            cx.simulate("shift-r O enter e backspace backspace backspace", example)
                .await
                .assert_matches();
            cx.simulate(
                "shift-r O enter n enter e backspace backspace backspace backspace backspace",
                example,
            )
            .await
            .assert_matches();
        }
    }

    #[gpui::test]
    async fn test_replace_multicursor(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state("ˇabcˇabcabc", Mode::Normal);
        cx.simulate_keystrokes("shift-r 1 2 3 4");
        cx.assert_state("1234ˇ234ˇbc", Mode::Replace);
        assert_eq!(cx.mode(), Mode::Replace);
        cx.simulate_keystrokes("backspace backspace backspace backspace backspace");
        cx.assert_state("ˇabˇcabcabc", Mode::Replace);
    }

    #[gpui::test]
    async fn test_replace_undo(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇaaaa", Mode::Normal);
        cx.simulate_keystrokes("0 shift-r b b b escape u");
        cx.assert_state("ˇaaaa", Mode::Normal);
    }
}
