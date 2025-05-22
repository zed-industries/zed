use editor::{Editor, movement};
use gpui::{Context, Window, actions};
use language::Point;

use crate::{
    Mode, Vim,
    motion::{Motion, MotionKind},
};

actions!(vim, [Substitute, SubstituteLine]);

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, _: &Substitute, window, cx| {
        vim.start_recording(cx);
        let count = Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        vim.substitute(count, vim.mode == Mode::VisualLine, window, cx);
    });

    Vim::action(editor, cx, |vim, _: &SubstituteLine, window, cx| {
        vim.start_recording(cx);
        if matches!(vim.mode, Mode::VisualBlock | Mode::Visual) {
            vim.switch_mode(Mode::VisualLine, false, window, cx)
        }
        let count = Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        vim.substitute(count, true, window, cx)
    });
}

impl Vim {
    pub fn substitute(
        &mut self,
        count: Option<usize>,
        line_mode: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.store_visual_marks(window, cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.set_clip_at_line_ends(false, cx);
            editor.transact(window, cx, |editor, window, cx| {
                let text_layout_details = editor.text_layout_details(window);
                editor.change_selections(None, window, cx, |s| {
                    s.move_with(|map, selection| {
                        if selection.start == selection.end {
                            Motion::Right.expand_selection(
                                map,
                                selection,
                                count,
                                &text_layout_details,
                                false,
                            );
                        }
                        if line_mode {
                            // in Visual mode when the selection contains the newline at the end
                            // of the line, we should exclude it.
                            if !selection.is_empty() && selection.end.column() == 0 {
                                selection.end = movement::left(map, selection.end);
                            }
                            Motion::CurrentLine.expand_selection(
                                map,
                                selection,
                                None,
                                &text_layout_details,
                                false,
                            );
                            if let Some((point, _)) = (Motion::FirstNonWhitespace {
                                display_lines: false,
                            })
                            .move_point(
                                map,
                                selection.start,
                                selection.goal,
                                None,
                                &text_layout_details,
                            ) {
                                selection.start = point;
                            }
                        }
                    })
                });
                let kind = if line_mode {
                    MotionKind::Linewise
                } else {
                    MotionKind::Exclusive
                };
                vim.copy_selections_content(editor, kind, window, cx);
                let selections = editor
                    .selections
                    .all::<Point>(&editor.selections.display_map(cx))
                    .into_iter();
                let edits = selections.map(|selection| (selection.start..selection.end, ""));
                editor.edit(edits, cx);
            });
        });
        self.switch_mode(Mode::Insert, true, window, cx);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use indoc::indoc;

    #[gpui::test]
    async fn test_substitute(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // supports a single cursor
        cx.set_state(indoc! {"ˇabc\n"}, Mode::Normal);
        cx.simulate_keystrokes("s x");
        cx.assert_editor_state("xˇbc\n");

        // supports a selection
        cx.set_state(indoc! {"a«bcˇ»\n"}, Mode::Visual);
        cx.assert_editor_state("a«bcˇ»\n");
        cx.simulate_keystrokes("s x");
        cx.assert_editor_state("axˇ\n");

        // supports counts
        cx.set_state(indoc! {"ˇabc\n"}, Mode::Normal);
        cx.simulate_keystrokes("2 s x");
        cx.assert_editor_state("xˇc\n");

        // supports multiple cursors
        cx.set_state(indoc! {"a«bcˇ»deˇffg\n"}, Mode::Normal);
        cx.simulate_keystrokes("2 s x");
        cx.assert_editor_state("axˇdexˇg\n");

        // does not read beyond end of line
        cx.set_state(indoc! {"ˇabc\n"}, Mode::Normal);
        cx.simulate_keystrokes("5 s x");
        cx.assert_editor_state("xˇ\n");

        // it handles multibyte characters
        cx.set_state(indoc! {"ˇcàfé\n"}, Mode::Normal);
        cx.simulate_keystrokes("4 s");
        cx.assert_editor_state("ˇ\n");

        // should transactionally undo selection changes
        cx.simulate_keystrokes("escape u");
        cx.assert_editor_state("ˇcàfé\n");

        // it handles visual line mode
        cx.set_state(
            indoc! {"
            alpha
              beˇta
            gamma"},
            Mode::Normal,
        );
        cx.simulate_keystrokes("shift-v s");
        cx.assert_editor_state(indoc! {"
            alpha
              ˇ
            gamma"});
    }

    #[gpui::test]
    async fn test_visual_change(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("The quick ˇbrown").await;
        cx.simulate_shared_keystrokes("v w c").await;
        cx.shared_state().await.assert_eq("The quick ˇ");

        cx.set_shared_state(indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("v w j c").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The ˇver
            the lazy dog"});

        cx.simulate_at_each_offset(
            "v w j c",
            indoc! {"
                    The ˇquick brown
                    fox jumps ˇover
                    the ˇlazy dog"},
        )
        .await
        .assert_matches();
        cx.simulate_at_each_offset(
            "v w k c",
            indoc! {"
                    The ˇquick brown
                    fox jumps ˇover
                    the ˇlazy dog"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_visual_line_change(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "shift-v c",
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog"},
        )
        .await
        .assert_matches();
        // Test pasting code copied on change
        cx.simulate_shared_keystrokes("escape j p").await;
        cx.shared_state().await.assert_matches();

        cx.simulate_at_each_offset(
            "shift-v c",
            indoc! {"
            The quick brown
            fox juˇmps over
            the laˇzy dog"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "shift-v j c",
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog"},
        )
        .await
        .assert_matches();
        // Test pasting code copied on delete
        cx.simulate_shared_keystrokes("escape j p").await;
        cx.shared_state().await.assert_matches();

        cx.simulate_at_each_offset(
            "shift-v j c",
            indoc! {"
            The quick brown
            fox juˇmps over
            the laˇzy dog"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_substitute_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        let initial_state = indoc! {"
                    The quick brown
                    fox juˇmps over
                    the lazy dog
                    "};

        // normal mode
        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("shift-s o").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            oˇ
            the lazy dog
            "});

        // visual mode
        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("v k shift-s o").await;
        cx.shared_state().await.assert_eq(indoc! {"
            oˇ
            the lazy dog
            "});

        // visual block mode
        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("ctrl-v j shift-s o").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            oˇ
            "});

        // visual mode including newline
        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("v $ shift-s o").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            oˇ
            the lazy dog
            "});

        // indentation
        cx.set_neovim_option("shiftwidth=4").await;
        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("> > shift-s o").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
                oˇ
            the lazy dog
            "});
    }
}
