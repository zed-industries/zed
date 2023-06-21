mod neovim_backed_binding_test_context;
mod neovim_backed_test_context;
mod neovim_connection;
mod vim_binding_test_context;
mod vim_test_context;

pub use neovim_backed_binding_test_context::*;
pub use neovim_backed_test_context::*;
pub use vim_binding_test_context::*;
pub use vim_test_context::*;

use indoc::indoc;
use search::BufferSearchBar;

use crate::state::Mode;

#[gpui::test]
async fn test_initially_disabled(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, false).await;
    cx.simulate_keystrokes(["h", "j", "k", "l"]);
    cx.assert_editor_state("hjklˇ");
}

#[gpui::test]
async fn test_neovim(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.simulate_shared_keystroke("i").await;
    cx.assert_state_matches().await;
    cx.simulate_shared_keystrokes([
        "shift-T", "e", "s", "t", " ", "t", "e", "s", "t", "escape", "0", "d", "w",
    ])
    .await;
    cx.assert_state_matches().await;
    cx.assert_editor_state("ˇtest");
}

#[gpui::test]
async fn test_toggle_through_settings(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.simulate_keystroke("i");
    assert_eq!(cx.mode(), Mode::Insert);

    // Editor acts as though vim is disabled
    cx.disable_vim();
    cx.simulate_keystrokes(["h", "j", "k", "l"]);
    cx.assert_editor_state("hjklˇ");

    // Selections aren't changed if editor is blurred but vim-mode is still disabled.
    cx.set_state("«hjklˇ»", Mode::Normal);
    cx.assert_editor_state("«hjklˇ»");
    cx.update_editor(|_, cx| cx.blur());
    cx.assert_editor_state("«hjklˇ»");
    cx.update_editor(|_, cx| cx.focus_self());
    cx.assert_editor_state("«hjklˇ»");

    // Enabling dynamically sets vim mode again and restores normal mode
    cx.enable_vim();
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keystrokes(["h", "h", "h", "l"]);
    assert_eq!(cx.buffer_text(), "hjkl".to_owned());
    cx.assert_editor_state("hˇjkl");
    cx.simulate_keystrokes(["i", "T", "e", "s", "t"]);
    cx.assert_editor_state("hTestˇjkl");

    // Disabling and enabling resets to normal mode
    assert_eq!(cx.mode(), Mode::Insert);
    cx.disable_vim();
    cx.enable_vim();
    assert_eq!(cx.mode(), Mode::Normal);
}

#[gpui::test]
async fn test_buffer_search(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(
        indoc! {"
            The quick brown
            fox juˇmps over
            the lazy dog"},
        Mode::Normal,
    );
    cx.simulate_keystroke("/");

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
        assert_eq!(bar.query_editor.read(cx).text(cx), "jumps");
    })
}


#[gpui::test]
async fn test_substitute(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(indoc! {"ˇabc\n"}, Mode::Normal);
    cx.simulate_keystrokes(["s", "x"]);
    cx.assert_editor_state("xˇbc\n");
}
