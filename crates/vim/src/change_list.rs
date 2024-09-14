use editor::{display_map::ToDisplayPoint, movement, scroll::Autoscroll, Bias, Direction, Editor};
use gpui::{actions, ViewContext};

use crate::{state::Mode, Vim};

actions!(vim, [ChangeListOlder, ChangeListNewer]);

pub(crate) fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, |vim, _: &ChangeListOlder, cx| {
        vim.move_to_change(Direction::Prev, cx);
    });
    Vim::action(editor, cx, |vim, _: &ChangeListNewer, cx| {
        vim.move_to_change(Direction::Next, cx);
    });
}

impl Vim {
    fn move_to_change(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let count = self.take_count(cx).unwrap_or(1);
        if self.change_list.is_empty() {
            return;
        }

        let prev = self.change_list_position.unwrap_or(self.change_list.len());
        let next = if direction == Direction::Prev {
            prev.saturating_sub(count)
        } else {
            (prev + count).min(self.change_list.len() - 1)
        };
        self.change_list_position = Some(next);
        let Some(selections) = self.change_list.get(next).cloned() else {
            return;
        };
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let map = s.display_map();
                s.select_display_ranges(selections.into_iter().map(|a| {
                    let point = a.to_display_point(&map);
                    point..point
                }))
            })
        });
    }

    pub(crate) fn push_to_change_list(&mut self, cx: &mut ViewContext<Self>) {
        let Some((map, selections)) = self.update_editor(cx, |_, editor, cx| {
            editor.selections.all_adjusted_display(cx)
        }) else {
            return;
        };

        let pop_state = self
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
                let point = if self.mode == Mode::Insert {
                    movement::saturating_left(&map, s.head())
                } else {
                    s.head()
                };
                map.display_point_to_anchor(point, Bias::Left)
            })
            .collect();

        self.change_list_position.take();
        if pop_state {
            self.change_list.pop();
        }
        self.change_list.push(new_positions);
    }
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
