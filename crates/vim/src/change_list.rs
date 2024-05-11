use editor::{display_map::ToDisplayPoint, movement, scroll::Autoscroll, Bias, Direction, Editor};
use gpui::{actions, View};
use ui::{ViewContext, WindowContext};
use workspace::Workspace;

use crate::{state::Mode, Vim};

actions!(vim, [ChangeListOlder, ChangeListNewer]);

pub(crate) fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_, _: &ChangeListOlder, cx| {
        Vim::update(cx, |vim, cx| {
            move_to_change(vim, Direction::Prev, cx);
        })
    });
    workspace.register_action(|_, _: &ChangeListNewer, cx| {
        Vim::update(cx, |vim, cx| {
            move_to_change(vim, Direction::Next, cx);
        })
    });
}

fn move_to_change(vim: &mut Vim, direction: Direction, cx: &mut WindowContext) {
    let count = vim.take_count(cx).unwrap_or(1);
    let selections = vim.update_state(|state| {
        if state.change_list.is_empty() {
            return None;
        }

        let prev = state
            .change_list_position
            .unwrap_or(state.change_list.len());
        let next = if direction == Direction::Prev {
            prev.saturating_sub(count)
        } else {
            (prev + count).min(state.change_list.len() - 1)
        };
        state.change_list_position = Some(next);
        state.change_list.get(next).cloned()
    });

    let Some(selections) = selections else {
        return;
    };
    vim.update_active_editor(cx, |_, editor, cx| {
        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
            let map = s.display_map();
            s.select_display_ranges(selections.into_iter().map(|a| {
                let point = a.to_display_point(&map);
                point..point
            }))
        })
    });
}

pub(crate) fn push_to_change_list(vim: &mut Vim, editor: View<Editor>, cx: &mut WindowContext) {
    let (map, selections) =
        editor.update(cx, |editor, cx| editor.selections.all_adjusted_display(cx));

    let pop_state =
        vim.state()
            .change_list
            .last()
            .map(|previous| {
                previous.len() == selections.len()
                    && previous.iter().enumerate().all(|(ix, p)| {
                        p.to_display_point(&map).row() == selections[ix].head().row()
                    })
            })
            .unwrap_or(false);

    let new_positions = selections
        .into_iter()
        .map(|s| {
            let point = if vim.state().mode == Mode::Insert {
                movement::saturating_left(&map, s.head())
            } else {
                s.head()
            };
            map.display_point_to_anchor(point, Bias::Left)
        })
        .collect();

    vim.update_state(|state| {
        state.change_list_position.take();
        if pop_state {
            state.change_list.pop();
        }
        state.change_list.push(new_positions);
    })
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test::NeovimBackedTestContext};

    #[gpui::test]
    async fn test_change_list_insert(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇ").await;

        cx.simulate_shared_keystrokes("i 1 1 escape shift-o 2 2 escape shift-g o 3 3 escape")
            .await;

        cx.shared_state().await.assert_eq(indoc! {
            "22
             11
             3ˇ3"
        });

        cx.simulate_shared_keystrokes("g ;").await;
        // NOTE: this matches nvim when I type it into it
        // but in tests, nvim always reports the column as 0...
        cx.assert_state(
            indoc! {
            "22
             11
             3ˇ3"
            },
            Mode::Normal,
        );
        cx.simulate_shared_keystrokes("g ;").await;
        cx.assert_state(
            indoc! {
            "2ˇ2
             11
             33"
            },
            Mode::Normal,
        );
        cx.simulate_shared_keystrokes("g ;").await;
        cx.assert_state(
            indoc! {
            "22
             1ˇ1
             33"
            },
            Mode::Normal,
        );
        cx.simulate_shared_keystrokes("g ,").await;
        cx.assert_state(
            indoc! {
            "2ˇ2
             11
             33"
            },
            Mode::Normal,
        );
        cx.simulate_shared_keystrokes("shift-g i 4 4 escape").await;
        cx.simulate_shared_keystrokes("g ;").await;
        cx.assert_state(
            indoc! {
            "22
             11
             34ˇ43"
            },
            Mode::Normal,
        );
        cx.simulate_shared_keystrokes("g ;").await;
        cx.assert_state(
            indoc! {
            "2ˇ2
             11
             3443"
            },
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_change_list_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {
        "one two
        three fˇour"})
            .await;
        cx.simulate_shared_keystrokes("x k d i w ^ x").await;
        cx.shared_state().await.assert_eq(indoc! {
        "ˇne•
        three fur"});
        cx.simulate_shared_keystrokes("2 g ;").await;
        cx.shared_state().await.assert_eq(indoc! {
        "ne•
        three fˇur"});
        cx.simulate_shared_keystrokes("g ,").await;
        cx.shared_state().await.assert_eq(indoc! {
        "ˇne•
        three fur"});
    }

    #[gpui::test]
    async fn test_gi(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {
        "one two
        three fˇr"})
            .await;
        cx.simulate_shared_keystrokes("i o escape k g i").await;
        cx.simulate_shared_keystrokes("u escape").await;
        cx.shared_state().await.assert_eq(indoc! {
        "one two
        three foˇur"});
    }

    #[gpui::test]
    async fn test_dot_mark(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {
        "one two
        three fˇr"})
            .await;
        cx.simulate_shared_keystrokes("i o escape k ` .").await;
        cx.shared_state().await.assert_eq(indoc! {
        "one two
        three fˇor"});
    }
}
