use gpui::{impl_actions, ViewContext};
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

impl_actions!(vim, [MoveToNext, MoveToPrev]);

pub(crate) fn move_to_next(
    workspace: &mut Workspace,
    action: &MoveToNext,
    cx: &mut ViewContext<Workspace>,
) {
    move_to_internal(workspace, Direction::Next, !action.partial_word, cx)
}

pub(crate) fn move_to_prev(
    workspace: &mut Workspace,
    action: &MoveToPrev,
    cx: &mut ViewContext<Workspace>,
) {
    move_to_internal(workspace, Direction::Prev, !action.partial_word, cx)
}

fn move_to_internal(
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
    use search::BufferSearchBar;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_move_to_next(cx: &mut gpui::TestAppContext) {
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
        search_bar.next_notification(&cx).await;
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes(["*"]);
        search_bar.next_notification(&cx).await;
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["#"]);
        search_bar.next_notification(&cx).await;
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes(["#"]);
        search_bar.next_notification(&cx).await;
        cx.assert_state("ˇhi\nhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["g", "*"]);
        search_bar.next_notification(&cx).await;
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);

        cx.simulate_keystrokes(["n"]);
        cx.assert_state("hi\nhigh\nˇhi\n", Mode::Normal);

        cx.simulate_keystrokes(["g", "#"]);
        search_bar.next_notification(&cx).await;
        cx.assert_state("hi\nˇhigh\nhi\n", Mode::Normal);
    }
}
