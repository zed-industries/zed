use std::ops::Range;

use editor::{scroll::Autoscroll, Editor, MultiBufferSnapshot, ToOffset, ToPoint};
use gpui::{impl_actions, ViewContext};
use language::{Bias, Point};
use serde::Deserialize;

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

pub fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, |vim, action: &Increment, cx| {
        vim.record_current_action(cx);
        let count = vim.take_count(cx).unwrap_or(1);
        let step = if action.step { 1 } else { 0 };
        vim.increment(count as i32, step, cx)
    });
    Vim::action(editor, cx, |vim, action: &Decrement, cx| {
        vim.record_current_action(cx);
        let count = vim.take_count(cx).unwrap_or(1);
        let step = if action.step { -1 } else { 0 };
        vim.increment(-(count as i32), step, cx)
    });
}

impl Vim {
    fn increment(&mut self, mut delta: i32, step: i32, cx: &mut ViewContext<Self>) {
        self.store_visual_marks(cx);
        self.update_editor(cx, |vim, editor, cx| {
            let mut edits = Vec::new();
            let mut new_anchors = Vec::new();

            let snapshot = editor.buffer().read(cx).snapshot(cx);
            for selection in editor.selections.all_adjusted(cx) {
                if !selection.is_empty()
                    && (vim.mode != Mode::VisualBlock || new_anchors.is_empty())
                {
                    new_anchors.push((true, snapshot.anchor_before(selection.start)))
                }
                for row in selection.start.row..=selection.end.row {
                    let start = if row == selection.start.row {
                        selection.start
                    } else {
                        Point::new(row, 0)
                    };

                    if let Some((range, num, radix)) = find_number(&snapshot, start) {
                        delta += step;
                        let replace = match radix {
                            10 => increment_decimal_string(&num, delta),
                            16 => increment_hex_string(&num, delta),
                            2 => increment_binary_string(&num, delta),
                            _ => unreachable!(),
                        };
                        edits.push((range.clone(), replace));
                        if selection.is_empty() {
                            new_anchors.push((false, snapshot.anchor_after(range.end)))
                        }
                    } else if selection.is_empty() {
                        new_anchors.push((true, snapshot.anchor_after(start)))
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
        self.switch_mode(Mode::Normal, true, cx)
    }
}

fn increment_decimal_string(mut num: &str, delta: i32) -> String {
    let mut negative = false;
    let mut n: u64;
    let mut decrement = false;
    if delta < 0 {
        decrement = true;
    }
    if num.chars().next() == Some('-') {
        negative = true;
        num = &num[1..];
    }
    if let Ok(result) = u64::from_str_radix(num, 10) {
        n = result;
    } else {
        if !negative {
            return u64::MAX.to_string()
        } else {
            return format!("-{}", u64::MAX)
        }
    }

    let (result, overflow) = if !negative && !decrement {
        adaptive_increment_u64(n, delta)
    } else if negative && !decrement {
        let (result, overflow) = adaptive_decrement_u64(n, delta);
        if result == 0 {
            negative = false;
        }
        (result, overflow)
    } else if negative && decrement {
        adaptive_increment_u64(n, -delta)
    } else {
        adaptive_decrement_u64(n, -delta)
    };

    n = result;
    if overflow {
        negative = !negative;
    }

    if !negative {
        format!("{}", n)
    } else {
        format!("-{}", n)
    }
}

fn adaptive_increment_u64(mut n: u64, delta: i32) -> (u64, bool) {
    if let Some(n) = n.checked_add(delta as u64) {
        return (n, false);
    } else {
        let diff = u64::MAX - n;
        n = u64::MAX - delta as u64 + diff + 1;
        return (n, true);
    }
}

fn adaptive_decrement_u64(mut n: u64, delta: i32) -> (u64, bool) {
    if let Some(n) = n.checked_sub(delta as u64) {
        return (n, false);
    } else {
        n = u64::MIN + delta as u64 - n;
        return (n, true);
    }
}

fn increment_hex_string(num: &str, delta: i32) -> String {
    if let Ok(val) = i32::from_str_radix(&num, 16) {
        let result = val + delta;
        if num.to_ascii_lowercase() == num {
            format!("{:0width$x}", result, width = num.len())
        } else {
            format!("{:0width$X}", result, width = num.len())
        }
    } else {
        unreachable!()
    }
}

fn increment_binary_string(num: &str, delta: i32) -> String {
    if let Ok(val) = i32::from_str_radix(&num, 2) {
        let result = val + delta;
        format!("{:b}", result)
    } else {
        unreachable!()
    }
}

fn find_number(
    snapshot: &MultiBufferSnapshot,
    start: Point,
) -> Option<(Range<Point>, String, u32)> {
    let mut offset = start.to_offset(snapshot);

    let ch0 = snapshot.chars_at(offset).next();
    if ch0.as_ref().is_some_and(char::is_ascii_hexdigit) || matches!(ch0, Some('-' | 'b' | 'x')) {
        // go backwards to the start of any number the selection is within
        for ch in snapshot.reversed_chars_at(offset) {
            if ch.is_ascii_hexdigit() || ch == '-' || ch == 'b' || ch == 'x' {
                offset -= ch.len_utf8();
                continue;
            }
            break;
        }
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
        if num == "0"
            && ch == 'x'
            && chars.peek().is_some()
            && chars.peek().unwrap().is_ascii_hexdigit()
        {
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
        } else if begin.is_some() {
            end = Some(offset);
            break;
        } else if ch == '\n' {
            break;
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

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            1ˇ3
            "});
        cx.simulate_shared_keystrokes("ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            1ˇ2
            "});

        cx.simulate_shared_keystrokes("9 9 ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            11ˇ1
            "});
        cx.simulate_shared_keystrokes("1 1 1 ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            ˇ0
            "});
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq(indoc! {"
            -11ˇ1
            "});
    }

    #[gpui::test]
    async fn test_increment_with_dot(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            1ˇ.2
            "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            1.ˇ3
            "});
        cx.simulate_shared_keystrokes("ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            1.ˇ2
            "});
    }

    #[gpui::test]
    async fn test_increment_with_two_dots(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            111.ˇ.2
            "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            111..ˇ3
            "});
        cx.simulate_shared_keystrokes("ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            111..ˇ2
            "});
    }

    #[gpui::test]
    async fn test_increment_radix(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.simulate("ctrl-a", "ˇ total: 0xff")
            .await
            .assert_matches();
        cx.simulate("ctrl-x", "ˇ total: 0xff")
            .await
            .assert_matches();
        cx.simulate("ctrl-x", "ˇ total: 0xFF")
            .await
            .assert_matches();
        cx.simulate("ctrl-a", "(ˇ0b10f)").await.assert_matches();
        cx.simulate("ctrl-a", "ˇ-1").await.assert_matches();
        cx.simulate("ctrl-a", "banˇana").await.assert_matches();
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

        cx.simulate_shared_keystrokes("j v shift-g g ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            1
            ˇ2
            3  2
            4
            5"});

        cx.simulate_shared_keystrokes("shift-g ctrl-v g g").await;
        cx.shared_state().await.assert_eq(indoc! {"
            «1ˇ»
            «2ˇ»
            «3ˇ»  2
            «4ˇ»
            «5ˇ»"});

        cx.simulate_shared_keystrokes("g ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            ˇ0
            0
            0  2
            0
            0"});
    }
}
