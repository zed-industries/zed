use std::ops::Range;

use editor::{scroll::Autoscroll, MultiBufferSnapshot, ToOffset, ToPoint};
use gpui::{impl_actions, ViewContext, WindowContext};
use language::{Bias, Point};
use serde::Deserialize;
use workspace::Workspace;

use crate::{state::Mode, Vim};

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Increment {
    #[serde(default)]
    step: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Decrement {
    #[serde(default)]
    step: bool,
}

impl_actions!(vim, [Increment, Decrement]);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, action: &Increment, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let count = vim.take_count(cx).unwrap_or(1);
            let step = if action.step { 1 } else { 0 };
            increment(vim, count as i32, step, cx)
        })
    });
    workspace.register_action(|_: &mut Workspace, action: &Decrement, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let count = vim.take_count(cx).unwrap_or(1);
            let step = if action.step { -1 } else { 0 };
            increment(vim, count as i32 * -1, step, cx)
        })
    });
}

fn increment(vim: &mut Vim, mut delta: i32, step: i32, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |vim, editor, cx| {
        let mut edits = Vec::new();
        let mut new_anchors = Vec::new();

        let snapshot = editor.buffer().read(cx).snapshot(cx);
        for selection in editor.selections.all_adjusted(cx) {
            if !selection.is_empty() {
                if vim.state().mode != Mode::VisualBlock || new_anchors.is_empty() {
                    new_anchors.push((true, snapshot.anchor_before(selection.start)))
                }
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
                        delta += step;
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
                        edits.push((range.clone(), replace));
                    }
                    if selection.is_empty() {
                        new_anchors.push((false, snapshot.anchor_after(range.end)))
                    }
                } else {
                    if selection.is_empty() {
                        new_anchors.push((true, snapshot.anchor_after(start)))
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
                        point.column -= 1;
                        point = snapshot.clip_point(point, Bias::Left)
                    }
                    new_ranges.push(point..point);
                }
                s.select_ranges(new_ranges)
            })
        });
    });
    vim.switch_mode(Mode::Normal, true, cx)
}

fn find_number(
    snapshot: &MultiBufferSnapshot,
    start: Point,
) -> Option<(Range<Point>, String, u32)> {
    let mut offset = start.to_offset(snapshot);

    // go backwards to the start of any number the selection is within
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
    // find the next number on the line (may start after the original cursor position)
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

        if ch.is_digit(radix)
            || (begin.is_none()
                && ch == '-'
                && chars.peek().is_some()
                && chars.peek().unwrap().is_digit(radix))
        {
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
        cx.simulate_shared_keystrokes(["ctrl-x"]).await;
        cx.assert_shared_state(indoc! {"
            1ˇ2
            "})
            .await;

        cx.simulate_shared_keystrokes(["9", "9", "ctrl-a"]).await;
        cx.assert_shared_state(indoc! {"
            11ˇ1
            "})
            .await;
        cx.simulate_shared_keystrokes(["1", "1", "1", "ctrl-x"])
            .await;
        cx.assert_shared_state(indoc! {"
            ˇ0
            "})
            .await;
        cx.simulate_shared_keystrokes(["."]).await;
        cx.assert_shared_state(indoc! {"
            -11ˇ1
            "})
            .await;
    }

    #[gpui::test]
    async fn test_increment_radix(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_matches_neovim("ˇ total: 0xff", ["ctrl-a"], " total: 0x10ˇ0")
            .await;
        cx.assert_matches_neovim("ˇ total: 0xff", ["ctrl-x"], " total: 0xfˇe")
            .await;
        cx.assert_matches_neovim("ˇ total: 0xFF", ["ctrl-x"], " total: 0xFˇE")
            .await;
        cx.assert_matches_neovim("(ˇ0b10f)", ["ctrl-a"], "(0b1ˇ1f)")
            .await;
        cx.assert_matches_neovim("ˇ-1", ["ctrl-a"], "ˇ0").await;
        cx.assert_matches_neovim("banˇana", ["ctrl-a"], "banˇana")
            .await;
    }

    #[gpui::test]
    async fn test_increment_steps(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ˇ1
            1
            1  2
            1
            1"})
            .await;

        cx.simulate_shared_keystrokes(["j", "v", "shift-g", "g", "ctrl-a"])
            .await;
        cx.assert_shared_state(indoc! {"
            1
            ˇ2
            3  2
            4
            5"})
            .await;

        cx.simulate_shared_keystrokes(["shift-g", "ctrl-v", "g", "g"])
            .await;
        cx.assert_shared_state(indoc! {"
            «1ˇ»
            «2ˇ»
            «3ˇ»  2
            «4ˇ»
            «5ˇ»"})
            .await;

        cx.simulate_shared_keystrokes(["g", "ctrl-x"]).await;
        cx.assert_shared_state(indoc! {"
            ˇ0
            0
            0  2
            0
            0"})
            .await;
    }
}
