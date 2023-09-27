use std::{ascii::AsciiExt, fmt::Binary, ops::Range};

use editor::{
    movement, scroll::autoscroll::Autoscroll, Editor, MultiBufferSnapshot, ToOffset, ToPoint,
};
use gpui::{actions, AppContext, WindowContext};
use language::Point;
use workspace::Workspace;

use crate::{state::Mode, Vim};

actions!(vim, [Increment, Decrement]);

pub fn init(cx: &mut AppContext) {
    dbg!("hi");

    cx.add_action(|_: &mut Workspace, _: &Increment, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let count = vim.take_count(cx).unwrap_or(1);
            increment(vim, count as i32, cx)
        })
    });
    cx.add_action(|_: &mut Workspace, _: &Decrement, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let count = vim.take_count(cx).unwrap_or(1);
            increment(vim, count as i32 * -1, cx)
        })
    });
}

fn increment(vim: &mut Vim, delta: i32, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |editor, cx| {
        let mut edits = Vec::new();
        let mut new_anchors = Vec::new();

        let snapshot = editor.buffer().read(cx).snapshot(cx);
        for selection in editor.selections.all_adjusted(cx) {
            if !selection.is_empty() {
                new_anchors.push((true, snapshot.anchor_before(selection.start)))
            }
            for row in selection.start.row..=selection.end.row {
                let start = if row == selection.start.row {
                    selection.start
                } else {
                    Point::new(row, 0)
                };

                if let Some((range, num, radix)) = find_number(&snapshot, start) {
                    if let Ok(val) = i32::from_str_radix(&num, radix) {
                        let result = val + delta;
                        let replace = match radix {
                            10 => format!("{}", result),
                            16 => {
                                if num.to_ascii_lowercase() == num {
                                    format!("{:x}", result)
                                } else {
                                    format!("{:X}", result)
                                }
                            }
                            2 => format!("{:b}", result),
                            _ => unreachable!(),
                        };
                        if selection.is_empty() {
                            new_anchors.push((false, snapshot.anchor_after(range.end)))
                        }
                        edits.push((range, replace));
                    }
                }
            }
        }
        editor.transact(cx, |editor, cx| {
            editor.edit(edits, cx);

            let snapshot = editor.buffer().read(cx).snapshot(cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let mut new_ranges = Vec::new();
                for (visual, anchor) in new_anchors.iter() {
                    let mut point = anchor.to_point(&snapshot);
                    if !*visual && point.column > 0 {
                        point.column -= 1
                    }
                    new_ranges.push(point..point);
                }
                s.select_ranges(new_ranges)
            })
        });
    });
    vim.switch_mode(Mode::Normal, false, cx)
}

fn find_number(
    snapshot: &MultiBufferSnapshot,
    start: Point,
) -> Option<(Range<Point>, String, u32)> {
    let mut offset = start.to_offset(snapshot);

    for ch in snapshot.reversed_chars_at(offset) {
        if ch.is_ascii_digit() || ch == '-' || ch == 'b' || ch == 'x' {
            offset -= ch.len_utf8();
            continue;
        }
        break;
    }

    let mut begin = None;
    let mut end = None;
    let mut num = String::new();
    let mut radix = 10;

    let mut chars = snapshot.chars_at(offset).peekable();
    while let Some(ch) = chars.next() {
        if num == "0" && ch == 'b' && chars.peek().is_some() && chars.peek().unwrap().is_digit(2) {
            radix = 2;
            begin = None;
            num = String::new();
        }
        if num == "0" && ch == 'x' && chars.peek().is_some() && chars.peek().unwrap().is_digit(16) {
            radix = 16;
            begin = None;
            num = String::new();
        }

        if ch.is_digit(radix) || ch == '-' {
            if begin.is_none() {
                begin = Some(offset);
            }
            num.push(ch);
        } else {
            if begin.is_some() {
                end = Some(offset);
                break;
            } else if ch == '\n' {
                break;
            }
        }
        offset += ch.len_utf8();
    }
    if let Some(begin) = begin {
        let end = end.unwrap_or(offset);
        Some((begin.to_point(snapshot)..end.to_point(snapshot), num, radix))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::test::NeovimBackedTestContext;

    #[gpui::test]
    async fn test_increment(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            1ˇ2
            "})
            .await;

        cx.simulate_shared_keystrokes(["ctrl-a"]).await;
        cx.assert_shared_state(indoc! {"
            1ˇ3
            "})
            .await;
    }
}
