mod boundary;
mod duplicate;
mod object;
mod paste;
mod select;
mod surround;

use editor::display_map::{DisplayRow, DisplaySnapshot};
use editor::{
    DisplayPoint, Editor, EditorSettings, HideMouseCursorOrigin, MultiBufferOffset,
    NavigationOverlayLabel, NavigationTargetOverlay, SelectionEffects, ToOffset, ToPoint, movement,
};
use gpui::actions;
use gpui::{App, Context, Font, Hsla, Pixels, Window, WindowTextSystem};
use language::{CharClassifier, CharKind, Point, Selection};
use multi_buffer::MultiBufferSnapshot;
use search::{BufferSearchBar, SearchOptions};
use settings::Settings;
use text::{Bias, SelectionGoal};
use theme::ActiveTheme as _;
use ui::px;
use workspace::searchable::{self, Direction, FilteredSearchRange};

use crate::motion::{self, MotionKind};
use crate::state::{HelixJumpBehaviour, HelixJumpLabel, Mode, Operator, SearchState};
use crate::{
    PushHelixSurroundAdd, PushHelixSurroundDelete, PushHelixSurroundReplace, Vim,
    motion::{Motion, right},
};
use std::ops::Range;

actions!(
    vim,
    [
        /// Yanks the current selection or character if no selection.
        HelixYank,
        /// Inserts at the beginning of the selection.
        HelixInsert,
        /// Appends at the end of the selection.
        HelixAppend,
        /// Inserts at the end of the current Helix cursor line.
        HelixInsertEndOfLine,
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
    Vim::action(editor, cx, Vim::helix_insert_end_of_line);
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

                s.move_with(&mut |map, selection| {
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
                        // EndOfLine positions after the last character, but in
                        // helix visual mode we want the selection to end ON the
                        // last character. Adjust left here so the subsequent
                        // right-expansion (below) includes the last char without
                        // spilling into the newline.
                        Motion::EndOfLine { .. } => {
                            let (point, goal) = motion
                                .move_point(
                                    map,
                                    current_head,
                                    selection.goal,
                                    times,
                                    &text_layout_details,
                                )
                                .unwrap_or((current_head, selection.goal));
                            (movement::saturating_left(map, point), goal)
                        }
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
                                    movement::find_boundary_trail(map, head, &mut |left, right| {
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
        change: &mut dyn FnMut(
            // the start of the cursor
            DisplayPoint,
            &DisplaySnapshot,
        ) -> Option<(DisplayPoint, DisplayPoint)>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(&mut |map, selection| {
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
        is_boundary: &mut dyn FnMut(char, char, &CharClassifier) -> bool,
    ) {
        let times = times.unwrap_or(1);
        self.helix_new_selections(window, cx, &mut |cursor, map| {
            let mut head = movement::right(map, cursor);
            let mut tail = cursor;
            let classifier = map.buffer_snapshot().char_classifier_at(head.to_point(map));
            if head == map.max_point() {
                return None;
            }
            for _ in 0..times {
                let (maybe_next_tail, next_head) =
                    movement::find_boundary_trail(map, head, &mut |left, right| {
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
        is_boundary: &mut dyn FnMut(char, char, &CharClassifier) -> bool,
    ) {
        let times = times.unwrap_or(1);
        self.helix_new_selections(window, cx, &mut |cursor, map| {
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
                    movement::find_preceding_boundary_trail(map, head, &mut |left, right| {
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
                s.move_with(&mut |map, selection| {
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

    /// When `reversed` is true (used with `helix_find_range_backward`), the
    /// `left` and `right` characters are yielded in reverse text order, so the
    /// camelCase transition check must be flipped accordingly.
    fn subword_boundary_start(
        ignore_punctuation: bool,
        reversed: bool,
    ) -> impl FnMut(char, char, &CharClassifier) -> bool {
        move |left, right, classifier| {
            let left_kind = classifier.kind_with(left, ignore_punctuation);
            let right_kind = classifier.kind_with(right, ignore_punctuation);
            let at_newline = (left == '\n') ^ (right == '\n');
            let is_separator = |c: char| "_$=".contains(c);

            let is_word = left_kind != right_kind && right_kind != CharKind::Whitespace;
            let is_subword = (is_separator(left) && !is_separator(right))
                || if reversed {
                    right.is_lowercase() && left.is_uppercase()
                } else {
                    left.is_lowercase() && right.is_uppercase()
                };

            is_word || (is_subword && !right.is_whitespace()) || at_newline
        }
    }

    /// When `reversed` is true (used with `helix_find_range_backward`), the
    /// `left` and `right` characters are yielded in reverse text order, so the
    /// camelCase transition check must be flipped accordingly.
    fn subword_boundary_end(
        ignore_punctuation: bool,
        reversed: bool,
    ) -> impl FnMut(char, char, &CharClassifier) -> bool {
        move |left, right, classifier| {
            let left_kind = classifier.kind_with(left, ignore_punctuation);
            let right_kind = classifier.kind_with(right, ignore_punctuation);
            let at_newline = (left == '\n') ^ (right == '\n');
            let is_separator = |c: char| "_$=".contains(c);

            let is_word = left_kind != right_kind && left_kind != CharKind::Whitespace;
            let is_subword = (!is_separator(left) && is_separator(right))
                || if reversed {
                    right.is_lowercase() && left.is_uppercase()
                } else {
                    left.is_lowercase() && right.is_uppercase()
                };

            is_word || (is_subword && !left.is_whitespace()) || at_newline
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
            Motion::NextWordStart { ignore_punctuation } => {
                let mut is_boundary = Self::is_boundary_right(ignore_punctuation);
                self.helix_find_range_forward(times, window, cx, &mut is_boundary)
            }
            Motion::NextWordEnd { ignore_punctuation } => {
                let mut is_boundary = Self::is_boundary_left(ignore_punctuation);
                self.helix_find_range_forward(times, window, cx, &mut is_boundary)
            }
            Motion::PreviousWordStart { ignore_punctuation } => {
                let mut is_boundary = Self::is_boundary_left(ignore_punctuation);
                self.helix_find_range_backward(times, window, cx, &mut is_boundary)
            }
            Motion::PreviousWordEnd { ignore_punctuation } => {
                let mut is_boundary = Self::is_boundary_right(ignore_punctuation);
                self.helix_find_range_backward(times, window, cx, &mut is_boundary)
            }
            // The subword motions implementation is based off of the same
            // commands present in Helix itself, namely:
            //
            // * `move_next_sub_word_start`
            // * `move_next_sub_word_end`
            // * `move_prev_sub_word_start`
            // * `move_prev_sub_word_end`
            Motion::NextSubwordStart { ignore_punctuation } => {
                let mut is_boundary = Self::subword_boundary_start(ignore_punctuation, false);
                self.helix_find_range_forward(times, window, cx, &mut is_boundary)
            }
            Motion::NextSubwordEnd { ignore_punctuation } => {
                let mut is_boundary = Self::subword_boundary_end(ignore_punctuation, false);
                self.helix_find_range_forward(times, window, cx, &mut is_boundary)
            }
            Motion::PreviousSubwordStart { ignore_punctuation } => {
                let mut is_boundary = Self::subword_boundary_end(ignore_punctuation, true);
                self.helix_find_range_backward(times, window, cx, &mut is_boundary)
            }
            Motion::PreviousSubwordEnd { ignore_punctuation } => {
                let mut is_boundary = Self::subword_boundary_start(ignore_punctuation, true);
                self.helix_find_range_backward(times, window, cx, &mut is_boundary)
            }
            Motion::EndOfLine { .. } => {
                // In Helix mode, EndOfLine should position cursor ON the last character,
                // not after it. We therefore need special handling for it.
                self.update_editor(cx, |_, editor, cx| {
                    let text_layout_details = editor.text_layout_details(window, cx);
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.move_with(&mut |map, selection| {
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
                self.helix_new_selections(window, cx, &mut |cursor, map| {
                    let start = cursor;
                    let mut last_boundary = start;
                    for _ in 0..times.unwrap_or(1) {
                        last_boundary = movement::find_boundary(
                            map,
                            movement::right(map, last_boundary),
                            mode,
                            &mut |left, right| {
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
                self.helix_new_selections(window, cx, &mut |cursor, map| {
                    let start = cursor;
                    let mut last_boundary = start;
                    for _ in 0..times.unwrap_or(1) {
                        last_boundary = movement::find_preceding_boundary_display_point(
                            map,
                            last_boundary,
                            mode,
                            &mut |left, right| {
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
                    s.move_with(&mut |map, selection| {
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
                    s.move_with(&mut |_map, selection| {
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
                s.move_with(&mut |_map, selection| {
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
                        cmd_f_search: false,
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
                s.move_with(&mut |map, selection| {
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

    /// Helix-specific implementation of `shift-a` that accounts for Helix's
    /// selection model, where selecting a line with `x` creates a selection
    /// from column 0 of the current row to column 0 of the next row, so the
    /// default [`vim::normal::InsertEndOfLine`] would move the cursor to the
    /// end of the wrong line.
    fn helix_insert_end_of_line(
        &mut self,
        _: &HelixInsertEndOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(&mut |map, selection| {
                    let cursor = if !selection.is_empty() && !selection.reversed {
                        movement::left(map, selection.head())
                    } else {
                        selection.head()
                    };
                    selection
                        .collapse_to(motion::next_line_end(map, cursor, 1), SelectionGoal::None);
                });
            });
        });
    }

    pub fn helix_replace(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let display_map = editor.display_snapshot(cx);
                let selections = editor.selections.all_display(&display_map);

                let mut edits = Vec::new();
                let mut selection_info = Vec::new();
                for selection in &selections {
                    let mut range = selection.range();
                    let was_empty = range.is_empty();
                    let was_reversed = selection.reversed;

                    if was_empty {
                        range.end = movement::saturating_right(&display_map, range.start);
                    }

                    let byte_range = range.start.to_offset(&display_map, Bias::Left)
                        ..range.end.to_offset(&display_map, Bias::Left);

                    let snapshot = display_map.buffer_snapshot();
                    let grapheme_count = snapshot.grapheme_count_for_range(&byte_range);
                    let anchor = snapshot.anchor_before(byte_range.start);

                    selection_info.push((anchor, grapheme_count, was_empty, was_reversed));

                    if !byte_range.is_empty() {
                        let replacement_text = text.repeat(grapheme_count);
                        edits.push((byte_range, replacement_text));
                    }
                }

                editor.edit(edits, cx);

                // Restore selections based on original info
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                let ranges: Vec<_> = selection_info
                    .into_iter()
                    .map(|(start_anchor, grapheme_count, was_empty, was_reversed)| {
                        let start_point = start_anchor.to_point(&snapshot);
                        if was_empty {
                            start_point..start_point
                        } else {
                            let replacement_len = text.len() * grapheme_count;
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
                let extra_line = if first_char == Some('\n') && selection.is_empty() {
                    1
                } else {
                    0
                };

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
                    s.move_with(&mut |map, selection| {
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
                    let buffer = snapshot.buffer_snapshot();

                    s.select_ranges(
                        prior_selections
                            .iter()
                            .cloned()
                            .chain(s.all_anchors(&snapshot).iter().map(|s| s.range()))
                            .map(|range| {
                                let start = range.start.to_offset(buffer);
                                let end = range.end.to_offset(buffer);
                                start..end
                            }),
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

        if !self.apply_helix_jump_ui(data.overlays, window, cx) {
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
            let visible_range = Self::visible_jump_range(editor, &snapshot, display_snapshot, cx);
            let start_offset = buffer_snapshot.point_to_offset(visible_range.start);
            let end_offset = buffer_snapshot.point_to_offset(visible_range.end);

            let selections = editor.selections.all::<Point>(&display_snapshot);
            let skip_data = Self::selection_skip_offsets(buffer_snapshot, &selections, is_visual);

            // Get the primary cursor position for alternating forward/backward labeling
            let cursor_offset = selections
                .first()
                .map(|s| buffer_snapshot.point_to_offset(s.head()))
                .unwrap_or(start_offset);

            let style = editor.style(cx);
            let font = style.text.font();
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            let label_color = cx.theme().colors().vim_helix_jump_label_foreground;

            Self::build_helix_jump_ui_data(
                buffer_snapshot,
                start_offset,
                end_offset,
                cursor_offset,
                label_color,
                &skip_data,
                window.text_system(),
                font,
                font_size,
            )
        })
    }

    fn visible_jump_range(
        editor: &Editor,
        snapshot: &editor::EditorSnapshot,
        display_snapshot: &DisplaySnapshot,
        cx: &App,
    ) -> Range<Point> {
        let visible_range = editor.multi_buffer_visible_range(display_snapshot, cx);
        if editor.visible_line_count().is_some() || visible_range.start != visible_range.end {
            return visible_range;
        }

        let scroll_position = snapshot.scroll_position();
        let top_row = scroll_position.y.floor().max(0.0) as u32;
        let visible_rows = display_snapshot
            .max_point()
            .row()
            .0
            .saturating_sub(top_row)
            .saturating_add(1);
        let start_display_point = DisplayPoint::new(DisplayRow(top_row), 0);
        let end_display_point =
            DisplayPoint::new(DisplayRow(top_row.saturating_add(visible_rows)), 0);

        display_snapshot.display_point_to_point(start_display_point, Bias::Left)
            ..display_snapshot.display_point_to_point(end_display_point, Bias::Right)
    }

    fn build_helix_jump_ui_data(
        buffer: &MultiBufferSnapshot,
        start_offset: MultiBufferOffset,
        end_offset: MultiBufferOffset,
        cursor_offset: MultiBufferOffset,
        label_color: Hsla,
        skip_data: &HelixJumpSkipData,
        text_system: &WindowTextSystem,
        font: Font,
        font_size: Pixels,
    ) -> HelixJumpUiData {
        if start_offset >= end_offset {
            return HelixJumpUiData::default();
        }

        // First pass: collect all word candidates without assigning labels
        let candidates = Self::collect_jump_candidates(buffer, start_offset, end_offset, skip_data);

        if candidates.is_empty() {
            return HelixJumpUiData::default();
        }

        let ordered_candidates = Self::order_jump_candidates(candidates, cursor_offset);

        // Now assign labels and build UI data
        let mut labels = Vec::with_capacity(ordered_candidates.len());
        let mut overlays = Vec::with_capacity(ordered_candidates.len());

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

        let is_monospace = Self::is_monospace_jump_font(text_system, &font, font_size);

        for (label_index, candidate) in ordered_candidates.into_iter().enumerate() {
            let start_anchor = buffer.anchor_after(candidate.word_start);
            let end_anchor = buffer.anchor_after(candidate.word_end);
            let label = Self::jump_label_for_index(label_index);
            let label_text = label.iter().collect::<String>();
            // Monospace fonts: the label always matches the width of the first two characters,
            // so no per-word measurement is needed.
            // Proportional fonts: a label like "mw" can be wider than a short word like "if",
            // so we hide enough of the word (and possibly trailing whitespace) to make room,
            // or shift the label left into preceding whitespace.
            let fit = if is_monospace {
                JumpLabelFit::monospace(candidate.first_two_end)
            } else {
                let label_width = width_of(&label_text);
                Self::fit_proportional_jump_label(
                    buffer,
                    &candidate,
                    end_offset,
                    label_width,
                    &width_of,
                )
            };

            let hide_end_anchor = buffer.anchor_after(fit.hide_end_offset);

            labels.push(HelixJumpLabel {
                label,
                range: start_anchor..end_anchor,
            });

            overlays.push(NavigationTargetOverlay {
                target_range: start_anchor..end_anchor,
                label: NavigationOverlayLabel {
                    text: label_text.into(),
                    text_color: label_color,
                    x_offset: -fit.left_shift,
                    scale_factor: fit.scale_factor,
                },
                covered_text_range: Some(start_anchor..hide_end_anchor),
            });
        }

        HelixJumpUiData { labels, overlays }
    }

    fn collect_jump_candidates(
        buffer: &MultiBufferSnapshot,
        start_offset: MultiBufferOffset,
        end_offset: MultiBufferOffset,
        skip_data: &HelixJumpSkipData,
    ) -> Vec<JumpCandidate> {
        let mut candidates = Vec::new();

        let mut offset = start_offset;
        let mut in_word = false;
        let mut word_start = start_offset;
        let mut first_two_end = start_offset;
        let mut char_count = 0;

        for chunk in buffer.text_for_range(start_offset..end_offset) {
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
                        && !Self::should_skip_jump_candidate(word_start, absolute, skip_data)
                    {
                        candidates.push(JumpCandidate {
                            word_start,
                            word_end: absolute,
                            first_two_end,
                        });
                    }
                    in_word = false;
                }
            }
            offset += chunk.len();
        }

        // Handle word at end of buffer
        if in_word
            && char_count >= 2
            && !Self::should_skip_jump_candidate(word_start, end_offset, skip_data)
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
    ) -> HelixJumpSkipData {
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

        skip_points.sort_unstable();

        skip_ranges.sort_unstable_by_key(|range| range.start);
        let mut merged_ranges: Vec<Range<MultiBufferOffset>> =
            Vec::with_capacity(skip_ranges.len());
        for range in skip_ranges {
            if let Some(previous_range) = merged_ranges.last_mut()
                && range.start <= previous_range.end
            {
                previous_range.end = previous_range.end.max(range.end);
            } else {
                merged_ranges.push(range);
            }
        }

        HelixJumpSkipData {
            points: skip_points,
            ranges: merged_ranges,
        }
    }

    fn should_skip_jump_candidate(
        word_start: MultiBufferOffset,
        word_end: MultiBufferOffset,
        skip_data: &HelixJumpSkipData,
    ) -> bool {
        // word_end is exclusive, so points at the following delimiter should not skip the word.
        let point_index = skip_data
            .points
            .partition_point(|offset| *offset < word_start);
        if skip_data
            .points
            .get(point_index)
            .is_some_and(|offset| *offset < word_end)
        {
            return true;
        }

        let range_index = skip_data
            .ranges
            .partition_point(|range| range.end <= word_start);
        skip_data
            .ranges
            .get(range_index)
            .is_some_and(|range| range.start < word_end)
    }

    /// Interleave candidates so forward targets get even label indices (aa, ac, ae...)
    /// and backward targets get odd indices (ab, ad, af...), matching Helix's algorithm.
    /// This keeps the earliest label assignments close to the cursor in both directions.
    fn order_jump_candidates(
        candidates: Vec<JumpCandidate>,
        cursor_offset: MultiBufferOffset,
    ) -> Vec<JumpCandidate> {
        let mut forward = Vec::with_capacity(candidates.len());
        let mut backward = Vec::new();

        for candidate in candidates {
            if candidate.word_start < cursor_offset {
                backward.push(candidate);
            } else {
                forward.push(candidate);
            }
        }

        backward.reverse();

        let mut ordered_candidates =
            Vec::with_capacity((forward.len() + backward.len()).min(HELIX_JUMP_LABEL_LIMIT));
        let mut forward_candidates = forward.into_iter();
        let mut backward_candidates = backward.into_iter();

        loop {
            let mut pushed_candidate = false;

            if ordered_candidates.len() < HELIX_JUMP_LABEL_LIMIT
                && let Some(candidate) = forward_candidates.next()
            {
                ordered_candidates.push(candidate);
                pushed_candidate = true;
            }

            if ordered_candidates.len() < HELIX_JUMP_LABEL_LIMIT
                && let Some(candidate) = backward_candidates.next()
            {
                ordered_candidates.push(candidate);
                pushed_candidate = true;
            }

            if !pushed_candidate {
                break;
            }
        }

        ordered_candidates
    }

    fn jump_label_for_index(index: usize) -> [char; 2] {
        [
            HELIX_JUMP_ALPHABET[index / HELIX_JUMP_ALPHABET.len()],
            HELIX_JUMP_ALPHABET[index % HELIX_JUMP_ALPHABET.len()],
        ]
    }

    fn is_monospace_jump_font(
        text_system: &WindowTextSystem,
        font: &Font,
        font_size: Pixels,
    ) -> bool {
        let font_id = text_system.resolve_font(font);
        let width_of_char = |ch| {
            text_system
                .advance(font_id, font_size, ch)
                .map(|size| size.width)
                .unwrap_or_else(|_| text_system.layout_width(font_id, font_size, ch))
        };

        let a = width_of_char('i');
        let b = width_of_char('w');
        let c = width_of_char('0');
        let d = width_of_char('1');
        let diff_1 = if a > b { a - b } else { b - a };
        let diff_2 = if c > d { c - d } else { d - c };
        diff_1 <= HELIX_JUMP_MONOSPACE_TOLERANCE && diff_2 <= HELIX_JUMP_MONOSPACE_TOLERANCE
    }

    /// Fit a jump label over a word in a proportional font.
    ///
    /// Prefer fitting within the word itself, using available whitespace to the left
    /// before consuming trailing whitespace after the word. If the label still cannot
    /// fit cleanly, allow a small amount of scaling.
    fn fit_proportional_jump_label<F: Fn(&str) -> Pixels>(
        buffer: &MultiBufferSnapshot,
        candidate: &JumpCandidate,
        end_offset: MultiBufferOffset,
        label_width: Pixels,
        width_of: &F,
    ) -> JumpLabelFit {
        let fit_budget = Self::jump_label_fit_budget(buffer, candidate, end_offset, width_of);

        let mut hidden_prefix = HiddenPrefixFitState::new(candidate.first_two_end);
        let min_label_scale = if fit_budget.preserve_full_scale {
            1.0
        } else {
            HELIX_JUMP_MIN_LABEL_SCALE
        };

        hidden_prefix.extend_to_fit(
            buffer,
            candidate.word_start,
            candidate.word_end,
            candidate.word_end,
            label_width,
            fit_budget.max_left_shift,
            min_label_scale,
            width_of,
        );

        if label_width > px(0.0)
            && hidden_prefix.needs_more_width(label_width, fit_budget.max_left_shift)
            && fit_budget.allowed_trailing_hide_end > candidate.word_end
        {
            hidden_prefix.extend_to_fit(
                buffer,
                candidate.word_end,
                fit_budget.allowed_trailing_hide_end,
                candidate.word_end,
                label_width,
                fit_budget.max_left_shift,
                min_label_scale,
                width_of,
            );
        }

        // Jump candidates always contain at least two word characters, and the initial
        // scan above always measures through that second character before we read the width.
        let hidden_width = hidden_prefix.hidden_width;

        let left_shift = if label_width > hidden_width {
            (label_width - hidden_width).min(fit_budget.max_left_shift)
        } else {
            px(0.0)
        };

        let scale_factor = if label_width > px(0.0) {
            let scale = ((hidden_width + left_shift) / label_width).min(1.0);
            if scale < 1.0 { scale * 0.99 } else { 1.0 }
        } else {
            1.0
        };

        JumpLabelFit {
            hide_end_offset: hidden_prefix.hide_end_offset,
            left_shift,
            scale_factor: if fit_budget.preserve_full_scale {
                1.0
            } else {
                scale_factor
            },
        }
    }

    fn jump_label_fit_budget<F: Fn(&str) -> Pixels>(
        buffer: &MultiBufferSnapshot,
        candidate: &JumpCandidate,
        end_offset: MultiBufferOffset,
        width_of: &F,
    ) -> JumpLabelFitBudget {
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
            if left_ws_count > HELIX_JUMP_MAX_LEFT_WS_CHARS {
                left_hit_limit = true;
                break;
            }

            left_ws_rev.push(ch);
        }

        let left_ws: String = left_ws_rev.chars().rev().collect();
        let left_ws_width = width_of(&left_ws);
        let left_is_indentation =
            left_stopped_at_line_break || (!left_stopped_at_non_ws && !left_hit_limit);
        // Between tokens leave a small gap so the label doesn't touch the previous word;
        // for line-leading indentation the full width is safe.
        let min_left_gap = if left_is_indentation {
            px(0.0)
        } else {
            px(2.0)
        };
        let max_left_shift = (left_ws_width - min_left_gap).max(px(0.0));

        let mut allowed_trailing_hide_end = candidate.word_end;
        let mut ws_count = 0usize;
        let mut last_ws_start = candidate.word_end;
        let mut ws_end_offset = candidate.word_end;
        let mut next_non_ws = None;
        let mut hit_line_break_after_word = false;

        let mut ws_scan_offset = candidate.word_end;
        'scan: for chunk in buffer.text_for_range(candidate.word_end..end_offset) {
            for (idx, ch) in chunk.char_indices() {
                let absolute = ws_scan_offset + idx;
                if ch == '\n' || ch == '\r' {
                    hit_line_break_after_word = true;
                    break 'scan;
                }
                if !ch.is_whitespace() {
                    next_non_ws = Some(ch);
                    break 'scan;
                }

                ws_count += 1;
                last_ws_start = absolute;
                ws_end_offset = absolute + ch.len_utf8();
            }
            ws_scan_offset += chunk.len();
        }

        let preserve_full_scale = hit_line_break_after_word && next_non_ws.is_none()
            || matches!(
                buffer.chars_at(candidate.word_end).next(),
                None | Some('\n') | Some('\r')
            );

        if ws_count > 0 {
            let next_is_word = match next_non_ws {
                Some(ch) => is_jump_word_char(ch),
                None => false,
            };

            if next_is_word {
                // Keep at least one whitespace character visible so adjacent labels
                // remain visually separated.
                if ws_count > 1 {
                    allowed_trailing_hide_end = last_ws_start;
                }
            } else {
                // Next token is punctuation or end-of-range — safe to hide all whitespace.
                allowed_trailing_hide_end = ws_end_offset;
            }
        }

        JumpLabelFitBudget {
            max_left_shift,
            allowed_trailing_hide_end,
            preserve_full_scale,
        }
    }
}

const HELIX_JUMP_ALPHABET: &[char; 26] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const HELIX_JUMP_LABEL_LIMIT: usize = HELIX_JUMP_ALPHABET.len() * HELIX_JUMP_ALPHABET.len();
const HELIX_JUMP_MONOSPACE_TOLERANCE: Pixels = px(0.5);
const HELIX_JUMP_MIN_LABEL_SCALE: f32 = 1.0;
const HELIX_JUMP_MAX_HIDDEN_CHARS: usize = 16;
const HELIX_JUMP_MAX_LEFT_WS_CHARS: usize = 32;

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

struct HelixJumpSkipData {
    points: Vec<MultiBufferOffset>,
    ranges: Vec<Range<MultiBufferOffset>>,
}

struct JumpLabelFit {
    hide_end_offset: MultiBufferOffset,
    left_shift: Pixels,
    scale_factor: f32,
}

struct JumpLabelFitBudget {
    max_left_shift: Pixels,
    allowed_trailing_hide_end: MultiBufferOffset,
    preserve_full_scale: bool,
}

struct HiddenPrefixFitState {
    text: String,
    hide_end_offset: MultiBufferOffset,
    hidden_width: Pixels,
    total_char_count: usize,
    word_char_count: usize,
}

impl JumpLabelFit {
    fn monospace(hide_end_offset: MultiBufferOffset) -> Self {
        Self {
            hide_end_offset,
            left_shift: px(0.0),
            scale_factor: 1.0,
        }
    }
}

impl HiddenPrefixFitState {
    fn new(hide_end_offset: MultiBufferOffset) -> Self {
        Self {
            text: String::new(),
            hide_end_offset,
            hidden_width: px(0.0),
            total_char_count: 0,
            word_char_count: 0,
        }
    }

    fn needs_more_width(&self, label_width: Pixels, max_left_shift: Pixels) -> bool {
        (self.hidden_width + max_left_shift) / label_width < HELIX_JUMP_MIN_LABEL_SCALE
    }

    fn extend_to_fit<F: Fn(&str) -> Pixels>(
        &mut self,
        buffer: &MultiBufferSnapshot,
        range_start: MultiBufferOffset,
        range_end: MultiBufferOffset,
        word_end: MultiBufferOffset,
        label_width: Pixels,
        max_left_shift: Pixels,
        min_label_scale: f32,
        width_of: &F,
    ) {
        let mut offset = range_start;
        for chunk in buffer.text_for_range(range_start..range_end) {
            for (idx, ch) in chunk.char_indices() {
                let absolute = offset + idx;

                self.total_char_count += 1;
                if self.total_char_count > HELIX_JUMP_MAX_HIDDEN_CHARS {
                    return;
                }

                self.text.push(ch);
                let end_offset = absolute + ch.len_utf8();

                if absolute < word_end && is_jump_word_char(ch) {
                    self.word_char_count += 1;
                }

                if self.word_char_count < 2 {
                    continue;
                }

                self.hide_end_offset = end_offset;
                self.hidden_width = width_of(self.text.as_str());

                let effective_width = self.hidden_width + max_left_shift;
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
}

#[derive(Default)]
struct HelixJumpUiData {
    labels: Vec<HelixJumpLabel>,
    overlays: Vec<NavigationTargetOverlay>,
}

#[cfg(test)]
mod test {
    use std::{fmt::Write, time::Duration};

    use editor::{HighlightKey, MultiBufferOffset};
    use gpui::{KeyBinding, UpdateGlobal, VisualTestContext};
    use indoc::indoc;
    use language::Point;
    use project::FakeFs;
    use search::{ProjectSearchView, project_search};
    use serde_json::json;
    use settings::{SettingsStore, ThemeColorsContent, ThemeStyleContent};
    use theme::ActiveTheme as _;
    use util::path;
    use workspace::{DeploySearch, MultiWorkspace};

    use super::HELIX_JUMP_LABEL_LIMIT;
    use crate::{
        HELIX_JUMP_OVERLAY_KEY, Vim, VimAddon,
        state::{Mode, Operator},
        test::VimTestContext,
    };

    fn active_helix_jump_labels(cx: &mut VimTestContext) -> Vec<(String, String)> {
        cx.update_editor(|editor, window, cx| {
            let labels = match editor
                .addon::<VimAddon>()
                .unwrap()
                .entity
                .read(cx)
                .operator_stack
                .last()
                .cloned()
            {
                Some(Operator::HelixJump { labels, .. }) => labels,
                other => panic!("expected active HelixJump operator, got {other:?}"),
            };

            let snapshot = editor.snapshot(window, cx);
            let buffer_snapshot = snapshot.display_snapshot.buffer_snapshot();

            labels
                .into_iter()
                .map(|label| {
                    let jump_label = label.label.iter().collect::<String>();
                    let word = buffer_snapshot
                        .text_for_range(label.range)
                        .collect::<String>();
                    (jump_label, word)
                })
                .collect()
        })
    }

    fn helix_jump_label_for_word(cx: &mut VimTestContext, target_word: &str) -> String {
        active_helix_jump_labels(cx)
            .into_iter()
            .find_map(|(label, word)| (word == target_word).then_some(label))
            .unwrap_or_else(|| {
                let mut message = String::new();
                let labels = active_helix_jump_labels(cx);
                let _ = write!(
                    &mut message,
                    "expected jump label for word {target_word:?}, available labels: {labels:?}"
                );
                panic!("{message}");
            })
    }

    fn jump_to_word(cx: &mut VimTestContext, target_word: &str) {
        cx.simulate_keystrokes("g w");

        let label = helix_jump_label_for_word(cx, target_word);

        let mut chars = label.chars();
        let first = chars.next().expect("jump labels are two characters long");
        let second = chars.next().expect("jump labels are two characters long");
        cx.simulate_keystrokes(&format!("{first} {second}"));
    }

    fn active_helix_jump_overlay_counts(cx: &mut VimTestContext) -> (usize, usize) {
        let covered_text_range_count = cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            snapshot
                .text_highlight_ranges(HighlightKey::NavigationOverlay(HELIX_JUMP_OVERLAY_KEY))
                .map(|ranges| ranges.as_ref().clone().1.len())
                .unwrap_or_default()
        });
        let label_count = match cx.active_operator() {
            Some(Operator::HelixJump { labels, .. }) => labels.len(),
            _ => 0,
        };

        (covered_text_range_count, label_count)
    }

    fn assert_helix_jump_cleared(cx: &mut VimTestContext, expected_overlay_counts: (usize, usize)) {
        assert_eq!(cx.active_operator(), None);
        assert_eq!(
            active_helix_jump_overlay_counts(cx),
            expected_overlay_counts,
            "expected Helix jump UI to be fully cleared"
        );
    }

    fn helix_jump_labels_for_full_buffer(cx: &mut VimTestContext) -> Vec<(String, String)> {
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let display_snapshot = &snapshot.display_snapshot;
            let buffer_snapshot = display_snapshot.buffer_snapshot();
            let selections = editor.selections.all::<Point>(display_snapshot);
            let skip_data = Vim::selection_skip_offsets(buffer_snapshot, &selections, false);
            let cursor_offset = selections
                .first()
                .map(|selection| buffer_snapshot.point_to_offset(selection.head()))
                .unwrap_or(MultiBufferOffset(0));
            let style = editor.style(cx);
            let font = style.text.font();
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            let label_color = cx.theme().colors().vim_helix_jump_label_foreground;
            let data = Vim::build_helix_jump_ui_data(
                buffer_snapshot,
                MultiBufferOffset(0),
                buffer_snapshot.len(),
                cursor_offset,
                label_color,
                &skip_data,
                window.text_system(),
                font,
                font_size,
            );

            data.labels
                .into_iter()
                .map(|label| {
                    let jump_label = label.label.iter().collect::<String>();
                    let word = buffer_snapshot
                        .text_for_range(label.range)
                        .collect::<String>();
                    (jump_label, word)
                })
                .collect()
        })
    }

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
    async fn test_next_subword_start(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Setup custom keybindings for subword motions so we can use the bindings
        // in `simulate_keystroke`.
        cx.update(|_window, cx| {
            cx.bind_keys([KeyBinding::new(
                "w",
                crate::motion::NextSubwordStart {
                    ignore_punctuation: false,
                },
                None,
            )]);
        });

        cx.set_state("ˇfoo.bar", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("«fooˇ».bar", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo«.ˇ»bar", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo.«barˇ»", Mode::HelixNormal);

        cx.set_state("ˇfoo(bar)", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("«fooˇ»(bar)", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo«(ˇ»bar)", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo(«barˇ»)", Mode::HelixNormal);

        cx.set_state("ˇfoo_bar_baz", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("«foo_ˇ»bar_baz", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo_«bar_ˇ»baz", Mode::HelixNormal);

        cx.set_state("ˇfooBarBaz", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("«fooˇ»BarBaz", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo«Barˇ»Baz", Mode::HelixNormal);

        cx.set_state("ˇfoo;bar", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("«fooˇ»;bar", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo«;ˇ»bar", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("foo;«barˇ»", Mode::HelixNormal);

        cx.set_state("ˇ<?php\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("«<?ˇ»php\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("<?«phpˇ»\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("<?php\n\n«$ˇ»someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("<?php\n\n$«someˇ»Variable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("<?php\n\n$some«Variable ˇ»= 2;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("<?php\n\n$someVariable «= ˇ»2;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("<?php\n\n$someVariable = «2ˇ»;", Mode::HelixNormal);
        cx.simulate_keystroke("w");
        cx.assert_state("<?php\n\n$someVariable = 2«;ˇ»", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_next_subword_end(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Setup custom keybindings for subword motions so we can use the bindings
        // in `simulate_keystroke`.
        cx.update(|_window, cx| {
            cx.bind_keys([KeyBinding::new(
                "e",
                crate::motion::NextSubwordEnd {
                    ignore_punctuation: false,
                },
                None,
            )]);
        });

        cx.set_state("ˇfoo.bar", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("«fooˇ».bar", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo«.ˇ»bar", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo.«barˇ»", Mode::HelixNormal);

        cx.set_state("ˇfoo(bar)", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("«fooˇ»(bar)", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo«(ˇ»bar)", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo(«barˇ»)", Mode::HelixNormal);

        cx.set_state("ˇfoo_bar_baz", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("«fooˇ»_bar_baz", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo«_barˇ»_baz", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo_bar«_bazˇ»", Mode::HelixNormal);

        cx.set_state("ˇfooBarBaz", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("«fooˇ»BarBaz", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo«Barˇ»Baz", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("fooBar«Bazˇ»", Mode::HelixNormal);

        cx.set_state("ˇfoo;bar", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("«fooˇ»;bar", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo«;ˇ»bar", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("foo;«barˇ»", Mode::HelixNormal);

        cx.set_state("ˇ<?php\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("«<?ˇ»php\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("<?«phpˇ»\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("<?php\n\n«$ˇ»someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("<?php\n\n$«someˇ»Variable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("<?php\n\n$some«Variableˇ» = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("<?php\n\n$someVariable« =ˇ» 2;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("<?php\n\n$someVariable =« 2ˇ»;", Mode::HelixNormal);
        cx.simulate_keystroke("e");
        cx.assert_state("<?php\n\n$someVariable = 2«;ˇ»", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_previous_subword_start(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Setup custom keybindings for subword motions so we can use the bindings
        // in `simulate_keystroke`.
        cx.update(|_window, cx| {
            cx.bind_keys([KeyBinding::new(
                "b",
                crate::motion::PreviousSubwordStart {
                    ignore_punctuation: false,
                },
                None,
            )]);
        });

        cx.set_state("foo.barˇ", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo.«ˇbar»", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo«ˇ.»bar", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("«ˇfoo».bar", Mode::HelixNormal);

        cx.set_state("foo(bar)ˇ", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo(bar«ˇ)»", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo(«ˇbar»)", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo«ˇ(»bar)", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("«ˇfoo»(bar)", Mode::HelixNormal);

        cx.set_state("foo_bar_bazˇ", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo_bar_«ˇbaz»", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo_«ˇbar_»baz", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("«ˇfoo_»bar_baz", Mode::HelixNormal);

        cx.set_state("foo;barˇ", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo;«ˇbar»", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo«ˇ;»bar", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("«ˇfoo»;bar", Mode::HelixNormal);

        cx.set_state("<?php\n\n$someVariable = 2;ˇ", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("<?php\n\n$someVariable = 2«ˇ;»", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("<?php\n\n$someVariable = «ˇ2»;", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("<?php\n\n$someVariable «ˇ= »2;", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("<?php\n\n$some«ˇVariable »= 2;", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("<?php\n\n$«ˇsome»Variable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("<?php\n\n«ˇ$»someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("<?«ˇphp»\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("«ˇ<?»php\n\n$someVariable = 2;", Mode::HelixNormal);

        cx.set_state("fooBarBazˇ", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("fooBar«ˇBaz»", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("foo«ˇBar»Baz", Mode::HelixNormal);
        cx.simulate_keystroke("b");
        cx.assert_state("«ˇfoo»BarBaz", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_previous_subword_end(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Setup custom keybindings for subword motions so we can use the bindings
        // in `simulate_keystrokes`.
        cx.update(|_window, cx| {
            cx.bind_keys([KeyBinding::new(
                "g e",
                crate::motion::PreviousSubwordEnd {
                    ignore_punctuation: false,
                },
                None,
            )]);
        });

        cx.set_state("foo.barˇ", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo.«ˇbar»", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo«ˇ.»bar", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("«ˇfoo».bar", Mode::HelixNormal);

        cx.set_state("foo(bar)ˇ", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo(bar«ˇ)»", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo(«ˇbar»)", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo«ˇ(»bar)", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("«ˇfoo»(bar)", Mode::HelixNormal);

        cx.set_state("foo_bar_bazˇ", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo_bar«ˇ_baz»", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo«ˇ_bar»_baz", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("«ˇfoo»_bar_baz", Mode::HelixNormal);

        cx.set_state("foo;barˇ", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo;«ˇbar»", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo«ˇ;»bar", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("«ˇfoo»;bar", Mode::HelixNormal);

        cx.set_state("<?php\n\n$someVariable = 2;ˇ", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("<?php\n\n$someVariable = 2«ˇ;»", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("<?php\n\n$someVariable =«ˇ 2»;", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("<?php\n\n$someVariable«ˇ =» 2;", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("<?php\n\n$some«ˇVariable» = 2;", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("<?php\n\n$«ˇsome»Variable = 2;", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("<?php\n\n«ˇ$»someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("<?«ˇphp»\n\n$someVariable = 2;", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("«ˇ<?»php\n\n$someVariable = 2;", Mode::HelixNormal);

        cx.set_state("fooBarBazˇ", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("fooBar«ˇBaz»", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("foo«ˇBar»Baz", Mode::HelixNormal);
        cx.simulate_keystrokes("g e");
        cx.assert_state("«ˇfoo»BarBaz", Mode::HelixNormal);
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
            line four
            line five
            line six"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            line one
            «
            line three
            ˇ»line four
            line five
            line six"},
            Mode::HelixNormal,
        );

        // Another x should only select the next line
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            line one
            «
            line three
            line four
            ˇ»line five
            line six"},
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

        // Test selecting with an empty line below the current line
        cx.set_state(
            indoc! {"
            line one
            line twoˇ

            line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            line one
            «line two
            ˇ»
            line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            line one
            «line two

            ˇ»line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            line one
            «line two

            line four
            ˇ»line five"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_insert_before_after_select_lines(cx: &mut gpui::TestAppContext) {
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
        cx.simulate_keystrokes("o");
        cx.assert_state("line one\nline two\nline three\nˇ\nline four", Mode::Insert);

        cx.set_state(
            "line one\nline ˇtwo\nline three\nline four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("2 x");
        cx.assert_state(
            "line one\n«line two\nline three\nˇ»line four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("shift-o");
        cx.assert_state("line one\nˇ\nline two\nline three\nline four", Mode::Insert);
    }

    #[gpui::test]
    async fn test_helix_insert_before_after_helix_select(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Test new line in selection direction
        cx.set_state(
            "ˇline one\nline two\nline three\nline four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("v j j");
        cx.assert_state(
            "«line one\nline two\nlˇ»ine three\nline four",
            Mode::HelixSelect,
        );
        cx.simulate_keystrokes("o");
        cx.assert_state("line one\nline two\nline three\nˇ\nline four", Mode::Insert);

        cx.set_state(
            "line one\nline two\nˇline three\nline four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("v k k");
        cx.assert_state(
            "«ˇline one\nline two\nl»ine three\nline four",
            Mode::HelixSelect,
        );
        cx.simulate_keystrokes("shift-o");
        cx.assert_state("ˇ\nline one\nline two\nline three\nline four", Mode::Insert);

        // Test new line in opposite selection direction
        cx.set_state(
            "ˇline one\nline two\nline three\nline four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("v j j");
        cx.assert_state(
            "«line one\nline two\nlˇ»ine three\nline four",
            Mode::HelixSelect,
        );
        cx.simulate_keystrokes("shift-o");
        cx.assert_state("ˇ\nline one\nline two\nline three\nline four", Mode::Insert);

        cx.set_state(
            "line one\nline two\nˇline three\nline four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("v k k");
        cx.assert_state(
            "«ˇline one\nline two\nl»ine three\nline four",
            Mode::HelixSelect,
        );
        cx.simulate_keystrokes("o");
        cx.assert_state("line one\nline two\nline three\nˇ\nline four", Mode::Insert);
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
    async fn test_helix_select_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // v g l d should delete to end of line without consuming the newline
        cx.set_state("ˇThe quick brown\nfox jumps over", Mode::HelixNormal);
        cx.simulate_keystrokes("v g l d");
        cx.assert_state("ˇ\nfox jumps over", Mode::HelixNormal);

        // same from the middle of a line — cursor lands on the last
        // remaining character (the space) after delete
        cx.set_state("The ˇquick brown\nfox jumps over", Mode::HelixNormal);
        cx.simulate_keystrokes("v g l d");
        cx.assert_state("Theˇ \nfox jumps over", Mode::HelixNormal);
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
    async fn test_helix_select_next_match_wrapping(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Three occurrences of "one". After selecting all three with `n n`,
        // pressing `n` again wraps the search to the first occurrence.
        // The prior selections (at higher offsets) are chained before the
        // wrapped selection (at a lower offset), producing unsorted anchors
        // that cause `rope::Cursor::summary` to panic with
        // "cannot summarize backward".
        cx.set_state("ˇhello two one two one two one", Mode::HelixSelect);
        cx.simulate_keystrokes("/ o n e");
        cx.simulate_keystrokes("enter");
        cx.simulate_keystrokes("n n n");
        // Should not panic; all three occurrences should remain selected.
        cx.assert_state("hello two «oneˇ» two «oneˇ» two «oneˇ»", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_select_next_match_wrapping_from_normal(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Exact repro for #51573: start in HelixNormal, search, then `v` to
        // enter HelixSelect, then `n` past last match.
        //
        // In HelixNormal, search collapses the cursor to the match start.
        // Pressing `v` expands by only one character, creating a partial
        // selection that overlaps the full match range when the search wraps.
        // The overlapping ranges must be merged (not just deduped) to avoid
        // a backward-seeking rope cursor panic.
        cx.set_state(
            indoc! {"
                searˇch term
                stuff
                search term
                other stuff
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("/ t e r m");
        cx.simulate_keystrokes("enter");
        cx.simulate_keystrokes("v");
        cx.simulate_keystrokes("n");
        cx.simulate_keystrokes("n");
        // Should not panic when wrapping past last match.
        cx.assert_state(
            indoc! {"
                search «termˇ»
                stuff
                search «termˇ»
                other stuff
            "},
            Mode::HelixSelect,
        );
    }

    #[gpui::test]
    async fn test_helix_select_star_then_match(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Repro attempts for #52852: `*` searches for word under cursor,
        // `v` enters select, `n` accumulates matches, `m` triggers match mode.
        // Try multiple cursor positions and match counts.

        // Cursor on first occurrence, 3 more occurrences to select through
        cx.set_state(
            indoc! {"
                ˇone two one three one four one
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("*");
        cx.simulate_keystrokes("v");
        cx.simulate_keystrokes("n n n");
        // Should not panic on wrapping `n`.

        // Cursor in the middle of text before matches
        cx.set_state(
            indoc! {"
                heˇllo one two one three one
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("*");
        cx.simulate_keystrokes("v");
        cx.simulate_keystrokes("n");
        // Should not panic.

        // The original #52852 sequence: * v n n n then m m
        cx.set_state(
            indoc! {"
                fn ˇfoo() { bar(foo()) }
                fn baz() { foo() }
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("*");
        cx.simulate_keystrokes("v");
        cx.simulate_keystrokes("n n n");
        cx.simulate_keystrokes("m m");
        // Should not panic.
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

    #[gpui::test]
    async fn test_helix_jump_cancels_on_escape(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇhello world\njump labels", Mode::HelixNormal);
        let overlay_counts = active_helix_jump_overlay_counts(&mut cx);

        cx.simulate_keystrokes("g w");
        cx.simulate_keystrokes("escape");

        cx.assert_state("ˇhello world\njump labels", Mode::HelixNormal);
        assert_helix_jump_cleared(&mut cx, overlay_counts);
    }

    #[gpui::test]
    async fn test_helix_jump_cancels_on_invalid_first_char(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇalpha beta gamma", Mode::HelixNormal);
        let overlay_counts = active_helix_jump_overlay_counts(&mut cx);

        cx.simulate_keystrokes("g w");
        cx.simulate_keystrokes("z");

        cx.assert_state("ˇalpha beta gamma", Mode::HelixNormal);
        assert_helix_jump_cleared(&mut cx, overlay_counts);
    }

    #[gpui::test]
    async fn test_helix_jump_cancels_on_invalid_second_char(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇalpha beta gamma", Mode::HelixNormal);
        let overlay_counts = active_helix_jump_overlay_counts(&mut cx);

        cx.simulate_keystrokes("g w");
        cx.simulate_keystrokes("a z");

        cx.assert_state("ˇalpha beta gamma", Mode::HelixNormal);
        assert_helix_jump_cleared(&mut cx, overlay_counts);
    }

    #[gpui::test]
    async fn test_helix_jump_keeps_full_overlay_after_first_key(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        let text = format!(
            "ˇ{}",
            (0..28)
                .map(|index| format!("w{index:02}"))
                .collect::<Vec<_>>()
                .join(" ")
        );
        cx.set_state(&text, Mode::HelixNormal);

        cx.simulate_keystrokes("g w");
        let labels = active_helix_jump_labels(&mut cx);
        let initial_overlay_counts = active_helix_jump_overlay_counts(&mut cx);
        let first_group = labels
            .first()
            .and_then(|(label, _)| label.chars().next())
            .expect("expected at least one helix jump label");
        let next_group = labels
            .iter()
            .filter_map(|(label, _)| label.chars().next())
            .find(|ch| *ch != first_group)
            .expect("expected labels spanning more than one first-character group");

        cx.simulate_keystrokes(&next_group.to_string());

        assert_eq!(
            active_helix_jump_overlay_counts(&mut cx),
            initial_overlay_counts
        );
        assert!(
            matches!(
                cx.active_operator(),
                Some(Operator::HelixJump {
                    first_char: Some(ch),
                    ..
                }) if ch == next_group
            ),
            "expected HelixJump operator to keep the first typed label character"
        );
    }

    #[gpui::test]
    async fn test_helix_jump_includes_word_before_cursor_boundary(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("oneˇ two three", Mode::HelixNormal);

        jump_to_word(&mut cx, "one");

        cx.assert_state("«oneˇ» two three", Mode::HelixNormal);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_helix_jump_skips_single_char_words(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇa bb c dd e", Mode::HelixNormal);

        let words = helix_jump_labels_for_full_buffer(&mut cx)
            .into_iter()
            .map(|(_, word)| word)
            .collect::<Vec<_>>();

        assert_eq!(words, vec!["bb".to_string(), "dd".to_string()]);
    }

    #[gpui::test]
    async fn test_helix_jump_handles_underscored_words(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("baz quxˇ foo_bar _private", Mode::HelixNormal);

        let words = helix_jump_labels_for_full_buffer(&mut cx)
            .into_iter()
            .map(|(_, word)| word)
            .collect::<Vec<_>>();

        assert!(words.iter().any(|word| word == "foo_bar"));
        assert!(words.iter().any(|word| word == "_private"));
        assert!(!words.iter().any(|word| word == "foo"));
        assert!(!words.iter().any(|word| word == "bar"));
    }

    #[gpui::test]
    async fn test_helix_jump_at_end_of_buffer(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("alpha beta gammaˇ", Mode::HelixNormal);

        jump_to_word(&mut cx, "gamma");

        cx.assert_state("alpha beta «gammaˇ»", Mode::HelixNormal);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_helix_jump_moves_to_target_word(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇone two three", Mode::HelixNormal);

        jump_to_word(&mut cx, "three");

        cx.assert_state("one two «threeˇ»", Mode::HelixNormal);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_helix_jump_extends_selection_forward(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("one «twoˇ» three four", Mode::HelixSelect);

        jump_to_word(&mut cx, "four");

        cx.assert_state("one «two three fourˇ»", Mode::HelixSelect);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_helix_jump_extends_selection_backward_from_forward_selection(
        cx: &mut gpui::TestAppContext,
    ) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("one «twoˇ» three four", Mode::HelixSelect);

        jump_to_word(&mut cx, "one");

        cx.assert_state("«ˇone two» three four", Mode::HelixSelect);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_helix_jump_extends_reversed_selection_backward(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("one two «ˇthree» four", Mode::HelixSelect);

        jump_to_word(&mut cx, "one");

        cx.assert_state("«ˇone two three» four", Mode::HelixSelect);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_helix_jump_prioritizes_nearby_targets_before_truncating(
        cx: &mut gpui::TestAppContext,
    ) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        let cursor_index = 850usize;
        let target_word = format!("w{:03}", cursor_index + 1);
        let early_word = "w010".to_string();
        let text = (0..900usize)
            .map(|index| {
                let word = format!("w{index:03}");
                if index == cursor_index {
                    format!("ˇ{word}")
                } else {
                    word
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        cx.set_state(&text, Mode::HelixNormal);

        let labels = helix_jump_labels_for_full_buffer(&mut cx);

        assert_eq!(labels.len(), HELIX_JUMP_LABEL_LIMIT);
        assert!(
            labels.iter().any(|(_, word)| word == &target_word),
            "expected nearby target {target_word:?} to survive truncation"
        );
        assert!(
            !labels.iter().any(|(_, word)| word == &early_word),
            "expected distant early target {early_word:?} to be truncated first"
        );
    }

    #[gpui::test]
    async fn test_helix_jump_label_ordering_alternates_directions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("aaa bbb ccc ˇddd eee fff ggg", Mode::HelixNormal);

        let first_labels = helix_jump_labels_for_full_buffer(&mut cx)
            .into_iter()
            .take(6)
            .collect::<Vec<_>>();

        assert_eq!(
            first_labels,
            vec![
                ("aa".to_string(), "eee".to_string()),
                ("ab".to_string(), "ccc".to_string()),
                ("ac".to_string(), "fff".to_string()),
                ("ad".to_string(), "bbb".to_string()),
                ("ae".to_string(), "ggg".to_string()),
                ("af".to_string(), "aaa".to_string()),
            ]
        );
    }

    #[gpui::test]
    async fn test_helix_jump_uses_theme_label_color(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.theme.experimental_theme_overrides = Some(ThemeStyleContent {
                        colors: ThemeColorsContent {
                            vim_helix_jump_label_foreground: Some("#00ff00".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            });
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();

        let configured_label_color =
            cx.update(|_, cx| cx.theme().colors().vim_helix_jump_label_foreground);
        assert_ne!(
            configured_label_color,
            cx.update(|_, cx| cx.theme().status().error)
        );
        cx.set_state("ˇalpha beta gamma", Mode::HelixNormal);

        let label_colors = cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let display_snapshot = &snapshot.display_snapshot;
            let buffer_snapshot = display_snapshot.buffer_snapshot();
            let selections = editor.selections.all::<Point>(display_snapshot);
            let skip_data = Vim::selection_skip_offsets(buffer_snapshot, &selections, false);
            let cursor_offset = selections
                .first()
                .map(|selection| buffer_snapshot.point_to_offset(selection.head()))
                .unwrap_or(MultiBufferOffset(0));
            let style = editor.style(cx);
            let font = style.text.font();
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            let data = Vim::build_helix_jump_ui_data(
                buffer_snapshot,
                MultiBufferOffset(0),
                buffer_snapshot.len(),
                cursor_offset,
                configured_label_color,
                &skip_data,
                window.text_system(),
                font,
                font_size,
            );

            data.overlays
                .into_iter()
                .map(|overlay| overlay.label.text_color)
                .collect::<Vec<_>>()
        });

        assert!(!label_colors.is_empty());
        assert!(
            label_colors
                .into_iter()
                .all(|color| color == configured_label_color)
        );
    }

    #[gpui::test]
    async fn test_helix_jump_input_is_case_insensitive(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇone two three", Mode::HelixNormal);

        cx.simulate_keystrokes("g w");
        let label = helix_jump_label_for_word(&mut cx, "three");
        let mut chars = label.chars();
        let first = chars
            .next()
            .expect("jump labels are two characters long")
            .to_ascii_uppercase();
        let second = chars
            .next()
            .expect("jump labels are two characters long")
            .to_ascii_uppercase();

        cx.simulate_keystrokes(&format!("{first} {second}"));

        cx.assert_state("one two «threeˇ»", Mode::HelixNormal);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_helix_jump_with_unicode_words(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state("ˇcafé résumé naïve", Mode::HelixNormal);

        jump_to_word(&mut cx, "naïve");

        cx.assert_state("café résumé «naïveˇ»", Mode::HelixNormal);
        assert_eq!(cx.active_operator(), None);
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        cx.update(|cx| {
            VimTestContext::init_keybindings(true, cx);
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |store| store.helix_mode = Some(true));
            })
        });

        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(workspace, &DeploySearch::default(), window, cx)
        });

        let search_view = workspace.update_in(cx, |workspace, _, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view should be active")
        });

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

    #[gpui::test]
    async fn test_helix_insert_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Ensure that, when lines are selected using `x`, pressing `shift-a`
        // actually puts the cursor at the end of the selected lines and not at
        // the end of the line below.
        cx.set_state(
            indoc! {"
            line oˇne
            line two"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «line one
            ˇ»line two"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("shift-a");
        cx.assert_state(
            indoc! {"
            line oneˇ
            line two"},
            Mode::Insert,
        );

        cx.set_state(
            indoc! {"
            line «one
            lineˇ» two"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("shift-a");
        cx.assert_state(
            indoc! {"
            line one
            line twoˇ"},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_helix_replace_uses_graphemes(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("«Hällöˇ» Wörld", Mode::HelixNormal);
        cx.simulate_keystrokes("r 1");
        cx.assert_state("«11111ˇ» Wörld", Mode::HelixNormal);

        cx.set_state("«e\u{301}ˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("r 1");
        cx.assert_state("«1ˇ»", Mode::HelixNormal);

        cx.set_state("«🙂ˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("r 1");
        cx.assert_state("«1ˇ»", Mode::HelixNormal);
    }
}
