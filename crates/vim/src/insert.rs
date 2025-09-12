use crate::{Vim, state::Mode};
use editor::{Bias, Editor};
use gpui::{Action, Context, Window, actions};
use language::SelectionGoal;
use settings::Settings;
use text::Point;
use vim_mode_setting::HelixModeSetting;
use workspace::searchable::Direction;

actions!(
    vim,
    [
        /// Switches to normal mode with cursor positioned before the current character.
        NormalBefore,
        /// Temporarily switches to normal mode for one command.
        TemporaryNormal,
        /// Inserts the next character from the line above into the current line.
        InsertFromAbove,
        /// Inserts the next character from the line below into the current line.
        InsertFromBelow
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::normal_before);
    Vim::action(editor, cx, Vim::temporary_normal);
    Vim::action(editor, cx, |vim, _: &InsertFromAbove, window, cx| {
        vim.insert_around(Direction::Prev, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &InsertFromBelow, window, cx| {
        vim.insert_around(Direction::Next, window, cx)
    })
}

impl Vim {
    pub(crate) fn normal_before(
        &mut self,
        action: &NormalBefore,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_operator().is_some() {
            self.operator_stack.clear();
            self.sync_vim_settings(window, cx);
            return;
        }
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        self.stop_recording_immediately(action.boxed_clone(), cx);
        if count <= 1 || Vim::globals(cx).dot_replaying {
            self.create_mark("^".into(), window, cx);

            self.update_editor(cx, |_, editor, cx| {
                editor.dismiss_menus_and_popups(false, window, cx);

                if !HelixModeSetting::get_global(cx).0 {
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.move_cursors_with(|map, mut cursor, _| {
                            *cursor.column_mut() = cursor.column().saturating_sub(1);
                            (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
                        });
                    });
                }
            });

            self.switch_mode(Mode::Normal, false, window, cx);
            return;
        }

        self.repeat(true, window, cx)
    }

    fn temporary_normal(
        &mut self,
        _: &TemporaryNormal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.switch_mode(Mode::Normal, true, window, cx);
        self.temp_mode = true;
    }

    fn insert_around(&mut self, direction: Direction, _: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let mut edits = Vec::new();
            for selection in editor.selections.all::<Point>(cx) {
                let point = selection.head();
                let new_row = match direction {
                    Direction::Next => point.row + 1,
                    Direction::Prev if point.row > 0 => point.row - 1,
                    _ => continue,
                };
                let source = snapshot.clip_point(Point::new(new_row, point.column), Bias::Left);
                if let Some(c) = snapshot.chars_at(source).next()
                    && c != '\n'
                {
                    edits.push((point..point, c.to_string()))
                }
            }

            editor.edit(edits, cx);
        });
    }
}

#[cfg(test)]
mod test {
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_enter_and_exit_insert_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.simulate_keystrokes("i");
        assert_eq!(cx.mode(), Mode::Insert);
        cx.simulate_keystrokes("T e s t");
        cx.assert_editor_state("Testˇ");
        cx.simulate_keystrokes("escape");
        assert_eq!(cx.mode(), Mode::Normal);
        cx.assert_editor_state("Tesˇt");
    }

    #[gpui::test]
    async fn test_insert_with_counts(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("5 i - escape").await;
        cx.shared_state().await.assert_eq("----ˇ-hello\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("5 a - escape").await;
        cx.shared_state().await.assert_eq("h----ˇ-ello\n");

        cx.simulate_shared_keystrokes("4 shift-i - escape").await;
        cx.shared_state().await.assert_eq("---ˇ-h-----ello\n");

        cx.simulate_shared_keystrokes("3 shift-a - escape").await;
        cx.shared_state().await.assert_eq("----h-----ello--ˇ-\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 o o i escape").await;
        cx.shared_state().await.assert_eq("hello\noi\noi\noˇi\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 shift-o o i escape").await;
        cx.shared_state().await.assert_eq("oi\noi\noˇi\nhello\n");
    }

    #[gpui::test]
    async fn test_insert_with_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 i - escape").await;
        cx.shared_state().await.assert_eq("--ˇ-hello\n");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("----ˇ--hello\n");
        cx.simulate_shared_keystrokes("2 .").await;
        cx.shared_state().await.assert_eq("-----ˇ---hello\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("2 o k k escape").await;
        cx.shared_state().await.assert_eq("hello\nkk\nkˇk\n");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state()
            .await
            .assert_eq("hello\nkk\nkk\nkk\nkˇk\n");
        cx.simulate_shared_keystrokes("1 .").await;
        cx.shared_state()
            .await
            .assert_eq("hello\nkk\nkk\nkk\nkk\nkˇk\n");
    }

    #[gpui::test]
    async fn test_insert_ctrl_r(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("heˇllo\n").await;
        cx.simulate_shared_keystrokes("y y i ctrl-r \"").await;
        cx.shared_state().await.assert_eq("hehello\nˇllo\n");

        cx.simulate_shared_keystrokes("ctrl-r x ctrl-r escape")
            .await;
        cx.shared_state().await.assert_eq("hehello\nˇllo\n");
    }

    #[gpui::test]
    async fn test_insert_ctrl_y(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("hello\nˇ\nworld").await;
        cx.simulate_shared_keystrokes("i ctrl-y ctrl-e").await;
        cx.shared_state().await.assert_eq("hello\nhoˇ\nworld");
    }
}
