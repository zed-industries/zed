use editor::scroll::Autoscroll;
use gpui::ViewContext;
use language::{Bias, Point};
use workspace::Workspace;

use crate::{
    normal::ChangeCase, normal::ConvertToLowerCase, normal::ConvertToUpperCase, state::Mode, Vim,
};

pub fn change_case(_: &mut Workspace, _: &ChangeCase, cx: &mut ViewContext<Workspace>) {
    manipulate_text(cx, |c| {
        if c.is_lowercase() {
            c.to_uppercase().collect::<Vec<char>>()
        } else {
            c.to_lowercase().collect::<Vec<char>>()
        }
    })
}

pub fn convert_to_upper_case(
    _: &mut Workspace,
    _: &ConvertToUpperCase,
    cx: &mut ViewContext<Workspace>,
) {
    manipulate_text(cx, |c| c.to_uppercase().collect::<Vec<char>>())
}

pub fn convert_to_lower_case(
    _: &mut Workspace,
    _: &ConvertToLowerCase,
    cx: &mut ViewContext<Workspace>,
) {
    manipulate_text(cx, |c| c.to_lowercase().collect::<Vec<char>>())
}

fn manipulate_text<F>(cx: &mut ViewContext<Workspace>, transform: F)
where
    F: Fn(char) -> Vec<char> + Copy,
{
    Vim::update(cx, |vim, cx| {
        vim.record_current_action(cx);
        let count = vim.take_count(cx).unwrap_or(1) as u32;
        vim.update_active_editor(cx, |vim, editor, cx| {
            let mut ranges = Vec::new();
            let mut cursor_positions = Vec::new();
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            for selection in editor.selections.all::<Point>(cx) {
                match vim.state().mode {
                    Mode::VisualLine => {
                        let start = Point::new(selection.start.row, 0);
                        let end =
                            Point::new(selection.end.row, snapshot.line_len(selection.end.row));
                        ranges.push(start..end);
                        cursor_positions.push(start..start);
                    }
                    Mode::Visual => {
                        ranges.push(selection.start..selection.end);
                        cursor_positions.push(selection.start..selection.start);
                    }
                    Mode::VisualBlock => {
                        ranges.push(selection.start..selection.end);
                        if cursor_positions.len() == 0 {
                            cursor_positions.push(selection.start..selection.start);
                        }
                    }
                    Mode::Insert | Mode::Normal => {
                        let start = selection.start;
                        let mut end = start;
                        for _ in 0..count {
                            end = snapshot.clip_point(end + Point::new(0, 1), Bias::Right);
                        }
                        ranges.push(start..end);

                        if end.column == snapshot.line_len(end.row) {
                            end = snapshot.clip_point(end - Point::new(0, 1), Bias::Left);
                        }
                        cursor_positions.push(end..end)
                    }
                }
            }
            editor.transact(cx, |editor, cx| {
                for range in ranges.into_iter().rev() {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    editor.buffer().update(cx, |buffer, cx| {
                        let text = snapshot
                            .text_for_range(range.start..range.end)
                            .flat_map(|s| s.chars())
                            .flat_map(|c| transform(c))
                            .collect::<String>();

                        buffer.edit([(range, text)], None, cx)
                    })
                }
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_ranges(cursor_positions)
                })
            });
        });
        vim.switch_mode(Mode::Normal, true, cx)
    })
}

#[cfg(test)]
mod test {
    use crate::{state::Mode, test::NeovimBackedTestContext};

    #[gpui::test]
    async fn test_change_case(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("ˇabC\n").await;
        cx.simulate_shared_keystrokes(["~"]).await;
        cx.assert_shared_state("AˇbC\n").await;
        cx.simulate_shared_keystrokes(["2", "~"]).await;
        cx.assert_shared_state("ABˇc\n").await;

        // works in visual mode
        cx.set_shared_state("a😀C«dÉ1*fˇ»\n").await;
        cx.simulate_shared_keystrokes(["~"]).await;
        cx.assert_shared_state("a😀CˇDé1*F\n").await;

        // works with multibyte characters
        cx.simulate_shared_keystrokes(["~"]).await;
        cx.set_shared_state("aˇC😀é1*F\n").await;
        cx.simulate_shared_keystrokes(["4", "~"]).await;
        cx.assert_shared_state("ac😀É1ˇ*F\n").await;

        // works with line selections
        cx.set_shared_state("abˇC\n").await;
        cx.simulate_shared_keystrokes(["shift-v", "~"]).await;
        cx.assert_shared_state("ˇABc\n").await;

        // works in visual block mode
        cx.set_shared_state("ˇaa\nbb\ncc").await;
        cx.simulate_shared_keystrokes(["ctrl-v", "j", "~"]).await;
        cx.assert_shared_state("ˇAa\nBb\ncc").await;

        // works with multiple cursors (zed only)
        cx.set_state("aˇßcdˇe\n", Mode::Normal);
        cx.simulate_keystroke("~");
        cx.assert_state("aSSˇcdˇE\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_convert_to_upper_case(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // works in visual mode
        cx.set_shared_state("a😀C«dÉ1*fˇ»\n").await;
        cx.simulate_shared_keystrokes(["U"]).await;
        cx.assert_shared_state("a😀CˇDÉ1*F\n").await;

        // works with line selections
        cx.set_shared_state("abˇC\n").await;
        cx.simulate_shared_keystrokes(["shift-v", "U"]).await;
        cx.assert_shared_state("ˇABC\n").await;

        // works in visual block mode
        cx.set_shared_state("ˇaa\nbb\ncc").await;
        cx.simulate_shared_keystrokes(["ctrl-v", "j", "U"]).await;
        cx.assert_shared_state("ˇAa\nBb\ncc").await;
    }

    #[gpui::test]
    async fn test_convert_to_lower_case(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // works in visual mode
        cx.set_shared_state("A😀c«DÉ1*fˇ»\n").await;
        cx.simulate_shared_keystrokes(["u"]).await;
        cx.assert_shared_state("A😀cˇdé1*f\n").await;

        // works with line selections
        cx.set_shared_state("ABˇc\n").await;
        cx.simulate_shared_keystrokes(["shift-v", "u"]).await;
        cx.assert_shared_state("ˇabc\n").await;

        // works in visual block mode
        cx.set_shared_state("ˇAa\nBb\nCc").await;
        cx.simulate_shared_keystrokes(["ctrl-v", "j", "u"]).await;
        cx.assert_shared_state("ˇaa\nbb\nCc").await;
    }
}
