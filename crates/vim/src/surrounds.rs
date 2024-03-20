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
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        let original_head = selection.head();
                        pair_object.expand_selection(map, selection, true);
                        let not_found = original_head == selection.head();
                        let is_inner =
                            selection.start < original_head && selection.end > original_head;
                        let is_same_row = selection.start.row() == original_head.row()
                            && selection.end.row() == original_head.row();

                        // Only surround within the current cursor and the next pair of surrounds that match the current row should be handled
                        if not_found || (is_inner == false && is_same_row == false) {
                            selection.start = original_head;
                            selection.end = original_head;
                        }
                    });
                });

                let (display_map, display_selections) = editor.selections.all_display(cx);
                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let offset_range = selection
                            .map(|p| p.to_offset(&display_map, Bias::Left))
                            .range();
                        let mut select_text = editor
                            .buffer()
                            .read(cx)
                            .snapshot(cx)
                            .text_for_range(offset_range.clone())
                            .collect::<String>();
                        if let Some(pos) = select_text.find(pair.start.as_str()) {
                            select_text.remove(pos);
                        }
                        if let Some(pos) = select_text.rfind(pair.end.as_str()) {
                            select_text.remove(pos);
                        }
                        if surround {
                            select_text = select_text.trim_matches(' ').to_string();
                        }
                        (offset_range, select_text)
                    })
                    .collect::<Vec<_>>();

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
    });
}

pub fn change_surrounds(text: Arc<str>, target: Object, cx: &mut WindowContext) {
    if let Some(will_replace_pair) = object_to_bracket_pair(target) {
        Vim::update(cx, |vim, cx| {
            vim.stop_recording();
            vim.update_active_editor(cx, |_, editor, cx| {
                editor.transact(cx, |editor, cx| {
                    editor.set_clip_at_line_ends(false, cx);

                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.move_with(|map, selection| {
                            target.expand_selection(map, selection, true);
                        });
                    });

                    let input_text = text.to_string();
                    let pair = match find_surround_pair(&all_support_surround_pair(), text.deref())
                    {
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
                        // This is a bit cumbersome, and it is written to deal with some special cases, as shown below
                        // hello«ˇ  "hello in a word"  »again.
                        // Sometimes the expand_selection will not be matched at both ends, and there will be extra spaces
                        // In order to be able to accurately match and replace in this case, some cumbersome methods are used
                        if let Some(pos) = select_text.find(will_replace_pair.start.as_str()) {
                            select_text.replace_range(pos..pos + 1, pair.start.as_str());
                            if let Some(space_pos) = select_text.find(" ") {
                                if surround {
                                    if space_pos != pos + 1 {
                                        select_text.replace_range(pos + 1..pos + 1, " ");
                                    }
                                } else {
                                    if space_pos == pos + 1 {
                                        select_text.remove(space_pos);
                                    }
                                }
                            } else if pos + 1 < select_text.len() && surround {
                                select_text.replace_range(pos + 1..pos + 1, " ");
                            }
                        }
                        if let Some(pos) = select_text.rfind(will_replace_pair.end.as_str()) {
                            select_text.replace_range(pos..pos + 1, pair.end.as_str());
                            if let Some(space_pos) = select_text.rfind(" ") {
                                if surround {
                                    if space_pos != pos - 1 {
                                        select_text.replace_range(pos..pos, " ");
                                    }
                                } else {
                                    if space_pos == pos - 1 {
                                        select_text.remove(space_pos);
                                    }
                                }
                            } else if pos - 1 > 0 && surround {
                                select_text.replace_range(pos - 1..pos - 1, " ");
                            }
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
        });
    }
}

pub fn is_valid_bracket_part(vim: &mut Vim, object: Object, cx: &mut WindowContext) -> bool {
    let mut valid = false;

    if let Some(pair) = object_to_bracket_pair(object) {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);

                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        let original_head = selection.head();
                        object.expand_selection(map, selection, true);

                        // Only surround within the current cursor and the next pair of surrounds that match the current row should be handled
                        let not_found = original_head == selection.head();
                        let is_inner =
                            selection.start < original_head && selection.end > original_head;
                        let is_same_row = selection.start.row() == original_head.row()
                            && selection.end.row() == original_head.row();
                        if not_found || (is_inner == false && is_same_row == false) {
                            selection.start = original_head;
                            selection.end = original_head;
                        } else {
                            // Jumps the cursor to the current matching location
                            selection.end = selection.start;
                            valid = true;
                        }
                    });
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
}
