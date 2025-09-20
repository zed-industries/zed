use editor::{Editor, EditorSettings};
use gpui::{Action, Context, Window, actions};
use language::Point;
use schemars::JsonSchema;
use search::{BufferSearchBar, SearchOptions, buffer_search};
use serde::Deserialize;
use settings::Settings;
use std::{iter::Peekable, str::Chars};
use util::serde::default_true;
use workspace::{notifications::NotifyResultExt, searchable::Direction};

use crate::{
    Vim,
    command::CommandRange,
    motion::Motion,
    state::{Mode, SearchState},
};

/// Moves to the next search match.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct MoveToNext {
    #[serde(default = "default_true")]
    case_sensitive: bool,
    #[serde(default)]
    partial_word: bool,
    #[serde(default = "default_true")]
    regex: bool,
}

/// Moves to the previous search match.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct MoveToPrevious {
    #[serde(default = "default_true")]
    case_sensitive: bool,
    #[serde(default)]
    partial_word: bool,
    #[serde(default = "default_true")]
    regex: bool,
}

/// Initiates a search operation with the specified parameters.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct Search {
    #[serde(default)]
    backwards: bool,
    #[serde(default = "default_true")]
    regex: bool,
}

/// Executes a find command to search for patterns in the buffer.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub struct FindCommand {
    pub query: String,
    pub backwards: bool,
}

/// Executes a search and replace command within the specified range.
#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
pub struct ReplaceCommand {
    pub(crate) range: CommandRange,
    pub(crate) replacement: Replacement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Replacement {
    search: String,
    replacement: String,
    case_sensitive: Option<bool>,
    flag_n: bool,
    flag_g: bool,
    flag_c: bool,
}

actions!(
    vim,
    [
        /// Submits the current search query.
        SearchSubmit,
        /// Moves to the next search match.
        MoveToNextMatch,
        /// Moves to the previous search match.
        MoveToPreviousMatch
    ]
);

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::move_to_next);
    Vim::action(editor, cx, Vim::move_to_previous);
    Vim::action(editor, cx, Vim::move_to_next_match);
    Vim::action(editor, cx, Vim::move_to_previous_match);
    Vim::action(editor, cx, Vim::search);
    Vim::action(editor, cx, Vim::search_deploy);
    Vim::action(editor, cx, Vim::find_command);
    Vim::action(editor, cx, Vim::replace_command);
}

impl Vim {
    fn move_to_next(&mut self, action: &MoveToNext, window: &mut Window, cx: &mut Context<Self>) {
        self.move_to_internal(
            Direction::Next,
            action.case_sensitive,
            !action.partial_word,
            action.regex,
            window,
            cx,
        )
    }

    fn move_to_previous(
        &mut self,
        action: &MoveToPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to_internal(
            Direction::Prev,
            action.case_sensitive,
            !action.partial_word,
            action.regex,
            window,
            cx,
        )
    }

    fn move_to_next_match(
        &mut self,
        _: &MoveToNextMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to_match_internal(self.search.direction, window, cx)
    }

    fn move_to_previous_match(
        &mut self,
        _: &MoveToPreviousMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to_match_internal(self.search.direction.opposite(), window, cx)
    }

    fn search(&mut self, action: &Search, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pane) = self.pane(window, cx) else {
            return;
        };
        let direction = if action.backwards {
            Direction::Prev
        } else {
            Direction::Next
        };
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
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
                    if action.regex {
                        options |= SearchOptions::REGEX;
                    }
                    if action.backwards {
                        options |= SearchOptions::BACKWARDS;
                    }
                    if EditorSettings::get_global(cx).search.case_sensitive {
                        options |= SearchOptions::CASE_SENSITIVE;
                    }
                    search_bar.set_search_options(options, cx);
                    let prior_mode = if self.temp_mode {
                        Mode::Insert
                    } else {
                        self.mode
                    };

                    self.search = SearchState {
                        direction,
                        count,
                        prior_selections,
                        prior_operator: self.operator_stack.last().cloned(),
                        prior_mode,
                    }
                });
            }
        })
    }

    // hook into the existing to clear out any vim search state on cmd+f or edit -> find.
    fn search_deploy(&mut self, _: &buffer_search::Deploy, _: &mut Window, cx: &mut Context<Self>) {
        // Preserve the current mode when resetting search state
        let current_mode = self.mode;
        self.search = Default::default();
        self.search.prior_mode = current_mode;
        cx.propagate();
    }

    pub fn search_submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.store_visual_marks(window, cx);
        let Some(pane) = self.pane(window, cx) else {
            return;
        };
        let new_selections = self.editor_selections(window, cx);
        let result = pane.update(cx, |pane, cx| {
            let search_bar = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>()?;
            search_bar.update(cx, |search_bar, cx| {
                let mut count = self.search.count;
                let direction = self.search.direction;
                search_bar.has_active_match();
                let new_head = new_selections.last()?.start;
                let is_different_head = self
                    .search
                    .prior_selections
                    .last()
                    .is_none_or(|range| range.start != new_head);

                if is_different_head {
                    count = count.saturating_sub(1)
                }
                self.search.count = 1;
                search_bar.select_match(direction, count, window, cx);
                search_bar.focus_editor(&Default::default(), window, cx);

                let prior_selections: Vec<_> = self.search.prior_selections.drain(..).collect();
                let prior_mode = self.search.prior_mode;
                let prior_operator = self.search.prior_operator.take();

                let query = search_bar.query(cx).into();
                Vim::globals(cx).registers.insert('/', query);
                Some((prior_selections, prior_mode, prior_operator))
            })
        });

        let Some((mut prior_selections, prior_mode, prior_operator)) = result else {
            return;
        };

        let new_selections = self.editor_selections(window, cx);

        // If the active editor has changed during a search, don't panic.
        if prior_selections.iter().any(|s| {
            self.update_editor(cx, |_, editor, cx| {
                !s.start
                    .is_valid(&editor.snapshot(window, cx).buffer_snapshot)
            })
            .unwrap_or(true)
        }) {
            prior_selections.clear();
        }

        if prior_mode != self.mode {
            self.switch_mode(prior_mode, true, window, cx);
        }
        if let Some(operator) = prior_operator {
            self.push_operator(operator, window, cx);
        };
        self.search_motion(
            Motion::ZedSearchResult {
                prior_selections,
                new_selections,
            },
            window,
            cx,
        );
    }

    pub fn move_to_match_internal(
        &mut self,
        direction: Direction,
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

        let new_selections = self.editor_selections(window, cx);
        self.search_motion(
            Motion::ZedSearchResult {
                prior_selections,
                new_selections,
            },
            window,
            cx,
        );
    }

    pub fn move_to_internal(
        &mut self,
        direction: Direction,
        case_sensitive: bool,
        whole_word: bool,
        regex: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(pane) = self.pane(window, cx) else {
            return;
        };
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        let prior_selections = self.editor_selections(window, cx);
        let cursor_word = self.editor_cursor_word(window, cx);
        let vim = cx.entity();

        let searched = pane.update(cx, |pane, cx| {
            self.search.direction = direction;
            let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() else {
                return false;
            };
            let search = search_bar.update(cx, |search_bar, cx| {
                let mut options = SearchOptions::NONE;
                if case_sensitive {
                    options |= SearchOptions::CASE_SENSITIVE;
                }
                if regex {
                    options |= SearchOptions::REGEX;
                }
                if whole_word {
                    options |= SearchOptions::WHOLE_WORD;
                }
                if !search_bar.show(window, cx) {
                    return None;
                }
                let Some(query) = search_bar
                    .query_suggestion(window, cx)
                    .or_else(|| cursor_word)
                else {
                    drop(search_bar.search("", None, false, window, cx));
                    return None;
                };

                let query = regex::escape(&query);
                Some(search_bar.search(&query, Some(options), true, window, cx))
            });

            let Some(search) = search else { return false };

            let search_bar = search_bar.downgrade();
            cx.spawn_in(window, async move |_, cx| {
                search.await?;
                search_bar.update_in(cx, |search_bar, window, cx| {
                    search_bar.select_match(direction, count, window, cx);

                    vim.update(cx, |vim, cx| {
                        let new_selections = vim.editor_selections(window, cx);
                        vim.search_motion(
                            Motion::ZedSearchResult {
                                prior_selections,
                                new_selections,
                            },
                            window,
                            cx,
                        )
                    });
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            true
        });
        if !searched {
            self.clear_operator(window, cx)
        }

        if self.mode.is_visual() {
            self.switch_mode(Mode::Normal, false, window, cx)
        }
    }

    fn find_command(&mut self, action: &FindCommand, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pane) = self.pane(window, cx) else {
            return;
        };
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                let search = search_bar.update(cx, |search_bar, cx| {
                    if !search_bar.show(window, cx) {
                        return None;
                    }
                    let mut query = action.query.clone();
                    if query.is_empty() {
                        query = search_bar.query(cx);
                    };

                    let mut options = SearchOptions::REGEX | SearchOptions::CASE_SENSITIVE;
                    if search_bar.should_use_smartcase_search(cx) {
                        options.set(
                            SearchOptions::CASE_SENSITIVE,
                            search_bar.is_contains_uppercase(&query),
                        );
                    }

                    Some(search_bar.search(&query, Some(options), true, window, cx))
                });
                let Some(search) = search else { return };
                let search_bar = search_bar.downgrade();
                let direction = if action.backwards {
                    Direction::Prev
                } else {
                    Direction::Next
                };
                cx.spawn_in(window, async move |_, cx| {
                    search.await?;
                    search_bar.update_in(cx, |search_bar, window, cx| {
                        search_bar.select_match(direction, 1, window, cx)
                    })?;
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
        })
    }

    fn replace_command(
        &mut self,
        action: &ReplaceCommand,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let replacement = action.replacement.clone();
        let Some(((pane, workspace), editor)) = self
            .pane(window, cx)
            .zip(self.workspace(window))
            .zip(self.editor())
        else {
            return;
        };
        if let Some(result) = self.update_editor(cx, |vim, editor, cx| {
            let range = action.range.buffer_range(vim, editor, window, cx)?;
            let snapshot = &editor.snapshot(window, cx).buffer_snapshot;
            let end_point = Point::new(range.end.0, snapshot.line_len(range.end));
            let range = snapshot.anchor_before(Point::new(range.start.0, 0))
                ..snapshot.anchor_after(end_point);
            editor.set_search_within_ranges(&[range], cx);
            anyhow::Ok(())
        }) {
            workspace.update(cx, |workspace, cx| {
                result.notify_err(workspace, cx);
            })
        }
        let Some(search_bar) = pane.update(cx, |pane, cx| {
            pane.toolbar().read(cx).item_of_type::<BufferSearchBar>()
        }) else {
            return;
        };
        let mut options = SearchOptions::REGEX;
        let search = search_bar.update(cx, |search_bar, cx| {
            if !search_bar.show(window, cx) {
                return None;
            }

            let search = if replacement.search.is_empty() {
                search_bar.query(cx)
            } else {
                replacement.search
            };

            if let Some(case) = replacement.case_sensitive {
                options.set(SearchOptions::CASE_SENSITIVE, case)
            } else if search_bar.should_use_smartcase_search(cx) {
                options.set(
                    SearchOptions::CASE_SENSITIVE,
                    search_bar.is_contains_uppercase(&search),
                );
            } else {
                options.set(SearchOptions::CASE_SENSITIVE, false)
            }

            if !replacement.flag_g {
                options.set(SearchOptions::ONE_MATCH_PER_LINE, true);
            }

            search_bar.set_replacement(Some(&replacement.replacement), cx);
            if replacement.flag_c {
                search_bar.focus_replace(window, cx);
            }
            Some(search_bar.search(&search, Some(options), true, window, cx))
        });
        if replacement.flag_n {
            self.move_cursor(
                Motion::StartOfLine {
                    display_lines: false,
                },
                None,
                window,
                cx,
            );
            return;
        }
        let Some(search) = search else { return };
        let search_bar = search_bar.downgrade();
        cx.spawn_in(window, async move |vim, cx| {
            search.await?;
            search_bar.update_in(cx, |search_bar, window, cx| {
                if replacement.flag_c {
                    search_bar.select_first_match(window, cx);
                    return;
                }
                search_bar.select_last_match(window, cx);
                search_bar.replace_all(&Default::default(), window, cx);
                editor.update(cx, |editor, cx| editor.clear_search_within_ranges(cx));
                let _ = search_bar.search(&search_bar.query(cx), None, false, window, cx);
                vim.update(cx, |vim, cx| {
                    vim.move_cursor(
                        Motion::StartOfLine {
                            display_lines: false,
                        },
                        None,
                        window,
                        cx,
                    )
                })
                .ok();

                // Disable the `ONE_MATCH_PER_LINE` search option when finished, as
                // this is not properly supported outside of vim mode, and
                // not disabling it makes the "Replace All Matches" button
                // actually replace only the first match on each line.
                options.set(SearchOptions::ONE_MATCH_PER_LINE, false);
                search_bar.set_search_options(options, cx);
            })
        })
        .detach_and_log_err(cx);
    }
}

impl Replacement {
    // convert a vim query into something more usable by zed.
    // we don't attempt to fully convert between the two regex syntaxes,
    // but we do flip \( and \) to ( and ) (and vice-versa) in the pattern,
    // and convert \0..\9 to $0..$9 in the replacement so that common idioms work.
    pub(crate) fn parse(mut chars: Peekable<Chars>) -> Option<Replacement> {
        let delimiter = chars
            .next()
            .filter(|c| !c.is_alphanumeric() && *c != '"' && *c != '|' && *c != '\'')?;

        let mut search = String::new();
        let mut replacement = String::new();
        let mut flags = String::new();

        let mut buffer = &mut search;

        let mut escaped = false;
        // 0 - parsing search
        // 1 - parsing replacement
        // 2 - parsing flags
        let mut phase = 0;

        for c in chars {
            if escaped {
                escaped = false;
                if phase == 1 && c.is_ascii_digit() {
                    buffer.push('$')
                // unescape escaped parens
                } else if phase == 0 && (c == '(' || c == ')') {
                } else if c != delimiter {
                    buffer.push('\\')
                }
                buffer.push(c)
            } else if c == '\\' {
                escaped = true;
            } else if c == delimiter {
                if phase == 0 {
                    buffer = &mut replacement;
                    phase = 1;
                } else if phase == 1 {
                    buffer = &mut flags;
                    phase = 2;
                } else {
                    break;
                }
            } else {
                // escape unescaped parens
                if phase == 0 && (c == '(' || c == ')') {
                    buffer.push('\\')
                }
                buffer.push(c)
            }
        }

        let mut replacement = Replacement {
            search,
            replacement,
            case_sensitive: None,
            flag_g: false,
            flag_n: false,
            flag_c: false,
        };

        for c in flags.chars() {
            match c {
                'g' => replacement.flag_g = true,
                'n' => replacement.flag_n = true,
                'c' => replacement.flag_c = true,
                'i' => replacement.case_sensitive = Some(false),
                'I' => replacement.case_sensitive = Some(true),
                _ => {}
            }
        }

        Some(replacement)
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use editor::{DisplayPoint, display_map::DisplayRow};

    use indoc::indoc;
    use search::BufferSearchBar;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_move_to_next(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("*");
        cx.run_until_parked();
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes("*");
        cx.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("#");
        cx.run_until_parked();
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes("#");
        cx.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("2 *");
        cx.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("g *");
        cx.run_until_parked();
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("n");
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes("g #");
        cx.run_until_parked();
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_move_to_next_with_no_search_wrap(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |s| s.editor.search_wrap = Some(false));
        });

        cx.set_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("*");
        cx.run_until_parked();
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes("*");
        cx.run_until_parked();
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes("#");
        cx.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("3 *");
        cx.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("g *");
        cx.run_until_parked();
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes("n");
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes("g #");
        cx.run_until_parked();
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_search(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("aa\nbˇb\ncc\ncc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes("/ c c");

        let search_bar = cx.workspace(|workspace, _, cx| {
            workspace
                .active_pane()
                .read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<BufferSearchBar>()
                .expect("Buffer search bar should be deployed")
        });

        cx.update_entity(search_bar, |bar, _window, cx| {
            assert_eq!(bar.query(cx), "cc");
        });

        cx.run_until_parked();

        cx.update_editor(|editor, window, cx| {
            let highlights = editor.all_text_background_highlights(window, cx);
            assert_eq!(3, highlights.len());
            assert_eq!(
                DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(2), 2),
                highlights[0].0
            )
        });

        cx.simulate_keystrokes("enter");
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);

        // n to go to next/N to go to previous
        cx.simulate_keystrokes("n");
        cx.assert_state("aa\nbb\ncc\nˇcc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes("shift-n");
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);

        // ?<enter> to go to previous
        cx.simulate_keystrokes("? enter");
        cx.assert_state("aa\nbb\ncc\ncc\nˇcc\n", Mode::Normal);
        cx.simulate_keystrokes("? enter");
        cx.assert_state("aa\nbb\ncc\nˇcc\ncc\n", Mode::Normal);

        // /<enter> to go to next
        cx.simulate_keystrokes("/ enter");
        cx.assert_state("aa\nbb\ncc\ncc\nˇcc\n", Mode::Normal);

        // ?{search}<enter> to search backwards
        cx.simulate_keystrokes("? b enter");
        cx.assert_state("aa\nbˇb\ncc\ncc\ncc\n", Mode::Normal);

        // works with counts
        cx.simulate_keystrokes("4 / c");
        cx.simulate_keystrokes("enter");
        cx.assert_state("aa\nbb\ncc\ncˇc\ncc\n", Mode::Normal);

        // check that searching resumes from cursor, not previous match
        cx.set_state("ˇaa\nbb\ndd\ncc\nbb\n", Mode::Normal);
        cx.simulate_keystrokes("/ d");
        cx.simulate_keystrokes("enter");
        cx.assert_state("aa\nbb\nˇdd\ncc\nbb\n", Mode::Normal);
        cx.update_editor(|editor, window, cx| {
            editor.move_to_beginning(&Default::default(), window, cx)
        });
        cx.assert_state("ˇaa\nbb\ndd\ncc\nbb\n", Mode::Normal);
        cx.simulate_keystrokes("/ b");
        cx.simulate_keystrokes("enter");
        cx.assert_state("aa\nˇbb\ndd\ncc\nbb\n", Mode::Normal);

        // check that searching switches to normal mode if in visual mode
        cx.set_state("ˇone two one", Mode::Normal);
        cx.simulate_keystrokes("v l l");
        cx.assert_editor_state("«oneˇ» two one");
        cx.simulate_keystrokes("*");
        cx.assert_state("one two ˇone", Mode::Normal);

        // check that a backward search after last match works correctly
        cx.set_state("aa\naa\nbbˇ", Mode::Normal);
        cx.simulate_keystrokes("? a a");
        cx.simulate_keystrokes("enter");
        cx.assert_state("aa\nˇaa\nbb", Mode::Normal);

        // check that searching with unable search wrap
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |s| s.editor.search_wrap = Some(false));
        });
        cx.set_state("aa\nbˇb\ncc\ncc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes("/ c c enter");

        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);

        // n to go to next/N to go to previous
        cx.simulate_keystrokes("n");
        cx.assert_state("aa\nbb\ncc\nˇcc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes("shift-n");
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);

        // ?<enter> to go to previous
        cx.simulate_keystrokes("? enter");
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes("? enter");
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_non_vim_search(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, false).await;
        cx.cx.set_state("ˇone one one one");
        cx.run_until_parked();
        cx.simulate_keystrokes("cmd-f");
        cx.run_until_parked();

        cx.assert_editor_state("«oneˇ» one one one");
        cx.simulate_keystrokes("enter");
        cx.assert_editor_state("one «oneˇ» one one");
        cx.simulate_keystrokes("shift-enter");
        cx.assert_editor_state("«oneˇ» one one one");
    }

    #[gpui::test]
    async fn test_visual_star_hash(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇa.c. abcd a.c. abcd").await;
        cx.simulate_shared_keystrokes("v 3 l *").await;
        cx.shared_state().await.assert_eq("a.c. abcd ˇa.c. abcd");
    }

    #[gpui::test]
    async fn test_d_search(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇa.c. abcd a.c. abcd").await;
        cx.simulate_shared_keystrokes("d / c d").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq("ˇcd a.c. abcd");
    }

    #[gpui::test]
    async fn test_backwards_n(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇa b a b a b a").await;
        cx.simulate_shared_keystrokes("*").await;
        cx.simulate_shared_keystrokes("n").await;
        cx.shared_state().await.assert_eq("a b a b ˇa b a");
        cx.simulate_shared_keystrokes("#").await;
        cx.shared_state().await.assert_eq("a b ˇa b a b a");
        cx.simulate_shared_keystrokes("n").await;
        cx.shared_state().await.assert_eq("ˇa b a b a b a");
    }

    #[gpui::test]
    async fn test_v_search(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇa.c. abcd a.c. abcd").await;
        cx.simulate_shared_keystrokes("v / c d").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq("«a.c. abcˇ»d a.c. abcd");

        cx.set_shared_state("a a aˇ a a a").await;
        cx.simulate_shared_keystrokes("v / a").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq("a a a« aˇ» a a");
        cx.simulate_shared_keystrokes("/ enter").await;
        cx.shared_state().await.assert_eq("a a a« a aˇ» a");
        cx.simulate_shared_keystrokes("? enter").await;
        cx.shared_state().await.assert_eq("a a a« aˇ» a a");
        cx.simulate_shared_keystrokes("? enter").await;
        cx.shared_state().await.assert_eq("a a «ˇa »a a a");
        cx.simulate_shared_keystrokes("/ enter").await;
        cx.shared_state().await.assert_eq("a a a« aˇ» a a");
        cx.simulate_shared_keystrokes("/ enter").await;
        cx.shared_state().await.assert_eq("a a a« a aˇ» a");
    }

    #[gpui::test]
    async fn test_v_search_aa(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇaa aa").await;
        cx.simulate_shared_keystrokes("v / a a").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq("«aa aˇ»a");
    }

    #[gpui::test]
    async fn test_visual_block_search(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "ˇone two
             three four
             five six
             "
        })
        .await;
        cx.simulate_shared_keystrokes("ctrl-v j / f").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq(indoc! {
            "«one twoˇ»
             «three fˇ»our
             five six
             "
        });
    }

    #[gpui::test]
    async fn test_replace_with_range_at_start(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "ˇa
            a
            a
            a
            a
            a
            a
             "
        })
        .await;
        cx.simulate_shared_keystrokes(": 2 , 5 s / ^ / b").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq(indoc! {
            "a
            ba
            ba
            ba
            ˇba
            a
            a
             "
        });

        cx.simulate_shared_keystrokes("/ a").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq(indoc! {
            "a
                ba
                ba
                ba
                bˇa
                a
                a
                 "
        });
    }

    #[gpui::test]
    async fn test_search_skipping(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {
            "ˇaa aa aa"
        })
        .await;

        cx.simulate_shared_keystrokes("/ a a").await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {
            "aa ˇaa aa"
        });

        cx.simulate_shared_keystrokes("left / a a").await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {
            "aa ˇaa aa"
        });
    }

    #[gpui::test]
    async fn test_replace_n(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {
            "ˇaa
            bb
            aa"
        })
        .await;

        cx.simulate_shared_keystrokes(": s / b b / d d / n").await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {
            "ˇaa
            bb
            aa"
        });

        let search_bar = cx.update_workspace(|workspace, _, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
                    .unwrap()
            })
        });
        cx.update_entity(search_bar, |search_bar, _, cx| {
            assert!(!search_bar.is_dismissed());
            assert_eq!(search_bar.query(cx), "bb".to_string());
            assert_eq!(search_bar.replacement(cx), "dd".to_string());
        })
    }

    #[gpui::test]
    async fn test_replace_g(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {
            "ˇaa aa aa aa
            aa
            aa"
        })
        .await;

        cx.simulate_shared_keystrokes(": s / a a / b b").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇbb aa aa aa
            aa
            aa"
        });
        cx.simulate_shared_keystrokes(": s / a a / b b / g").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇbb bb bb bb
            aa
            aa"
        });
    }

    #[gpui::test]
    async fn test_replace_c(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            indoc! {
                "ˇaa
            aa
            aa"
            },
            Mode::Normal,
        );

        cx.simulate_keystrokes("v j : s / a a / d d / c");
        cx.simulate_keystrokes("enter");

        cx.assert_state(
            indoc! {
                "ˇaa
            aa
            aa"
            },
            Mode::Normal,
        );

        cx.simulate_keystrokes("enter");

        cx.assert_state(
            indoc! {
                "dd
            ˇaa
            aa"
            },
            Mode::Normal,
        );

        cx.simulate_keystrokes("enter");
        cx.assert_state(
            indoc! {
                "dd
            ddˇ
            aa"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("enter");
        cx.assert_state(
            indoc! {
                "dd
            ddˇ
            aa"
            },
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_replace_with_range(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "ˇa
            a
            a
            a
            a
            a
            a
             "
        })
        .await;
        cx.simulate_shared_keystrokes(": 2 , 5 s / a / b").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq(indoc! {
            "a
            b
            b
            b
            ˇb
            a
            a
             "
        });
        cx.executor().advance_clock(Duration::from_millis(250));
        cx.run_until_parked();

        cx.simulate_shared_keystrokes("/ a enter").await;
        cx.shared_state().await.assert_eq(indoc! {
            "a
                b
                b
                b
                b
                ˇa
                a
                 "
        });
    }
}
