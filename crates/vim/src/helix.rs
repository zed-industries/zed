mod boundary;
mod duplicate;
mod object;
mod paste;
mod select;

use editor::display_map::{
    BlockContext, BlockPlacement, BlockProperties, BlockStyle, DisplayRow, DisplaySnapshot,
};
use editor::{
    Anchor, DisplayPoint, Editor, EditorSettings, HideMouseCursorOrigin, MultiBufferOffset,
    SelectionEffects, ToOffset, ToPoint, movement,
};
use gpui::actions;
use gpui::{Context, Font, Hsla, Pixels, Window, WindowTextSystem};
use language::{CharClassifier, CharKind, Point, Selection};
use multi_buffer::MultiBufferSnapshot;
use search::{BufferSearchBar, SearchOptions};
use settings::{RegisterSetting, Settings};
use text::{Bias, SelectionGoal};
use ui::prelude::*;
use workspace::searchable::{self, Direction, FilteredSearchRange};

use crate::motion::{self, MotionKind};
use crate::state::SearchState;
use crate::{
    Vim,
    motion::{Motion, right},
    state::{HelixJumpBehaviour, HelixJumpLabel, Mode, Operator},
};
use std::{ops::Range, sync::Arc};

pub(crate) const HELIX_JUMP_ACCENT: Hsla = Hsla {
    h: 0.0,
    s: 0.78,
    l: 0.55,
    a: 1.0,
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
        /// Activate Helix-style word jump labels.
        HelixJumpToWord,
        /// Delete the selection and enter edit mode.
        HelixSelectNext,
        /// Delete the selection and enter edit mode, without yanking the selection.
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
    Vim::action(editor, cx, Vim::helix_jump_to_word);
    Vim::action(editor, cx, Vim::helix_select_next);
    Vim::action(editor, cx, Vim::helix_select_previous);
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
            let text_layout_details = editor.text_layout_details(window);
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
            let text_layout_details = editor.text_layout_details(window);
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
                    let text_layout_details = editor.text_layout_details(window);
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

    pub fn helix_jump_to_word(
        &mut self,
        _: &HelixJumpToWord,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let behaviour = if self.mode.is_visual() {
            HelixJumpBehaviour::Extend
        } else {
            HelixJumpBehaviour::Move
        };
        self.start_helix_jump(behaviour, window, cx);
    }

    fn start_helix_jump(
        &mut self,
        behaviour: HelixJumpBehaviour,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_visual = self.mode.is_visual();
        let Some(data) = self.collect_helix_jump_data(is_visual, window, cx) else {
            return;
        };

        if data.labels.is_empty() {
            self.clear_helix_jump_ui(window, cx);
            return;
        }

        if !self.apply_helix_jump_ui(data.highlights, data.blocks, window, cx) {
            return;
        }

        self.push_operator(
            Operator::HelixJump {
                behaviour,
                first_char: None,
                labels: data.labels,
            },
            window,
            cx,
        );
    }

    fn collect_helix_jump_data(
        &mut self,
        is_visual: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<HelixJumpUiData> {
        self.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let display_snapshot = &snapshot.display_snapshot;
            let buffer_snapshot = display_snapshot.buffer_snapshot();

            let scroll_position = snapshot.scroll_position();
            let top_row = scroll_position.y.floor().max(0.0) as u32;

            let visible_rows = editor
                .visible_line_count()
                .map(|count| count.ceil() as u32 + 1)
                .filter(|count| *count > 0)
                .unwrap_or_else(|| {
                    display_snapshot
                        .max_point()
                        .row()
                        .0
                        .saturating_sub(top_row)
                        .saturating_add(1)
                });

            let start_display_point = DisplayPoint::new(DisplayRow(top_row), 0);
            let end_display_point =
                DisplayPoint::new(DisplayRow(top_row.saturating_add(visible_rows)), 0);

            let start_point =
                display_snapshot.display_point_to_point(start_display_point, Bias::Left);
            let end_point = display_snapshot.display_point_to_point(end_display_point, Bias::Right);

            let start_offset = buffer_snapshot.point_to_offset(start_point);
            let end_offset = buffer_snapshot.point_to_offset(end_point);

            let selections = editor.selections.all::<Point>(&display_snapshot);
            let (skip_points, skip_ranges) =
                Self::selection_skip_offsets(buffer_snapshot, &selections, is_visual);

            // Get the primary cursor position for alternating forward/backward labeling
            let cursor_offset = selections
                .first()
                .map(|s| buffer_snapshot.point_to_offset(s.head()))
                .unwrap_or(start_offset);

            let style = editor.style(cx);
            let font = style.text.font();
            let font_size = style.text.font_size.to_pixels(window.rem_size());

            let accent = HelixSettings::get_global(cx).jump_label_accent;
            Self::build_helix_jump_ui_data(
                buffer_snapshot,
                start_offset,
                end_offset,
                cursor_offset,
                accent,
                &skip_points,
                &skip_ranges,
                window.text_system(),
                font,
                font_size,
            )
        })
    }

    fn build_helix_jump_ui_data(
        buffer: &MultiBufferSnapshot,
        start_offset: MultiBufferOffset,
        end_offset: MultiBufferOffset,
        cursor_offset: MultiBufferOffset,
        accent: Hsla,
        skip_points: &[MultiBufferOffset],
        skip_ranges: &[Range<MultiBufferOffset>],
        text_system: &WindowTextSystem,
        font: Font,
        font_size: Pixels,
    ) -> HelixJumpUiData {
        if start_offset >= end_offset {
            return HelixJumpUiData::default();
        }

        // First pass: collect all word candidates without assigning labels
        let candidates = Self::collect_jump_candidates(
            buffer,
            start_offset,
            end_offset,
            skip_points,
            skip_ranges,
        );

        if candidates.is_empty() {
            return HelixJumpUiData::default();
        }

        // Partition candidates into forward (>= cursor) and backward (< cursor)
        let mut forward: Vec<_> = candidates
            .iter()
            .filter(|c| c.word_start >= cursor_offset)
            .cloned()
            .collect();
        let mut backward: Vec<_> = candidates
            .iter()
            .filter(|c| c.word_start < cursor_offset)
            .cloned()
            .collect();

        // Sort forward by distance from cursor (ascending)
        forward.sort_by_key(|c| c.word_start.0);
        // Sort backward by distance from cursor (descending, so closest first)
        backward.sort_by_key(|c| std::cmp::Reverse(c.word_start.0));

        // Interleave: forward gets even indices (aa, ac, ae...), backward gets odd (ab, ad, af...)
        let limit = HELIX_JUMP_ALPHABET.len() * HELIX_JUMP_ALPHABET.len();
        let mut ordered_candidates = Vec::with_capacity(forward.len() + backward.len());
        let mut fwd_iter = forward.into_iter();
        let mut bwd_iter = backward.into_iter();

        loop {
            if ordered_candidates.len() >= limit {
                break;
            }
            if let Some(fwd) = fwd_iter.next() {
                ordered_candidates.push(fwd);
            } else if bwd_iter.len() == 0 {
                break;
            }

            if ordered_candidates.len() >= limit {
                break;
            }
            if let Some(bwd) = bwd_iter.next() {
                ordered_candidates.push(bwd);
            } else if fwd_iter.len() == 0 {
                break;
            }
        }

        // Now assign labels and build UI data
        let mut labels = Vec::with_capacity(ordered_candidates.len());
        let mut highlights = Vec::with_capacity(ordered_candidates.len());
        let mut blocks = Vec::with_capacity(ordered_candidates.len());

        let width_of = |text: &str| -> Pixels {
            if text.is_empty() {
                return px(0.0);
            }

            let run = gpui::TextRun {
                len: text.len(),
                font: font.clone(),
                color: Hsla::default(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };

            text_system.layout_line(text, font_size, &[run], None).width
        };

        // Fast path for fixed-width fonts: no per-word measurement required; the label width will
        // match the width of any two characters.
        let is_monospace = {
            let monospace_tolerance = px(0.5);
            let a = width_of("iiiiiiii");
            let b = width_of("wwwwwwww");
            let c = width_of("00000000");
            let d = width_of("11111111");
            let diff_1 = if a > b { a - b } else { b - a };
            let diff_2 = if c > d { c - d } else { d - c };
            diff_1 <= monospace_tolerance && diff_2 <= monospace_tolerance
        };

        fn scan_hidden_prefix<F: Fn(&str) -> Pixels>(
            buffer: &MultiBufferSnapshot,
            range_start: MultiBufferOffset,
            range_end: MultiBufferOffset,
            word_end: MultiBufferOffset,
            label_width: Pixels,
            max_left_shift: Pixels,
            min_label_scale: f32,
            max_hidden_chars: usize,
            width_of: &F,
            hidden_prefix: &mut String,
            hide_end_offset: &mut MultiBufferOffset,
            hidden_width: &mut Pixels,
            total_char_count: &mut usize,
            word_char_count: &mut usize,
        ) {
            let mut offset = range_start;
            for chunk in buffer.text_for_range(range_start..range_end) {
                for (idx, ch) in chunk.char_indices() {
                    let absolute = offset + idx;

                    *total_char_count += 1;
                    if *total_char_count > max_hidden_chars {
                        return;
                    }

                    hidden_prefix.push(ch);
                    let end_offset = absolute + ch.len_utf8();

                    if absolute < word_end && is_jump_word_char(ch) {
                        *word_char_count += 1;
                    }

                    if *word_char_count < 2 {
                        continue;
                    }

                    *hide_end_offset = end_offset;
                    *hidden_width = width_of(hidden_prefix.as_str());

                    let effective_width = *hidden_width + max_left_shift;
                    let scale_needed = if label_width > px(0.0) {
                        (effective_width / label_width).min(1.0)
                    } else {
                        1.0
                    };

                    if scale_needed >= min_label_scale {
                        return;
                    }
                }
                offset += chunk.len();
            }
        }

        for (label_index, candidate) in ordered_candidates.into_iter().enumerate() {
            let start_anchor = buffer.anchor_after(candidate.word_start);
            let end_anchor = buffer.anchor_after(candidate.word_end);

            let label = [
                HELIX_JUMP_ALPHABET[label_index / HELIX_JUMP_ALPHABET.len()],
                HELIX_JUMP_ALPHABET[label_index % HELIX_JUMP_ALPHABET.len()],
            ];

            if is_monospace {
                let hide_end_anchor = buffer.anchor_after(candidate.first_two_end);
                highlights.push(start_anchor..hide_end_anchor);
                labels.push(HelixJumpLabel {
                    label,
                    range: start_anchor..end_anchor,
                });
                blocks.push(Self::jump_label_block(
                    start_anchor,
                    label,
                    accent,
                    label_index,
                    font.clone(),
                    font_size,
                    1.0,
                    px(0.0),
                ));
                continue;
            }

            // In proportional fonts, labels like "mw" can be wider than the first two letters of a
            // word like "if". We hide enough of the word to ensure the label doesn't overlap
            // visible text.
            //
            // To avoid "eating" punctuation between targets, we only extend the hidden region past
            // the end of the word into *whitespace*, and only when it doesn't eliminate all
            // separation from the next word.
            //
            // For short words (e.g. `if`) we prefer shifting the label left into preceding
            // whitespace (indentation) rather than shrinking the label or consuming punctuation.
            const MAX_HIDDEN_CHARS: usize = 16;
            const MIN_LABEL_SCALE: f32 = 1.00;
            let label_text: String = label.iter().collect();
            let label_width = width_of(&label_text);

            // Compute how much we can shift the label left into whitespace immediately preceding
            // the word. This helps avoid tiny labels on short words without hiding punctuation.
            const MAX_LEFT_WS_CHARS: usize = 32;
            let mut left_ws_rev = String::new();
            let mut left_ws_count = 0usize;
            let mut left_stopped_at_line_break = false;
            let mut left_stopped_at_non_ws = false;
            let mut left_hit_limit = false;

            for ch in buffer.reversed_chars_at(candidate.word_start) {
                if ch == '\n' || ch == '\r' {
                    left_stopped_at_line_break = true;
                    break;
                }

                if !ch.is_whitespace() {
                    left_stopped_at_non_ws = true;
                    break;
                }

                left_ws_count += 1;
                if left_ws_count > MAX_LEFT_WS_CHARS {
                    left_hit_limit = true;
                    break;
                }

                left_ws_rev.push(ch);
            }

            let left_ws: String = left_ws_rev.chars().rev().collect();
            let left_ws_width = width_of(&left_ws);

            // Leave a small gap before the label when it's between tokens; for indentation at the
            // start of a line, it's safe to use the full width.
            let left_is_indentation =
                left_stopped_at_line_break || (!left_stopped_at_non_ws && !left_hit_limit);
            let min_left_gap = if left_is_indentation { px(0.0) } else { px(2.0) };
            let max_left_shift = (left_ws_width - min_left_gap).max(px(0.0));

            // Determine how much whitespace after the word is safe to hide (if needed).
            let mut allowed_ws_end_offset = candidate.word_end;
            let mut ws_count = 0usize;
            let mut last_ws_start = candidate.word_end;
            let mut ws_end_offset = candidate.word_end;
            let mut next_non_ws = None;
            let mut hit_line_break_after_word = false;

            let mut ws_scan_offset = candidate.word_end;
            'ws: for chunk in buffer.text_for_range(candidate.word_end..end_offset) {
                for (idx, ch) in chunk.char_indices() {
                    let absolute = ws_scan_offset + idx;
                    if ch == '\n' || ch == '\r' {
                        hit_line_break_after_word = true;
                        break 'ws;
                    }
                    if !ch.is_whitespace() {
                        next_non_ws = Some(ch);
                        break 'ws;
                    }

                    ws_count += 1;
                    last_ws_start = absolute;
                    ws_end_offset = absolute + ch.len_utf8();
                }
                ws_scan_offset += chunk.len();
            }

            let mut is_end_of_line = hit_line_break_after_word && next_non_ws.is_none();
            if !is_end_of_line {
                is_end_of_line = matches!(
                    buffer.chars_at(candidate.word_end).next(),
                    None | Some('\n') | Some('\r')
                );
            }

            if ws_count > 0 {
                let next_is_word = match next_non_ws {
                    Some(ch) => is_jump_word_char(ch),
                    None => false,
                };

                if next_is_word {
                    // Only hide whitespace between words if we can leave at least one whitespace
                    // character visible, so adjacent labels remain visually separated.
                    if ws_count > 1 {
                        allowed_ws_end_offset = last_ws_start;
                    }
                } else {
                    // Next is punctuation (e.g. `if (`) or end-of-range: it's safe to hide all the
                    // leading whitespace.
                    allowed_ws_end_offset = ws_end_offset;
                }
            }

            let mut hidden_prefix = String::new();
            let mut hide_end_offset = candidate.first_two_end;
            let mut hidden_width = px(0.0);
            let mut total_char_count = 0usize;
            let mut word_char_count = 0usize;
            let min_label_scale = if is_end_of_line { 1.0 } else { MIN_LABEL_SCALE };

            // First, try to fit within the word itself (plus any available left shift).
            scan_hidden_prefix(
                buffer,
                candidate.word_start,
                candidate.word_end,
                candidate.word_end,
                label_width,
                max_left_shift,
                min_label_scale,
                MAX_HIDDEN_CHARS,
                &width_of,
                &mut hidden_prefix,
                &mut hide_end_offset,
                &mut hidden_width,
                &mut total_char_count,
                &mut word_char_count,
            );

            // If still too small, fall back to hiding some whitespace after the word (if allowed).
            if label_width > px(0.0)
                && (hidden_width + max_left_shift) / label_width < MIN_LABEL_SCALE
                && allowed_ws_end_offset > candidate.word_end
            {
                scan_hidden_prefix(
                    buffer,
                    candidate.word_end,
                    allowed_ws_end_offset,
                    candidate.word_end,
                    label_width,
                    max_left_shift,
                    min_label_scale,
                    MAX_HIDDEN_CHARS,
                    &width_of,
                    &mut hidden_prefix,
                    &mut hide_end_offset,
                    &mut hidden_width,
                    &mut total_char_count,
                    &mut word_char_count,
                );
            }

            // Fallback for unexpected measurement failure.
            if hidden_width <= px(0.0) {
                hidden_width = width_of(&hidden_prefix);
            }

            let hide_end_anchor = buffer.anchor_after(hide_end_offset);
            highlights.push(start_anchor..hide_end_anchor);

            // Leave a tiny margin to account for rounding differences between measurement and paint.
            let left_shift = if label_width > hidden_width {
                (label_width - hidden_width).min(max_left_shift)
            } else {
                px(0.0)
            };

            let scale_factor = if label_width > px(0.0) {
                let scale = ((hidden_width + left_shift) / label_width).min(1.0);
                if scale < 1.0 {
                    scale * 0.99
                } else {
                    1.0
                }
            } else {
                1.0
            };
            let scale_factor = if is_end_of_line { 1.0 } else { scale_factor };

            labels.push(HelixJumpLabel {
                label,
                range: start_anchor..end_anchor,
            });

            blocks.push(Self::jump_label_block(
                start_anchor,
                label,
                accent,
                label_index,
                font.clone(),
                font_size,
                scale_factor,
                left_shift,
            ));
        }

        // Sort highlights by position - the editor's binary search expects them sorted
        highlights.sort_by(|a, b| a.start.cmp(&b.start, buffer));

        HelixJumpUiData {
            labels,
            highlights,
            blocks,
        }
    }

    fn collect_jump_candidates(
        buffer: &MultiBufferSnapshot,
        start_offset: MultiBufferOffset,
        end_offset: MultiBufferOffset,
        skip_points: &[MultiBufferOffset],
        skip_ranges: &[Range<MultiBufferOffset>],
    ) -> Vec<JumpCandidate> {
        let mut candidates = Vec::new();
        let limit = HELIX_JUMP_ALPHABET.len() * HELIX_JUMP_ALPHABET.len();

        let mut offset = start_offset;
        let mut in_word = false;
        let mut word_start = start_offset;
        let mut first_two_end = start_offset;
        let mut char_count = 0;

        'chunks: for chunk in buffer.text_for_range(start_offset..end_offset) {
            for (idx, ch) in chunk.char_indices() {
                let absolute = offset + idx;
                let is_word = is_jump_word_char(ch);
                if is_word {
                    if !in_word {
                        in_word = true;
                        word_start = absolute;
                        char_count = 0;
                    }
                    if char_count == 1 {
                        first_two_end = absolute + ch.len_utf8();
                    }
                    char_count += 1;
                }

                if !is_word && in_word {
                    if char_count >= 2
                        && !Self::should_skip_jump_candidate(
                            word_start,
                            absolute,
                            skip_points,
                            skip_ranges,
                        )
                    {
                        candidates.push(JumpCandidate {
                            word_start,
                            word_end: absolute,
                            first_two_end,
                        });
                    }
                    in_word = false;
                    if candidates.len() >= limit {
                        break 'chunks;
                    }
                }
            }
            offset += chunk.len();
        }

        // Handle word at end of buffer
        if in_word
            && char_count >= 2
            && candidates.len() < limit
            && !Self::should_skip_jump_candidate(word_start, end_offset, skip_points, skip_ranges)
        {
            candidates.push(JumpCandidate {
                word_start,
                word_end: end_offset,
                first_two_end,
            });
        }

        candidates
    }

    fn selection_skip_offsets(
        buffer: &MultiBufferSnapshot,
        selections: &[Selection<Point>],
        is_visual: bool,
    ) -> (Vec<MultiBufferOffset>, Vec<Range<MultiBufferOffset>>) {
        let mut skip_points = Vec::with_capacity(selections.len());
        let mut skip_ranges = Vec::new();

        for selection in selections {
            let head_offset = buffer.point_to_offset(selection.head());
            skip_points.push(head_offset);

            // In visual mode, don't skip ranges so we can shrink the selection
            if !is_visual && selection.start != selection.end {
                let mut start = buffer.point_to_offset(selection.start);
                let mut end = buffer.point_to_offset(selection.end);
                if start > end {
                    std::mem::swap(&mut start, &mut end);
                }
                skip_ranges.push(start..end);
            }
        }

        (skip_points, skip_ranges)
    }

    fn should_skip_jump_candidate(
        word_start: MultiBufferOffset,
        word_end: MultiBufferOffset,
        skip_points: &[MultiBufferOffset],
        skip_ranges: &[Range<MultiBufferOffset>],
    ) -> bool {
        // Use inclusive end (<=) so cursor at last char of word skips that word
        skip_points
            .iter()
            .copied()
            .any(|offset| offset >= word_start && offset <= word_end)
            || skip_ranges
                .iter()
                .any(|range| range.start < word_end && word_start < range.end)
    }

    fn jump_label_block(
        anchor: Anchor,
        label: [char; 2],
        accent: Hsla,
        label_index: usize,
        font: Font,
        font_size: Pixels,
        scale_factor: f32,
        left_shift: Pixels,
    ) -> BlockProperties<Anchor> {
        let text: SharedString = label.iter().collect::<String>().into();
        BlockProperties {
            placement: BlockPlacement::Near(anchor),
            height: Some(0),
            style: BlockStyle::Fixed,
            render: Arc::new(move |_cx: &mut BlockContext| {
                let scaled_font_size = (font_size * scale_factor).max(px(1.0));
                div()
                    .block_mouse_except_scroll()
                    .relative()
                    .left(-left_shift)
                    .font(font.clone())
                    .text_size(scaled_font_size)
                    .text_color(accent)
                    .child(text.clone())
                    .into_any_element()
            }),
            priority: label_index,
        }
    }
}

#[derive(RegisterSetting)]
struct HelixSettings {
    jump_label_accent: Hsla,
}

impl Settings for HelixSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let helix = content.helix.clone().unwrap_or_default();
        Self {
            jump_label_accent: helix.jump_label_accent.unwrap_or(HELIX_JUMP_ACCENT),
        }
    }
}

const HELIX_JUMP_ALPHABET: &[char; 26] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn is_jump_word_char(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

/// A word candidate for jump labels, before label assignment.
#[derive(Clone)]
struct JumpCandidate {
    word_start: MultiBufferOffset,
    word_end: MultiBufferOffset,
    first_two_end: MultiBufferOffset,
}

#[derive(Default)]
struct HelixJumpUiData {
    labels: Vec<HelixJumpLabel>,
    highlights: Vec<Range<Anchor>>,
    blocks: Vec<BlockProperties<Anchor>>,
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{
        state::{Mode, Operator},
        test::VimTestContext,
    };

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
    async fn test_helix_jump_starts_operator(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇhello world\njump labels", Mode::HelixNormal);

        cx.simulate_keystrokes("g w");

        assert!(
            matches!(cx.active_operator(), Some(Operator::HelixJump { .. })),
            "expected HelixJump operator to be active"
        )
    }
}
