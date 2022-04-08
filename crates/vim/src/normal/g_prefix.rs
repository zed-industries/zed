use crate::{mode::Mode, SwitchMode, VimState};
use gpui::{actions, keymap::Binding, MutableAppContext, ViewContext};
use workspace::Workspace;

actions!(vim, [MoveToStart]);

pub fn init(cx: &mut MutableAppContext) {
    let context = Some("Editor && vim_mode == normal && vim_submode == g");
    cx.add_bindings(vec![
        Binding::new("g", MoveToStart, context),
        Binding::new("escape", SwitchMode(Mode::normal()), context),
    ]);

    cx.add_action(move_to_start);
}

fn move_to_start(_: &mut Workspace, _: &MoveToStart, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_to_beginning(&editor::MoveToBeginning, cx);
        });
        state.switch_mode(&SwitchMode(Mode::normal()), cx);
    })
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{
        mode::{Mode, NormalState},
        vim_test_context::VimTestContext,
    };

    #[gpui::test]
    async fn test_g_prefix_and_abort(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true, "").await;

        // Can abort with escape to get back to normal mode
        cx.simulate_keystroke("g");
        assert_eq!(cx.mode(), Mode::Normal(NormalState::GPrefix));
        cx.simulate_keystroke("escape");
        assert_eq!(cx.mode(), Mode::normal());
    }

    #[gpui::test]
    async fn test_move_to_start(cx: &mut gpui::TestAppContext) {
        let initial_content = indoc! {"
            The quick
            
            brown fox jumps
            over the lazy dog"};
        let mut cx = VimTestContext::new(cx, true, initial_content).await;

        // Jump to the end to
        cx.simulate_keystroke("shift-G");
        cx.assert_editor_state(indoc! {"
            The quick
            
            brown fox jumps
            over the lazy do|g"});

        // Jump to the start
        cx.simulate_keystrokes(&["g", "g"]);
        cx.assert_editor_state(indoc! {"
            |The quick
            
            brown fox jumps
            over the lazy dog"});
        assert_eq!(cx.mode(), Mode::normal());

        // Repeat action doesn't change
        cx.simulate_keystrokes(&["g", "g"]);
        cx.assert_editor_state(indoc! {"
            |The quick
            
            brown fox jumps
            over the lazy dog"});
        assert_eq!(cx.mode(), Mode::normal());
    }
}
