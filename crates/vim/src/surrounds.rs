use crate::{motion::Motion, object::Object, state::Mode, Vim};
use editor::{scroll::Autoscroll, Bias};
use gpui::WindowContext;
use language::BracketPair;
use serde::Deserialize;
use std::ops::Deref;
use std::sync::Arc;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum SurroundsType {
    Motion(Motion),
    Object(Object),
}

pub fn add_surrounds(text: Arc<str>, target: SurroundsType, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();
        vim.update_active_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| match &target {
                        SurroundsType::Object(object) => {
                            object.expand_selection(map, selection, false);
                        }
                        SurroundsType::Motion(motion) => {
                            motion.expand_selection(
                                map,
                                selection,
                                Some(1),
                                true,
                                &text_layout_details,
                            );
                        }
                    });
                });

                let input_text = text.to_string();
                let pair = match find_surround_pair(&all_support_surround_pair(), text.deref()) {
                    Some(pair) => pair,
                    None => return,
                };
                let surround = pair.end != input_text;
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);
                let mut edits = Vec::new();
                for selection in &selections {
                    let selection = selection.clone();
                    let offset_range = selection
                        .map(|p| p.to_offset(&display_map, Bias::Left))
                        .range();
                    let mut select_text = editor
                        .buffer()
                        .read(cx)
                        .snapshot(cx)
                        .text_for_range(offset_range.clone())
                        .collect::<String>();
                    if surround {
                        select_text = format!("{} {} {}", pair.start, select_text, pair.end);
                    } else {
                        select_text = format!("{}{}{}", pair.start, select_text, pair.end);
                    }
                    edits.push((offset_range, select_text));
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
                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(stable_anchors);
                });
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
    });
}

pub fn delete_surrounds(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();

        let input_text = text.to_string();
        // only legitimate surrounds can be removed
        let pair = match find_surround_pair(&all_support_surround_pair(), text.deref()) {
            Some(pair) => pair,
            None => return,
        };
        let pair_object = match pair_to_object(&pair) {
            Some(pair_object) => pair_object,
            None => return,
        };
        let surround = pair.end != input_text;

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
                                    match chars_and_offset.peek() {
                                        Some((next_ch, _)) => {
                                            if next_ch.to_string() == " " {
                                                end += 1;
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                edits.push((start..end, ""));
                                anchors.push(start..start);
                                break;
                            }
                        }
                        let mut reverse_chars_and_points = display_map
                            .reverse_buffer_chars_at(range.end.to_offset(&display_map, Bias::Left))
                            .peekable();
                        while let Some((ch, point)) = reverse_chars_and_points.next() {
                            if ch.to_string() == pair.end {
                                let mut start = point;
                                let end = start + 1;
                                if surround {
                                    match reverse_chars_and_points.peek() {
                                        Some((next_ch, _)) => {
                                            if next_ch.to_string() == " " {
                                                start -= 1;
                                            }
                                        }
                                        None => {}
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

                editor.change_selections(None, cx, |s| {
                    s.select_ranges(anchors);
                });
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

                    let pair = match find_surround_pair(&all_support_surround_pair(), text.deref())
                    {
                        Some(pair) => pair,
                        None => BracketPair {
                            start: text.to_string(),
                            end: text.to_string(),
                            close: true,
                            newline: false,
                        },
                    };
                    let surround = pair.end != text.to_string();
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
                                            if next_ch.to_string() != " " && surround {
                                                open_str.push_str(" ");
                                            } else if next_ch.to_string() == " " && !surround {
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
                                    match reverse_chars_and_offsets.peek() {
                                        Some((next_ch, _)) => {
                                            if next_ch.to_string() != " " && surround {
                                                close_str.insert_str(0, " ")
                                            } else if next_ch.to_string() == " " && !surround {
                                                start -= 1;
                                            }
                                        }
                                        None => {}
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

                    editor.buffer().update(cx, |buffer, cx| {
                        buffer.edit(edits.clone(), None, cx);
                    });
                    editor.set_clip_at_line_ends(true, cx);
                    editor.change_selections(None, cx, |s| {
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

                editor.change_selections(None, cx, |s| {
                    s.select_ranges(anchors);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }
    return valid;
}

fn find_surround_pair(pairs: &[BracketPair], ch: &str) -> Option<BracketPair> {
    for pair in pairs {
        if pair.start == ch || pair.end == ch {
            return Some(pair.clone());
        }
    }
    None
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

        // test multi cursor add surrounds
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
    }

    #[gpui::test]
    async fn test_delete_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

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

        // test delete surround forward
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

        cx.set_state(
            indoc! {"
            Tˇhe [quick] brown
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

        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the \"laˇzy\" dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "s", "\""]);
        cx.assert_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the ˇlazy dog."},
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
