use crate::{
    motion::{self, Motion},
    object::Object,
    state::Mode,
    Vim,
};
use editor::{movement, scroll::Autoscroll, Bias};
use gpui::WindowContext;
use language::BracketPair;
use serde::Deserialize;
use std::sync::Arc;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurroundsType {
    Motion(Motion),
    Object(Object),
}

// This exists so that we can have Deserialize on Operators, but not on Motions.
impl<'de> Deserialize<'de> for SurroundsType {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom("Cannot deserialize SurroundsType"))
    }
}

pub fn add_surrounds(text: Arc<str>, target: SurroundsType, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();
        let count = vim.take_count(cx);
        vim.update_active_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let pair = match find_surround_pair(&all_support_surround_pair(), &text) {
                    Some(pair) => pair.clone(),
                    None => BracketPair {
                        start: text.to_string(),
                        end: text.to_string(),
                        close: true,
                        newline: false,
                    },
                };
                let surround = pair.end != *text;
                let (display_map, display_selections) = editor.selections.all_adjusted_display(cx);
                let mut edits = Vec::new();
                let mut anchors = Vec::new();

                for selection in &display_selections {
                    let range = match &target {
                        SurroundsType::Object(object) => {
                            object.range(&display_map, selection.clone(), false)
                        }
                        SurroundsType::Motion(motion) => {
                            let range = motion
                                .range(
                                    &display_map,
                                    selection.clone(),
                                    count,
                                    true,
                                    &text_layout_details,
                                )
                                .map(|mut range| {
                                    // The Motion::CurrentLine operation will contain the newline of the current line and leading/trailing whitespace
                                    if let Motion::CurrentLine = motion {
                                        range.start = motion::first_non_whitespace(
                                            &display_map,
                                            false,
                                            range.start,
                                        );
                                        range.end = movement::saturating_right(
                                            &display_map,
                                            motion::last_non_whitespace(
                                                &display_map,
                                                movement::left(&display_map, range.end),
                                                1,
                                            ),
                                        );
                                    }
                                    range
                                });
                            range
                        }
                    };

                    if let Some(range) = range {
                        let start = range.start.to_offset(&display_map, Bias::Right);
                        let end = range.end.to_offset(&display_map, Bias::Left);
                        let start_cursor_str =
                            format!("{}{}", pair.start, if surround { " " } else { "" });
                        let close_cursor_str =
                            format!("{}{}", if surround { " " } else { "" }, pair.end);
                        let start_anchor = display_map.buffer_snapshot.anchor_before(start);

                        edits.push((start..start, start_cursor_str));
                        edits.push((end..end, close_cursor_str));
                        anchors.push(start_anchor..start_anchor);
                    } else {
                        let start_anchor = display_map
                            .buffer_snapshot
                            .anchor_before(selection.head().to_offset(&display_map, Bias::Left));
                        anchors.push(start_anchor..start_anchor);
                    }
                }

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_anchor_ranges(anchors)
                });
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
    });
}

pub fn delete_surrounds(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();

        // only legitimate surrounds can be removed
        let pair = match find_surround_pair(&all_support_surround_pair(), &text) {
            Some(pair) => pair.clone(),
            None => return,
        };
        let pair_object = match pair_to_object(&pair) {
            Some(pair_object) => pair_object,
            None => return,
        };
        let surround = pair.end != *text;

        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let (display_map, display_selections) = editor.selections.all_display(cx);
                let mut edits = Vec::new();
                let mut anchors = Vec::new();

                for selection in &display_selections {
                    let start = selection.start.to_offset(&display_map, Bias::Left);
                    if let Some(range) = pair_object.range(&display_map, selection.clone(), true) {
                        // If the current parenthesis object is single-line,
                        // then we need to filter whether it is the current line or not
                        if !pair_object.is_multiline() {
                            let is_same_row = selection.start.row() == range.start.row()
                                && selection.end.row() == range.end.row();
                            if !is_same_row {
                                anchors.push(start..start);
                                continue;
                            }
                        }
                        // This is a bit cumbersome, and it is written to deal with some special cases, as shown below
                        // hello«ˇ  "hello in a word"  »again.
                        // Sometimes the expand_selection will not be matched at both ends, and there will be extra spaces
                        // In order to be able to accurately match and replace in this case, some cumbersome methods are used
                        let mut chars_and_offset = display_map
                            .buffer_chars_at(range.start.to_offset(&display_map, Bias::Left))
                            .peekable();
                        while let Some((ch, offset)) = chars_and_offset.next() {
                            if ch.to_string() == pair.start {
                                let start = offset;
                                let mut end = start + 1;
                                if surround {
                                    if let Some((next_ch, _)) = chars_and_offset.peek() {
                                        if next_ch.eq(&' ') {
                                            end += 1;
                                        }
                                    }
                                }
                                edits.push((start..end, ""));
                                anchors.push(start..start);
                                break;
                            }
                        }
                        let mut reverse_chars_and_offsets = display_map
                            .reverse_buffer_chars_at(range.end.to_offset(&display_map, Bias::Left))
                            .peekable();
                        while let Some((ch, offset)) = reverse_chars_and_offsets.next() {
                            if ch.to_string() == pair.end {
                                let mut start = offset;
                                let end = start + 1;
                                if surround {
                                    if let Some((next_ch, _)) = reverse_chars_and_offsets.peek() {
                                        if next_ch.eq(&' ') {
                                            start -= 1;
                                        }
                                    }
                                }
                                edits.push((start..end, ""));
                                break;
                            }
                        }
                    } else {
                        anchors.push(start..start);
                    }
                }

                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_ranges(anchors);
                });
                edits.sort_by_key(|(range, _)| range.start);
                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    });
}

pub fn change_surrounds(text: Arc<str>, target: Object, cx: &mut WindowContext) {
    if let Some(will_replace_pair) = object_to_bracket_pair(target) {
        Vim::update(cx, |vim, cx| {
            vim.stop_recording();
            vim.update_active_editor(cx, |_, editor, cx| {
                editor.transact(cx, |editor, cx| {
                    editor.set_clip_at_line_ends(false, cx);

                    let pair = match find_surround_pair(&all_support_surround_pair(), &text) {
                        Some(pair) => pair.clone(),
                        None => BracketPair {
                            start: text.to_string(),
                            end: text.to_string(),
                            close: true,
                            newline: false,
                        },
                    };
                    let surround = pair.end != *text;
                    let (display_map, selections) = editor.selections.all_adjusted_display(cx);
                    let mut edits = Vec::new();
                    let mut anchors = Vec::new();

                    for selection in &selections {
                        let start = selection.start.to_offset(&display_map, Bias::Left);
                        if let Some(range) = target.range(&display_map, selection.clone(), true) {
                            if !target.is_multiline() {
                                let is_same_row = selection.start.row() == range.start.row()
                                    && selection.end.row() == range.end.row();
                                if !is_same_row {
                                    anchors.push(start..start);
                                    continue;
                                }
                            }
                            let mut chars_and_offset = display_map
                                .buffer_chars_at(range.start.to_offset(&display_map, Bias::Left))
                                .peekable();
                            while let Some((ch, offset)) = chars_and_offset.next() {
                                if ch.to_string() == will_replace_pair.start {
                                    let mut open_str = pair.start.clone();
                                    let start = offset;
                                    let mut end = start + 1;
                                    match chars_and_offset.peek() {
                                        Some((next_ch, _)) => {
                                            // If the next position is already a space or line break,
                                            // we don't need to splice another space even under arround
                                            if surround && !next_ch.is_whitespace() {
                                                open_str.push_str(" ");
                                            } else if !surround && next_ch.to_string() == " " {
                                                end += 1;
                                            }
                                        }
                                        None => {}
                                    }
                                    edits.push((start..end, open_str));
                                    anchors.push(start..start);
                                    break;
                                }
                            }

                            let mut reverse_chars_and_offsets = display_map
                                .reverse_buffer_chars_at(
                                    range.end.to_offset(&display_map, Bias::Left),
                                )
                                .peekable();
                            while let Some((ch, offset)) = reverse_chars_and_offsets.next() {
                                if ch.to_string() == will_replace_pair.end {
                                    let mut close_str = pair.end.clone();
                                    let mut start = offset;
                                    let end = start + 1;
                                    if let Some((next_ch, _)) = reverse_chars_and_offsets.peek() {
                                        if surround && !next_ch.is_whitespace() {
                                            close_str.insert_str(0, " ")
                                        } else if !surround && next_ch.to_string() == " " {
                                            start -= 1;
                                        }
                                    }
                                    edits.push((start..end, close_str));
                                    break;
                                }
                            }
                        } else {
                            anchors.push(start..start);
                        }
                    }

                    let stable_anchors = editor
                        .selections
                        .disjoint_anchors()
                        .into_iter()
                        .map(|selection| {
                            let start = selection.start.bias_left(&display_map.buffer_snapshot);
                            start..start
                        })
                        .collect::<Vec<_>>();
                    edits.sort_by_key(|(range, _)| range.start);
                    editor.buffer().update(cx, |buffer, cx| {
                        buffer.edit(edits, None, cx);
                    });
                    editor.set_clip_at_line_ends(true, cx);
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select_anchor_ranges(stable_anchors);
                    });
                });
            });
        });
    }
}

/// Checks if any of the current cursors are surrounded by a valid pair of brackets.
///
/// This method supports multiple cursors and checks each cursor for a valid pair of brackets.
/// A pair of brackets is considered valid if it is well-formed and properly closed.
///
/// If a valid pair of brackets is found, the method returns `true` and the cursor is automatically moved to the start of the bracket pair.
/// If no valid pair of brackets is found for any cursor, the method returns `false`.
pub fn check_and_move_to_valid_bracket_pair(
    vim: &mut Vim,
    object: Object,
    cx: &mut WindowContext,
) -> bool {
    let mut valid = false;
    if let Some(pair) = object_to_bracket_pair(object) {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);
                let mut anchors = Vec::new();

                for selection in &selections {
                    let start = selection.start.to_offset(&display_map, Bias::Left);
                    if let Some(range) = object.range(&display_map, selection.clone(), true) {
                        // If the current parenthesis object is single-line,
                        // then we need to filter whether it is the current line or not
                        if object.is_multiline()
                            || (!object.is_multiline()
                                && selection.start.row() == range.start.row()
                                && selection.end.row() == range.end.row())
                        {
                            valid = true;
                            let mut chars_and_offset = display_map
                                .buffer_chars_at(range.start.to_offset(&display_map, Bias::Left))
                                .peekable();
                            while let Some((ch, offset)) = chars_and_offset.next() {
                                if ch.to_string() == pair.start {
                                    anchors.push(offset..offset);
                                    break;
                                }
                            }
                        } else {
                            anchors.push(start..start)
                        }
                    } else {
                        anchors.push(start..start)
                    }
                }
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_ranges(anchors);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }
    return valid;
}

fn find_surround_pair<'a>(pairs: &'a [BracketPair], ch: &str) -> Option<&'a BracketPair> {
    pairs.iter().find(|pair| pair.start == ch || pair.end == ch)
}

fn all_support_surround_pair() -> Vec<BracketPair> {
    return vec![
        BracketPair {
            start: "{".into(),
            end: "}".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "'".into(),
            end: "'".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "`".into(),
            end: "`".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "\"".into(),
            end: "\"".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "(".into(),
            end: ")".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "|".into(),
            end: "|".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "[".into(),
            end: "]".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "{".into(),
            end: "}".into(),
            close: true,
            newline: false,
        },
        BracketPair {
            start: "<".into(),
            end: ">".into(),
            close: true,
            newline: false,
        },
    ];
}

fn pair_to_object(pair: &BracketPair) -> Option<Object> {
    match pair.start.as_str() {
        "'" => Some(Object::Quotes),
        "`" => Some(Object::BackQuotes),
        "\"" => Some(Object::DoubleQuotes),
        "|" => Some(Object::VerticalBars),
        "(" => Some(Object::Parentheses),
        "[" => Some(Object::SquareBrackets),
        "{" => Some(Object::CurlyBrackets),
        "<" => Some(Object::AngleBrackets),
        _ => None,
    }
}

fn object_to_bracket_pair(object: Object) -> Option<BracketPair> {
    match object {
        Object::Quotes => Some(BracketPair {
            start: "'".to_string(),
            end: "'".to_string(),
            close: true,
            newline: false,
        }),
        Object::BackQuotes => Some(BracketPair {
            start: "`".to_string(),
            end: "`".to_string(),
            close: true,
            newline: false,
        }),
        Object::DoubleQuotes => Some(BracketPair {
            start: "\"".to_string(),
            end: "\"".to_string(),
            close: true,
            newline: false,
        }),
        Object::VerticalBars => Some(BracketPair {
            start: "|".to_string(),
            end: "|".to_string(),
            close: true,
            newline: false,
        }),
        Object::Parentheses => Some(BracketPair {
            start: "(".to_string(),
            end: ")".to_string(),
            close: true,
            newline: false,
        }),
        Object::SquareBrackets => Some(BracketPair {
            start: "[".to_string(),
            end: "]".to_string(),
            close: true,
            newline: false,
        }),
        Object::CurlyBrackets => Some(BracketPair {
            start: "{".to_string(),
            end: "}".to_string(),
            close: true,
            newline: false,
        }),
        Object::AngleBrackets => Some(BracketPair {
            start: "<".to_string(),
            end: ">".to_string(),
            close: true,
            newline: false,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_add_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // test add surrounds with arround
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "i", "w", "{"]);
        cx.assert_state(
            indoc! {"
            The ˇ{ quick } brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds not with arround
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "i", "w", "}"]);
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds with motion
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "$", "}"]);
        cx.assert_state(
            indoc! {"
            The quˇ{ick brown}
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds with multi cursor
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the laˇzy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "i", "w", "'"]);
        cx.assert_state(
            indoc! {"
            The ˇ'quick' brown
            fox jumps over
            the ˇ'lazy' dog."},
            Mode::Normal,
        );

        // test multi cursor add surrounds with motion
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the laˇzy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "$", "'"]);
        cx.assert_state(
            indoc! {"
            The quˇ'ick brown'
            fox jumps over
            the laˇ'zy dog.'"},
            Mode::Normal,
        );

        // test multi cursor add surrounds with motion and custom string
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the laˇzy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "$", "1"]);
        cx.assert_state(
            indoc! {"
            The quˇ1ick brown1
            fox jumps over
            the laˇ1zy dog.1"},
            Mode::Normal,
        );

        // test add surrounds with motion current line
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "s", "{"]);
        cx.assert_state(
            indoc! {"
            ˇ{ The quick brown }
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
                The quˇick brown•
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "s", "{"]);
        cx.assert_state(
            indoc! {"
                ˇ{ The quick brown }•
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["2", "y", "s", "s", ")"]);
        cx.assert_state(
            indoc! {"
                ˇ({ The quick brown }•
            fox jumps over)
            the lazy dog."},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_delete_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // test delete surround
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "{"]);
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test delete not exist surrounds
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "["]);
        cx.assert_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test delete surround forward exist, in the surrounds plugin of other editors,
        // the bracket pair in front of the current line will be deleted here, which is not implemented at the moment
        cx.set_state(
            indoc! {"
            The {quick} brˇown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "{"]);
        cx.assert_state(
            indoc! {"
            The {quick} brˇown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test cursor delete inner surrounds
        cx.set_state(
            indoc! {"
            The { quick brown
            fox jumˇps over }
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "{"]);
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test multi cursor delete surrounds
        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the [laˇzy] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "]"]);
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        // test multi cursor delete surrounds with arround
        cx.set_state(
            indoc! {"
            Tˇhe [ quick ] brown
            fox jumps over
            the [laˇzy] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "["]);
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            Tˇhe [ quick ] brown
            fox jumps over
            the [laˇzy ] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "["]);
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        // test multi cursor delete different surrounds
        // the pair corresponding to the two cursors is the same,
        // so they are combined into one cursor
        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "{"]);
        cx.assert_state(
            indoc! {"
            The [quick] brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        // test delete surround with multi cursor and nest surrounds
        cx.set_state(
            indoc! {"
            fn test_surround() {
                ifˇ 2 > 1 {
                    ˇprintln!(\"it is fine\");
                };
            }"},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "}"]);
        cx.assert_state(
            indoc! {"
            fn test_surround() ˇ
                if 2 > 1 ˇ
                    println!(\"it is fine\");
                ;
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_change_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "s", "{", "["]);
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test multi cursor change surrounds
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "s", "{", "["]);
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the ˇ[ lazy ] dog."},
            Mode::Normal,
        );

        // test multi cursor delete different surrounds with after cursor
        cx.set_state(
            indoc! {"
            Thˇe {quick} brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "s", "{", "["]);
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the ˇ[ lazy ] dog."},
            Mode::Normal,
        );

        // test multi cursor change surrount with not arround
        cx.set_state(
            indoc! {"
            Thˇe { quick } brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "s", "{", "]"]);
        cx.assert_state(
            indoc! {"
            The ˇ[quick] brown
            fox jumps over
            the ˇ[lazy] dog."},
            Mode::Normal,
        );

        // test multi cursor change with not exist surround
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the [laˇzy] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "s", "[", "'"]);
        cx.assert_state(
            indoc! {"
            The {quick} brown
            fox jumps over
            the ˇ'lazy' dog."},
            Mode::Normal,
        );

        // test change nesting surrounds
        cx.set_state(
            indoc! {"
            fn test_surround() {
                ifˇ 2 > 1 {
                    ˇprintln!(\"it is fine\");
                }
            };"},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "s", "{", "["]);
        cx.assert_state(
            indoc! {"
            fn test_surround() ˇ[
                if 2 > 1 ˇ[
                    println!(\"it is fine\");
                ]
            ];"},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["y", "s", "i", "w", "["]);
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.simulate_keystrokes(["c", "s", "[", "}"]);
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.simulate_keystrokes(["d", "s", "{"]);
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.simulate_keystrokes(["u"]);
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
    }
}
