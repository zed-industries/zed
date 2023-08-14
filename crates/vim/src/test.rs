mod neovim_backed_binding_test_context;
mod neovim_backed_test_context;
mod neovim_connection;
mod vim_binding_test_context;
mod vim_test_context;

use std::sync::Arc;

use command_palette::CommandPalette;
use editor::DisplayPoint;
pub use neovim_backed_binding_test_context::*;
pub use neovim_backed_test_context::*;
pub use vim_binding_test_context::*;
pub use vim_test_context::*;

use indoc::indoc;
use search::BufferSearchBar;

use crate::{state::Mode, ModeIndicator};

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
        assert_eq!(bar.query(cx), "");
    })
}

#[gpui::test]
async fn test_count_down(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(indoc! {"aˇa\nbb\ncc\ndd\nee"}, Mode::Normal);
    cx.simulate_keystrokes(["2", "down"]);
    cx.assert_editor_state("aa\nbb\ncˇc\ndd\nee");
    cx.simulate_keystrokes(["9", "down"]);
    cx.assert_editor_state("aa\nbb\ncc\ndd\neˇe");
}

#[gpui::test]
async fn test_end_of_document_710(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // goes to end by default
    cx.set_state(indoc! {"aˇa\nbb\ncc"}, Mode::Normal);
    cx.simulate_keystrokes(["shift-g"]);
    cx.assert_editor_state("aa\nbb\ncˇc");

    // can go to line 1 (https://github.com/zed-industries/community/issues/710)
    cx.simulate_keystrokes(["1", "shift-g"]);
    cx.assert_editor_state("aˇa\nbb\ncc");
}

#[gpui::test]
async fn test_indent_outdent(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // works in normal mode
    cx.set_state(indoc! {"aa\nbˇb\ncc"}, Mode::Normal);
    cx.simulate_keystrokes([">", ">"]);
    cx.assert_editor_state("aa\n    bˇb\ncc");
    cx.simulate_keystrokes(["<", "<"]);
    cx.assert_editor_state("aa\nbˇb\ncc");

    // works in visuial mode
    cx.simulate_keystrokes(["shift-v", "down", ">"]);
    cx.assert_editor_state("aa\n    b«b\n    cˇ»c");
}

#[gpui::test]
async fn test_escape_command_palette(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("aˇbc\n", Mode::Normal);
    cx.simulate_keystrokes(["i", "cmd-shift-p"]);

    assert!(cx.workspace(|workspace, _| workspace.modal::<CommandPalette>().is_some()));
    cx.simulate_keystroke("escape");
    assert!(!cx.workspace(|workspace, _| workspace.modal::<CommandPalette>().is_some()));
    cx.assert_state("aˇbc\n", Mode::Insert);
}

#[gpui::test]
async fn test_escape_cancels(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("aˇbˇc", Mode::Normal);
    cx.simulate_keystrokes(["escape"]);

    cx.assert_state("aˇbc", Mode::Normal);
}

#[gpui::test]
async fn test_selection_on_search(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(indoc! {"aa\nbˇb\ncc\ncc\ncc\n"}, Mode::Normal);
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

    cx.assert_state(indoc! {"aa\nbb\nˇcc\ncc\ncc\n"}, Mode::Normal);
    cx.simulate_keystrokes(["n"]);
    cx.assert_state(indoc! {"aa\nbb\ncc\nˇcc\ncc\n"}, Mode::Normal);
    cx.simulate_keystrokes(["shift-n"]);
    cx.assert_state(indoc! {"aa\nbb\nˇcc\ncc\ncc\n"}, Mode::Normal);
}

#[gpui::test]
async fn test_status_indicator(
    cx: &mut gpui::TestAppContext,
    deterministic: Arc<gpui::executor::Deterministic>,
) {
    let mut cx = VimTestContext::new(cx, true).await;
    deterministic.run_until_parked();

    let mode_indicator = cx.workspace(|workspace, cx| {
        let status_bar = workspace.status_bar().read(cx);
        let mode_indicator = status_bar.item_of_type::<ModeIndicator>();
        assert!(mode_indicator.is_some());
        mode_indicator.unwrap()
    });

    assert_eq!(
        cx.workspace(|_, cx| mode_indicator.read(cx).mode),
        Some(Mode::Normal)
    );

    // shows the correct mode
    cx.simulate_keystrokes(["i"]);
    deterministic.run_until_parked();
    assert_eq!(
        cx.workspace(|_, cx| mode_indicator.read(cx).mode),
        Some(Mode::Insert)
    );

    // shows even in search
    cx.simulate_keystrokes(["escape", "v", "/"]);
    deterministic.run_until_parked();
    assert_eq!(
        cx.workspace(|_, cx| mode_indicator.read(cx).mode),
        Some(Mode::Visual { line: false })
    );

    // hides if vim mode is disabled
    cx.disable_vim();
    deterministic.run_until_parked();
    cx.workspace(|workspace, cx| {
        let status_bar = workspace.status_bar().read(cx);
        let mode_indicator = status_bar.item_of_type::<ModeIndicator>().unwrap();
        assert!(mode_indicator.read(cx).mode.is_none());
    });

    cx.enable_vim();
    deterministic.run_until_parked();
    cx.workspace(|workspace, cx| {
        let status_bar = workspace.status_bar().read(cx);
        let mode_indicator = status_bar.item_of_type::<ModeIndicator>().unwrap();
        assert!(mode_indicator.read(cx).mode.is_some());
    });
}
