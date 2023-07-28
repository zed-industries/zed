use gpui::WindowContext;
use language::Point;

use crate::{motion::Motion, Mode, Vim};

pub fn substitute(vim: &mut Vim, count: Option<usize>, cx: &mut WindowContext) {
    let line_mode = vim.state.mode == Mode::Visual { line: true };
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
            let selections = editor.selections.all::<Point>(cx);
            for selection in selections.into_iter().rev() {
                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit([(selection.start..selection.end, "")], None, cx)
                })
            }
        });
    });
}

#[cfg(test)]
mod test {
    use crate::{state::Mode, test::VimTestContext};
    use indoc::indoc;

    #[gpui::test]
    async fn test_substitute(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // supports a single cursor
        cx.set_state(indoc! {"ˇabc\n"}, Mode::Normal);
        cx.simulate_keystrokes(["s", "x"]);
        cx.assert_editor_state("xˇbc\n");

        // supports a selection
        cx.set_state(indoc! {"a«bcˇ»\n"}, Mode::Visual { line: false });
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
}
