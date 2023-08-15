use gpui::WindowContext;
use language::Point;

use crate::{motion::Motion, utils::copy_selections_content, Mode, Vim};

pub fn substitute(vim: &mut Vim, count: Option<usize>, cx: &mut WindowContext) {
    let line_mode = vim.state.mode == Mode::VisualLine;
    vim.switch_mode(Mode::Insert, true, cx);
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    if selection.start == selection.end {
                        Motion::Right.expand_selection(map, selection, count, true);
                    }
                    if line_mode {
                        Motion::CurrentLine.expand_selection(map, selection, None, false);
                        if let Some((point, _)) = Motion::FirstNonWhitespace.move_point(
                            map,
                            selection.start,
                            selection.goal,
                            None,
                        ) {
                            selection.start = point;
                        }
                    }
                })
            });
            copy_selections_content(editor, line_mode, cx);
            let selections = editor.selections.all::<Point>(cx).into_iter();
            let edits = selections.map(|selection| (selection.start..selection.end, ""));
            editor.edit(edits, cx);
        });
    });
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
        cx.simulate_keystrokes(["s", "x"]);
        cx.assert_editor_state("xˇbc\n");

        // supports a selection
        cx.set_state(indoc! {"a«bcˇ»\n"}, Mode::Visual);
        cx.assert_editor_state("a«bcˇ»\n");
        cx.simulate_keystrokes(["s", "x"]);
        cx.assert_editor_state("axˇ\n");

        // supports counts
        cx.set_state(indoc! {"ˇabc\n"}, Mode::Normal);
        cx.simulate_keystrokes(["2", "s", "x"]);
        cx.assert_editor_state("xˇc\n");

        // supports multiple cursors
        cx.set_state(indoc! {"a«bcˇ»deˇffg\n"}, Mode::Normal);
        cx.simulate_keystrokes(["2", "s", "x"]);
        cx.assert_editor_state("axˇdexˇg\n");

        // does not read beyond end of line
        cx.set_state(indoc! {"ˇabc\n"}, Mode::Normal);
        cx.simulate_keystrokes(["5", "s", "x"]);
        cx.assert_editor_state("xˇ\n");

        // it handles multibyte characters
        cx.set_state(indoc! {"ˇcàfé\n"}, Mode::Normal);
        cx.simulate_keystrokes(["4", "s"]);
        cx.assert_editor_state("ˇ\n");

        // should transactionally undo selection changes
        cx.simulate_keystrokes(["escape", "u"]);
        cx.assert_editor_state("ˇcàfé\n");

        // it handles visual line mode
        cx.set_state(
            indoc! {"
            alpha
              beˇta
            gamma"},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["shift-v", "s"]);
        cx.assert_editor_state(indoc! {"
            alpha
              ˇ
            gamma"});
    }

    #[gpui::test]
    async fn test_visual_change(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("The quick ˇbrown").await;
        cx.simulate_shared_keystrokes(["v", "w", "c"]).await;
        cx.assert_shared_state("The quick ˇ").await;

        cx.set_shared_state(indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "w", "j", "c"]).await;
        cx.assert_shared_state(indoc! {"
            The ˇver
            the lazy dog"})
            .await;

        let cases = cx.each_marked_position(indoc! {"
            The ˇquick brown
            fox jumps ˇover
            the ˇlazy dog"});
        for initial_state in cases {
            cx.assert_neovim_compatible(&initial_state, ["v", "w", "j", "c"])
                .await;
            cx.assert_neovim_compatible(&initial_state, ["v", "w", "k", "c"])
                .await;
        }
    }

    #[gpui::test]
    async fn test_visual_line_change(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["shift-v", "c"]);
        cx.assert(indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog"})
            .await;
        // Test pasting code copied on change
        cx.simulate_shared_keystrokes(["escape", "j", "p"]).await;
        cx.assert_state_matches().await;

        cx.assert_all(indoc! {"
            The quick brown
            fox juˇmps over
            the laˇzy dog"})
            .await;
        let mut cx = cx.binding(["shift-v", "j", "c"]);
        cx.assert(indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog"})
            .await;
        // Test pasting code copied on delete
        cx.simulate_shared_keystrokes(["escape", "j", "p"]).await;
        cx.assert_state_matches().await;

        cx.assert_all(indoc! {"
            The quick brown
            fox juˇmps over
            the laˇzy dog"})
            .await;
    }
}
