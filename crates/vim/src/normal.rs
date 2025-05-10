mod change;
mod convert;
mod delete;
mod increment;
pub(crate) mod mark;
mod paste;
pub(crate) mod repeat;
mod scroll;
pub(crate) mod search;
pub mod substitute;
mod toggle_comments;
pub(crate) mod yank;

use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    Vim,
    indent::IndentDirection,
    motion::{self, Motion, first_non_whitespace, next_line_end, right},
    object::Object,
    state::{Mark, Mode, Operator},
    surrounds::SurroundsType,
};
use collections::BTreeSet;
use convert::ConvertTarget;
use editor::Anchor;
use editor::Bias;
use editor::Editor;
use editor::scroll::Autoscroll;
use editor::{display_map::ToDisplayPoint, movement};
use gpui::{Context, Window, actions};
use language::{Point, SelectionGoal, ToPoint};
use log::error;
use multi_buffer::MultiBufferRow;

actions!(
    vim,
    [
        InsertAfter,
        InsertBefore,
        InsertFirstNonWhitespace,
        InsertEndOfLine,
        InsertLineAbove,
        InsertLineBelow,
        InsertEmptyLineAbove,
        InsertEmptyLineBelow,
        InsertAtPrevious,
        JoinLines,
        JoinLinesNoWhitespace,
        DeleteLeft,
        DeleteRight,
        HelixDelete,
        ChangeToEndOfLine,
        DeleteToEndOfLine,
        Yank,
        YankLine,
        ChangeCase,
        ConvertToUpperCase,
        ConvertToLowerCase,
        ConvertToRot13,
        ConvertToRot47,
        ToggleComments,
        ShowLocation,
        Undo,
        Redo,
    ]
);

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::insert_after);
    Vim::action(editor, cx, Vim::insert_before);
    Vim::action(editor, cx, Vim::insert_first_non_whitespace);
    Vim::action(editor, cx, Vim::insert_end_of_line);
    Vim::action(editor, cx, Vim::insert_line_above);
    Vim::action(editor, cx, Vim::insert_line_below);
    Vim::action(editor, cx, Vim::insert_empty_line_above);
    Vim::action(editor, cx, Vim::insert_empty_line_below);
    Vim::action(editor, cx, Vim::insert_at_previous);
    Vim::action(editor, cx, Vim::change_case);
    Vim::action(editor, cx, Vim::convert_to_upper_case);
    Vim::action(editor, cx, Vim::convert_to_lower_case);
    Vim::action(editor, cx, Vim::convert_to_rot13);
    Vim::action(editor, cx, Vim::convert_to_rot47);
    Vim::action(editor, cx, Vim::yank_line);
    Vim::action(editor, cx, Vim::toggle_comments);
    Vim::action(editor, cx, Vim::paste);
    Vim::action(editor, cx, Vim::show_location);

    Vim::action(editor, cx, |vim, _: &DeleteLeft, window, cx| {
        vim.record_current_action(cx);
        let times = Vim::take_count(cx);
        let forced_motion = Vim::take_forced_motion(cx);
        vim.delete_motion(Motion::Left, times, forced_motion, window, cx);
    });
    Vim::action(editor, cx, |vim, _: &DeleteRight, window, cx| {
        vim.record_current_action(cx);
        let times = Vim::take_count(cx);
        let forced_motion = Vim::take_forced_motion(cx);
        vim.delete_motion(Motion::Right, times, forced_motion, window, cx);
    });

    Vim::action(editor, cx, |vim, _: &HelixDelete, window, cx| {
        vim.record_current_action(cx);
        vim.update_editor(window, cx, |_, editor, window, cx| {
            editor.change_selections(None, window, cx, |s| {
                s.move_with(|map, selection| {
                    if selection.is_empty() {
                        selection.end = movement::right(map, selection.end)
                    }
                })
            })
        });
        vim.visual_delete(false, window, cx);
    });

    Vim::action(editor, cx, |vim, _: &ChangeToEndOfLine, window, cx| {
        vim.start_recording(cx);
        let times = Vim::take_count(cx);
        let forced_motion = Vim::take_forced_motion(cx);
        vim.change_motion(
            Motion::EndOfLine {
                display_lines: false,
            },
            times,
            forced_motion,
            window,
            cx,
        );
    });
    Vim::action(editor, cx, |vim, _: &DeleteToEndOfLine, window, cx| {
        vim.record_current_action(cx);
        let times = Vim::take_count(cx);
        let forced_motion = Vim::take_forced_motion(cx);
        vim.delete_motion(
            Motion::EndOfLine {
                display_lines: false,
            },
            times,
            forced_motion,
            window,
            cx,
        );
    });
    Vim::action(editor, cx, |vim, _: &JoinLines, window, cx| {
        vim.join_lines_impl(true, window, cx);
    });

    Vim::action(editor, cx, |vim, _: &JoinLinesNoWhitespace, window, cx| {
        vim.join_lines_impl(false, window, cx);
    });

    Vim::action(editor, cx, |vim, _: &Undo, window, cx| {
        let times = Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        vim.update_editor(window, cx, |_, editor, window, cx| {
            for _ in 0..times.unwrap_or(1) {
                editor.undo(&editor::actions::Undo, window, cx);
            }
        });
    });
    Vim::action(editor, cx, |vim, _: &Redo, window, cx| {
        let times = Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        vim.update_editor(window, cx, |_, editor, window, cx| {
            for _ in 0..times.unwrap_or(1) {
                editor.redo(&editor::actions::Redo, window, cx);
            }
        });
    });

    repeat::register(editor, cx);
    scroll::register(editor, cx);
    search::register(editor, cx);
    substitute::register(editor, cx);
    increment::register(editor, cx);
}

impl Vim {
    pub fn normal_motion(
        &mut self,
        motion: Motion,
        operator: Option<Operator>,
        times: Option<usize>,
        forced_motion: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match operator {
            None => self.move_cursor(motion, times, window, cx),
            Some(Operator::Change) => self.change_motion(motion, times, forced_motion, window, cx),
            Some(Operator::Delete) => self.delete_motion(motion, times, forced_motion, window, cx),
            Some(Operator::Yank) => self.yank_motion(motion, times, forced_motion, window, cx),
            Some(Operator::AddSurrounds { target: None }) => {}
            Some(Operator::Indent) => self.indent_motion(
                motion,
                times,
                forced_motion,
                IndentDirection::In,
                window,
                cx,
            ),
            Some(Operator::Rewrap) => self.rewrap_motion(motion, times, forced_motion, window, cx),
            Some(Operator::Outdent) => self.indent_motion(
                motion,
                times,
                forced_motion,
                IndentDirection::Out,
                window,
                cx,
            ),
            Some(Operator::AutoIndent) => self.indent_motion(
                motion,
                times,
                forced_motion,
                IndentDirection::Auto,
                window,
                cx,
            ),
            Some(Operator::ShellCommand) => {
                self.shell_command_motion(motion, times, forced_motion, window, cx)
            }
            Some(Operator::Lowercase) => self.convert_motion(
                motion,
                times,
                forced_motion,
                ConvertTarget::LowerCase,
                window,
                cx,
            ),
            Some(Operator::Uppercase) => self.convert_motion(
                motion,
                times,
                forced_motion,
                ConvertTarget::UpperCase,
                window,
                cx,
            ),
            Some(Operator::OppositeCase) => self.convert_motion(
                motion,
                times,
                forced_motion,
                ConvertTarget::OppositeCase,
                window,
                cx,
            ),
            Some(Operator::Rot13) => self.convert_motion(
                motion,
                times,
                forced_motion,
                ConvertTarget::Rot13,
                window,
                cx,
            ),
            Some(Operator::Rot47) => self.convert_motion(
                motion,
                times,
                forced_motion,
                ConvertTarget::Rot47,
                window,
                cx,
            ),
            Some(Operator::ToggleComments) => {
                self.toggle_comments_motion(motion, times, forced_motion, window, cx)
            }
            Some(Operator::ReplaceWithRegister) => {
                self.replace_with_register_motion(motion, times, forced_motion, window, cx)
            }
            Some(Operator::Exchange) => {
                self.exchange_motion(motion, times, forced_motion, window, cx)
            }
            Some(operator) => {
                // Can't do anything for text objects, Ignoring
                error!("Unexpected normal mode motion operator: {:?}", operator)
            }
        }
        // Exit temporary normal mode (if active).
        self.exit_temporary_normal(window, cx);
    }

    pub fn normal_object(&mut self, object: Object, window: &mut Window, cx: &mut Context<Self>) {
        let mut waiting_operator: Option<Operator> = None;
        match self.maybe_pop_operator() {
            Some(Operator::Object { around }) => match self.maybe_pop_operator() {
                Some(Operator::Change) => self.change_object(object, around, window, cx),
                Some(Operator::Delete) => self.delete_object(object, around, window, cx),
                Some(Operator::Yank) => self.yank_object(object, around, window, cx),
                Some(Operator::Indent) => {
                    self.indent_object(object, around, IndentDirection::In, window, cx)
                }
                Some(Operator::Outdent) => {
                    self.indent_object(object, around, IndentDirection::Out, window, cx)
                }
                Some(Operator::AutoIndent) => {
                    self.indent_object(object, around, IndentDirection::Auto, window, cx)
                }
                Some(Operator::ShellCommand) => {
                    self.shell_command_object(object, around, window, cx);
                }
                Some(Operator::Rewrap) => self.rewrap_object(object, around, window, cx),
                Some(Operator::Lowercase) => {
                    self.convert_object(object, around, ConvertTarget::LowerCase, window, cx)
                }
                Some(Operator::Uppercase) => {
                    self.convert_object(object, around, ConvertTarget::UpperCase, window, cx)
                }
                Some(Operator::OppositeCase) => {
                    self.convert_object(object, around, ConvertTarget::OppositeCase, window, cx)
                }
                Some(Operator::Rot13) => {
                    self.convert_object(object, around, ConvertTarget::Rot13, window, cx)
                }
                Some(Operator::Rot47) => {
                    self.convert_object(object, around, ConvertTarget::Rot47, window, cx)
                }
                Some(Operator::AddSurrounds { target: None }) => {
                    waiting_operator = Some(Operator::AddSurrounds {
                        target: Some(SurroundsType::Object(object, around)),
                    });
                }
                Some(Operator::ToggleComments) => {
                    self.toggle_comments_object(object, around, window, cx)
                }
                Some(Operator::ReplaceWithRegister) => {
                    self.replace_with_register_object(object, around, window, cx)
                }
                Some(Operator::Exchange) => self.exchange_object(object, around, window, cx),
                _ => {
                    // Can't do anything for namespace operators. Ignoring
                }
            },
            Some(Operator::DeleteSurrounds) => {
                waiting_operator = Some(Operator::DeleteSurrounds);
            }
            Some(Operator::ChangeSurrounds { target: None }) => {
                if self.check_and_move_to_valid_bracket_pair(object, window, cx) {
                    waiting_operator = Some(Operator::ChangeSurrounds {
                        target: Some(object),
                    });
                }
            }
            _ => {
                // Can't do anything with change/delete/yank/surrounds and text objects. Ignoring
            }
        }
        self.clear_operator(window, cx);
        if let Some(operator) = waiting_operator {
            self.push_operator(operator, window, cx);
        }
    }

    pub(crate) fn move_cursor(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(window, cx, |_, editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window);

            match motion {
                Motion::Down {
                    display_lines: false,
                } => {
                    editor.change_selections_without_nav(
                        Some(Autoscroll::fit()),
                        window,
                        cx,
                        |s| {
                            s.move_cursors_with(|map, cursor, goal| {
                                motion
                                    .move_point(map, cursor, goal, times, &text_layout_details)
                                    .unwrap_or((cursor, goal))
                            })
                        },
                    );
                }
                Motion::Up {
                    display_lines: false,
                } => {
                    editor.change_selections_without_nav(
                        Some(Autoscroll::fit()),
                        window,
                        cx,
                        |s| {
                            s.move_cursors_with(|map, cursor, goal| {
                                motion
                                    .move_point(map, cursor, goal, times, &text_layout_details)
                                    .unwrap_or((cursor, goal))
                            })
                        },
                    );
                }
                _ => {
                    editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                        s.move_cursors_with(|map, cursor, goal| {
                            motion
                                .move_point(map, cursor, goal, times, &text_layout_details)
                                .unwrap_or((cursor, goal))
                        })
                    });
                }
            }
        });
    }

    fn insert_after(&mut self, _: &InsertAfter, window: &mut Window, cx: &mut Context<Self>) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                s.move_cursors_with(|map, cursor, _| (right(map, cursor, 1), SelectionGoal::None));
            });
        });
    }

    fn insert_before(&mut self, _: &InsertBefore, window: &mut Window, cx: &mut Context<Self>) {
        self.start_recording(cx);
        if self.mode.is_visual() {
            let current_mode = self.mode;
            self.update_editor(window, cx, |_, editor, window, cx| {
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.move_with(|map, selection| {
                        if current_mode == Mode::VisualLine {
                            let start_of_line = motion::start_of_line(map, false, selection.start);
                            selection.collapse_to(start_of_line, SelectionGoal::None)
                        } else {
                            selection.collapse_to(selection.start, SelectionGoal::None)
                        }
                    });
                });
            });
        }
        self.switch_mode(Mode::Insert, false, window, cx);
    }

    fn insert_first_non_whitespace(
        &mut self,
        _: &InsertFirstNonWhitespace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                s.move_cursors_with(|map, cursor, _| {
                    (
                        first_non_whitespace(map, false, cursor),
                        SelectionGoal::None,
                    )
                });
            });
        });
    }

    fn insert_end_of_line(
        &mut self,
        _: &InsertEndOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                s.move_cursors_with(|map, cursor, _| {
                    (next_line_end(map, cursor, 1), SelectionGoal::None)
                });
            });
        });
    }

    fn insert_at_previous(
        &mut self,
        _: &InsertAtPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            let Some(Mark::Local(marks)) = vim.get_mark("^", editor, window, cx) else {
                return;
            };

            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                s.select_anchor_ranges(marks.iter().map(|mark| *mark..*mark))
            });
        });
    }

    fn insert_line_above(
        &mut self,
        _: &InsertLineAbove,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let selections = editor.selections.all::<Point>(cx);
                let snapshot = editor.buffer().read(cx).snapshot(cx);

                let selection_start_rows: BTreeSet<u32> = selections
                    .into_iter()
                    .map(|selection| selection.start.row)
                    .collect();
                let edits = selection_start_rows
                    .into_iter()
                    .map(|row| {
                        let indent = snapshot
                            .indent_and_comment_for_line(MultiBufferRow(row), cx)
                            .chars()
                            .collect::<String>();

                        let start_of_line = Point::new(row, 0);
                        (start_of_line..start_of_line, indent + "\n")
                    })
                    .collect::<Vec<_>>();
                editor.edit_with_autoindent(edits, cx);
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.move_cursors_with(|map, cursor, _| {
                        let previous_line = motion::start_of_relative_buffer_row(map, cursor, -1);
                        let insert_point = motion::end_of_line(map, false, previous_line, 1);
                        (insert_point, SelectionGoal::None)
                    });
                });
            });
        });
    }

    fn insert_line_below(
        &mut self,
        _: &InsertLineBelow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.transact(window, cx, |editor, window, cx| {
                let selections = editor.selections.all::<Point>(cx);
                let snapshot = editor.buffer().read(cx).snapshot(cx);

                let selection_end_rows: BTreeSet<u32> = selections
                    .into_iter()
                    .map(|selection| selection.end.row)
                    .collect();
                let edits = selection_end_rows
                    .into_iter()
                    .map(|row| {
                        let indent = snapshot
                            .indent_and_comment_for_line(MultiBufferRow(row), cx)
                            .chars()
                            .collect::<String>();

                        let end_of_line = Point::new(row, snapshot.line_len(MultiBufferRow(row)));
                        (end_of_line..end_of_line, "\n".to_string() + &indent)
                    })
                    .collect::<Vec<_>>();
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
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
    }

    fn insert_empty_line_above(
        &mut self,
        _: &InsertEmptyLineAbove,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.record_current_action(cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.transact(window, cx, |editor, _, cx| {
                let selections = editor.selections.all::<Point>(cx);

                let selection_start_rows: BTreeSet<u32> = selections
                    .into_iter()
                    .map(|selection| selection.start.row)
                    .collect();
                let edits = selection_start_rows
                    .into_iter()
                    .map(|row| {
                        let start_of_line = Point::new(row, 0);
                        (start_of_line..start_of_line, "\n".to_string())
                    })
                    .collect::<Vec<_>>();
                editor.edit(edits, cx);
            });
        });
    }

    fn insert_empty_line_below(
        &mut self,
        _: &InsertEmptyLineBelow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.record_current_action(cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.transact(window, cx, |editor, _, cx| {
                let selections = editor.selections.all::<Point>(cx);
                let snapshot = editor.buffer().read(cx).snapshot(cx);

                let selection_end_rows: BTreeSet<u32> = selections
                    .into_iter()
                    .map(|selection| selection.end.row)
                    .collect();
                let edits = selection_end_rows
                    .into_iter()
                    .map(|row| {
                        let end_of_line = Point::new(row, snapshot.line_len(MultiBufferRow(row)));
                        (end_of_line..end_of_line, "\n".to_string())
                    })
                    .collect::<Vec<_>>();
                editor.edit(edits, cx);
            });
        });
    }

    fn join_lines_impl(
        &mut self,
        insert_whitespace: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.record_current_action(cx);
        let mut times = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        if self.mode.is_visual() {
            times = 1;
        } else if times > 1 {
            // 2J joins two lines together (same as J or 1J)
            times -= 1;
        }

        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                for _ in 0..times {
                    editor.join_lines_impl(insert_whitespace, window, cx)
                }
            })
        });
        if self.mode.is_visual() {
            self.switch_mode(Mode::Normal, true, window, cx)
        }
    }

    fn yank_line(&mut self, _: &YankLine, window: &mut Window, cx: &mut Context<Self>) {
        let count = Vim::take_count(cx);
        let forced_motion = Vim::take_forced_motion(cx);
        self.yank_motion(
            motion::Motion::CurrentLine,
            count,
            forced_motion,
            window,
            cx,
        )
    }

    fn show_location(&mut self, _: &ShowLocation, window: &mut Window, cx: &mut Context<Self>) {
        let count = Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        self.update_editor(window, cx, |vim, editor, _window, cx| {
            let selection = editor.selections.newest_anchor();
            if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
                let filename = if let Some(file) = buffer.read(cx).file() {
                    if count.is_some() {
                        if let Some(local) = file.as_local() {
                            local.abs_path(cx).to_string_lossy().to_string()
                        } else {
                            file.full_path(cx).to_string_lossy().to_string()
                        }
                    } else {
                        file.path().to_string_lossy().to_string()
                    }
                } else {
                    "[No Name]".into()
                };
                let buffer = buffer.read(cx);
                let snapshot = buffer.snapshot();
                let lines = buffer.max_point().row + 1;
                let current_line = selection.head().text_anchor.to_point(&snapshot).row;
                let percentage = current_line as f32 / lines as f32;
                let modified = if buffer.is_dirty() { " [modified]" } else { "" };
                vim.status_label = Some(
                    format!(
                        "{}{} {} lines --{:.0}%--",
                        filename,
                        modified,
                        lines,
                        percentage * 100.0,
                    )
                    .into(),
                );
                cx.notify();
            }
        });
    }

    fn toggle_comments(&mut self, _: &ToggleComments, window: &mut Window, cx: &mut Context<Self>) {
        self.record_current_action(cx);
        self.store_visual_marks(window, cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let original_positions = vim.save_selection_starts(editor, cx);
                editor.toggle_comments(&Default::default(), window, cx);
                vim.restore_selection_cursors(editor, window, cx, original_positions);
            });
        });
        if self.mode.is_visual() {
            self.switch_mode(Mode::Normal, true, window, cx)
        }
    }

    pub(crate) fn normal_replace(
        &mut self,
        text: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_return_char = text == "\n".into() || text == "\r".into();
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        self.stop_recording(cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let (map, display_selections) = editor.selections.all_display(cx);

                let mut edits = Vec::new();
                for selection in &display_selections {
                    let mut range = selection.range();
                    for _ in 0..count {
                        let new_point = movement::saturating_right(&map, range.end);
                        if range.end == new_point {
                            return;
                        }
                        range.end = new_point;
                    }

                    edits.push((
                        range.start.to_offset(&map, Bias::Left)
                            ..range.end.to_offset(&map, Bias::Left),
                        text.repeat(if is_return_char { 0 } else { count }),
                    ));
                }

                editor.edit(edits, cx);
                if is_return_char {
                    editor.newline(&editor::actions::Newline, window, cx);
                }
                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(None, window, cx, |s| {
                    s.move_with(|map, selection| {
                        let point = movement::saturating_left(map, selection.head());
                        selection.collapse_to(point, SelectionGoal::None)
                    });
                });
            });
        });
        self.pop_operator(window, cx);
    }

    pub fn save_selection_starts(
        &self,
        editor: &Editor,

        cx: &mut Context<Editor>,
    ) -> HashMap<usize, Anchor> {
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

    pub fn restore_selection_cursors(
        &self,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
        mut positions: HashMap<usize, Anchor>,
    ) {
        editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
            s.move_with(|map, selection| {
                if let Some(anchor) = positions.remove(&selection.id) {
                    selection.collapse_to(anchor.to_display_point(map), SelectionGoal::None);
                }
            });
        });
    }

    fn exit_temporary_normal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.temp_mode {
            self.switch_mode(Mode::Insert, true, window, cx);
        }
    }
}
#[cfg(test)]
mod test {
    use gpui::{KeyBinding, TestAppContext, UpdateGlobal};
    use indoc::indoc;
    use language::language_settings::AllLanguageSettings;
    use settings::SettingsStore;

    use crate::{
        VimSettings, motion,
        state::Mode::{self},
        test::{NeovimBackedTestContext, VimTestContext},
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
    async fn test_insert_empty_line_above(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("[ space", "ˇ").await.assert_matches();
        cx.simulate("[ space", "The ˇquick").await.assert_matches();
        cx.simulate_at_each_offset(
            "[ space",
            indoc! {"
            The qˇuick
            brown ˇfox
            jumps ˇover"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "[ space",
            indoc! {"
            The quick
            ˇ
            brown fox"},
        )
        .await
        .assert_matches();
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
        cx.update(|_, cx| {
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

    #[gpui::test]
    async fn test_r(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("r -").await;
        cx.shared_state().await.assert_eq("ˇ-ello\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 r -").await;
        cx.shared_state().await.assert_eq("--ˇ-lo\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("r - 2 l .").await;
        cx.shared_state().await.assert_eq("-eˇ-lo\n");

        cx.set_shared_state("ˇhello world\n").await;
        cx.simulate_shared_keystrokes("2 r - f w .").await;
        cx.shared_state().await.assert_eq("--llo -ˇ-rld\n");

        cx.set_shared_state("ˇhello world\n").await;
        cx.simulate_shared_keystrokes("2 0 r - ").await;
        cx.shared_state().await.assert_eq("ˇhello world\n");

        cx.set_shared_state("  helloˇ world\n").await;
        cx.simulate_shared_keystrokes("r enter").await;
        cx.shared_state().await.assert_eq("  hello\n ˇ world\n");

        cx.set_shared_state("  helloˇ world\n").await;
        cx.simulate_shared_keystrokes("2 r enter").await;
        cx.shared_state().await.assert_eq("  hello\n ˇ orld\n");
    }

    #[gpui::test]
    async fn test_gq(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_neovim_option("textwidth=5").await;

        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                    settings.defaults.preferred_line_length = Some(5);
                });
            })
        });

        cx.set_shared_state("ˇth th th th th th\n").await;
        cx.simulate_shared_keystrokes("g q q").await;
        cx.shared_state().await.assert_eq("th th\nth th\nˇth th\n");

        cx.set_shared_state("ˇth th th th th th\nth th th th th th\n")
            .await;
        cx.simulate_shared_keystrokes("v j g q").await;
        cx.shared_state()
            .await
            .assert_eq("th th\nth th\nth th\nth th\nth th\nˇth th\n");
    }

    #[gpui::test]
    async fn test_o_comment(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_neovim_option("filetype=rust").await;

        cx.set_shared_state("// helloˇ\n").await;
        cx.simulate_shared_keystrokes("o").await;
        cx.shared_state().await.assert_eq("// hello\n// ˇ\n");
        cx.simulate_shared_keystrokes("x escape shift-o").await;
        cx.shared_state().await.assert_eq("// hello\n// ˇ\n// x\n");
    }

    #[gpui::test]
    async fn test_yank_line_with_trailing_newline(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("heˇllo\n").await;
        cx.simulate_shared_keystrokes("y y p").await;
        cx.shared_state().await.assert_eq("hello\nˇhello\n");
    }

    #[gpui::test]
    async fn test_yank_line_without_trailing_newline(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("heˇllo").await;
        cx.simulate_shared_keystrokes("y y p").await;
        cx.shared_state().await.assert_eq("hello\nˇhello");
    }

    #[gpui::test]
    async fn test_yank_multiline_without_trailing_newline(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("heˇllo\nhello").await;
        cx.simulate_shared_keystrokes("2 y y p").await;
        cx.shared_state()
            .await
            .assert_eq("hello\nˇhello\nhello\nhello");
    }

    #[gpui::test]
    async fn test_dd_then_paste_without_trailing_newline(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("heˇllo").await;
        cx.simulate_shared_keystrokes("d d").await;
        cx.shared_state().await.assert_eq("ˇ");
        cx.simulate_shared_keystrokes("p p").await;
        cx.shared_state().await.assert_eq("\nhello\nˇhello");
    }

    #[gpui::test]
    async fn test_visual_mode_insert_before_after(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("heˇllo").await;
        cx.simulate_shared_keystrokes("v i w shift-i").await;
        cx.shared_state().await.assert_eq("ˇhello");

        cx.set_shared_state(indoc! {"
            The quick brown
            fox ˇjumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("shift-v shift-i").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            ˇfox jumps over
            the lazy dog"});

        cx.set_shared_state(indoc! {"
            The quick brown
            fox ˇjumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("shift-v shift-a").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            fox jˇumps over
            the lazy dog"});
    }
}
