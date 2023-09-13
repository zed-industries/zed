mod neovim_backed_binding_test_context;
mod neovim_backed_test_context;
mod neovim_connection;
mod vim_test_context;

use std::sync::Arc;

use command_palette::CommandPalette;
use editor::DisplayPoint;
pub use neovim_backed_binding_test_context::*;
pub use neovim_backed_test_context::*;
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
    cx.assert_editor_state("aa\n    b«b\n    ccˇ»");
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
        Some(Mode::Visual)
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

#[gpui::test]
async fn test_word_characters(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new_typescript(cx).await;
    cx.set_state(
        indoc! { "
        class A {
            #ˇgoop = 99;
            $ˇgoop () { return this.#gˇoop };
        };
        console.log(new A().$gooˇp())
    "},
        Mode::Normal,
    );
    cx.simulate_keystrokes(["v", "i", "w"]);
    cx.assert_state(
        indoc! {"
        class A {
            «#goopˇ» = 99;
            «$goopˇ» () { return this.«#goopˇ» };
        };
        console.log(new A().«$goopˇ»())
    "},
        Mode::Visual,
    )
}

#[gpui::test]
async fn test_join_lines(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
      ˇone
      two
      three
      four
      five
      six
      "})
        .await;
    cx.simulate_shared_keystrokes(["shift-j"]).await;
    cx.assert_shared_state(indoc! {"
          oneˇ two
          three
          four
          five
          six
          "})
        .await;
    cx.simulate_shared_keystrokes(["3", "shift-j"]).await;
    cx.assert_shared_state(indoc! {"
          one two threeˇ four
          five
          six
          "})
        .await;

    cx.set_shared_state(indoc! {"
      ˇone
      two
      three
      four
      five
      six
      "})
        .await;
    cx.simulate_shared_keystrokes(["j", "v", "3", "j", "shift-j"])
        .await;
    cx.assert_shared_state(indoc! {"
      one
      two three fourˇ five
      six
      "})
        .await;
}

#[gpui::test]
async fn test_wrapped_lines(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_wrap(12).await;
    // tests line wrap as follows:
    //  1: twelve char
    //     twelve char
    //  2: twelve char
    cx.set_shared_state(indoc! { "
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["j"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char twelve char
        tˇwelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["k"]).await;
    cx.assert_shared_state(indoc! { "
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["g", "j"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char tˇwelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["g", "j"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char twelve char
        tˇwelve char
    "})
        .await;

    cx.simulate_shared_keystrokes(["g", "k"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char tˇwelve char
        twelve char
    "})
        .await;

    cx.simulate_shared_keystrokes(["g", "^"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char ˇtwelve char
        twelve char
    "})
        .await;

    cx.simulate_shared_keystrokes(["^"]).await;
    cx.assert_shared_state(indoc! { "
        ˇtwelve char twelve char
        twelve char
    "})
        .await;

    cx.simulate_shared_keystrokes(["g", "$"]).await;
    cx.assert_shared_state(indoc! { "
        twelve charˇ twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["$"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char twelve chaˇr
        twelve char
    "})
        .await;

    cx.set_shared_state(indoc! { "
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["enter"]).await;
    cx.assert_shared_state(indoc! { "
            twelve char twelve char
            ˇtwelve char
        "})
        .await;

    cx.set_shared_state(indoc! { "
        twelve char
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["o", "o", "escape"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char
        twelve char twelve char
        ˇo
        twelve char
    "})
        .await;

    cx.set_shared_state(indoc! { "
        twelve char
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["shift-a", "a", "escape"])
        .await;
    cx.assert_shared_state(indoc! { "
        twelve char
        twelve char twelve charˇa
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["shift-i", "i", "escape"])
        .await;
    cx.assert_shared_state(indoc! { "
        twelve char
        ˇitwelve char twelve chara
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["shift-d"]).await;
    cx.assert_shared_state(indoc! { "
        twelve char
        ˇ
        twelve char
    "})
        .await;

    cx.set_shared_state(indoc! { "
        twelve char
        twelve char tˇwelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes(["shift-o", "o", "escape"])
        .await;
    cx.assert_shared_state(indoc! { "
        twelve char
        ˇo
        twelve char twelve char
        twelve char
    "})
        .await;

    // line wraps as:
    // fourteen ch
    // ar
    // fourteen ch
    // ar
    cx.set_shared_state(indoc! { "
        fourteen chaˇr
        fourteen char
    "})
        .await;

    cx.simulate_shared_keystrokes(["d", "i", "w"]).await;
    cx.assert_shared_state(indoc! {"
        fourteenˇ•
        fourteen char
    "})
        .await;
    cx.simulate_shared_keystrokes(["j", "shift-f", "e", "f", "r"])
        .await;
    cx.assert_shared_state(indoc! {"
        fourteen•
        fourteen chaˇr
    "})
        .await;
}

#[gpui::test]
async fn test_folds(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_neovim_option("foldmethod=manual").await;

    cx.set_shared_state(indoc! { "
        fn boop() {
          ˇbarp()
          bazp()
        }
    "})
        .await;
    cx.simulate_shared_keystrokes(["shift-v", "j", "z", "f"])
        .await;

    // visual display is now:
    // fn boop () {
    //  [FOLDED]
    // }

    // TODO: this should not be needed but currently zf does not
    // return to normal mode.
    cx.simulate_shared_keystrokes(["escape"]).await;

    // skip over fold downward
    cx.simulate_shared_keystrokes(["g", "g"]).await;
    cx.assert_shared_state(indoc! { "
        ˇfn boop() {
          barp()
          bazp()
        }
    "})
        .await;

    cx.simulate_shared_keystrokes(["j", "j"]).await;
    cx.assert_shared_state(indoc! { "
        fn boop() {
          barp()
          bazp()
        ˇ}
    "})
        .await;

    // skip over fold upward
    cx.simulate_shared_keystrokes(["2", "k"]).await;
    cx.assert_shared_state(indoc! { "
        ˇfn boop() {
          barp()
          bazp()
        }
    "})
        .await;

    // yank the fold
    cx.simulate_shared_keystrokes(["down", "y", "y"]).await;
    cx.assert_shared_clipboard("  barp()\n  bazp()\n").await;

    // re-open
    cx.simulate_shared_keystrokes(["z", "o"]).await;
    cx.assert_shared_state(indoc! { "
        fn boop() {
        ˇ  barp()
          bazp()
        }
    "})
        .await;
}

#[gpui::test]
async fn test_clear_counts(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        The quick brown
        fox juˇmps over
        the lazy dog"})
        .await;

    cx.simulate_shared_keystrokes(["4", "escape", "3", "d", "l"])
        .await;
    cx.assert_shared_state(indoc! {"
        The quick brown
        fox juˇ over
        the lazy dog"})
        .await;
}

#[gpui::test]
async fn test_zero(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        The quˇick brown
        fox jumps over
        the lazy dog"})
        .await;

    cx.simulate_shared_keystrokes(["0"]).await;
    cx.assert_shared_state(indoc! {"
        ˇThe quick brown
        fox jumps over
        the lazy dog"})
        .await;

    cx.simulate_shared_keystrokes(["1", "0", "l"]).await;
    cx.assert_shared_state(indoc! {"
        The quick ˇbrown
        fox jumps over
        the lazy dog"})
        .await;
}
