use gpui::{actions, impl_actions, AppContext, ViewContext};
use search::{buffer_search, BufferSearchBar, SearchMode, SearchOptions};
use serde_derive::Deserialize;
use workspace::{searchable::Direction, Pane, Workspace};

use crate::{state::SearchState, Vim};

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

impl_actions!(vim, [MoveToNext, MoveToPrev, Search]);
actions!(vim, [SearchSubmit]);

pub(crate) fn init(cx: &mut AppContext) {
    cx.add_action(move_to_next);
    cx.add_action(move_to_prev);
    cx.add_action(search);
    cx.add_action(search_submit);
    cx.add_action(search_deploy);
}

fn move_to_next(workspace: &mut Workspace, action: &MoveToNext, cx: &mut ViewContext<Workspace>) {
    move_to_internal(workspace, Direction::Next, !action.partial_word, cx)
}

fn move_to_prev(workspace: &mut Workspace, action: &MoveToPrev, cx: &mut ViewContext<Workspace>) {
    move_to_internal(workspace, Direction::Prev, !action.partial_word, cx)
}

fn search(workspace: &mut Workspace, action: &Search, cx: &mut ViewContext<Workspace>) {
    let pane = workspace.active_pane().clone();
    let direction = if action.backwards {
        Direction::Prev
    } else {
        Direction::Next
    };
    Vim::update(cx, |vim, cx| {
        let count = vim.take_count().unwrap_or(1);
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
                        search_bar.set_search_options(SearchOptions::CASE_SENSITIVE, cx);
                        search_bar.activate_search_mode(SearchMode::Regex, cx);
                    }
                    vim.workspace_state.search = SearchState {
                        direction,
                        count,
                        initial_query: query.clone(),
                    };
                });
            }
        })
    })
}

// hook into the existing to clear out any vim search state on cmd+f or edit -> find.
fn search_deploy(_: &mut Pane, _: &buffer_search::Deploy, cx: &mut ViewContext<Pane>) {
    Vim::update(cx, |vim, _| vim.workspace_state.search = Default::default());
    cx.propagate_action();
}

fn search_submit(workspace: &mut Workspace, _: &SearchSubmit, cx: &mut ViewContext<Workspace>) {
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
                });
            }
        });
    })
}

pub fn move_to_internal(
    workspace: &mut Workspace,
    direction: Direction,
    whole_word: bool,
    cx: &mut ViewContext<Workspace>,
) {
    Vim::update(cx, |vim, cx| {
        let pane = workspace.active_pane().clone();
        let count = vim.take_count().unwrap_or(1);
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                let search = search_bar.update(cx, |search_bar, cx| {
                    let mut options = SearchOptions::CASE_SENSITIVE;
                    options.set(SearchOptions::WHOLE_WORD, whole_word);
                    if search_bar.show(cx) {
                        search_bar
                            .query_suggestion(cx)
                            .map(|query| search_bar.search(&query, Some(options), cx))
                    } else {
                        None
                    }
                });

                if let Some(search) = search {
                    let search_bar = search_bar.downgrade();
                    cx.spawn(|_, mut cx| async move {
                        search.await?;
                        search_bar.update(&mut cx, |search_bar, cx| {
                            search_bar.select_match(direction, count, cx)
                        })?;
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                }
            }
        });
        vim.clear_operator(cx);
    });
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use editor::DisplayPoint;
    use search::BufferSearchBar;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_move_to_next(
        cx: &mut gpui::TestAppContext,
        deterministic: Arc<gpui::executor::Deterministic>,
    ) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["*"]);
        deterministic.run_until_parked();
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes(["*"]);
        deterministic.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["#"]);
        deterministic.run_until_parked();
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes(["#"]);
        deterministic.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["2", "*"]);
        deterministic.run_until_parked();
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["g", "*"]);
        deterministic.run_until_parked();
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["n"]);
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes(["g", "#"]);
        deterministic.run_until_parked();
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_search(
        cx: &mut gpui::TestAppContext,
        deterministic: Arc<gpui::executor::Deterministic>,
    ) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("aa\nbˇb\ncc\ncc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes(["/", "c", "c"]);

        let search_bar = cx.workspace(|workspace, cx| {
            workspace
                .active_pane()
                .read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<BufferSearchBar>()
                .expect("Buffer search bar should be deployed")
        });

        search_bar.read_with(cx.cx, |bar, cx| {
            assert_eq!(bar.query(cx), "cc");
        });

        deterministic.run_until_parked();

        cx.update_editor(|editor, cx| {
            let highlights = editor.all_background_highlights(cx);
            assert_eq!(3, highlights.len());
            assert_eq!(
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 2),
                highlights[0].0
            )
        });

        cx.simulate_keystrokes(["enter"]);
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);

        // n to go to next/N to go to previous
        cx.simulate_keystrokes(["n"]);
        cx.assert_state("aa\nbb\ncc\nˇcc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes(["shift-n"]);
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);

        // ?<enter> to go to previous
        cx.simulate_keystrokes(["?", "enter"]);
        deterministic.run_until_parked();
        cx.assert_state("aa\nbb\ncc\ncc\nˇcc\n", Mode::Normal);
        cx.simulate_keystrokes(["?", "enter"]);
        deterministic.run_until_parked();
        cx.assert_state("aa\nbb\ncc\nˇcc\ncc\n", Mode::Normal);

        // /<enter> to go to next
        cx.simulate_keystrokes(["/", "enter"]);
        deterministic.run_until_parked();
        cx.assert_state("aa\nbb\ncc\ncc\nˇcc\n", Mode::Normal);

        // ?{search}<enter> to search backwards
        cx.simulate_keystrokes(["?", "b", "enter"]);
        deterministic.run_until_parked();
        cx.assert_state("aa\nbˇb\ncc\ncc\ncc\n", Mode::Normal);

        // works with counts
        cx.simulate_keystrokes(["4", "/", "c"]);
        deterministic.run_until_parked();
        cx.simulate_keystrokes(["enter"]);
        cx.assert_state("aa\nbb\ncc\ncˇc\ncc\n", Mode::Normal);

        // check that searching resumes from cursor, not previous match
        cx.set_state("ˇaa\nbb\ndd\ncc\nbb\n", Mode::Normal);
        cx.simulate_keystrokes(["/", "d"]);
        deterministic.run_until_parked();
        cx.simulate_keystrokes(["enter"]);
        cx.assert_state("aa\nbb\nˇdd\ncc\nbb\n", Mode::Normal);
        cx.update_editor(|editor, cx| editor.move_to_beginning(&Default::default(), cx));
        cx.assert_state("ˇaa\nbb\ndd\ncc\nbb\n", Mode::Normal);
        cx.simulate_keystrokes(["/", "b"]);
        deterministic.run_until_parked();
        cx.simulate_keystrokes(["enter"]);
        cx.assert_state("aa\nˇbb\ndd\ncc\nbb\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_non_vim_search(
        cx: &mut gpui::TestAppContext,
        deterministic: Arc<gpui::executor::Deterministic>,
    ) {
        let mut cx = VimTestContext::new(cx, false).await;
        cx.set_state("ˇone one one one", Mode::Normal);
        cx.simulate_keystrokes(["cmd-f"]);
        deterministic.run_until_parked();

        cx.assert_editor_state("«oneˇ» one one one");
        cx.simulate_keystrokes(["enter"]);
        cx.assert_editor_state("one «oneˇ» one one");
        cx.simulate_keystrokes(["shift-enter"]);
        cx.assert_editor_state("«oneˇ» one one one");
    }
}
