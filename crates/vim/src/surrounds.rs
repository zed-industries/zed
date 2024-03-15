use crate::{motion::Motion, object::Object, state::Mode, Vim};
use editor::{scroll::Autoscroll, Bias};
use gpui::WindowContext;
use language::BracketPair;
use serde::Deserialize;
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
            editor.set_clip_at_line_ends(false, cx);

            let text_layout_details = editor.text_layout_details(cx);
            editor.transact(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        match &target {
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
                        }
                        let offset_range = selection.map(|p| p.to_offset(map, Bias::Left)).range();
                    });
                });

                let input_text = text.to_string();
                let mut surround = false;
                let mut open_str = input_text.clone();
                let mut close_str = input_text.clone();
                let pairs = all_support_surround_pair();
                for pair in pairs {
                    if pair.start == input_text || pair.end == input_text {
                        // Spaces are added only if the current input is open parenthesis
                        // Does not contain ', ", |", etc
                        surround = !(pair.end == input_text);
                        open_str = pair.start;
                        close_str = pair.end;
                        break;
                    }
                }
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);
                let mut edits = Vec::new();
                for selection in selections.iter() {
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
                        select_text = format!("{} {} {}", open_str, select_text, close_str);
                    } else {
                        select_text = format!("{}{}{}", open_str, select_text, close_str);
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
                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(stable_anchors);
                });
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
    });
}

pub fn delete_surrounds(vim: &mut Vim, object: Object, cx: &mut WindowContext) {
    if let Some(surround_pair) = object_to_bracbracket_pair(object) {
        vim.stop_recording();
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        object.expand_selection(map, selection, true);
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
                        if let Some(pos) = select_text.find(surround_pair.start.as_str()) {
                            select_text.remove(pos);
                        }
                        if let Some(pos) = select_text.rfind(surround_pair.end.as_str()) {
                            select_text.remove(pos);
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
                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(stable_anchors);
                });
            });
        });
    }
}

pub fn change_surrounds(text: Arc<str>, target: Object, cx: &mut WindowContext) {
    if let Some(will_replace_pair) = object_to_bracbracket_pair(target) {
        Vim::update(cx, |vim, cx| {
            vim.stop_recording();
            vim.update_active_editor(cx, |_, editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                // let text_layout_details = editor.text_layout_details(cx);

                editor.transact(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.move_with(|map, selection| {
                            target.expand_selection(map, selection, true);
                        });
                    });

                    let input_text = text.to_string();
                    let mut open_str = input_text.clone();
                    let mut close_str = input_text.clone();
                    let pairs = all_support_surround_pair();
                    for pair in pairs {
                        if pair.start == input_text || pair.end == input_text {
                            open_str = pair.start;
                            close_str = pair.end;
                            break;
                        }
                    }
                    let (display_map, selections) = editor.selections.all_adjusted_display(cx);
                    let mut edits = Vec::new();
                    for selection in selections.iter() {
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
                        if select_text.starts_with(&will_replace_pair.start)
                            && select_text.ends_with(&will_replace_pair.end)
                        {
                            let len = select_text.len();
                            select_text.replace_range(0..1, &open_str);
                            select_text.replace_range(len - 1..len, &close_str);
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
                    editor.change_selections(None, cx, |s| {
                        s.select_anchor_ranges(stable_anchors);
                    });
                });
            });
            vim.switch_mode(Mode::Normal, false, cx);
        });
    }
}

pub fn is_valid_bracket_part(object: Object) -> bool {
    if let Some(_) = object_to_bracbracket_pair(object) {
        return true;
    }
    return false;
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

fn object_to_bracbracket_pair(object: Object) -> Option<BracketPair> {
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
            The [quick] brown
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
            The ˇ[quick] brown
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
}
