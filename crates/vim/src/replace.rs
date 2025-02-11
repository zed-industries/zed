use crate::{
    motion::{self, Motion},
    object::Object,
    state::Mode,
    Vim,
};
use editor::{display_map::ToDisplayPoint, Anchor, Bias, Editor, EditorSnapshot, ToPoint};
use gpui::{actions, Context, Window};
use language::Point;
use std::ops::Range;
use std::sync::Arc;

actions!(vim, [ToggleReplace, UndoReplace]);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, _: &ToggleReplace, window, cx| {
        vim.replacements = vec![];
        vim.start_recording(cx);
        vim.switch_mode(Mode::Replace, false, window, cx);
    });

    Vim::action(editor, cx, |vim, _: &UndoReplace, window, cx| {
        if vim.mode != Mode::Replace {
            return;
        }
        let count = Vim::take_count(cx);
        vim.undo_replace(count, window, cx)
    });
}

struct VimExchange;

impl Vim {
    pub(crate) fn multi_replace(
        &mut self,
        text: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let map = editor.snapshot(window, cx);
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

                editor.edit_with_block_indent(edits.clone(), Vec::new(), cx);

                editor.change_selections(None, window, cx, |s| {
                    s.select_anchor_ranges(edits.iter().map(|(range, _)| range.end..range.end));
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }

    fn undo_replace(
        &mut self,
        maybe_times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let map = editor.snapshot(window, cx);
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

                editor.edit(edits, cx);

                editor.change_selections(None, window, cx, |s| {
                    s.select_ranges(new_selections);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }

    pub fn exchange_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut selection = editor.selections.newest_display(cx);
            let snapshot = editor.snapshot(window, cx);
            object.expand_selection(&snapshot, &mut selection, around);
            let start = snapshot
                .buffer_snapshot
                .anchor_before(selection.start.to_point(&snapshot));
            let end = snapshot
                .buffer_snapshot
                .anchor_before(selection.end.to_point(&snapshot));
            let new_range = start..end;
            vim.exchange_impl(new_range, editor, &snapshot, window, cx);
            editor.set_clip_at_line_ends(true, cx);
        });
    }

    pub fn exchange_visual(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.stop_recording(cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            let selection = editor.selections.newest_anchor();
            let new_range = selection.start..selection.end;
            let snapshot = editor.snapshot(window, cx);
            vim.exchange_impl(new_range, editor, &snapshot, window, cx);
        });
        self.switch_mode(Mode::Normal, false, window, cx);
    }

    pub fn clear_exchange(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.stop_recording(cx);
        self.update_editor(window, cx, |_, editor, _, cx| {
            editor.clear_highlights::<VimExchange>(cx);
        });
    }

    pub fn exchange_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let text_layout_details = editor.text_layout_details(window);
            let mut selection = editor.selections.newest_display(cx);
            let snapshot = editor.snapshot(window, cx);
            motion.expand_selection(
                &snapshot,
                &mut selection,
                times,
                false,
                &text_layout_details,
            );
            let start = snapshot
                .buffer_snapshot
                .anchor_before(selection.start.to_point(&snapshot));
            let end = snapshot
                .buffer_snapshot
                .anchor_before(selection.end.to_point(&snapshot));
            let new_range = start..end;
            vim.exchange_impl(new_range, editor, &snapshot, window, cx);
            editor.set_clip_at_line_ends(true, cx);
        });
    }

    pub fn exchange_impl(
        &self,
        new_range: Range<Anchor>,
        editor: &mut Editor,
        snapshot: &EditorSnapshot,
        _window: &Window,
        cx: &mut Context<Editor>,
    ) {
        if let Some((_, ranges)) = editor.clear_background_highlights::<VimExchange>(cx) {
            let previous_range = ranges[0].clone();

            // ranges are separate
            if previous_range.end < new_range.start || new_range.end < previous_range.start {
                let previous_text: String = snapshot
                    .buffer_snapshot
                    .text_for_range(previous_range.clone())
                    .collect();
                let new_text: String = snapshot
                    .buffer_snapshot
                    .text_for_range(new_range.clone())
                    .collect();

                editor.edit([(previous_range, new_text), (new_range, previous_text)], cx);
                return;
            }

            // there is complete overlap, replace the large with the smaller
            if (previous_range.start <= new_range.start && previous_range.end >= new_range.end)
                || (new_range.start <= previous_range.start && new_range.end >= previous_range.end)
            {
                let smaller = if previous_range.len() < new_range.len() {
                    previous_range
                } else {
                    new_range
                };
                let new_text = snapshot
                    .buffer_snapshot
                    .text_for_range(smaller.clone())
                    .collect();
                editor.edit([(smaller, new_text)], cx);
                return;
            }

            // there is partial overlap, clear exchange and abort
            self.clear_exchange(window, cx);
            return;
        } else {
            let ranges = [new_range];
            editor.highlight_background::<VimExchange>(
                &ranges,
                |theme| theme.editor_document_highlight_read_background,
                cx,
            );
        }
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
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
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

        const UNDO_REPLACE_EXAMPLES: &[&str] = &[
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
