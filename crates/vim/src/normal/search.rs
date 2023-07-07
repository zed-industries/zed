use gpui::{impl_actions, AppContext, ViewContext};
use search::{BufferSearchBar, SearchOptions};
use serde_derive::Deserialize;
use workspace::{searchable::Direction, Workspace};

use crate::Vim;

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

pub(crate) fn init(cx: &mut AppContext) {
    cx.add_action(move_to_next);
    cx.add_action(move_to_prev);
    cx.add_action(search);
}

fn move_to_next(workspace: &mut Workspace, action: &MoveToNext, cx: &mut ViewContext<Workspace>) {
    move_to_internal(workspace, Direction::Next, !action.partial_word, cx)
}

fn move_to_prev(workspace: &mut Workspace, action: &MoveToPrev, cx: &mut ViewContext<Workspace>) {
    move_to_internal(workspace, Direction::Prev, !action.partial_word, cx)
}

fn search(workspace: &mut Workspace, action: &Search, cx: &mut ViewContext<Workspace>) {
    let pane = workspace.active_pane().clone();
    pane.update(cx, |pane, cx| {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |search_bar, cx| {
                let options = SearchOptions::CASE_SENSITIVE | SearchOptions::REGEX;
                let direction = if action.backwards {
                    Direction::Prev
                } else {
                    Direction::Next
                };
                search_bar.select_match(direction, cx);
                search_bar.show_with_options(true, false, options, cx);
            })
        }
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
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                search_bar.update(cx, |search_bar, cx| {
                    let mut options = SearchOptions::CASE_SENSITIVE;
                    options.set(SearchOptions::WHOLE_WORD, whole_word);
                    search_bar.select_word_under_cursor(direction, options, cx);
                });
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
        let search_bar = cx.workspace(|workspace, cx| {
            workspace
                .active_pane()
                .read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<BufferSearchBar>()
                .expect("Buffer search bar should be deployed")
        });
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
    async fn test_search(cx: &mut gpui::TestAppContext) {
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
            assert_eq!(bar.query_editor.read(cx).text(cx), "cc");
        });

        // wait for the query editor change event to fire.
        search_bar.next_notification(&cx).await;

        cx.update_editor(|editor, cx| {
            let highlights = editor.all_background_highlights(cx);
            assert_eq!(3, highlights.len());
            assert_eq!(
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 2),
                highlights[0].0
            )
        });

        cx.simulate_keystrokes(["enter"]);

        // n to go to next/N to go to previous
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes(["n"]);
        cx.assert_state("aa\nbb\ncc\nˇcc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes(["shift-n"]);

        // ?<enter> to go to previous
        cx.assert_state("aa\nbb\nˇcc\ncc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes(["?", "enter"]);
        cx.assert_state("aa\nbb\ncc\ncc\nˇcc\n", Mode::Normal);
        cx.simulate_keystrokes(["?", "enter"]);

        // /<enter> to go to next
        cx.assert_state("aa\nbb\ncc\nˇcc\ncc\n", Mode::Normal);
        cx.simulate_keystrokes(["/", "enter"]);
        cx.assert_state("aa\nbb\ncc\ncc\nˇcc\n", Mode::Normal);

        // ?{search}<enter> to search backwards
        cx.simulate_keystrokes(["?", "b", "enter"]);

        // wait for the query editor change event to fire.
        search_bar.next_notification(&cx).await;

        cx.assert_state("aa\nbˇb\ncc\ncc\ncc\n", Mode::Normal);
    }
}
