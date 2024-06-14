mod case;
mod change;
mod delete;
mod increment;
mod indent;
pub(crate) mod mark;
mod paste;
pub(crate) mod repeat;
mod scroll;
pub(crate) mod search;
pub mod substitute;
pub(crate) mod yank;

use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    motion::{self, first_non_whitespace, next_line_end, right, Motion},
    object::Object,
    state::{Mode, Operator},
    surrounds::{check_and_move_to_valid_bracket_pair, SurroundsType},
    Vim,
};
use case::{change_case_motion, change_case_object, CaseTarget};
use collections::BTreeSet;
use editor::display_map::ToDisplayPoint;
use editor::scroll::Autoscroll;
use editor::Anchor;
use editor::Bias;
use editor::Editor;
use gpui::{actions, ViewContext, WindowContext};
use language::{Point, SelectionGoal};
use log::error;
use multi_buffer::MultiBufferRow;
use workspace::Workspace;

use self::{
    case::{change_case, convert_to_lower_case, convert_to_upper_case},
    change::{change_motion, change_object},
    delete::{delete_motion, delete_object},
    indent::{indent_motion, indent_object, IndentDirection},
    yank::{yank_motion, yank_object},
};

actions!(
    vim,
    [
        InsertAfter,
        InsertBefore,
        InsertFirstNonWhitespace,
        InsertEndOfLine,
        InsertLineAbove,
        InsertLineBelow,
        InsertAtPrevious,
        DeleteLeft,
        DeleteRight,
        ChangeToEndOfLine,
        DeleteToEndOfLine,
        Yank,
        YankLine,
        ChangeCase,
        ConvertToUpperCase,
        ConvertToLowerCase,
        JoinLines,
        Indent,
        Outdent,
    ]
);

pub(crate) fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    workspace.register_action(insert_after);
    workspace.register_action(insert_before);
    workspace.register_action(insert_first_non_whitespace);
    workspace.register_action(insert_end_of_line);
    workspace.register_action(insert_line_above);
    workspace.register_action(insert_line_below);
    workspace.register_action(insert_at_previous);
    workspace.register_action(change_case);
    workspace.register_action(convert_to_upper_case);
    workspace.register_action(convert_to_lower_case);
    workspace.register_action(yank_line);

    workspace.register_action(|_: &mut Workspace, _: &DeleteLeft, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let times = vim.take_count(cx);
            delete_motion(vim, Motion::Left, times, cx);
        })
    });
    workspace.register_action(|_: &mut Workspace, _: &DeleteRight, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let times = vim.take_count(cx);
            delete_motion(vim, Motion::Right, times, cx);
        })
    });
    workspace.register_action(|_: &mut Workspace, _: &ChangeToEndOfLine, cx| {
        Vim::update(cx, |vim, cx| {
            vim.start_recording(cx);
            let times = vim.take_count(cx);
            change_motion(
                vim,
                Motion::EndOfLine {
                    display_lines: false,
                },
                times,
                cx,
            );
        })
    });
    workspace.register_action(|_: &mut Workspace, _: &DeleteToEndOfLine, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let times = vim.take_count(cx);
            delete_motion(
                vim,
                Motion::EndOfLine {
                    display_lines: false,
                },
                times,
                cx,
            );
        })
    });
    workspace.register_action(|_: &mut Workspace, _: &JoinLines, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            let mut times = vim.take_count(cx).unwrap_or(1);
            if vim.state().mode.is_visual() {
                times = 1;
            } else if times > 1 {
                // 2J joins two lines together (same as J or 1J)
                times -= 1;
            }

            vim.update_active_editor(cx, |_, editor, cx| {
                editor.transact(cx, |editor, cx| {
                    for _ in 0..times {
                        editor.join_lines(&Default::default(), cx)
                    }
                })
            });
            if vim.state().mode.is_visual() {
                vim.switch_mode(Mode::Normal, false, cx)
            }
        });
    });

    workspace.register_action(|_: &mut Workspace, _: &Indent, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            vim.update_active_editor(cx, |_, editor, cx| {
                editor.transact(cx, |editor, cx| {
                    let mut original_positions = save_selection_starts(editor, cx);
                    editor.indent(&Default::default(), cx);
                    restore_selection_cursors(editor, cx, &mut original_positions);
                });
            });
            if vim.state().mode.is_visual() {
                vim.switch_mode(Mode::Normal, false, cx)
            }
        });
    });

    workspace.register_action(|_: &mut Workspace, _: &Outdent, cx| {
        Vim::update(cx, |vim, cx| {
            vim.record_current_action(cx);
            vim.update_active_editor(cx, |_, editor, cx| {
                editor.transact(cx, |editor, cx| {
                    let mut original_positions = save_selection_starts(editor, cx);
                    editor.outdent(&Default::default(), cx);
                    restore_selection_cursors(editor, cx, &mut original_positions);
                });
            });
            if vim.state().mode.is_visual() {
                vim.switch_mode(Mode::Normal, false, cx)
            }
        });
    });

    paste::register(workspace, cx);
    repeat::register(workspace, cx);
    scroll::register(workspace, cx);
    search::register(workspace, cx);
    substitute::register(workspace, cx);
    increment::register(workspace, cx);
}

pub fn normal_motion(
    motion: Motion,
    operator: Option<Operator>,
    times: Option<usize>,
    cx: &mut WindowContext,
) {
    Vim::update(cx, |vim, cx| {
        match operator {
            None => move_cursor(vim, motion, times, cx),
            Some(Operator::Change) => change_motion(vim, motion, times, cx),
            Some(Operator::Delete) => delete_motion(vim, motion, times, cx),
            Some(Operator::Yank) => yank_motion(vim, motion, times, cx),
            Some(Operator::AddSurrounds { target: None }) => {}
            Some(Operator::Indent) => indent_motion(vim, motion, times, IndentDirection::In, cx),
            Some(Operator::Outdent) => indent_motion(vim, motion, times, IndentDirection::Out, cx),
            Some(Operator::Lowercase) => {
                change_case_motion(vim, motion, times, CaseTarget::Lowercase, cx)
            }
            Some(Operator::Uppercase) => {
                change_case_motion(vim, motion, times, CaseTarget::Uppercase, cx)
            }
            Some(Operator::OppositeCase) => {
                change_case_motion(vim, motion, times, CaseTarget::OppositeCase, cx)
            }
            Some(operator) => {
                // Can't do anything for text objects, Ignoring
                error!("Unexpected normal mode motion operator: {:?}", operator)
            }
        }
    });
}

pub fn normal_object(object: Object, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        let mut waiting_operator: Option<Operator> = None;
        match vim.maybe_pop_operator() {
            Some(Operator::Object { around }) => match vim.maybe_pop_operator() {
                Some(Operator::Change) => change_object(vim, object, around, cx),
                Some(Operator::Delete) => delete_object(vim, object, around, cx),
                Some(Operator::Yank) => yank_object(vim, object, around, cx),
                Some(Operator::Indent) => {
                    indent_object(vim, object, around, IndentDirection::In, cx)
                }
                Some(Operator::Outdent) => {
                    indent_object(vim, object, around, IndentDirection::Out, cx)
                }
                Some(Operator::Lowercase) => {
                    change_case_object(vim, object, around, CaseTarget::Lowercase, cx)
                }
                Some(Operator::Uppercase) => {
                    change_case_object(vim, object, around, CaseTarget::Uppercase, cx)
                }
                Some(Operator::OppositeCase) => {
                    change_case_object(vim, object, around, CaseTarget::OppositeCase, cx)
                }
                Some(Operator::AddSurrounds { target: None }) => {
                    waiting_operator = Some(Operator::AddSurrounds {
                        target: Some(SurroundsType::Object(object)),
                    });
                }
                _ => {
                    // Can't do anything for namespace operators. Ignoring
                }
            },
            Some(Operator::DeleteSurrounds) => {
                waiting_operator = Some(Operator::DeleteSurrounds);
            }
            Some(Operator::ChangeSurrounds { target: None }) => {
                if check_and_move_to_valid_bracket_pair(vim, object, cx) {
                    waiting_operator = Some(Operator::ChangeSurrounds {
                        target: Some(object),
                    });
                }
            }
            _ => {
                // Can't do anything with change/delete/yank/surrounds and text objects. Ignoring
            }
        }
        vim.clear_operator(cx);
        if let Some(operator) = waiting_operator {
            vim.push_operator(operator, cx);
        }
    });
}

pub(crate) fn move_cursor(
    vim: &mut Vim,
    motion: Motion,
    times: Option<usize>,
    cx: &mut WindowContext,
) {
    vim.update_active_editor(cx, |_, editor, cx| {
        let text_layout_details = editor.text_layout_details(cx);
        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, cursor, goal| {
                motion
                    .move_point(map, cursor, goal, times, &text_layout_details)
                    .unwrap_or((cursor, goal))
            })
        })
    });
}

fn insert_after(_: &mut Workspace, _: &InsertAfter, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_cursors_with(|map, cursor, _| (right(map, cursor, 1), SelectionGoal::None));
            });
        });
    });
}

fn insert_before(_: &mut Workspace, _: &InsertBefore, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
    });
}

fn insert_first_non_whitespace(
    _: &mut Workspace,
    _: &InsertFirstNonWhitespace,
    cx: &mut ViewContext<Workspace>,
) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_cursors_with(|map, cursor, _| {
                    (
                        first_non_whitespace(map, false, cursor),
                        SelectionGoal::None,
                    )
                });
            });
        });
    });
}

fn insert_end_of_line(_: &mut Workspace, _: &InsertEndOfLine, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_cursors_with(|map, cursor, _| {
                    (next_line_end(map, cursor, 1), SelectionGoal::None)
                });
            });
        });
    });
}

fn insert_at_previous(_: &mut Workspace, _: &InsertAtPrevious, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |vim, editor, cx| {
            if let Some(marks) = vim.state().marks.get("^") {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_anchor_ranges(marks.iter().map(|mark| *mark..*mark))
                });
            }
        });
    });
}

fn insert_line_above(_: &mut Workspace, _: &InsertLineAbove, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let selections = editor.selections.all::<Point>(cx);
                let snapshot = editor.buffer().read(cx).snapshot(cx);

                let selection_start_rows: BTreeSet<u32> = selections
                    .into_iter()
                    .map(|selection| selection.start.row)
                    .collect();
                let edits = selection_start_rows.into_iter().map(|row| {
                    let indent = snapshot
                        .indent_size_for_line(MultiBufferRow(row))
                        .chars()
                        .collect::<String>();
                    let start_of_line = Point::new(row, 0);
                    (start_of_line..start_of_line, indent + "\n")
                });
                editor.edit_with_autoindent(edits, cx);
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_cursors_with(|map, cursor, _| {
                        let previous_line = motion::start_of_relative_buffer_row(map, cursor, -1);
                        let insert_point = motion::end_of_line(map, false, previous_line, 1);
                        (insert_point, SelectionGoal::None)
                    });
                });
            });
        });
    });
}

fn insert_line_below(_: &mut Workspace, _: &InsertLineBelow, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.transact(cx, |editor, cx| {
                let selections = editor.selections.all::<Point>(cx);
                let snapshot = editor.buffer().read(cx).snapshot(cx);

                let selection_end_rows: BTreeSet<u32> = selections
                    .into_iter()
                    .map(|selection| selection.end.row)
                    .collect();
                let edits = selection_end_rows.into_iter().map(|row| {
                    let indent = snapshot
                        .indent_size_for_line(MultiBufferRow(row))
                        .chars()
                        .collect::<String>();
                    let end_of_line = Point::new(row, snapshot.line_len(MultiBufferRow(row)));
                    (end_of_line..end_of_line, "\n".to_string() + &indent)
                });
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.maybe_move_cursors_with(|map, cursor, goal| {
                        Motion::CurrentLine.move_point(
                            map,
                            cursor,
                            goal,
                            None,
                            &text_layout_details,
                        )
                    });
                });
                editor.edit_with_autoindent(edits, cx);
            });
        });
    });
}

fn yank_line(_: &mut Workspace, _: &YankLine, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        let count = vim.take_count(cx);
        yank_motion(vim, motion::Motion::CurrentLine, count, cx)
    })
}

fn save_selection_starts(editor: &Editor, cx: &mut ViewContext<Editor>) -> HashMap<usize, Anchor> {
    let (map, selections) = editor.selections.all_display(cx);
    selections
        .iter()
        .map(|selection| {
            (
                selection.id,
                map.display_point_to_anchor(selection.start, Bias::Right),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn restore_selection_cursors(
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
    positions: &mut HashMap<usize, Anchor>,
) {
    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
        s.move_with(|map, selection| {
            if let Some(anchor) = positions.remove(&selection.id) {
                selection.collapse_to(anchor.to_display_point(map), SelectionGoal::None);
            }
        });
    });
}

pub(crate) fn normal_replace(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let (map, display_selections) = editor.selections.all_display(cx);
                // Selections are biased right at the start. So we need to store
                // anchors that are biased left so that we can restore the selections
                // after the change
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    .map(|selection| {
                        let start = selection.start.bias_left(&map.buffer_snapshot);
                        start..start
                    })
                    .collect::<Vec<_>>();

                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let mut range = selection.range();
                        *range.end.column_mut() += 1;
                        range.end = map.clip_point(range.end, Bias::Right);

                        (
                            range.start.to_offset(&map, Bias::Left)
                                ..range.end.to_offset(&map, Bias::Left),
                            text.clone(),
                        )
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
        vim.pop_operator(cx)
    });
}

#[cfg(test)]
mod test {
    use gpui::{KeyBinding, TestAppContext};
    use indoc::indoc;
    use settings::SettingsStore;

    use crate::{
        motion,
        state::Mode::{self},
        test::{NeovimBackedTestContext, VimTestContext},
        VimSettings,
    };

    #[gpui::test]
    async fn test_h(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "h",
            indoc! {"
            ˇThe qˇuick
            ˇbrown"
            },
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_backspace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "backspace",
            indoc! {"
            ˇThe qˇuick
            ˇbrown"
            },
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            aaˇaa
            😃😃"
        })
        .await;
        cx.simulate_shared_keystrokes("j").await;
        cx.shared_state().await.assert_eq(indoc! {"
            aaaa
            😃ˇ😃"
        });

        cx.simulate_at_each_offset(
            "j",
            indoc! {"
                ˇThe qˇuick broˇwn
                ˇfox jumps"
            },
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_enter(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "enter",
            indoc! {"
            ˇThe qˇuick broˇwn
            ˇfox jumps"
            },
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "k",
            indoc! {"
            ˇThe qˇuick
            ˇbrown fˇox jumˇps"
            },
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_l(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "l",
            indoc! {"
            ˇThe qˇuicˇk
            ˇbrowˇn"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_jump_to_line_boundaries(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "$",
            indoc! {"
            ˇThe qˇuicˇk
            ˇbrowˇn"},
        )
        .await
        .assert_matches();
        cx.simulate_at_each_offset(
            "0",
            indoc! {"
                ˇThe qˇuicˇk
                ˇbrowˇn"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_jump_to_end(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.simulate_at_each_offset(
            "shift-g",
            indoc! {"
                The ˇquick

                brown fox jumps
                overˇ the lazy doˇg"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "shift-g",
            indoc! {"
            The quiˇck

            brown"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "shift-g",
            indoc! {"
            The quiˇck

            "},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_w(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "w",
            indoc! {"
            The ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇe"},
        )
        .await
        .assert_matches();
        cx.simulate_at_each_offset(
            "shift-w",
            indoc! {"
            The ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇe"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_end_of_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "e",
            indoc! {"
            Thˇe quicˇkˇ-browˇn


            fox_jumpˇs oveˇr
            thˇe"},
        )
        .await
        .assert_matches();
        cx.simulate_at_each_offset(
            "shift-e",
            indoc! {"
            Thˇe quicˇkˇ-browˇn


            fox_jumpˇs oveˇr
            thˇe"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "b",
            indoc! {"
            ˇThe ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"},
        )
        .await
        .assert_matches();
        cx.simulate_at_each_offset(
            "shift-b",
            indoc! {"
            ˇThe ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "g g",
            indoc! {"
                The qˇuick

                brown fox jumps
                over ˇthe laˇzy dog"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "g g",
            indoc! {"


                brown fox jumps
                over the laˇzy dog"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "2 g g",
            indoc! {"
                ˇ

                brown fox jumps
                over the lazydog"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "shift-g",
            indoc! {"
                The qˇuick

                brown fox jumps
                over ˇthe laˇzy dog"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "shift-g",
            indoc! {"


                brown fox jumps
                over the laˇzy dog"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "2 shift-g",
            indoc! {"
                ˇ

                brown fox jumps
                over the lazydog"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_a(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset("a", "The qˇuicˇk")
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_insert_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset(
            "shift-a",
            indoc! {"
            ˇ
            The qˇuick
            brown ˇfox "},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_jump_to_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("^", "The qˇuick").await.assert_matches();
        cx.simulate("^", " The qˇuick").await.assert_matches();
        cx.simulate("^", "ˇ").await.assert_matches();
        cx.simulate(
            "^",
            indoc! {"
                The qˇuick
                brown fox"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "^",
            indoc! {"
                ˇ
                The quick"},
        )
        .await
        .assert_matches();
        // Indoc disallows trailing whitespace.
        cx.simulate("^", "   ˇ \nThe quick").await.assert_matches();
    }

    #[gpui::test]
    async fn test_insert_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("shift-i", "The qˇuick").await.assert_matches();
        cx.simulate("shift-i", " The qˇuick").await.assert_matches();
        cx.simulate("shift-i", "ˇ").await.assert_matches();
        cx.simulate(
            "shift-i",
            indoc! {"
                The qˇuick
                brown fox"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "shift-i",
            indoc! {"
                ˇ
                The quick"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_to_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "shift-d",
            indoc! {"
                The qˇuick
                brown fox"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "shift-d",
            indoc! {"
                The quick
                ˇ
                brown fox"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_x(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset("x", "ˇTeˇsˇt")
            .await
            .assert_matches();
        cx.simulate(
            "x",
            indoc! {"
                Tesˇt
                test"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_left(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset("shift-x", "ˇTˇeˇsˇt")
            .await
            .assert_matches();
        cx.simulate(
            "shift-x",
            indoc! {"
                Test
                ˇtest"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_o(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("o", "ˇ").await.assert_matches();
        cx.simulate("o", "The ˇquick").await.assert_matches();
        cx.simulate_at_each_offset(
            "o",
            indoc! {"
                The qˇuick
                brown ˇfox
                jumps ˇover"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "o",
            indoc! {"
                The quick
                ˇ
                brown fox"},
        )
        .await
        .assert_matches();

        cx.assert_binding(
            "o",
            indoc! {"
                fn test() {
                    println!(ˇ);
                }"},
            Mode::Normal,
            indoc! {"
                fn test() {
                    println!();
                    ˇ
                }"},
            Mode::Insert,
        );

        cx.assert_binding(
            "o",
            indoc! {"
                fn test(ˇ) {
                    println!();
                }"},
            Mode::Normal,
            indoc! {"
                fn test() {
                    ˇ
                    println!();
                }"},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_insert_line_above(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("shift-o", "ˇ").await.assert_matches();
        cx.simulate("shift-o", "The ˇquick").await.assert_matches();
        cx.simulate_at_each_offset(
            "shift-o",
            indoc! {"
            The qˇuick
            brown ˇfox
            jumps ˇover"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "shift-o",
            indoc! {"
            The quick
            ˇ
            brown fox"},
        )
        .await
        .assert_matches();

        // Our indentation is smarter than vims. So we don't match here
        cx.assert_binding(
            "shift-o",
            indoc! {"
                fn test() {
                    println!(ˇ);
                }"},
            Mode::Normal,
            indoc! {"
                fn test() {
                    ˇ
                    println!();
                }"},
            Mode::Insert,
        );
        cx.assert_binding(
            "shift-o",
            indoc! {"
                fn test(ˇ) {
                    println!();
                }"},
            Mode::Normal,
            indoc! {"
                ˇ
                fn test() {
                    println!();
                }"},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_dd(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("d d", "ˇ").await.assert_matches();
        cx.simulate("d d", "The ˇquick").await.assert_matches();
        cx.simulate_at_each_offset(
            "d d",
            indoc! {"
            The qˇuick
            brown ˇfox
            jumps ˇover"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d d",
            indoc! {"
                The quick
                ˇ
                brown fox"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_cc(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("c c", "ˇ").await.assert_matches();
        cx.simulate("c c", "The ˇquick").await.assert_matches();
        cx.simulate_at_each_offset(
            "c c",
            indoc! {"
                The quˇick
                brown ˇfox
                jumps ˇover"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "c c",
            indoc! {"
                The quick
                ˇ
                brown fox"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_repeated_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for count in 1..=5 {
            cx.simulate_at_each_offset(
                &format!("{count} w"),
                indoc! {"
                    ˇThe quˇickˇ browˇn
                    ˇ
                    ˇfox ˇjumpsˇ-ˇoˇver
                    ˇthe lazy dog
                "},
            )
            .await
            .assert_matches();
        }
    }

    #[gpui::test]
    async fn test_h_through_unicode(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset("h", "Testˇ├ˇ──ˇ┐ˇTest")
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_f_and_t(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for count in 1..=3 {
            let test_case = indoc! {"
                ˇaaaˇbˇ ˇbˇ   ˇbˇbˇ aˇaaˇbaaa
                ˇ    ˇbˇaaˇa ˇbˇbˇb
                ˇ
                ˇb
            "};

            cx.simulate_at_each_offset(&format!("{count} f b"), test_case)
                .await
                .assert_matches();

            cx.simulate_at_each_offset(&format!("{count} t b"), test_case)
                .await
                .assert_matches();
        }
    }

    #[gpui::test]
    async fn test_capital_f_and_capital_t(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        let test_case = indoc! {"
            ˇaaaˇbˇ ˇbˇ   ˇbˇbˇ aˇaaˇbaaa
            ˇ    ˇbˇaaˇa ˇbˇbˇb
            ˇ•••
            ˇb
            "
        };

        for count in 1..=3 {
            cx.simulate_at_each_offset(&format!("{count} shift-f b"), test_case)
                .await
                .assert_matches();

            cx.simulate_at_each_offset(&format!("{count} shift-t b"), test_case)
                .await
                .assert_matches();
        }
    }

    #[gpui::test]
    async fn test_f_and_t_multiline(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<VimSettings>(cx, |s| {
                s.use_multiline_find = Some(true);
            });
        });

        cx.assert_binding(
            "f l",
            indoc! {"
            ˇfunction print() {
                console.log('ok')
            }
            "},
            Mode::Normal,
            indoc! {"
            function print() {
                consoˇle.log('ok')
            }
            "},
            Mode::Normal,
        );

        cx.assert_binding(
            "t l",
            indoc! {"
            ˇfunction print() {
                console.log('ok')
            }
            "},
            Mode::Normal,
            indoc! {"
            function print() {
                consˇole.log('ok')
            }
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_capital_f_and_capital_t_multiline(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<VimSettings>(cx, |s| {
                s.use_multiline_find = Some(true);
            });
        });

        cx.assert_binding(
            "shift-f p",
            indoc! {"
            function print() {
                console.ˇlog('ok')
            }
            "},
            Mode::Normal,
            indoc! {"
            function ˇprint() {
                console.log('ok')
            }
            "},
            Mode::Normal,
        );

        cx.assert_binding(
            "shift-t p",
            indoc! {"
            function print() {
                console.ˇlog('ok')
            }
            "},
            Mode::Normal,
            indoc! {"
            function pˇrint() {
                console.log('ok')
            }
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_f_and_t_smartcase(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<VimSettings>(cx, |s| {
                s.use_smartcase_find = Some(true);
            });
        });

        cx.assert_binding(
            "f p",
            indoc! {"ˇfmt.Println(\"Hello, World!\")"},
            Mode::Normal,
            indoc! {"fmt.ˇPrintln(\"Hello, World!\")"},
            Mode::Normal,
        );

        cx.assert_binding(
            "shift-f p",
            indoc! {"fmt.Printlnˇ(\"Hello, World!\")"},
            Mode::Normal,
            indoc! {"fmt.ˇPrintln(\"Hello, World!\")"},
            Mode::Normal,
        );

        cx.assert_binding(
            "t p",
            indoc! {"ˇfmt.Println(\"Hello, World!\")"},
            Mode::Normal,
            indoc! {"fmtˇ.Println(\"Hello, World!\")"},
            Mode::Normal,
        );

        cx.assert_binding(
            "shift-t p",
            indoc! {"fmt.Printlnˇ(\"Hello, World!\")"},
            Mode::Normal,
            indoc! {"fmt.Pˇrintln(\"Hello, World!\")"},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_percent(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate_at_each_offset("%", "ˇconsole.logˇ(ˇvaˇrˇ)ˇ;")
            .await
            .assert_matches();
        cx.simulate_at_each_offset("%", "ˇconsole.logˇ(ˇ'var', ˇ[ˇ1, ˇ2, 3ˇ]ˇ)ˇ;")
            .await
            .assert_matches();
        cx.simulate_at_each_offset("%", "let result = curried_funˇ(ˇ)ˇ(ˇ)ˇ;")
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_end_of_line_with_neovim(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // goes to current line end
        cx.set_shared_state(indoc! {"ˇaa\nbb\ncc"}).await;
        cx.simulate_shared_keystrokes("$").await;
        cx.shared_state().await.assert_eq("aˇa\nbb\ncc");

        // goes to next line end
        cx.simulate_shared_keystrokes("2 $").await;
        cx.shared_state().await.assert_eq("aa\nbˇb\ncc");

        // try to exceed the final line.
        cx.simulate_shared_keystrokes("4 $").await;
        cx.shared_state().await.assert_eq("aa\nbb\ncˇc");
    }

    #[gpui::test]
    async fn test_subword_motions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.update(|cx| {
            cx.bind_keys(vec![
                KeyBinding::new(
                    "w",
                    motion::NextSubwordStart {
                        ignore_punctuation: false,
                    },
                    Some("Editor && VimControl && !VimWaiting && !menu"),
                ),
                KeyBinding::new(
                    "b",
                    motion::PreviousSubwordStart {
                        ignore_punctuation: false,
                    },
                    Some("Editor && VimControl && !VimWaiting && !menu"),
                ),
                KeyBinding::new(
                    "e",
                    motion::NextSubwordEnd {
                        ignore_punctuation: false,
                    },
                    Some("Editor && VimControl && !VimWaiting && !menu"),
                ),
                KeyBinding::new(
                    "g e",
                    motion::PreviousSubwordEnd {
                        ignore_punctuation: false,
                    },
                    Some("Editor && VimControl && !VimWaiting && !menu"),
                ),
            ]);
        });

        cx.assert_binding_normal("w", indoc! {"ˇassert_binding"}, indoc! {"assert_ˇbinding"});
        // Special case: In 'cw', 'w' acts like 'e'
        cx.assert_binding(
            "c w",
            indoc! {"ˇassert_binding"},
            Mode::Normal,
            indoc! {"ˇ_binding"},
            Mode::Insert,
        );

        cx.assert_binding_normal("e", indoc! {"ˇassert_binding"}, indoc! {"asserˇt_binding"});

        cx.assert_binding_normal("b", indoc! {"assert_ˇbinding"}, indoc! {"ˇassert_binding"});

        cx.assert_binding_normal(
            "g e",
            indoc! {"assert_bindinˇg"},
            indoc! {"asserˇt_binding"},
        );
    }
}
