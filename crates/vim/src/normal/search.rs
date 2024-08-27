use std::{iter::Peekable, str::Chars, time::Duration};

use editor::Editor;
use gpui::{actions, impl_actions, ViewContext};
use language::Point;
use search::{buffer_search, BufferSearchBar, SearchOptions};
use serde_derive::Deserialize;
use workspace::{notifications::NotifyResultExt, searchable::Direction};

use crate::{
    command::CommandRange,
    motion::Motion,
    state::{Mode, SearchState},
    Vim,
};

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MoveToNext {
    #[serde(default)]
    partial_word: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MoveToPrev {
    #[serde(default)]
    partial_word: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
pub(crate) struct Search {
    #[serde(default)]
    backwards: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FindCommand {
    pub query: String,
    pub backwards: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ReplaceCommand {
    pub(crate) range: Option<CommandRange>,
    pub(crate) replacement: Replacement,
}

#[derive(Debug, Default, PartialEq, Deserialize, Clone)]
pub(crate) struct Replacement {
    search: String,
    replacement: String,
    should_replace_all: bool,
    is_case_sensitive: bool,
}

actions!(vim, [SearchSubmit, MoveToNextMatch, MoveToPrevMatch]);
impl_actions!(
    vim,
    [FindCommand, ReplaceCommand, Search, MoveToPrev, MoveToNext]
);

pub(crate) fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, Vim::move_to_next);
    Vim::action(editor, cx, Vim::move_to_prev);
    Vim::action(editor, cx, Vim::move_to_next_match);
    Vim::action(editor, cx, Vim::move_to_prev_match);
    Vim::action(editor, cx, Vim::search);
    Vim::action(editor, cx, Vim::search_deploy);
    Vim::action(editor, cx, Vim::find_command);
    Vim::action(editor, cx, Vim::replace_command);
}

impl Vim {
    fn move_to_next(&mut self, action: &MoveToNext, cx: &mut ViewContext<Self>) {
        self.move_to_internal(Direction::Next, !action.partial_word, cx)
    }

    fn move_to_prev(&mut self, action: &MoveToPrev, cx: &mut ViewContext<Self>) {
        self.move_to_internal(Direction::Prev, !action.partial_word, cx)
    }

    fn move_to_next_match(&mut self, _: &MoveToNextMatch, cx: &mut ViewContext<Self>) {
        self.move_to_match_internal(self.search.direction, cx)
    }

    fn move_to_prev_match(&mut self, _: &MoveToPrevMatch, cx: &mut ViewContext<Self>) {
        self.move_to_match_internal(self.search.direction.opposite(), cx)
    }

    fn search(&mut self, action: &Search, cx: &mut ViewContext<Self>) {
        let Some(pane) = self.pane(cx) else { return };
        let direction = if action.backwards {
            Direction::Prev
        } else {
            Direction::Next
        };
        let count = self.take_count(cx).unwrap_or(1);
        let prior_selections = self.editor_selections(cx);
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                search_bar.update(cx, |search_bar, cx| {
                    if !search_bar.show(cx) {
                        return;
                    }
                    let query = search_bar.query(cx);

                    search_bar.select_query(cx);
                    cx.focus_self();

                    if query.is_empty() {
                        search_bar.set_replacement(None, cx);
                        search_bar.set_search_options(SearchOptions::REGEX, cx);
                    }
                    self.search = SearchState {
                        direction,
                        count,
                        initial_query: query.clone(),
                        prior_selections,
                        prior_operator: self.operator_stack.last().cloned(),
                        prior_mode: self.mode,
                    }
                });
            }
        })
    }

    // hook into the existing to clear out any vim search state on cmd+f or edit -> find.
    fn search_deploy(&mut self, _: &buffer_search::Deploy, cx: &mut ViewContext<Self>) {
        self.search = Default::default();
        cx.propagate();
    }

    pub fn search_submit(&mut self, cx: &mut ViewContext<Self>) {
        self.store_visual_marks(cx);
        let Some(pane) = self.pane(cx) else { return };
        let result = pane.update(cx, |pane, cx| {
            let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() else {
                return None;
            };
            search_bar.update(cx, |search_bar, cx| {
                let mut count = self.search.count;
                let direction = self.search.direction;
                // in the case that the query has changed, the search bar
                // will have selected the next match already.
                if (search_bar.query(cx) != self.search.initial_query)
                    && self.search.direction == Direction::Next
                {
                    count = count.saturating_sub(1)
                }
                self.search.count = 1;
                search_bar.select_match(direction, count, cx);
                search_bar.focus_editor(&Default::default(), cx);

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

        let new_selections = self.editor_selections(cx);

        // If the active editor has changed during a search, don't panic.
        if prior_selections.iter().any(|s| {
            self.update_editor(cx, |_, editor, cx| {
                !s.start.is_valid(&editor.snapshot(cx).buffer_snapshot)
            })
            .unwrap_or(true)
        }) {
            prior_selections.clear();
        }

        if prior_mode != self.mode {
            self.switch_mode(prior_mode, true, cx);
        }
        if let Some(operator) = prior_operator {
            self.push_operator(operator, cx);
        };
        self.search_motion(
            Motion::ZedSearchResult {
                prior_selections,
                new_selections,
            },
            cx,
        );
    }

    pub fn move_to_match_internal(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let Some(pane) = self.pane(cx) else { return };
        let count = self.take_count(cx).unwrap_or(1);
        let prior_selections = self.editor_selections(cx);

        let success = pane.update(cx, |pane, cx| {
            let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() else {
                return false;
            };
            search_bar.update(cx, |search_bar, cx| {
                if !search_bar.has_active_match() || !search_bar.show(cx) {
                    return false;
                }
                search_bar.select_match(direction, count, cx);
                true
            })
        });
        if !success {
            return;
        }

        let new_selections = self.editor_selections(cx);
        self.search_motion(
            Motion::ZedSearchResult {
                prior_selections,
                new_selections,
            },
            cx,
        );
    }

    pub fn move_to_internal(
        &mut self,
        direction: Direction,
        whole_word: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(pane) = self.pane(cx) else { return };
        let count = self.take_count(cx).unwrap_or(1);
        let prior_selections = self.editor_selections(cx);
        let vim = cx.view().clone();

        let searched = pane.update(cx, |pane, cx| {
            self.search.direction = direction;
            let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() else {
                return false;
            };
            let search = search_bar.update(cx, |search_bar, cx| {
                let options = SearchOptions::CASE_SENSITIVE | SearchOptions::REGEX;
                if !search_bar.show(cx) {
                    return None;
                }
                let Some(query) = search_bar.query_suggestion(cx) else {
                    drop(search_bar.search("", None, cx));
                    return None;
                };
                let mut query = regex::escape(&query);
                if whole_word {
                    query = format!(r"\<{}\>", query);
                }
                Some(search_bar.search(&query, Some(options), cx))
            });

            let Some(search) = search else { return false };

            let search_bar = search_bar.downgrade();
            cx.spawn(|_, mut cx| async move {
                search.await?;
                search_bar.update(&mut cx, |search_bar, cx| {
                    search_bar.select_match(direction, count, cx);

                    vim.update(cx, |vim, cx| {
                        let new_selections = vim.editor_selections(cx);
                        vim.search_motion(
                            Motion::ZedSearchResult {
                                prior_selections,
                                new_selections,
                            },
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
            self.clear_operator(cx)
        }

        if self.mode.is_visual() {
            self.switch_mode(Mode::Normal, false, cx)
        }
    }

    fn find_command(&mut self, action: &FindCommand, cx: &mut ViewContext<Self>) {
        let Some(pane) = self.pane(cx) else { return };
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                let search = search_bar.update(cx, |search_bar, cx| {
                    if !search_bar.show(cx) {
                        return None;
                    }
                    let mut query = action.query.clone();
                    if query == "" {
                        query = search_bar.query(cx);
                    };

                    Some(search_bar.search(
                        &query,
                        Some(SearchOptions::CASE_SENSITIVE | SearchOptions::REGEX),
                        cx,
                    ))
                });
                let Some(search) = search else { return };
                let search_bar = search_bar.downgrade();
                let direction = if action.backwards {
                    Direction::Prev
                } else {
                    Direction::Next
                };
                cx.spawn(|_, mut cx| async move {
                    search.await?;
                    search_bar.update(&mut cx, |search_bar, cx| {
                        search_bar.select_match(direction, 1, cx)
                    })?;
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
        })
    }

    fn replace_command(&mut self, action: &ReplaceCommand, cx: &mut ViewContext<Self>) {
        let replacement = action.replacement.clone();
        let Some(((pane, workspace), editor)) =
            self.pane(cx).zip(self.workspace(cx)).zip(self.editor())
        else {
            return;
        };
        if let Some(range) = &action.range {
            if let Some(result) = self.update_editor(cx, |vim, editor, cx| {
                let range = range.buffer_range(vim, editor, cx)?;
                let snapshot = &editor.snapshot(cx).buffer_snapshot;
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
        }
        let vim = cx.view().clone();
        pane.update(cx, |pane, cx| {
            let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() else {
                return;
            };
            let search = search_bar.update(cx, |search_bar, cx| {
                if !search_bar.show(cx) {
                    return None;
                }

                let mut options = SearchOptions::REGEX;
                if replacement.is_case_sensitive {
                    options.set(SearchOptions::CASE_SENSITIVE, true)
                }
                let search = if replacement.search == "" {
                    search_bar.query(cx)
                } else {
                    replacement.search
                };

                search_bar.set_replacement(Some(&replacement.replacement), cx);
                Some(search_bar.search(&search, Some(options), cx))
            });
            let Some(search) = search else { return };
            let search_bar = search_bar.downgrade();
            cx.spawn(|_, mut cx| async move {
                search.await?;
                search_bar.update(&mut cx, |search_bar, cx| {
                    if replacement.should_replace_all {
                        search_bar.select_last_match(cx);
                        search_bar.replace_all(&Default::default(), cx);
                        cx.spawn(|_, mut cx| async move {
                            cx.background_executor()
                                .timer(Duration::from_millis(200))
                                .await;
                            editor
                                .update(&mut cx, |editor, cx| editor.clear_search_within_ranges(cx))
                                .ok();
                        })
                        .detach();
                        vim.update(cx, |vim, cx| {
                            vim.move_cursor(
                                Motion::StartOfLine {
                                    display_lines: false,
                                },
                                None,
                                cx,
                            )
                        });
                    }
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        })
    }
}

impl Replacement {
    // convert a vim query into something more usable by zed.
    // we don't attempt to fully convert between the two regex syntaxes,
    // but we do flip \( and \) to ( and ) (and vice-versa) in the pattern,
    // and convert \0..\9 to $0..$9 in the replacement so that common idioms work.
    pub(crate) fn parse(mut chars: Peekable<Chars>) -> Option<Replacement> {
        let Some(delimiter) = chars
            .next()
            .filter(|c| !c.is_alphanumeric() && *c != '"' && *c != '|' && *c != '\'')
        else {
            return None;
        };

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
                if phase == 1 && c.is_digit(10) {
                    buffer.push('$')
                // unescape escaped parens
                } else if phase == 0 && c == '(' || c == ')' {
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
                if phase == 0 && c == '(' || c == ')' {
                    buffer.push('\\')
                }
                buffer.push(c)
            }
        }

        let mut replacement = Replacement {
            search,
            replacement,
            should_replace_all: true,
            is_case_sensitive: true,
        };

        for c in flags.chars() {
            match c {
                'g' | 'I' => {}
                'c' | 'n' => replacement.should_replace_all = false,
                'i' => replacement.is_case_sensitive = false,
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
    use editor::EditorSettings;
    use editor::{display_map::DisplayRow, DisplayPoint};
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
            store.update_user_settings::<EditorSettings>(cx, |s| s.search_wrap = Some(false));
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

        let search_bar = cx.workspace(|workspace, cx| {
            workspace
                .active_pane()
                .read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<BufferSearchBar>()
                .expect("Buffer search bar should be deployed")
        });

        cx.update_view(search_bar, |bar, cx| {
            assert_eq!(bar.query(cx), "cc");
        });

        cx.run_until_parked();

        cx.update_editor(|editor, cx| {
            let highlights = editor.all_text_background_highlights(cx);
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
        cx.update_editor(|editor, cx| editor.move_to_beginning(&Default::default(), cx));
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

        // check that searching with unable search wrap
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<EditorSettings>(cx, |s| s.search_wrap = Some(false));
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

    // cargo test -p vim --features neovim test_replace_with_range_at_start
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
        cx.executor().advance_clock(Duration::from_millis(250));
        cx.run_until_parked();

        cx.simulate_shared_keystrokes("/ a enter").await;
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

    // cargo test -p vim --features neovim test_replace_with_range
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
