use editor::{scroll::Autoscroll, Editor, MultiBufferSnapshot, ToOffset, ToPoint};
use gpui::{impl_actions, ViewContext};
use language::{Bias, Point};
use schemars::JsonSchema;
use serde::Deserialize;
use std::ops::Range;

use crate::{state::Mode, Vim};

#[derive(Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Increment {
    #[serde(default)]
    step: bool,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Decrement {
    #[serde(default)]
    step: bool,
}

impl_actions!(vim, [Increment, Decrement]);

pub fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, |vim, action: &Increment, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        let step = if action.step { 1 } else { 0 };
        vim.increment(count as i64, step, cx)
    });
    Vim::action(editor, cx, |vim, action: &Decrement, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        let step = if action.step { -1 } else { 0 };
        vim.increment(-(count as i64), step, cx)
    });
}

impl Vim {
    fn increment(&mut self, mut delta: i64, step: i32, cx: &mut ViewContext<Self>) {
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
                        let replace = match radix {
                            10 => increment_decimal_string(&num, delta),
                            16 => increment_hex_string(&num, delta),
                            2 => increment_binary_string(&num, delta),
                            _ => unreachable!(),
                        };
                        delta += step as i64;
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

fn increment_decimal_string(num: &str, delta: i64) -> String {
    let (negative, delta, num_str) = match num.strip_prefix('-') {
        Some(n) => (true, -delta, n),
        None => (false, delta, num),
    };
    let num_length = num_str.len();
    let leading_zero = num_str.starts_with('0');

    let (result, new_negative) = match u64::from_str_radix(num_str, 10) {
        Ok(value) => {
            let wrapped = value.wrapping_add_signed(delta);
            if delta < 0 && wrapped > value {
                ((u64::MAX - wrapped).wrapping_add(1), !negative)
            } else if delta > 0 && wrapped < value {
                (u64::MAX - wrapped, !negative)
            } else {
                (wrapped, negative)
            }
        }
        Err(_) => (u64::MAX, negative),
    };

    let formatted = format!("{}", result);
    let new_significant_digits = formatted.len();
    let padding = if leading_zero {
        num_length.saturating_sub(new_significant_digits)
    } else {
        0
    };

    if new_negative && result != 0 {
        format!("-{}{}", "0".repeat(padding), formatted)
    } else {
        format!("{}{}", "0".repeat(padding), formatted)
    }
}

fn increment_hex_string(num: &str, delta: i64) -> String {
    let result = if let Ok(val) = u64::from_str_radix(&num, 16) {
        val.wrapping_add_signed(delta)
    } else {
        u64::MAX
    };
    if should_use_lowercase(num) {
        format!("{:0width$x}", result, width = num.len())
    } else {
        format!("{:0width$X}", result, width = num.len())
    }
}

fn should_use_lowercase(num: &str) -> bool {
    let mut use_uppercase = false;
    for ch in num.chars() {
        if ch.is_ascii_lowercase() {
            return true;
        }
        if ch.is_ascii_uppercase() {
            use_uppercase = true;
        }
    }
    !use_uppercase
}

fn increment_binary_string(num: &str, delta: i64) -> String {
    let result = if let Ok(val) = u64::from_str_radix(&num, 2) {
        val.wrapping_add_signed(delta)
    } else {
        u64::MAX
    };
    format!("{:0width$b}", result, width = num.len())
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
    async fn test_increment_with_leading_zeros(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            000ˇ9
            "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            001ˇ0
            "});
        cx.simulate_shared_keystrokes("2 ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            000ˇ8
            "});
    }

    #[gpui::test]
    async fn test_increment_with_leading_zeros_and_zero(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            01ˇ1
            "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            01ˇ2
            "});
        cx.simulate_shared_keystrokes("1 2 ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            00ˇ0
            "});
    }

    #[gpui::test]
    async fn test_increment_with_changing_leading_zeros(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            099ˇ9
            "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            100ˇ0
            "});
        cx.simulate_shared_keystrokes("2 ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            99ˇ8
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
    async fn test_increment_sign_change(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                ˇ0
                "})
            .await;
        cx.simulate_shared_keystrokes("ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
                -ˇ1
                "});
        cx.simulate_shared_keystrokes("2 ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                ˇ1
                "});
    }

    #[gpui::test]
    async fn test_increment_sign_change_with_leading_zeros(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                00ˇ1
                "})
            .await;
        cx.simulate_shared_keystrokes("ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
                00ˇ0
                "});
        cx.simulate_shared_keystrokes("ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
                -00ˇ1
                "});
        cx.simulate_shared_keystrokes("2 ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                00ˇ1
                "});
    }

    #[gpui::test]
    async fn test_increment_bin_wrapping_and_padding(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                    0b111111111111111111111111111111111111111111111111111111111111111111111ˇ1
                    "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0b000000111111111111111111111111111111111111111111111111111111111111111ˇ1
                    "});
        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0b000000000000000000000000000000000000000000000000000000000000000000000ˇ0
                    "});

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0b000000000000000000000000000000000000000000000000000000000000000000000ˇ1
                    "});
        cx.simulate_shared_keystrokes("2 ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0b000000111111111111111111111111111111111111111111111111111111111111111ˇ1
                    "});
    }

    #[gpui::test]
    async fn test_increment_hex_wrapping_and_padding(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                    0xfffffffffffffffffffˇf
                    "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0x0000fffffffffffffffˇf
                    "});
        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0x0000000000000000000ˇ0
                    "});
        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0x0000000000000000000ˇ1
                    "});
        cx.simulate_shared_keystrokes("2 ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0x0000fffffffffffffffˇf
                    "});
    }

    #[gpui::test]
    async fn test_increment_wrapping(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                    1844674407370955161ˇ9
                    "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    1844674407370955161ˇ5
                    "});
        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    -1844674407370955161ˇ5
                    "});
        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    -1844674407370955161ˇ4
                    "});
        cx.simulate_shared_keystrokes("3 ctrl-x").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    1844674407370955161ˇ4
                    "});
        cx.simulate_shared_keystrokes("2 ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    -1844674407370955161ˇ5
                    "});
    }

    #[gpui::test]
    async fn test_increment_inline(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                    inline0x3ˇ9u32
                    "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    inline0x3ˇau32
                    "});
        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    inline0x3ˇbu32
                    "});
        cx.simulate_shared_keystrokes("l l l ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    inline0x3bu3ˇ3
                    "});
    }

    #[gpui::test]
    async fn test_increment_hex_casing(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                        0xFˇa
                    "})
            .await;

        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0xfˇb
                    "});
        cx.simulate_shared_keystrokes("ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    0xfˇc
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
