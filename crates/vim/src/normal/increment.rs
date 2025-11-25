use editor::{Editor, MultiBufferSnapshot, ToOffset, ToPoint};
use gpui::{Action, Context, Window};
use language::{Bias, Point};
use schemars::JsonSchema;
use serde::Deserialize;
use std::ops::Range;

use crate::{Vim, state::Mode};

const BOOLEAN_PAIRS: &[(&str, &str)] = &[("true", "false"), ("yes", "no"), ("on", "off")];

/// Increments the number under the cursor or toggles boolean values.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct Increment {
    #[serde(default)]
    step: bool,
}

/// Decrements the number under the cursor or toggles boolean values.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct Decrement {
    #[serde(default)]
    step: bool,
}

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, action: &Increment, window, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        let step = if action.step { count as i32 } else { 0 };
        vim.increment(count as i64, step, window, cx)
    });
    Vim::action(editor, cx, |vim, action: &Decrement, window, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        let step = if action.step { -1 * (count as i32) } else { 0 };
        vim.increment(-(count as i64), step, window, cx)
    });
}

impl Vim {
    fn increment(
        &mut self,
        mut delta: i64,
        step: i32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.store_visual_marks(window, cx);
        self.update_editor(cx, |vim, editor, cx| {
            let mut edits = Vec::new();
            let mut new_anchors = Vec::new();

            let snapshot = editor.buffer().read(cx).snapshot(cx);
            for selection in editor.selections.all_adjusted(&editor.display_snapshot(cx)) {
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
                    let end = if row == selection.end.row {
                        selection.end
                    } else {
                        Point::new(row, snapshot.line_len(multi_buffer::MultiBufferRow(row)))
                    };

                    let find_result = if !selection.is_empty() {
                        find_target(&snapshot, start, end, true)
                    } else {
                        find_target(&snapshot, start, end, false)
                    };

                    if let Some((range, target, radix)) = find_result {
                        let replace = match radix {
                            10 => increment_decimal_string(&target, delta),
                            16 => increment_hex_string(&target, delta),
                            2 => increment_binary_string(&target, delta),
                            0 => increment_toggle_string(&target),
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
            editor.transact(window, cx, |editor, window, cx| {
                editor.edit(edits, cx);

                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor.change_selections(Default::default(), window, cx, |s| {
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
        self.switch_mode(Mode::Normal, true, window, cx)
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
    let result = if let Ok(val) = u64::from_str_radix(num, 16) {
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
    let result = if let Ok(val) = u64::from_str_radix(num, 2) {
        val.wrapping_add_signed(delta)
    } else {
        u64::MAX
    };
    format!("{:0width$b}", result, width = num.len())
}

fn find_target(
    snapshot: &MultiBufferSnapshot,
    start: Point,
    end: Point,
    need_range: bool,
) -> Option<(Range<Point>, String, u32)> {
    let start_offset = start.to_offset(snapshot);
    let end_offset = end.to_offset(snapshot);

    let mut offset = start_offset;
    let mut first_char_is_num = snapshot
        .chars_at(offset)
        .next()
        .map_or(false, |ch| ch.is_ascii_hexdigit());
    let mut pre_char = String::new();

    let next_offset = offset
        + snapshot
            .chars_at(start_offset)
            .next()
            .map_or(0, |ch| ch.len_utf8());
    // Backward scan to find the start of the number, but stop at start_offset
    for ch in snapshot.reversed_chars_at(next_offset) {
        // Search boundaries
        if offset.0 == 0 || ch.is_whitespace() || (need_range && offset <= start_offset) {
            break;
        }

        // Avoid the influence of hexadecimal letters
        if first_char_is_num
            && !ch.is_ascii_hexdigit()
            && (ch != 'b' && ch != 'B')
            && (ch != 'x' && ch != 'X')
            && ch != '-'
        {
            // Used to determine if the initial character is a number.
            if is_numeric_string(&pre_char) {
                break;
            } else {
                first_char_is_num = false;
            }
        }

        pre_char.insert(0, ch);
        offset -= ch.len_utf8();
    }

    let mut begin = None;
    let mut end = None;
    let mut target = String::new();
    let mut radix = 10;
    let mut is_num = false;

    let mut chars = snapshot.chars_at(offset).peekable();

    while let Some(ch) = chars.next() {
        if need_range && offset >= end_offset {
            break; // stop at end of selection
        }

        if target == "0"
            && (ch == 'b' || ch == 'B')
            && chars.peek().is_some()
            && chars.peek().unwrap().is_digit(2)
        {
            radix = 2;
            begin = None;
            target = String::new();
        } else if target == "0"
            && (ch == 'x' || ch == 'X')
            && chars.peek().is_some()
            && chars.peek().unwrap().is_ascii_hexdigit()
        {
            radix = 16;
            begin = None;
            target = String::new();
        } else if ch == '.' {
            is_num = false;
            begin = None;
            target = String::new();
        } else if ch.is_digit(radix)
            || ((begin.is_none() || !is_num)
                && ch == '-'
                && chars.peek().is_some()
                && chars.peek().unwrap().is_digit(radix))
        {
            if !is_num {
                is_num = true;
                begin = Some(offset);
                target = String::new();
            } else if begin.is_none() {
                begin = Some(offset);
            }
            target.push(ch);
        } else if ch.is_ascii_alphabetic() && !is_num {
            if begin.is_none() {
                begin = Some(offset);
            }
            target.push(ch);
        } else if begin.is_some() && (is_num || !is_num && is_toggle_word(&target)) {
            // End of matching
            end = Some(offset);
            break;
        } else if ch == '\n' {
            break;
        } else {
            // To match the next word
            is_num = false;
            begin = None;
            target = String::new();
        }

        offset += ch.len_utf8();
    }

    if let Some(begin) = begin
        && (is_num || !is_num && is_toggle_word(&target))
    {
        if !is_num {
            radix = 0;
        }

        let end = end.unwrap_or(offset);
        Some((
            begin.to_point(snapshot)..end.to_point(snapshot),
            target,
            radix,
        ))
    } else {
        None
    }
}

fn is_numeric_string(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let (_, rest) = if let Some(r) = s.strip_prefix('-') {
        (true, r)
    } else {
        (false, s)
    };

    if rest.is_empty() {
        return false;
    }

    if let Some(digits) = rest.strip_prefix("0b").or_else(|| rest.strip_prefix("0B")) {
        digits.is_empty() || digits.chars().all(|c| c == '0' || c == '1')
    } else if let Some(digits) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
        digits.is_empty() || digits.chars().all(|c| c.is_ascii_hexdigit())
    } else {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
    }
}

fn is_toggle_word(word: &str) -> bool {
    let lower = word.to_lowercase();
    BOOLEAN_PAIRS
        .iter()
        .any(|(a, b)| lower == *a || lower == *b)
}

fn increment_toggle_string(boolean: &str) -> String {
    let lower = boolean.to_lowercase();

    let target = BOOLEAN_PAIRS
        .iter()
        .find_map(|(a, b)| {
            if lower == *a {
                Some(b)
            } else if lower == *b {
                Some(a)
            } else {
                None
            }
        })
        .unwrap_or(&boolean);

    if boolean.chars().all(|c| c.is_uppercase()) {
        // Upper case
        target.to_uppercase()
    } else if boolean.chars().next().unwrap_or(' ').is_uppercase() {
        // Title case
        let mut chars = target.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        }
    } else {
        target.to_string()
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
        cx.simulate_shared_keystrokes("v shift-g g ctrl-a").await;
        cx.simulate_shared_keystrokes("v shift-g 5 g ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            ˇ6
            12
            18  2
            24
            30"});
    }

    #[gpui::test]
    async fn test_increment_toggle(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("let enabled = trˇue;", Mode::Normal);
        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("let enabled = falsˇe;", Mode::Normal);

        cx.simulate_keystrokes("0 ctrl-a");
        cx.assert_state("let enabled = truˇe;", Mode::Normal);

        cx.set_state(
            indoc! {"
                ˇlet enabled = TRUE;
                let enabled = TRUE;
                let enabled = TRUE;
            "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("shift-v j j ctrl-x");
        cx.assert_state(
            indoc! {"
                ˇlet enabled = FALSE;
                let enabled = FALSE;
                let enabled = FALSE;
            "},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
                let enabled = ˇYes;
                let enabled = Yes;
                let enabled = Yes;
            "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("ctrl-v j j e ctrl-x");
        cx.assert_state(
            indoc! {"
                let enabled = ˇNo;
                let enabled = No;
                let enabled = No;
            "},
            Mode::Normal,
        );

        cx.set_state("ˇlet enabled = True;", Mode::Normal);
        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("let enabled = Falsˇe;", Mode::Normal);

        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("let enabled = Truˇe;", Mode::Normal);

        cx.set_state("let enabled = Onˇ;", Mode::Normal);
        cx.simulate_keystrokes("v b ctrl-a");
        cx.assert_state("let enabled = ˇOff;", Mode::Normal);
    }

    #[gpui::test]
    async fn test_increment_order(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("aaˇa false 1 2 3", Mode::Normal);
        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("aaa truˇe 1 2 3", Mode::Normal);

        cx.set_state("aaˇa 1 false 2 3", Mode::Normal);
        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("aaa ˇ2 false 2 3", Mode::Normal);

        cx.set_state("trueˇ 1 2 3", Mode::Normal);
        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("true ˇ2 2 3", Mode::Normal);

        cx.set_state("falseˇ", Mode::Normal);
        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("truˇe", Mode::Normal);

        cx.set_state("⚡️ˇ⚡️", Mode::Normal);
        cx.simulate_keystrokes("ctrl-a");
        cx.assert_state("⚡️ˇ⚡️", Mode::Normal);
    }

    #[gpui::test]
    async fn test_increment_visual_partial_number(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇ123").await;
        cx.simulate_shared_keystrokes("v l ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"ˇ133"});
        cx.simulate_shared_keystrokes("l v l ctrl-a").await;
        cx.shared_state().await.assert_eq(indoc! {"1ˇ34"});
        cx.simulate_shared_keystrokes("shift-v y p p ctrl-v k k l ctrl-a")
            .await;
        cx.shared_state().await.assert_eq(indoc! {"ˇ144\n144\n144"});
    }
}
