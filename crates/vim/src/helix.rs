mod boundary;
mod duplicate;
mod object;
mod paste;
mod select;
mod surround;

use editor::display_map::DisplaySnapshot;
use editor::{
    DisplayPoint, Editor, EditorSettings, HideMouseCursorOrigin, MultiBufferOffset,
    SelectionEffects, ToOffset, ToPoint, movement,
};
use gpui::actions;
use gpui::{Context, Window};
use language::{CharClassifier, CharKind, Point};
use search::{BufferSearchBar, SearchOptions};
use settings::Settings;
use text::{Bias, SelectionGoal};
use workspace::searchable::FilteredSearchRange;
use workspace::searchable::{self, Direction};

use crate::motion::{self, MotionKind};
use crate::state::{Operator, SearchState};
use crate::{
    PushHelixSurroundAdd, PushHelixSurroundDelete, PushHelixSurroundReplace, Vim,
    motion::{Motion, right},
    state::Mode,
};

actions!(
    vim,
    [
        /// Yanks the current selection or character if no selection.
        HelixYank,
        /// Inserts at the beginning of the selection.
        HelixInsert,
        /// Appends at the end of the selection.
        HelixAppend,
        /// Goes to the location of the last modification.
        HelixGotoLastModification,
        /// Select entire line or multiple lines, extending downwards.
        HelixSelectLine,
        /// Select all matches of a given pattern within the current selection.
        HelixSelectRegex,
        /// Removes all but the one selection that was created last.
        /// `Newest` can eventually be `Primary`.
        HelixKeepNewestSelection,
        /// Copies all selections below.
        HelixDuplicateBelow,
        /// Copies all selections above.
        HelixDuplicateAbove,
        /// Delete the selection and enter edit mode.
        HelixSubstitute,
        /// Delete the selection and enter edit mode, without yanking the selection.
        HelixSubstituteNoYank,
        /// Select the next match for the current search query.
        HelixSelectNext,
        /// Select the previous match for the current search query.
        HelixSelectPrevious,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_select_lines);
    Vim::action(editor, cx, Vim::helix_insert);
    Vim::action(editor, cx, Vim::helix_append);
    Vim::action(editor, cx, Vim::helix_yank);
    Vim::action(editor, cx, Vim::helix_goto_last_modification);
    Vim::action(editor, cx, Vim::helix_paste);
    Vim::action(editor, cx, Vim::helix_select_regex);
    Vim::action(editor, cx, Vim::helix_keep_newest_selection);
    Vim::action(editor, cx, |vim, _: &HelixDuplicateBelow, window, cx| {
        let times = Vim::take_count(cx);
        vim.helix_duplicate_selections_below(times, window, cx);
    });
    Vim::action(editor, cx, |vim, _: &HelixDuplicateAbove, window, cx| {
        let times = Vim::take_count(cx);
        vim.helix_duplicate_selections_above(times, window, cx);
    });
    Vim::action(editor, cx, Vim::helix_substitute);
    Vim::action(editor, cx, Vim::helix_substitute_no_yank);
    Vim::action(editor, cx, Vim::helix_select_next);
    Vim::action(editor, cx, Vim::helix_select_previous);
    Vim::action(editor, cx, |vim, _: &PushHelixSurroundAdd, window, cx| {
        vim.clear_operator(window, cx);
        vim.push_operator(Operator::HelixSurroundAdd, window, cx);
    });
    Vim::action(
        editor,
        cx,
        |vim, _: &PushHelixSurroundReplace, window, cx| {
            vim.clear_operator(window, cx);
            vim.push_operator(
                Operator::HelixSurroundReplace {
                    replaced_char: None,
                },
                window,
                cx,
            );
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, _: &PushHelixSurroundDelete, window, cx| {
            vim.clear_operator(window, cx);
            vim.push_operator(Operator::HelixSurroundDelete, window, cx);
        },
    );
}

impl Vim {
    pub fn helix_normal_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.helix_move_cursor(motion, times, window, cx);
    }

    pub fn helix_select_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window, cx);
            editor.change_selections(Default::default(), window, cx, |s| {
                if let Motion::ZedSearchResult { new_selections, .. } = &motion {
                    s.select_anchor_ranges(new_selections.clone());
                    return;
                };

                s.move_with(|map, selection| {
                    let was_reversed = selection.reversed;
                    let mut current_head = selection.head();

                    // our motions assume the current character is after the cursor,
                    // but in (forward) visual mode the current character is just
                    // before the end of the selection.

                    // If the file ends with a newline (which is common) we don't do this.
                    // so that if you go to the end of such a file you can use "up" to go
                    // to the previous line and have it work somewhat as expected.
                    if !selection.reversed
                        && !selection.is_empty()
                        && !(selection.end.column() == 0 && selection.end == map.max_point())
                    {
                        current_head = movement::left(map, selection.end)
                    }

                    let (new_head, goal) = match motion {
                        // Going to next word start is special cased
                        // since Vim differs from Helix in that motion
                        // Vim: `w` goes to the first character of a word
                        // Helix: `w` goes to the character before a word
                        Motion::NextWordStart { ignore_punctuation } => {
                            let mut head = movement::right(map, current_head);
                            let classifier =
                                map.buffer_snapshot().char_classifier_at(head.to_point(map));
                            for _ in 0..times.unwrap_or(1) {
                                let (_, new_head) =
                                    movement::find_boundary_trail(map, head, |left, right| {
                                        Self::is_boundary_right(ignore_punctuation)(
                                            left,
                                            right,
                                            &classifier,
                                        )
                                    });
                                head = new_head;
                            }
                            head = movement::left(map, head);
                            (head, SelectionGoal::None)
                        }
                        _ => motion
                            .move_point(
                                map,
                                current_head,
                                selection.goal,
                                times,
                                &text_layout_details,
                            )
                            .unwrap_or((current_head, selection.goal)),
                    };

                    selection.set_head(new_head, goal);

                    // ensure the current character is included in the selection.
                    if !selection.reversed {
                        let next_point = movement::right(map, selection.end);

                        if !(next_point.column() == 0 && next_point == map.max_point()) {
                            selection.end = next_point;
                        }
                    }

                    // vim always ensures the anchor character stays selected.
                    // if our selection has reversed, we need to move the opposite end
                    // to ensure the anchor is still selected.
                    if was_reversed && !selection.reversed {
                        selection.start = movement::left(map, selection.start);
                    } else if !was_reversed && selection.reversed {
                        selection.end = movement::right(map, selection.end);
                    }
                })
            });
        });
    }

    /// Updates all selections based on where the cursors are.
    fn helix_new_selections(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut change: impl FnMut(
            // the start of the cursor
            DisplayPoint,
            &DisplaySnapshot,
        ) -> Option<(DisplayPoint, DisplayPoint)>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let cursor_start = if selection.reversed || selection.is_empty() {
                        selection.head()
                    } else {
                        movement::left(map, selection.head())
                    };
                    let Some((head, tail)) = change(cursor_start, map) else {
                        return;
                    };

                    selection.set_head_tail(head, tail, SelectionGoal::None);
                });
            });
        });
    }

    fn helix_find_range_forward(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut is_boundary: impl FnMut(char, char, &CharClassifier) -> bool,
    ) {
        let times = times.unwrap_or(1);
        self.helix_new_selections(window, cx, |cursor, map| {
            let mut head = movement::right(map, cursor);
            let mut tail = cursor;
            let classifier = map.buffer_snapshot().char_classifier_at(head.to_point(map));
            if head == map.max_point() {
                return None;
            }
            for _ in 0..times {
                let (maybe_next_tail, next_head) =
                    movement::find_boundary_trail(map, head, |left, right| {
                        is_boundary(left, right, &classifier)
                    });

                if next_head == head && maybe_next_tail.unwrap_or(next_head) == tail {
                    break;
                }

                head = next_head;
                if let Some(next_tail) = maybe_next_tail {
                    tail = next_tail;
                }
            }
            Some((head, tail))
        });
    }

    fn helix_find_range_backward(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut is_boundary: impl FnMut(char, char, &CharClassifier) -> bool,
    ) {
        let times = times.unwrap_or(1);
        self.helix_new_selections(window, cx, |cursor, map| {
            let mut head = cursor;
            // The original cursor was one character wide,
            // but the search starts from the left side of it,
            // so to include that space the selection must end one character to the right.
            let mut tail = movement::right(map, cursor);
            let classifier = map.buffer_snapshot().char_classifier_at(head.to_point(map));
            if head == DisplayPoint::zero() {
                return None;
            }
            for _ in 0..times {
                let (maybe_next_tail, next_head) =
                    movement::find_preceding_boundary_trail(map, head, |left, right| {
                        is_boundary(left, right, &classifier)
                    });

                if next_head == head && maybe_next_tail.unwrap_or(next_head) == tail {
                    break;
                }

                head = next_head;
                if let Some(next_tail) = maybe_next_tail {
                    tail = next_tail;
                }
            }
            Some((head, tail))
        });
    }

    pub fn helix_move_and_collapse(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window, cx);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let goal = selection.goal;
                    let cursor = if selection.is_empty() || selection.reversed {
                        selection.head()
                    } else {
                        movement::left(map, selection.head())
                    };

                    let (point, goal) = motion
                        .move_point(map, cursor, selection.goal, times, &text_layout_details)
                        .unwrap_or((cursor, goal));

                    selection.collapse_to(point, goal)
                })
            });
        });
    }

    fn is_boundary_right(
        ignore_punctuation: bool,
    ) -> impl FnMut(char, char, &CharClassifier) -> bool {
        move |left, right, classifier| {
            let left_kind = classifier.kind_with(left, ignore_punctuation);
            let right_kind = classifier.kind_with(right, ignore_punctuation);
            let at_newline = (left == '\n') ^ (right == '\n');

            (left_kind != right_kind && right_kind != CharKind::Whitespace) || at_newline
        }
    }

    fn is_boundary_left(
        ignore_punctuation: bool,
    ) -> impl FnMut(char, char, &CharClassifier) -> bool {
        move |left, right, classifier| {
            let left_kind = classifier.kind_with(left, ignore_punctuation);
            let right_kind = classifier.kind_with(right, ignore_punctuation);
            let at_newline = (left == '\n') ^ (right == '\n');

            (left_kind != right_kind && left_kind != CharKind::Whitespace) || at_newline
        }
    }

    pub fn helix_move_cursor(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match motion {
            Motion::NextWordStart { ignore_punctuation } => self.helix_find_range_forward(
                times,
                window,
                cx,
                Self::is_boundary_right(ignore_punctuation),
            ),
            Motion::NextWordEnd { ignore_punctuation } => self.helix_find_range_forward(
                times,
                window,
                cx,
                Self::is_boundary_left(ignore_punctuation),
            ),
            Motion::PreviousWordStart { ignore_punctuation } => self.helix_find_range_backward(
                times,
                window,
                cx,
                Self::is_boundary_left(ignore_punctuation),
            ),
            Motion::PreviousWordEnd { ignore_punctuation } => self.helix_find_range_backward(
                times,
                window,
                cx,
                Self::is_boundary_right(ignore_punctuation),
            ),
            Motion::EndOfLine { .. } => {
                // In Helix mode, EndOfLine should position cursor ON the last character,
                // not after it. We therefore need special handling for it.
                self.update_editor(cx, |_, editor, cx| {
                    let text_layout_details = editor.text_layout_details(window, cx);
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.move_with(|map, selection| {
                            let goal = selection.goal;
                            let cursor = if selection.is_empty() || selection.reversed {
                                selection.head()
                            } else {
                                movement::left(map, selection.head())
                            };

                            let (point, _goal) = motion
                                .move_point(map, cursor, goal, times, &text_layout_details)
                                .unwrap_or((cursor, goal));

                            // Move left by one character to position on the last character
                            let adjusted_point = movement::saturating_left(map, point);
                            selection.collapse_to(adjusted_point, SelectionGoal::None)
                        })
                    });
                });
            }
            Motion::FindForward {
                before,
                char,
                mode,
                smartcase,
            } => {
                self.helix_new_selections(window, cx, |cursor, map| {
                    let start = cursor;
                    let mut last_boundary = start;
                    for _ in 0..times.unwrap_or(1) {
                        last_boundary = movement::find_boundary(
                            map,
                            movement::right(map, last_boundary),
                            mode,
                            |left, right| {
                                let current_char = if before { right } else { left };
                                motion::is_character_match(char, current_char, smartcase)
                            },
                        );
                    }
                    Some((last_boundary, start))
                });
            }
            Motion::FindBackward {
                after,
                char,
                mode,
                smartcase,
            } => {
                self.helix_new_selections(window, cx, |cursor, map| {
                    let start = cursor;
                    let mut last_boundary = start;
                    for _ in 0..times.unwrap_or(1) {
                        last_boundary = movement::find_preceding_boundary_display_point(
                            map,
                            last_boundary,
                            mode,
                            |left, right| {
                                let current_char = if after { left } else { right };
                                motion::is_character_match(char, current_char, smartcase)
                            },
                        );
                    }
                    // The original cursor was one character wide,
                    // but the search started from the left side of it,
                    // so to include that space the selection must end one character to the right.
                    Some((last_boundary, movement::right(map, start)))
                });
            }
            _ => self.helix_move_and_collapse(motion, times, window, cx),
        }
    }

    pub fn helix_yank(&mut self, _: &HelixYank, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |vim, editor, cx| {
            let has_selection = editor
                .selections
                .all_adjusted(&editor.display_snapshot(cx))
                .iter()
                .any(|selection| !selection.is_empty());

            if !has_selection {
                // If no selection, expand to current character (like 'v' does)
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let head = selection.head();
                        let new_head = movement::saturating_right(map, head);
                        selection.set_tail(head, SelectionGoal::None);
                        selection.set_head(new_head, SelectionGoal::None);
                    });
                });
                vim.yank_selections_content(
                    editor,
                    crate::motion::MotionKind::Exclusive,
                    window,
                    cx,
                );
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.move_with(|_map, selection| {
                        selection.collapse_to(selection.start, SelectionGoal::None);
                    });
                });
            } else {
                // Yank the selection(s)
                vim.yank_selections_content(
                    editor,
                    crate::motion::MotionKind::Exclusive,
                    window,
                    cx,
                );
            }
        });

        // Drop back to normal mode after yanking
        self.switch_mode(Mode::HelixNormal, true, window, cx);
    }

    fn helix_insert(&mut self, _: &HelixInsert, window: &mut Window, cx: &mut Context<Self>) {
        self.start_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|_map, selection| {
                    // In helix normal mode, move cursor to start of selection and collapse
                    if !selection.is_empty() {
                        selection.collapse_to(selection.start, SelectionGoal::None);
                    }
                });
            });
        });
        self.switch_mode(Mode::Insert, false, window, cx);
    }

    fn helix_select_regex(
        &mut self,
        _: &HelixSelectRegex,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        Vim::take_forced_motion(cx);
        let Some(pane) = self.pane(window, cx) else {
            return;
        };
        let prior_selections = self.editor_selections(window, cx);
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                search_bar.update(cx, |search_bar, cx| {
                    if !search_bar.show(window, cx) {
                        return;
                    }

                    search_bar.select_query(window, cx);
                    cx.focus_self(window);

                    search_bar.set_replacement(None, cx);
                    let mut options = SearchOptions::NONE;
                    options |= SearchOptions::REGEX;
                    if EditorSettings::get_global(cx).search.case_sensitive {
                        options |= SearchOptions::CASE_SENSITIVE;
                    }
                    search_bar.set_search_options(options, cx);
                    if let Some(search) = search_bar.set_search_within_selection(
                        Some(FilteredSearchRange::Selection),
                        window,
                        cx,
                    ) {
                        cx.spawn_in(window, async move |search_bar, cx| {
                            if search.await.is_ok() {
                                search_bar.update_in(cx, |search_bar, window, cx| {
                                    search_bar.activate_current_match(window, cx)
                                })
                            } else {
                                Ok(())
                            }
                        })
                        .detach_and_log_err(cx);
                    }
                    self.search = SearchState {
                        direction: searchable::Direction::Next,
                        count: 1,
                        prior_selections,
                        prior_operator: self.operator_stack.last().cloned(),
                        prior_mode: self.mode,
                        helix_select: true,
                        _dismiss_subscription: None,
                    }
                });
            }
        });
        self.start_recording(cx);
    }

    fn helix_append(&mut self, _: &HelixAppend, window: &mut Window, cx: &mut Context<Self>) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let point = if selection.is_empty() {
                        right(map, selection.head(), 1)
                    } else {
                        selection.end
                    };
                    selection.collapse_to(point, SelectionGoal::None);
                });
            });
        });
    }

    pub fn helix_replace(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let display_map = editor.display_snapshot(cx);
                let selections = editor.selections.all_display(&display_map);

                // Store selection info for positioning after edit
                let selection_info: Vec<_> = selections
                    .iter()
                    .map(|selection| {
                        let range = selection.range();
                        let start_offset = range.start.to_offset(&display_map, Bias::Left);
                        let end_offset = range.end.to_offset(&display_map, Bias::Left);
                        let was_empty = range.is_empty();
                        let was_reversed = selection.reversed;
                        (
                            display_map.buffer_snapshot().anchor_before(start_offset),
                            end_offset - start_offset,
                            was_empty,
                            was_reversed,
                        )
                    })
                    .collect();

                let mut edits = Vec::new();
                for selection in &selections {
                    let mut range = selection.range();

                    // For empty selections, extend to replace one character
                    if range.is_empty() {
                        range.end = movement::saturating_right(&display_map, range.start);
                    }

                    let byte_range = range.start.to_offset(&display_map, Bias::Left)
                        ..range.end.to_offset(&display_map, Bias::Left);

                    if !byte_range.is_empty() {
                        let replacement_text = text.repeat(byte_range.end - byte_range.start);
                        edits.push((byte_range, replacement_text));
                    }
                }

                editor.edit(edits, cx);

                // Restore selections based on original info
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                let ranges: Vec<_> = selection_info
                    .into_iter()
                    .map(|(start_anchor, original_len, was_empty, was_reversed)| {
                        let start_point = start_anchor.to_point(&snapshot);
                        if was_empty {
                            // For cursor-only, collapse to start
                            start_point..start_point
                        } else {
                            // For selections, span the replaced text
                            let replacement_len = text.len() * original_len;
                            let end_offset = start_anchor.to_offset(&snapshot) + replacement_len;
                            let end_point = snapshot.offset_to_point(end_offset);
                            if was_reversed {
                                end_point..start_point
                            } else {
                                start_point..end_point
                            }
                        }
                    })
                    .collect();

                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges(ranges);
                });
            });
        });
        self.switch_mode(Mode::HelixNormal, true, window, cx);
    }

    pub fn helix_goto_last_modification(
        &mut self,
        _: &HelixGotoLastModification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.jump(".".into(), false, false, window, cx);
    }

    pub fn helix_select_lines(
        &mut self,
        _: &HelixSelectLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = Vim::take_count(cx).unwrap_or(1);
        self.update_editor(cx, |_, editor, cx| {
            editor.hide_mouse_cursor(HideMouseCursorOrigin::MovementAction, cx);
            let display_map = editor.display_map.update(cx, |map, cx| map.snapshot(cx));
            let mut selections = editor.selections.all::<Point>(&display_map);
            let max_point = display_map.buffer_snapshot().max_point();
            let buffer_snapshot = &display_map.buffer_snapshot();

            for selection in &mut selections {
                // Start always goes to column 0 of the first selected line
                let start_row = selection.start.row;
                let current_end_row = selection.end.row;

                // Check if cursor is on empty line by checking first character
                let line_start_offset = buffer_snapshot.point_to_offset(Point::new(start_row, 0));
                let first_char = buffer_snapshot.chars_at(line_start_offset).next();
                let extra_line = if first_char == Some('\n') { 1 } else { 0 };

                let end_row = current_end_row + count as u32 + extra_line;

                selection.start = Point::new(start_row, 0);
                selection.end = if end_row > max_point.row {
                    max_point
                } else {
                    Point::new(end_row, 0)
                };
                selection.reversed = false;
            }

            editor.change_selections(Default::default(), window, cx, |s| {
                s.select(selections);
            });
        });
    }

    fn helix_keep_newest_selection(
        &mut self,
        _: &HelixKeepNewestSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            let newest = editor
                .selections
                .newest::<MultiBufferOffset>(&editor.display_snapshot(cx));
            editor.change_selections(Default::default(), window, cx, |s| s.select(vec![newest]));
        });
    }

    fn do_helix_substitute(&mut self, yank: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |vim, editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            editor.transact(window, cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        if selection.start == selection.end {
                            selection.end = movement::right(map, selection.end);
                        }

                        // If the selection starts and ends on a newline, we exclude the last one.
                        if !selection.is_empty()
                            && selection.start.column() == 0
                            && selection.end.column() == 0
                        {
                            selection.end = movement::left(map, selection.end);
                        }
                    })
                });
                if yank {
                    vim.copy_selections_content(editor, MotionKind::Exclusive, window, cx);
                }
                let selections = editor
                    .selections
                    .all::<Point>(&editor.display_snapshot(cx))
                    .into_iter();
                let edits = selections.map(|selection| (selection.start..selection.end, ""));
                editor.edit(edits, cx);
            });
        });
        self.switch_mode(Mode::Insert, true, window, cx);
    }

    fn helix_substitute(
        &mut self,
        _: &HelixSubstitute,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.do_helix_substitute(true, window, cx);
    }

    fn helix_substitute_no_yank(
        &mut self,
        _: &HelixSubstituteNoYank,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.do_helix_substitute(false, window, cx);
    }

    fn helix_select_next(
        &mut self,
        _: &HelixSelectNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.do_helix_select(Direction::Next, window, cx);
    }

    fn helix_select_previous(
        &mut self,
        _: &HelixSelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.do_helix_select(Direction::Prev, window, cx);
    }

    fn do_helix_select(
        &mut self,
        direction: searchable::Direction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(pane) = self.pane(window, cx) else {
            return;
        };
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        let prior_selections = self.editor_selections(window, cx);

        let success = pane.update(cx, |pane, cx| {
            let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() else {
                return false;
            };
            search_bar.update(cx, |search_bar, cx| {
                if !search_bar.has_active_match() || !search_bar.show(window, cx) {
                    return false;
                }
                search_bar.select_match(direction, count, window, cx);
                true
            })
        });

        if !success {
            return;
        }
        if self.mode == Mode::HelixSelect {
            self.update_editor(cx, |_vim, editor, cx| {
                let snapshot = editor.snapshot(window, cx);
                editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                    s.select_anchor_ranges(
                        prior_selections
                            .iter()
                            .cloned()
                            .chain(s.all_anchors(&snapshot).iter().map(|s| s.range())),
                    );
                })
            });
        }
    }
}

#[cfg(test)]
mod test {
    use gpui::{UpdateGlobal, VisualTestContext};
    use indoc::indoc;
    use project::FakeFs;
    use search::{ProjectSearchView, project_search};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::DeploySearch;

    use crate::{VimAddon, state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_word_motions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        // «
        // ˇ
        // »
        cx.set_state(
            indoc! {"
            Th«e quiˇ»ck brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("w");

        cx.assert_state(
            indoc! {"
            The qu«ick ˇ»brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("w");

        cx.assert_state(
            indoc! {"
            The quick «brownˇ»
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("2 b");

        cx.assert_state(
            indoc! {"
            The «ˇquick »brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("down e up");

        cx.assert_state(
            indoc! {"
            The quicˇk brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.set_state("aa\n  «ˇbb»", Mode::HelixNormal);

        cx.simulate_keystroke("b");

        cx.assert_state("aa\n«ˇ  »bb", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // test delete a selection
        cx.set_state(
            indoc! {"
            The qu«ick ˇ»brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("d");

        cx.assert_state(
            indoc! {"
            The quˇbrown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // test deleting a single character
        cx.simulate_keystrokes("d");

        cx.assert_state(
            indoc! {"
            The quˇrown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_delete_character_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The quick brownˇ
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("d");

        cx.assert_state(
            indoc! {"
            The quick brownˇfox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
    }

    // #[gpui::test]
    // async fn test_delete_character_end_of_buffer(cx: &mut gpui::TestAppContext) {
    //     let mut cx = VimTestContext::new(cx, true).await;

    //     cx.set_state(
    //         indoc! {"
    //         The quick brown
    //         fox jumps over
    //         the lazy dog.ˇ"},
    //         Mode::HelixNormal,
    //     );

    //     cx.simulate_keystrokes("d");

    //     cx.assert_state(
    //         indoc! {"
    //         The quick brown
    //         fox jumps over
    //         the lazy dog.ˇ"},
    //         Mode::HelixNormal,
    //     );
    // }

    #[gpui::test]
    async fn test_f_and_t(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("f z");

        cx.assert_state(
            indoc! {"
                The qu«ick brown
                fox jumps over
                the lazˇ»y dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("F e F e");

        cx.assert_state(
            indoc! {"
                The quick brown
                fox jumps ov«ˇer
                the» lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("e 2 F e");

        cx.assert_state(
            indoc! {"
                Th«ˇe quick brown
                fox jumps over»
                the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("t r t r");

        cx.assert_state(
            indoc! {"
                The quick «brown
                fox jumps oveˇ»r
                the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_newline_char(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("aa«\nˇ»bb cc", Mode::HelixNormal);

        cx.simulate_keystroke("w");

        cx.assert_state("aa\n«bb ˇ»cc", Mode::HelixNormal);

        cx.set_state("aa«\nˇ»", Mode::HelixNormal);

        cx.simulate_keystroke("b");

        cx.assert_state("«ˇaa»\n", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_insert_selected(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state(
            indoc! {"
            «The ˇ»quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("i");

        cx.assert_state(
            indoc! {"
            ˇThe quick brown
            fox jumps over
            the lazy dog."},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_append(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        // test from the end of the selection
        cx.set_state(
            indoc! {"
            «Theˇ» quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("a");

        cx.assert_state(
            indoc! {"
            Theˇ quick brown
            fox jumps over
            the lazy dog."},
            Mode::Insert,
        );

        // test from the beginning of the selection
        cx.set_state(
            indoc! {"
            «ˇThe» quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("a");

        cx.assert_state(
            indoc! {"
            Theˇ quick brown
            fox jumps over
            the lazy dog."},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_replace(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // No selection (single character)
        cx.set_state("ˇaa", Mode::HelixNormal);

        cx.simulate_keystrokes("r x");

        cx.assert_state("ˇxa", Mode::HelixNormal);

        // Cursor at the beginning
        cx.set_state("«ˇaa»", Mode::HelixNormal);

        cx.simulate_keystrokes("r x");

        cx.assert_state("«ˇxx»", Mode::HelixNormal);

        // Cursor at the end
        cx.set_state("«aaˇ»", Mode::HelixNormal);

        cx.simulate_keystrokes("r x");

        cx.assert_state("«xxˇ»", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_yank(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Test yanking current character with no selection
        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("y");

        // Test cursor remains at the same position after yanking single character
        cx.assert_state("hello ˇworld", Mode::HelixNormal);
        cx.shared_clipboard().assert_eq("w");

        // Move cursor and yank another character
        cx.simulate_keystrokes("l");
        cx.simulate_keystrokes("y");
        cx.shared_clipboard().assert_eq("o");

        // Test yanking with existing selection
        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("y");
        cx.shared_clipboard().assert_eq("worl");
        cx.assert_state("hello «worlˇ»d", Mode::HelixNormal);

        // Test yanking in select mode character by character
        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystroke("v");
        cx.assert_state("hello «wˇ»orld", Mode::HelixSelect);
        cx.simulate_keystroke("y");
        cx.assert_state("hello «wˇ»orld", Mode::HelixNormal);
        cx.shared_clipboard().assert_eq("w");
    }

    #[gpui::test]
    async fn test_shift_r_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // First copy some text to clipboard
        cx.set_state("«hello worldˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("y");

        // Test paste with shift-r on single cursor
        cx.set_state("foo ˇbar", Mode::HelixNormal);
        cx.simulate_keystrokes("shift-r");

        cx.assert_state("foo hello worldˇbar", Mode::HelixNormal);

        // Test paste with shift-r on selection
        cx.set_state("foo «barˇ» baz", Mode::HelixNormal);
        cx.simulate_keystrokes("shift-r");

        cx.assert_state("foo hello worldˇ baz", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_select_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        assert_eq!(cx.mode(), Mode::Normal);
        cx.enable_helix();

        cx.simulate_keystrokes("v");
        assert_eq!(cx.mode(), Mode::HelixSelect);
        cx.simulate_keystrokes("escape");
        assert_eq!(cx.mode(), Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_insert_mode_stickiness(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Make a modification at a specific location
        cx.set_state("ˇhello", Mode::HelixNormal);
        assert_eq!(cx.mode(), Mode::HelixNormal);
        cx.simulate_keystrokes("i");
        assert_eq!(cx.mode(), Mode::Insert);
        cx.simulate_keystrokes("escape");
        assert_eq!(cx.mode(), Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_goto_last_modification(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Make a modification at a specific location
        cx.set_state("line one\nline ˇtwo\nline three", Mode::HelixNormal);
        cx.assert_state("line one\nline ˇtwo\nline three", Mode::HelixNormal);
        cx.simulate_keystrokes("i");
        cx.simulate_keystrokes("escape");
        cx.simulate_keystrokes("i");
        cx.simulate_keystrokes("m o d i f i e d space");
        cx.simulate_keystrokes("escape");

        // TODO: this fails, because state is no longer helix
        cx.assert_state(
            "line one\nline modified ˇtwo\nline three",
            Mode::HelixNormal,
        );

        // Move cursor away from the modification
        cx.simulate_keystrokes("up");

        // Use "g ." to go back to last modification
        cx.simulate_keystrokes("g .");

        // Verify we're back at the modification location and still in HelixNormal mode
        cx.assert_state(
            "line one\nline modifiedˇ two\nline three",
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_lines(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            "line one\nline ˇtwo\nline three\nline four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("2 x");
        cx.assert_state(
            "line one\n«line two\nline three\nˇ»line four",
            Mode::HelixNormal,
        );

        // Test extending existing line selection
        cx.set_state(
            indoc! {"
            li«ˇne one
            li»ne two
            line three
            line four"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «line one
            line two
            ˇ»line three
            line four"},
            Mode::HelixNormal,
        );

        // Pressing x in empty line, select next line (because helix considers cursor a selection)
        cx.set_state(
            indoc! {"
            line one
            ˇ
            line three
            line four"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            line one
            «
            line three
            ˇ»line four"},
            Mode::HelixNormal,
        );

        // Empty line with count selects extra + count lines
        cx.set_state(
            indoc! {"
            line one
            ˇ
            line three
            line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("2 x");
        cx.assert_state(
            indoc! {"
            line one
            «
            line three
            line four
            ˇ»line five"},
            Mode::HelixNormal,
        );

        // Compare empty vs non-empty line behavior
        cx.set_state(
            indoc! {"
            ˇnon-empty line
            line two
            line three"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «non-empty line
            ˇ»line two
            line three"},
            Mode::HelixNormal,
        );

        // Same test but with empty line - should select one extra
        cx.set_state(
            indoc! {"
            ˇ
            line two
            line three"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «
            line two
            ˇ»line three"},
            Mode::HelixNormal,
        );

        // Test selecting multiple lines with count
        cx.set_state(
            indoc! {"
            ˇline one
            line two
            line threeˇ
            line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «line one
            ˇ»line two
            «line three
            ˇ»line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        // Adjacent line selections stay separate (not merged)
        cx.assert_state(
            indoc! {"
            «line one
            line two
            ˇ»«line three
            line four
            ˇ»line five"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_mode_motion(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        assert_eq!(cx.mode(), Mode::Normal);
        cx.enable_helix();

        cx.set_state("ˇhello", Mode::HelixNormal);
        cx.simulate_keystrokes("l v l l");
        cx.assert_state("h«ellˇ»o", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_select_mode_motion_multiple_cursors(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        assert_eq!(cx.mode(), Mode::Normal);
        cx.enable_helix();

        // Start with multiple cursors (no selections)
        cx.set_state("ˇhello\nˇworld", Mode::HelixNormal);

        // Enter select mode and move right twice
        cx.simulate_keystrokes("v l l");

        // Each cursor should independently create and extend its own selection
        cx.assert_state("«helˇ»lo\n«worˇ»ld", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_select_word_motions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇone two", Mode::Normal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one tˇ»wo", Mode::Visual);

        // In Vim, this selects "t". In helix selections stops just before "t"

        cx.enable_helix();
        cx.set_state("ˇone two", Mode::HelixNormal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one ˇ»two", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_exit_visual_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇone two", Mode::Normal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one tˇ»wo", Mode::Visual);
        cx.simulate_keystrokes("escape");
        cx.assert_state("one ˇtwo", Mode::Normal);

        cx.enable_helix();
        cx.set_state("ˇone two", Mode::HelixNormal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one ˇ»two", Mode::HelixSelect);
        cx.simulate_keystrokes("escape");
        cx.assert_state("«one ˇ»two", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_select_motion(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("«ˇ»one two three", Mode::HelixSelect);
        cx.simulate_keystrokes("w");
        cx.assert_state("«one ˇ»two three", Mode::HelixSelect);

        cx.set_state("«ˇ»one two three", Mode::HelixSelect);
        cx.simulate_keystrokes("e");
        cx.assert_state("«oneˇ» two three", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_full_cursor_selection(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("ˇone two three", Mode::HelixNormal);
        cx.simulate_keystrokes("l l v h h h");
        cx.assert_state("«ˇone» two three", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_select_regex(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("ˇone two one", Mode::HelixNormal);
        cx.simulate_keystrokes("x");
        cx.assert_state("«one two oneˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("s o n e");
        cx.run_until_parked();
        cx.simulate_keystrokes("enter");
        cx.assert_state("«oneˇ» two «oneˇ»", Mode::HelixNormal);

        cx.simulate_keystrokes("x");
        cx.simulate_keystrokes("s");
        cx.run_until_parked();
        cx.simulate_keystrokes("enter");
        cx.assert_state("«oneˇ» two «oneˇ»", Mode::HelixNormal);

        // TODO: change "search_in_selection" to not perform any search when in helix select mode with no selection
        // cx.set_state("ˇstuff one two one", Mode::HelixNormal);
        // cx.simulate_keystrokes("s o n e enter");
        // cx.assert_state("ˇstuff one two one", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_select_next_match(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇhello two one two one two one", Mode::Visual);
        cx.simulate_keystrokes("/ o n e");
        cx.simulate_keystrokes("enter");
        cx.simulate_keystrokes("n n");
        cx.assert_state("«hello two one two one two oˇ»ne", Mode::Visual);

        cx.set_state("ˇhello two one two one two one", Mode::Normal);
        cx.simulate_keystrokes("/ o n e");
        cx.simulate_keystrokes("enter");
        cx.simulate_keystrokes("n n");
        cx.assert_state("hello two one two one two ˇone", Mode::Normal);

        cx.set_state("ˇhello two one two one two one", Mode::Normal);
        cx.simulate_keystrokes("/ o n e");
        cx.simulate_keystrokes("enter");
        cx.simulate_keystrokes("n g n g n");
        cx.assert_state("hello two one two «one two oneˇ»", Mode::Visual);

        cx.enable_helix();

        cx.set_state("ˇhello two one two one two one", Mode::HelixNormal);
        cx.simulate_keystrokes("/ o n e");
        cx.simulate_keystrokes("enter");
        cx.simulate_keystrokes("n n");
        cx.assert_state("hello two one two one two «oneˇ»", Mode::HelixNormal);

        cx.set_state("ˇhello two one two one two one", Mode::HelixSelect);
        cx.simulate_keystrokes("/ o n e");
        cx.simulate_keystrokes("enter");
        cx.simulate_keystrokes("n n");
        cx.assert_state("hello two «oneˇ» two «oneˇ» two «oneˇ»", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_substitute(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇone two", Mode::HelixNormal);
        cx.simulate_keystrokes("c");
        cx.assert_state("ˇne two", Mode::Insert);

        cx.set_state("«oneˇ» two", Mode::HelixNormal);
        cx.simulate_keystrokes("c");
        cx.assert_state("ˇ two", Mode::Insert);

        cx.set_state(
            indoc! {"
            oneˇ two
            three
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x c");
        cx.assert_state(
            indoc! {"
            ˇ
            three
            "},
            Mode::Insert,
        );

        cx.set_state(
            indoc! {"
            one twoˇ
            three
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("c");
        cx.assert_state(
            indoc! {"
            one twoˇthree
            "},
            Mode::Insert,
        );

        // Helix doesn't set the cursor to the first non-blank one when
        // replacing lines: it uses language-dependent indent queries instead.
        cx.set_state(
            indoc! {"
            one two
            «    indented
            three not indentedˇ»
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("c");
        cx.set_state(
            indoc! {"
            one two
            ˇ
            "},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_g_l_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Test g l moves to last character, not after it
        cx.set_state("hello ˇworld!", Mode::HelixNormal);
        cx.simulate_keystrokes("g l");
        cx.assert_state("hello worldˇ!", Mode::HelixNormal);

        // Test with Chinese characters, test if work with UTF-8?
        cx.set_state("ˇ你好世界", Mode::HelixNormal);
        cx.simulate_keystrokes("g l");
        cx.assert_state("你好世ˇ界", Mode::HelixNormal);

        // Test with end of line
        cx.set_state("endˇ", Mode::HelixNormal);
        cx.simulate_keystrokes("g l");
        cx.assert_state("enˇd", Mode::HelixNormal);

        // Test with empty line
        cx.set_state(
            indoc! {"
                hello
                ˇ
                world"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("g l");
        cx.assert_state(
            indoc! {"
                hello
                ˇ
                world"},
            Mode::HelixNormal,
        );

        // Test with multiple lines
        cx.set_state(
            indoc! {"
                ˇfirst line
                second line
                third line"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("g l");
        cx.assert_state(
            indoc! {"
                first linˇe
                second line
                third line"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_project_search_opens_in_normal_mode(cx: &mut gpui::TestAppContext) {
        VimTestContext::init(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "file_a.rs": "// File A.",
                "file_b.rs": "// File B.",
            }),
        )
        .await;

        let project = project::Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| workspace::Workspace::test_new(project.clone(), window, cx));

        cx.update(|cx| {
            VimTestContext::init_keybindings(true, cx);
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |store| store.helix_mode = Some(true));
            })
        });

        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        workspace
            .update(cx, |workspace, window, cx| {
                ProjectSearchView::deploy_search(workspace, &DeploySearch::default(), window, cx)
            })
            .unwrap();

        let search_view = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_pane()
                    .read(cx)
                    .items()
                    .find_map(|item| item.downcast::<ProjectSearchView>())
                    .expect("Project search view should be active")
            })
            .unwrap();

        project_search::perform_project_search(&search_view, "File A", cx);

        search_view.update(cx, |search_view, cx| {
            let vim_mode = search_view
                .results_editor()
                .read(cx)
                .addon::<VimAddon>()
                .map(|addon| addon.entity.read(cx).mode);

            assert_eq!(vim_mode, Some(Mode::HelixNormal));
        });
    }

    #[gpui::test]
    async fn test_scroll_with_selection(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Start with a selection
        cx.set_state(
            indoc! {"
            «lineˇ» one
            line two
            line three
            line four
            line five"},
            Mode::HelixNormal,
        );

        // Scroll down, selection should collapse
        cx.simulate_keystrokes("ctrl-d");
        cx.assert_state(
            indoc! {"
            line one
            line two
            line three
            line four
            line fiveˇ"},
            Mode::HelixNormal,
        );

        // Make a new selection
        cx.simulate_keystroke("b");
        cx.assert_state(
            indoc! {"
            line one
            line two
            line three
            line four
            line «ˇfive»"},
            Mode::HelixNormal,
        );

        // And scroll up, once again collapsing the selection.
        cx.simulate_keystroke("ctrl-u");
        cx.assert_state(
            indoc! {"
            line one
            line two
            line three
            line ˇfour
            line five"},
            Mode::HelixNormal,
        );

        // Enter select mode
        cx.simulate_keystroke("v");
        cx.assert_state(
            indoc! {"
            line one
            line two
            line three
            line «fˇ»our
            line five"},
            Mode::HelixSelect,
        );

        // And now the selection should be kept/expanded.
        cx.simulate_keystroke("ctrl-d");
        cx.assert_state(
            indoc! {"
            line one
            line two
            line three
            line «four
            line fiveˇ»"},
            Mode::HelixSelect,
        );
    }
}
