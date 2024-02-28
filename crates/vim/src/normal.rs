mod case;
mod change;
mod delete;
mod increment;
mod paste;
pub(crate) mod repeat;
mod scroll;
pub(crate) mod search;
pub mod substitute;
mod yank;

use std::sync::Arc;

use crate::{
    motion::{self, first_non_whitespace, next_line_end, right, Motion},
    object::Object,
    state::{Mode, Operator},
    Vim,
};
use collections::HashSet;
use editor::scroll::Autoscroll;
use editor::{Bias, DisplayPoint};
use gpui::{actions, ViewContext, WindowContext};
use language::SelectionGoal;
use log::error;
use workspace::Workspace;

use self::{
    case::{change_case, convert_to_lower_case, convert_to_upper_case},
    change::{change_motion, change_object},
    delete::{delete_motion, delete_object},
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
                editor.transact(cx, |editor, cx| editor.indent(&Default::default(), cx))
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
                editor.transact(cx, |editor, cx| editor.outdent(&Default::default(), cx))
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
            Some(operator) => {
                // Can't do anything for text objects, Ignoring
                error!("Unexpected normal mode motion operator: {:?}", operator)
            }
        }
    });
}

pub fn normal_object(object: Object, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        match vim.maybe_pop_operator() {
            Some(Operator::Object { around }) => match vim.maybe_pop_operator() {
                Some(Operator::Change) => change_object(vim, object, around, cx),
                Some(Operator::Delete) => delete_object(vim, object, around, cx),
                Some(Operator::Yank) => yank_object(vim, object, around, cx),
                _ => {
                    // Can't do anything for namespace operators. Ignoring
                }
            },
            _ => {
                // Can't do anything with change/delete/yank and text objects. Ignoring
            }
        }
        vim.clear_operator(cx);
    })
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

fn insert_line_above(_: &mut Workspace, _: &InsertLineAbove, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.start_recording(cx);
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (map, old_selections) = editor.selections.all_display(cx);
                let selection_start_rows: HashSet<u32> = old_selections
                    .into_iter()
                    .map(|selection| selection.start.row())
                    .collect();
                let edits = selection_start_rows.into_iter().map(|row| {
                    let (indent, _) = map.line_indent(row);
                    let start_of_line =
                        motion::start_of_line(&map, false, DisplayPoint::new(row, 0))
                            .to_point(&map);
                    let mut new_text = " ".repeat(indent as usize);
                    new_text.push('\n');
                    (start_of_line..start_of_line, new_text)
                });
                editor.edit_with_autoindent(edits, cx);
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_cursors_with(|map, cursor, _| {
                        let previous_line = motion::start_of_relative_buffer_row(map, cursor, -1);
                        let insert_point = motion::end_of_line(map, false, previous_line);
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
                let (map, old_selections) = editor.selections.all_display(cx);

                let selection_end_rows: HashSet<u32> = old_selections
                    .into_iter()
                    .map(|selection| selection.end.row())
                    .collect();
                let edits = selection_end_rows.into_iter().map(|row| {
                    let (indent, _) = map.line_indent(row);
                    let end_of_line =
                        motion::end_of_line(&map, false, DisplayPoint::new(row, 0)).to_point(&map);

                    let mut new_text = "\n".to_string();
                    new_text.push_str(&" ".repeat(indent as usize));
                    (end_of_line..end_of_line, new_text)
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
    use gpui::TestAppContext;
    use indoc::indoc;
    use settings::SettingsStore;

    use crate::{
        state::Mode::{self},
        test::{NeovimBackedTestContext, VimTestContext},
        VimSettings,
    };

    #[gpui::test]
    async fn test_h(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["h"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick
            ˇbrown"
        })
        .await;
    }

    #[gpui::test]
    async fn test_backspace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["backspace"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick
            ˇbrown"
        })
        .await;
    }

    #[gpui::test]
    async fn test_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
                    aaˇaa
                    😃😃"
        })
        .await;
        cx.simulate_shared_keystrokes(["j"]).await;
        cx.assert_shared_state(indoc! {"
                    aaaa
                    😃ˇ😃"
        })
        .await;

        for marked_position in cx.each_marked_position(indoc! {"
                    ˇThe qˇuick broˇwn
                    ˇfox jumps"
        }) {
            cx.assert_neovim_compatible(&marked_position, ["j"]).await;
        }
    }

    #[gpui::test]
    async fn test_enter(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["enter"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick broˇwn
            ˇfox jumps"
        })
        .await;
    }

    #[gpui::test]
    async fn test_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["k"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick
            ˇbrown fˇox jumˇps"
        })
        .await;
    }

    #[gpui::test]
    async fn test_l(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["l"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuicˇk
            ˇbrowˇn"})
            .await;
    }

    #[gpui::test]
    async fn test_jump_to_line_boundaries(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches_all(
            ["$"],
            indoc! {"
            ˇThe qˇuicˇk
            ˇbrowˇn"},
        )
        .await;
        cx.assert_binding_matches_all(
            ["0"],
            indoc! {"
                ˇThe qˇuicˇk
                ˇbrowˇn"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_jump_to_end(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-g"]);

        cx.assert_all(indoc! {"
                The ˇquick

                brown fox jumps
                overˇ the lazy doˇg"})
            .await;
        cx.assert(indoc! {"
            The quiˇck

            brown"})
            .await;
        cx.assert(indoc! {"
            The quiˇck

            "})
            .await;
    }

    #[gpui::test]
    async fn test_w(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["w"]);
        cx.assert_all(indoc! {"
            The ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇe"})
            .await;
        let mut cx = cx.binding(["shift-w"]);
        cx.assert_all(indoc! {"
            The ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇe"})
            .await;
    }

    #[gpui::test]
    async fn test_end_of_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["e"]);
        cx.assert_all(indoc! {"
            Thˇe quicˇkˇ-browˇn


            fox_jumpˇs oveˇr
            thˇe"})
            .await;
        let mut cx = cx.binding(["shift-e"]);
        cx.assert_all(indoc! {"
            Thˇe quicˇkˇ-browˇn


            fox_jumpˇs oveˇr
            thˇe"})
            .await;
    }

    #[gpui::test]
    async fn test_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["b"]);
        cx.assert_all(indoc! {"
            ˇThe ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"})
            .await;
        let mut cx = cx.binding(["shift-b"]);
        cx.assert_all(indoc! {"
            ˇThe ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"})
            .await;
    }

    #[gpui::test]
    async fn test_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches_all(
            ["g", "g"],
            indoc! {"
                The qˇuick

                brown fox jumps
                over ˇthe laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["g", "g"],
            indoc! {"


                brown fox jumps
                over the laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["2", "g", "g"],
            indoc! {"
                ˇ

                brown fox jumps
                over the lazydog"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches_all(
            ["shift-g"],
            indoc! {"
                The qˇuick

                brown fox jumps
                over ˇthe laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["shift-g"],
            indoc! {"


                brown fox jumps
                over the laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["2", "shift-g"],
            indoc! {"
                ˇ

                brown fox jumps
                over the lazydog"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_a(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["a"]);
        cx.assert_all("The qˇuicˇk").await;
    }

    #[gpui::test]
    async fn test_insert_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-a"]);
        cx.assert_all(indoc! {"
            ˇ
            The qˇuick
            brown ˇfox "})
            .await;
    }

    #[gpui::test]
    async fn test_jump_to_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["^"]);
        cx.assert("The qˇuick").await;
        cx.assert(" The qˇuick").await;
        cx.assert("ˇ").await;
        cx.assert(indoc! {"
                The qˇuick
                brown fox"})
            .await;
        cx.assert(indoc! {"
                ˇ
                The quick"})
            .await;
        // Indoc disallows trailing whitespace.
        cx.assert("   ˇ \nThe quick").await;
    }

    #[gpui::test]
    async fn test_insert_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-i"]);
        cx.assert("The qˇuick").await;
        cx.assert(" The qˇuick").await;
        cx.assert("ˇ").await;
        cx.assert(indoc! {"
                The qˇuick
                brown fox"})
            .await;
        cx.assert(indoc! {"
                ˇ
                The quick"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_to_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-d"]);
        cx.assert(indoc! {"
                The qˇuick
                brown fox"})
            .await;
        cx.assert(indoc! {"
                The quick
                ˇ
                brown fox"})
            .await;
    }

    #[gpui::test]
    async fn test_x(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["x"]);
        cx.assert_all("ˇTeˇsˇt").await;
        cx.assert(indoc! {"
                Tesˇt
                test"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_left(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-x"]);
        cx.assert_all("ˇTˇeˇsˇt").await;
        cx.assert(indoc! {"
                Test
                ˇtest"})
            .await;
    }

    #[gpui::test]
    async fn test_o(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["o"]);
        cx.assert("ˇ").await;
        cx.assert("The ˇquick").await;
        cx.assert_all(indoc! {"
                The qˇuick
                brown ˇfox
                jumps ˇover"})
            .await;
        cx.assert(indoc! {"
                The quick
                ˇ
                brown fox"})
            .await;

        cx.assert_manual(
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

        cx.assert_manual(
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
        let cx = NeovimBackedTestContext::new(cx).await;
        let mut cx = cx.binding(["shift-o"]);
        cx.assert("ˇ").await;
        cx.assert("The ˇquick").await;
        cx.assert_all(indoc! {"
            The qˇuick
            brown ˇfox
            jumps ˇover"})
            .await;
        cx.assert(indoc! {"
            The quick
            ˇ
            brown fox"})
            .await;

        // Our indentation is smarter than vims. So we don't match here
        cx.assert_manual(
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
        cx.assert_manual(
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
        cx.assert_neovim_compatible("ˇ", ["d", "d"]).await;
        cx.assert_neovim_compatible("The ˇquick", ["d", "d"]).await;
        for marked_text in cx.each_marked_position(indoc! {"
            The qˇuick
            brown ˇfox
            jumps ˇover"})
        {
            cx.assert_neovim_compatible(&marked_text, ["d", "d"]).await;
        }
        cx.assert_neovim_compatible(
            indoc! {"
                The quick
                ˇ
                brown fox"},
            ["d", "d"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_cc(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "c"]);
        cx.assert("ˇ").await;
        cx.assert("The ˇquick").await;
        cx.assert_all(indoc! {"
                The quˇick
                brown ˇfox
                jumps ˇover"})
            .await;
        cx.assert(indoc! {"
                The quick
                ˇ
                brown fox"})
            .await;
    }

    #[gpui::test]
    async fn test_repeated_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for count in 1..=5 {
            cx.assert_binding_matches_all(
                [&count.to_string(), "w"],
                indoc! {"
                    ˇThe quˇickˇ browˇn
                    ˇ
                    ˇfox ˇjumpsˇ-ˇoˇver
                    ˇthe lazy dog
                "},
            )
            .await;
        }
    }

    #[gpui::test]
    async fn test_h_through_unicode(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["h"]);
        cx.assert_all("Testˇ├ˇ──ˇ┐ˇTest").await;
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

            cx.assert_binding_matches_all([&count.to_string(), "f", "b"], test_case)
                .await;

            cx.assert_binding_matches_all([&count.to_string(), "t", "b"], test_case)
                .await;
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
            cx.assert_binding_matches_all([&count.to_string(), "shift-f", "b"], test_case)
                .await;

            cx.assert_binding_matches_all([&count.to_string(), "shift-t", "b"], test_case)
                .await;
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
            ["f", "l"],
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
            ["t", "l"],
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
            ["shift-f", "p"],
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
            ["shift-t", "p"],
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
    async fn test_percent(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["%"]);
        cx.assert_all("ˇconsole.logˇ(ˇvaˇrˇ)ˇ;").await;
        cx.assert_all("ˇconsole.logˇ(ˇ'var', ˇ[ˇ1, ˇ2, 3ˇ]ˇ)ˇ;")
            .await;
        cx.assert_all("let result = curried_funˇ(ˇ)ˇ(ˇ)ˇ;").await;
    }
}
