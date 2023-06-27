use gpui::ViewContext;
use language::Point;
use workspace::Workspace;

use crate::{motion::Motion, normal::ChangeCase, Vim};

pub fn change_case(_: &mut Workspace, _: &ChangeCase, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        let count = vim.pop_number_operator(cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            editor.transact(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        if selection.start == selection.end {
                            Motion::Right.expand_selection(map, selection, count, true);
                        }
                    })
                });
                let selections = editor.selections.all::<Point>(cx);
                for selection in selections.into_iter().rev() {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    editor.buffer().update(cx, |buffer, cx| {
                        let range = selection.start..selection.end;
                        let text = snapshot
                            .text_for_range(selection.start..selection.end)
                            .flat_map(|s| s.chars())
                            .flat_map(|c| {
                                if c.is_lowercase() {
                                    c.to_uppercase().collect::<Vec<char>>()
                                } else {
                                    c.to_lowercase().collect::<Vec<char>>()
                                }
                            })
                            .collect::<String>();

                        buffer.edit([(range, text)], None, cx)
                    })
                }
            });
            editor.set_clip_at_line_ends(true, cx);
        });
    })
}

#[cfg(test)]
mod test {
    use crate::{state::Mode, test::VimTestContext};
    use indoc::indoc;

    #[gpui::test]
    async fn test_change_case(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(indoc! {"ˇabC\n"}, Mode::Normal);
        cx.simulate_keystrokes(["~"]);
        cx.assert_editor_state("AˇbC\n");
        cx.simulate_keystrokes(["2", "~"]);
        cx.assert_editor_state("ABcˇ\n");

        cx.set_state(indoc! {"a😀C«dÉ1*fˇ»\n"}, Mode::Normal);
        cx.simulate_keystrokes(["~"]);
        cx.assert_editor_state("a😀CDé1*Fˇ\n");
    }
}
