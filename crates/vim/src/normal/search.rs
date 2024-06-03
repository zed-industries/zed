use std::{ops::Range, sync::OnceLock, time::Duration};

use gpui::{actions, impl_actions, ViewContext};
use language::Point;
use regex::Regex;
use search::{buffer_search, BufferSearchBar, SearchOptions};
use serde_derive::Deserialize;
use workspace::{searchable::Direction, Workspace};

use crate::{
    motion::{search_motion, Motion},
    normal::move_cursor,
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
    pub query: String,
}

#[derive(Debug, Default)]
struct Replacement {
    search: String,
    replacement: String,
    should_replace_all: bool,
    is_case_sensitive: bool,
    range: Option<Range<usize>>,
}

actions!(vim, [SearchSubmit, MoveToNextMatch, MoveToPrevMatch]);
impl_actions!(
    vim,
    [FindCommand, ReplaceCommand, Search, MoveToPrev, MoveToNext]
);

static RANGE_REGEX: OnceLock<Regex> = OnceLock::new();
pub(crate) fn range_regex() -> &'static Regex {
    RANGE_REGEX.get_or_init(|| Regex::new(r"^(\d+),(\d+)s(.*)").unwrap())
}

pub(crate) fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(move_to_next);
    workspace.register_action(move_to_prev);
    workspace.register_action(move_to_next_match);
    workspace.register_action(move_to_prev_match);
    workspace.register_action(search);
    workspace.register_action(search_submit);
    workspace.register_action(search_deploy);

    workspace.register_action(find_command);
    workspace.register_action(replace_command);
}

fn move_to_next(workspace: &mut Workspace, action: &MoveToNext, cx: &mut ViewContext<Workspace>) {
    move_to_internal(workspace, Direction::Next, !action.partial_word, cx)
}

fn move_to_prev(workspace: &mut Workspace, action: &MoveToPrev, cx: &mut ViewContext<Workspace>) {
    move_to_internal(workspace, Direction::Prev, !action.partial_word, cx)
}

fn move_to_next_match(
    workspace: &mut Workspace,
    _: &MoveToNextMatch,
    cx: &mut ViewContext<Workspace>,
) {
    move_to_match_internal(workspace, Direction::Next, cx)
}

fn move_to_prev_match(
    workspace: &mut Workspace,
    _: &MoveToPrevMatch,
    cx: &mut ViewContext<Workspace>,
) {
    move_to_match_internal(workspace, Direction::Prev, cx)
}

fn search(workspace: &mut Workspace, action: &Search, cx: &mut ViewContext<Workspace>) {
    let pane = workspace.active_pane().clone();
    let direction = if action.backwards {
        Direction::Prev
    } else {
        Direction::Next
    };
    Vim::update(cx, |vim, cx| {
        let count = vim.take_count(cx).unwrap_or(1);
        let prior_selections = vim.editor_selections(cx);
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
                    vim.workspace_state.search = SearchState {
                        direction,
                        count,
                        initial_query: query.clone(),
                        prior_selections,
                        prior_operator: vim.active_operator(),
                        prior_mode: vim.state().mode,
                    };
                });
            }
        })
    })
}

// hook into the existing to clear out any vim search state on cmd+f or edit -> find.
fn search_deploy(_: &mut Workspace, _: &buffer_search::Deploy, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, _| vim.workspace_state.search = Default::default());
    cx.propagate();
}

fn search_submit(workspace: &mut Workspace, _: &SearchSubmit, cx: &mut ViewContext<Workspace>) {
    let mut motion = None;
    Vim::update(cx, |vim, cx| {
        let pane = workspace.active_pane().clone();
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                search_bar.update(cx, |search_bar, cx| {
                    let state = &mut vim.workspace_state.search;
                    let mut count = state.count;
                    let direction = state.direction;

                    // in the case that the query has changed, the search bar
                    // will have selected the next match already.
                    if (search_bar.query(cx) != state.initial_query)
                        && state.direction == Direction::Next
                    {
                        count = count.saturating_sub(1)
                    }
                    state.count = 1;
                    search_bar.select_match(direction, count, cx);
                    search_bar.focus_editor(&Default::default(), cx);

                    let mut prior_selections: Vec<_> = state.prior_selections.drain(..).collect();
                    let prior_mode = state.prior_mode;
                    let prior_operator = state.prior_operator.take();
                    let new_selections = vim.editor_selections(cx);

                    // If the active editor has changed during a search, don't panic.
                    if prior_selections.iter().any(|s| {
                        vim.update_active_editor(cx, |_vim, editor, cx| {
                            !s.start.is_valid(&editor.snapshot(cx).buffer_snapshot)
                        })
                        .unwrap_or(true)
                    }) {
                        prior_selections.clear();
                    }

                    if prior_mode != vim.state().mode {
                        vim.switch_mode(prior_mode, true, cx);
                    }
                    if let Some(operator) = prior_operator {
                        vim.push_operator(operator, cx);
                    };
                    motion = Some(Motion::ZedSearchResult {
                        prior_selections,
                        new_selections,
                    });
                });
            }
        });
    });

    if let Some(motion) = motion {
        search_motion(motion, cx)
    }
}

pub fn move_to_match_internal(
    workspace: &mut Workspace,
    direction: Direction,
    cx: &mut ViewContext<Workspace>,
) {
    let mut motion = None;
    Vim::update(cx, |vim, cx| {
        let pane = workspace.active_pane().clone();
        let count = vim.take_count(cx).unwrap_or(1);
        let prior_selections = vim.editor_selections(cx);

        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                search_bar.update(cx, |search_bar, cx| {
                    if !search_bar.has_active_match() || !search_bar.show(cx) {
                        return;
                    }
                    search_bar.select_match(direction, count, cx);

                    let new_selections = vim.editor_selections(cx);
                    motion = Some(Motion::ZedSearchResult {
                        prior_selections,
                        new_selections,
                    });
                })
            }
        })
    });
    if let Some(motion) = motion {
        search_motion(motion, cx);
    }
}

pub fn move_to_internal(
    workspace: &mut Workspace,
    direction: Direction,
    whole_word: bool,
    cx: &mut ViewContext<Workspace>,
) {
    Vim::update(cx, |vim, cx| {
        let pane = workspace.active_pane().clone();
        let count = vim.take_count(cx).unwrap_or(1);
        let prior_selections = vim.editor_selections(cx);

        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                let search = search_bar.update(cx, |search_bar, cx| {
                    let options = SearchOptions::CASE_SENSITIVE | SearchOptions::REGEX;
                    if !search_bar.show(cx) {
                        return None;
                    }
                    let Some(query) = search_bar.query_suggestion(cx) else {
                        vim.clear_operator(cx);
                        let _ = search_bar.search("", None, cx);
                        return None;
                    };
                    let mut query = regex::escape(&query);
                    if whole_word {
                        query = format!(r"\<{}\>", query);
                    }
                    Some(search_bar.search(&query, Some(options), cx))
                });

                if let Some(search) = search {
                    let search_bar = search_bar.downgrade();
                    cx.spawn(|_, mut cx| async move {
                        search.await?;
                        search_bar.update(&mut cx, |search_bar, cx| {
                            search_bar.select_match(direction, count, cx);

                            let new_selections =
                                Vim::update(cx, |vim, cx| vim.editor_selections(cx));
                            search_motion(
                                Motion::ZedSearchResult {
                                    prior_selections,
                                    new_selections,
                                },
                                cx,
                            )
                        })?;
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                }
            }
        });

        if vim.state().mode.is_visual() {
            vim.switch_mode(Mode::Normal, false, cx)
        }
    });
}

fn find_command(workspace: &mut Workspace, action: &FindCommand, cx: &mut ViewContext<Workspace>) {
    let pane = workspace.active_pane().clone();
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

fn replace_command(
    workspace: &mut Workspace,
    action: &ReplaceCommand,
    cx: &mut ViewContext<Workspace>,
) {
    let replacement = parse_replace_all(&action.query);
    let pane = workspace.active_pane().clone();
    let mut editor = Vim::read(cx)
        .active_editor
        .as_ref()
        .and_then(|editor| editor.upgrade());
    if let Some(range) = &replacement.range {
        if let Some(editor) = editor.as_mut() {
            editor.update(cx, |editor, cx| {
                let snapshot = &editor.snapshot(cx).buffer_snapshot;
                let range = snapshot
                    .anchor_before(Point::new(range.start.saturating_sub(1) as u32, 0))
                    ..snapshot.anchor_before(Point::new(range.end as u32, 0));
                editor.set_search_within_ranges(&[range], cx)
            })
        }
    }
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
                    if let Some(editor) = editor {
                        cx.spawn(|_, mut cx| async move {
                            cx.background_executor()
                                .timer(Duration::from_millis(200))
                                .await;
                            editor
                                .update(&mut cx, |editor, cx| editor.clear_search_within_ranges(cx))
                                .ok();
                        })
                        .detach();
                    }
                    Vim::update(cx, |vim, cx| {
                        move_cursor(
                            vim,
                            Motion::StartOfLine {
                                display_lines: false,
                            },
                            None,
                            cx,
                        )
                    })
                }
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    })
}

// convert a vim query into something more usable by zed.
// we don't attempt to fully convert between the two regex syntaxes,
// but we do flip \( and \) to ( and ) (and vice-versa) in the pattern,
// and convert \0..\9 to $0..$9 in the replacement so that common idioms work.
fn parse_replace_all(query: &str) -> Replacement {
    let mut chars = query.chars();
    let mut range = None;
    let maybe_line_range_and_rest: Option<(Range<usize>, &str)> =
        range_regex().captures(query).map(|captures| {
            (
                captures.get(1).unwrap().as_str().parse().unwrap()
                    ..captures.get(2).unwrap().as_str().parse().unwrap(),
                captures.get(3).unwrap().as_str(),
            )
        });
    if maybe_line_range_and_rest.is_some() {
        let (line_range, rest) = maybe_line_range_and_rest.unwrap();
        range = Some(line_range);
        chars = rest.chars();
    } else if Some('%') != chars.next() || Some('s') != chars.next() {
        return Replacement::default();
    }

    let Some(delimiter) = chars.next() else {
        return Replacement::default();
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
        range,
    };

    for c in flags.chars() {
        match c {
            'g' | 'I' => {}
            'c' | 'n' => replacement.should_replace_all = false,
            'i' => replacement.is_case_sensitive = false,
            _ => {}
        }
    }

    replacement
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use editor::{display_map::DisplayRow, DisplayPoint};
    use indoc::indoc;
    use search::BufferSearchBar;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

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
    }

    #[gpui::test]
    async fn test_non_vim_search(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, false).await;
        cx.set_state("ˇone one one one", Mode::Normal);
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
