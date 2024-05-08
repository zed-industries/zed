mod neovim_backed_binding_test_context;
mod neovim_backed_test_context;
mod neovim_connection;
mod vim_test_context;

use std::time::Duration;

use command_palette::CommandPalette;
use editor::DisplayPoint;
use futures::StreamExt;
use gpui::{KeyBinding, Modifiers, MouseButton, TestAppContext};
pub use neovim_backed_binding_test_context::*;
pub use neovim_backed_test_context::*;
pub use vim_test_context::*;

use indoc::indoc;
use search::BufferSearchBar;

use crate::{insert::NormalBefore, motion, state::Mode, ModeIndicator};

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
async fn test_cancel_selection(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(
        indoc! {"The quick brown fox juˇmps over the lazy dog"},
        Mode::Normal,
    );
    // jumps
    cx.simulate_keystrokes(["v", "l", "l"]);
    cx.assert_editor_state("The quick brown fox ju«mpsˇ» over the lazy dog");

    cx.simulate_keystrokes(["escape"]);
    cx.assert_editor_state("The quick brown fox jumpˇs over the lazy dog");

    // go back to the same selection state
    cx.simulate_keystrokes(["v", "h", "h"]);
    cx.assert_editor_state("The quick brown fox ju«ˇmps» over the lazy dog");

    // Ctrl-[ should behave like Esc
    cx.simulate_keystrokes(["ctrl-["]);
    cx.assert_editor_state("The quick brown fox juˇmps over the lazy dog");
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

    cx.update_view(search_bar, |bar, cx| {
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

    // can go to line 1 (https://github.com/zed-industries/zed/issues/5812)
    cx.simulate_keystrokes(["1", "shift-g"]);
    cx.assert_editor_state("aˇa\nbb\ncc");
}

#[gpui::test]
async fn test_end_of_line_with_times(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // goes to current line end
    cx.set_state(indoc! {"ˇaa\nbb\ncc"}, Mode::Normal);
    cx.simulate_keystrokes(["$"]);
    cx.assert_editor_state("aˇa\nbb\ncc");

    // goes to next line end
    cx.simulate_keystrokes(["2", "$"]);
    cx.assert_editor_state("aa\nbˇb\ncc");

    // try to exceed the final line.
    cx.simulate_keystrokes(["4", "$"]);
    cx.assert_editor_state("aa\nbb\ncˇc");
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

    // works in visual mode
    cx.simulate_keystrokes(["shift-v", "down", ">"]);
    cx.assert_editor_state("aa\n    bb\n    cˇc");
}

#[gpui::test]
async fn test_escape_command_palette(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("aˇbc\n", Mode::Normal);
    cx.simulate_keystrokes(["i", "cmd-shift-p"]);

    assert!(cx.workspace(|workspace, cx| workspace.active_modal::<CommandPalette>(cx).is_some()));
    cx.simulate_keystroke("escape");
    cx.run_until_parked();
    assert!(!cx.workspace(|workspace, cx| workspace.active_modal::<CommandPalette>(cx).is_some()));
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

    cx.update_view(search_bar, |bar, cx| {
        assert_eq!(bar.query(cx), "cc");
    });

    cx.update_editor(|editor, cx| {
        let highlights = editor.all_text_background_highlights(cx);
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
async fn test_status_indicator(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

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
    assert_eq!(
        cx.workspace(|_, cx| mode_indicator.read(cx).mode),
        Some(Mode::Insert)
    );
    cx.simulate_keystrokes(["escape", "shift-r"]);
    assert_eq!(
        cx.workspace(|_, cx| mode_indicator.read(cx).mode),
        Some(Mode::Replace)
    );

    // shows even in search
    cx.simulate_keystrokes(["escape", "v", "/"]);
    assert_eq!(
        cx.workspace(|_, cx| mode_indicator.read(cx).mode),
        Some(Mode::Visual)
    );

    // hides if vim mode is disabled
    cx.disable_vim();
    cx.run_until_parked();
    cx.workspace(|workspace, cx| {
        let status_bar = workspace.status_bar().read(cx);
        let mode_indicator = status_bar.item_of_type::<ModeIndicator>().unwrap();
        assert!(mode_indicator.read(cx).mode.is_none());
    });

    cx.enable_vim();
    cx.run_until_parked();
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
async fn test_folds_panic(cx: &mut gpui::TestAppContext) {
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
    cx.simulate_shared_keystrokes(["escape"]).await;
    cx.simulate_shared_keystrokes(["g", "g"]).await;
    cx.simulate_shared_keystrokes(["5", "d", "j"]).await;
    cx.assert_shared_state(indoc! { "ˇ"}).await;

    cx.set_shared_state(indoc! { "
            fn boop() {
              ˇbarp()
              bazp()
            }
        "})
        .await;
    cx.simulate_shared_keystrokes(["shift-v", "j", "j", "z", "f"])
        .await;
    cx.simulate_shared_keystrokes(["escape"]).await;
    cx.simulate_shared_keystrokes(["shift-g", "shift-v"]).await;
    cx.assert_shared_state(indoc! { "
            fn boop() {
              barp()
              bazp()
            }
            ˇ"})
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

#[gpui::test]
async fn test_selection_goal(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        ;;ˇ;
        Lorem Ipsum"})
        .await;

    cx.simulate_shared_keystrokes(["a", "down", "up", ";", "down", "up"])
        .await;
    cx.assert_shared_state(indoc! {"
        ;;;;ˇ
        Lorem Ipsum"})
        .await;
}

#[gpui::test]
async fn test_wrapped_motions(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_wrap(12).await;

    cx.set_shared_state(indoc! {"
                aaˇaa
                😃😃"
    })
    .await;
    cx.simulate_shared_keystrokes(["j"]).await;
    cx.assert_shared_state(indoc! {"
                aaaa
                😃ˇ😃"
    })
    .await;

    cx.set_shared_state(indoc! {"
                123456789012aaˇaa
                123456789012😃😃"
    })
    .await;
    cx.simulate_shared_keystrokes(["j"]).await;
    cx.assert_shared_state(indoc! {"
        123456789012aaaa
        123456789012😃ˇ😃"
    })
    .await;

    cx.set_shared_state(indoc! {"
                123456789012aaˇaa
                123456789012😃😃"
    })
    .await;
    cx.simulate_shared_keystrokes(["j"]).await;
    cx.assert_shared_state(indoc! {"
        123456789012aaaa
        123456789012😃ˇ😃"
    })
    .await;

    cx.set_shared_state(indoc! {"
        123456789012aaaaˇaaaaaaaa123456789012
        wow
        123456789012😃😃😃😃😃😃123456789012"
    })
    .await;
    cx.simulate_shared_keystrokes(["j", "j"]).await;
    cx.assert_shared_state(indoc! {"
        123456789012aaaaaaaaaaaa123456789012
        wow
        123456789012😃😃ˇ😃😃😃😃123456789012"
    })
    .await;
}

#[gpui::test]
async fn test_paragraphs_dont_wrap(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        one
        ˇ
        two"})
        .await;

    cx.simulate_shared_keystrokes(["}", "}"]).await;
    cx.assert_shared_state(indoc! {"
        one

        twˇo"})
        .await;

    cx.simulate_shared_keystrokes(["{", "{", "{"]).await;
    cx.assert_shared_state(indoc! {"
        ˇone

        two"})
        .await;
}

#[gpui::test]
async fn test_select_all_issue_2170(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(
        indoc! {"
        defmodule Test do
            def test(a, ˇ[_, _] = b), do: IO.puts('hi')
        end
    "},
        Mode::Normal,
    );
    cx.simulate_keystrokes(["g", "a"]);
    cx.assert_state(
        indoc! {"
        defmodule Test do
            def test(a, «[ˇ»_, _] = b), do: IO.puts('hi')
        end
    "},
        Mode::Visual,
    );
}

#[gpui::test]
async fn test_jk(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "j k",
            NormalBefore,
            Some("vim_mode == insert"),
        )])
    });
    cx.neovim.exec("imap jk <esc>").await;

    cx.set_shared_state("ˇhello").await;
    cx.simulate_shared_keystrokes(["i", "j", "o", "j", "k"])
        .await;
    cx.assert_shared_state("jˇohello").await;
}

#[gpui::test]
async fn test_jk_delay(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "j k",
            NormalBefore,
            Some("vim_mode == insert"),
        )])
    });

    cx.set_state("ˇhello", Mode::Normal);
    cx.simulate_keystrokes(["i", "j"]);
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.assert_state("ˇhello", Mode::Insert);
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.assert_state("jˇhello", Mode::Insert);
    cx.simulate_keystrokes(["k", "j", "k"]);
    cx.assert_state("jˇkhello", Mode::Normal);
}

#[gpui::test]
async fn test_comma_w(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            ", w",
            motion::Down {
                display_lines: false,
            },
            Some("vim_mode == normal"),
        )])
    });
    cx.neovim.exec("map ,w j").await;

    cx.set_shared_state("ˇhello hello\nhello hello").await;
    cx.simulate_shared_keystrokes(["f", "o", ";", ",", "w"])
        .await;
    cx.assert_shared_state("hello hello\nhello hellˇo").await;

    cx.set_shared_state("ˇhello hello\nhello hello").await;
    cx.simulate_shared_keystrokes(["f", "o", ";", ",", "i"])
        .await;
    cx.assert_shared_state("hellˇo hello\nhello hello").await;
    cx.assert_shared_mode(Mode::Insert).await;
}

#[gpui::test]
async fn test_rename(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new_typescript(cx).await;

    cx.set_state("const beˇfore = 2; console.log(before)", Mode::Normal);
    let def_range = cx.lsp_range("const «beforeˇ» = 2; console.log(before)");
    let tgt_range = cx.lsp_range("const before = 2; console.log(«beforeˇ»)");
    let mut prepare_request =
        cx.handle_request::<lsp::request::PrepareRenameRequest, _, _>(move |_, _, _| async move {
            Ok(Some(lsp::PrepareRenameResponse::Range(def_range)))
        });
    let mut rename_request =
        cx.handle_request::<lsp::request::Rename, _, _>(move |url, params, _| async move {
            Ok(Some(lsp::WorkspaceEdit {
                changes: Some(
                    [(
                        url.clone(),
                        vec![
                            lsp::TextEdit::new(def_range, params.new_name.clone()),
                            lsp::TextEdit::new(tgt_range, params.new_name),
                        ],
                    )]
                    .into(),
                ),
                ..Default::default()
            }))
        });

    cx.simulate_keystrokes(["c", "d"]);
    prepare_request.next().await.unwrap();
    cx.simulate_input("after");
    cx.simulate_keystrokes(["enter"]);
    rename_request.next().await.unwrap();
    cx.assert_state("const afterˇ = 2; console.log(after)", Mode::Normal)
}

#[gpui::test]
async fn test_remap(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // test moving the cursor
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "g z",
            workspace::SendKeystrokes("l l l l".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ123456789", Mode::Normal);
    cx.simulate_keystrokes(["g", "z"]);
    cx.assert_state("1234ˇ56789", Mode::Normal);

    // test switching modes
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "g y",
            workspace::SendKeystrokes("i f o o escape l".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ123456789", Mode::Normal);
    cx.simulate_keystrokes(["g", "y"]);
    cx.assert_state("fooˇ123456789", Mode::Normal);

    // test recursion
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "g x",
            workspace::SendKeystrokes("g z g y".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ123456789", Mode::Normal);
    cx.simulate_keystrokes(["g", "x"]);
    cx.assert_state("1234fooˇ56789", Mode::Normal);

    cx.executor().allow_parking();

    // test command
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "g w",
            workspace::SendKeystrokes(": j enter".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ1234\n56789", Mode::Normal);
    cx.simulate_keystrokes(["g", "w"]);
    cx.assert_state("1234ˇ 56789", Mode::Normal);

    // test leaving command
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "g u",
            workspace::SendKeystrokes("g w g z".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ1234\n56789", Mode::Normal);
    cx.simulate_keystrokes(["g", "u"]);
    cx.assert_state("1234 567ˇ89", Mode::Normal);

    // test leaving command
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "g t",
            workspace::SendKeystrokes("i space escape".to_string()),
            None,
        )])
    });
    cx.set_state("12ˇ34", Mode::Normal);
    cx.simulate_keystrokes(["g", "t"]);
    cx.assert_state("12ˇ 34", Mode::Normal);
}

#[gpui::test]
async fn test_undo(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("hello quˇoel world").await;
    cx.simulate_shared_keystrokes(["v", "i", "w", "s", "c", "o", "escape", "u"])
        .await;
    cx.assert_shared_state("hello ˇquoel world").await;
    cx.simulate_shared_keystrokes(["ctrl-r"]).await;
    cx.assert_shared_state("hello ˇco world").await;
    cx.simulate_shared_keystrokes(["a", "o", "right", "l", "escape"])
        .await;
    cx.assert_shared_state("hello cooˇl world").await;
    cx.simulate_shared_keystrokes(["u"]).await;
    cx.assert_shared_state("hello cooˇ world").await;
    cx.simulate_shared_keystrokes(["u"]).await;
    cx.assert_shared_state("hello cˇo world").await;
    cx.simulate_shared_keystrokes(["u"]).await;
    cx.assert_shared_state("hello ˇquoel world").await;

    cx.set_shared_state("hello quˇoel world").await;
    cx.simulate_shared_keystrokes(["v", "i", "w", "~", "u"])
        .await;
    cx.assert_shared_state("hello ˇquoel world").await;

    cx.set_shared_state("\nhello quˇoel world\n").await;
    cx.simulate_shared_keystrokes(["shift-v", "s", "c", "escape", "u"])
        .await;
    cx.assert_shared_state("\nˇhello quoel world\n").await;

    cx.set_shared_state(indoc! {"
        ˇ1
        2
        3"})
        .await;

    cx.simulate_shared_keystrokes(["ctrl-v", "shift-g", "ctrl-a"])
        .await;
    cx.assert_shared_state(indoc! {"
        ˇ2
        3
        4"})
        .await;

    cx.simulate_shared_keystrokes(["u"]).await;
    cx.assert_shared_state(indoc! {"
        ˇ1
        2
        3"})
        .await;
}

#[gpui::test]
async fn test_mouse_selection(cx: &mut TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("ˇone two three", Mode::Normal);

    let start_point = cx.pixel_position("one twˇo three");
    let end_point = cx.pixel_position("one ˇtwo three");

    cx.simulate_mouse_down(start_point, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_move(end_point, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_up(end_point, MouseButton::Left, Modifiers::none());

    cx.assert_state("one «ˇtwo» three", Mode::Visual)
}
