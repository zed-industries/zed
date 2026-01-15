mod neovim_backed_test_context;
mod neovim_connection;
mod vim_test_context;

use std::{sync::Arc, time::Duration};

use collections::HashMap;
use command_palette::CommandPalette;
use editor::{
    AnchorRangeExt, Bias, DisplayPoint, Editor, EditorMode, MultiBuffer, MultiBufferOffset,
    ToOffset, ToPoint,
    actions::{DeleteLine, WrapSelectionsInTag},
    code_context_menus::CodeContextMenu,
    display_map::{DisplayRow, ToDisplayPoint},
    test::editor_test_context::EditorTestContext,
};
use futures::StreamExt;
use gpui::{KeyBinding, Modifiers, MouseButton, TestAppContext, px};
use itertools::Itertools;
use language::{CursorShape, Language, LanguageConfig, Point};
pub use neovim_backed_test_context::*;
use settings::SettingsStore;
use ui::Pixels;
use util::{path, test::marked_text_ranges};
pub use vim_test_context::*;

use gpui::VisualTestContext;
use indoc::indoc;
use project::FakeFs;
use search::BufferSearchBar;
use search::{ProjectSearchView, project_search};
use serde_json::json;
use workspace::DeploySearch;

use crate::beam_jump::{BEAM_JUMP_PENDING_COMMIT_TIMEOUT, BeamJumpAction, BeamJumpState};
use crate::{PushSneak, PushSneakBackward, VimAddon, insert::NormalBefore, motion, state::Mode};

use util_macros::perf;

#[perf]
#[gpui::test]
async fn test_initially_disabled(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, false).await;
    cx.simulate_keystrokes("h j k l");
    cx.assert_editor_state("hjklˇ");
}

#[perf]
#[gpui::test]
async fn test_neovim(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.simulate_shared_keystrokes("i").await;
    cx.shared_state().await.assert_matches();
    cx.simulate_shared_keystrokes("shift-t e s t space t e s t escape 0 d w")
        .await;
    cx.shared_state().await.assert_matches();
    cx.assert_editor_state("ˇtest");
}

#[perf]
#[gpui::test]
async fn test_toggle_through_settings(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.simulate_keystrokes("i");
    assert_eq!(cx.mode(), Mode::Insert);

    // Editor acts as though vim is disabled
    cx.disable_vim();
    cx.simulate_keystrokes("h j k l");
    cx.assert_editor_state("hjklˇ");

    // Selections aren't changed if editor is blurred but vim-mode is still disabled.
    cx.cx.set_state("«hjklˇ»");
    cx.assert_editor_state("«hjklˇ»");
    cx.update_editor(|_, window, _cx| window.blur());
    cx.assert_editor_state("«hjklˇ»");
    cx.update_editor(|_, window, cx| cx.focus_self(window));
    cx.assert_editor_state("«hjklˇ»");

    // Enabling dynamically sets vim mode again and restores normal mode
    cx.enable_vim();
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keystrokes("h h h l");
    assert_eq!(cx.buffer_text(), "hjkl".to_owned());
    cx.assert_editor_state("hˇjkl");
    cx.simulate_keystrokes("i T e s t");
    cx.assert_editor_state("hTestˇjkl");

    // Disabling and enabling resets to normal mode
    assert_eq!(cx.mode(), Mode::Insert);
    cx.disable_vim();
    cx.enable_vim();
    assert_eq!(cx.mode(), Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_cancel_selection(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(
        indoc! {"The quick brown fox juˇmps over the lazy dog"},
        Mode::Normal,
    );
    // jumps
    cx.simulate_keystrokes("v l l");
    cx.assert_editor_state("The quick brown fox ju«mpsˇ» over the lazy dog");

    cx.simulate_keystrokes("escape");
    cx.assert_editor_state("The quick brown fox jumpˇs over the lazy dog");

    // go back to the same selection state
    cx.simulate_keystrokes("v h h");
    cx.assert_editor_state("The quick brown fox ju«ˇmps» over the lazy dog");

    // Ctrl-[ should behave like Esc
    cx.simulate_keystrokes("ctrl-[");
    cx.assert_editor_state("The quick brown fox juˇmps over the lazy dog");
}

#[perf]
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
    cx.simulate_keystrokes("/");

    let search_bar = cx.workspace(|workspace, _, cx| {
        workspace
            .active_pane()
            .read(cx)
            .toolbar()
            .read(cx)
            .item_of_type::<BufferSearchBar>()
            .expect("Buffer search bar should be deployed")
    });

    cx.update_entity(search_bar, |bar, _, cx| {
        assert_eq!(bar.query(cx), "");
    })
}

#[perf]
#[gpui::test]
async fn test_count_down(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(indoc! {"aˇa\nbb\ncc\ndd\nee"}, Mode::Normal);
    cx.simulate_keystrokes("2 down");
    cx.assert_editor_state("aa\nbb\ncˇc\ndd\nee");
    cx.simulate_keystrokes("9 down");
    cx.assert_editor_state("aa\nbb\ncc\ndd\neˇe");
}

#[perf]
#[gpui::test]
async fn test_end_of_document_710(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // goes to end by default
    cx.set_state(indoc! {"aˇa\nbb\ncc"}, Mode::Normal);
    cx.simulate_keystrokes("shift-g");
    cx.assert_editor_state("aa\nbb\ncˇc");

    // can go to line 1 (https://github.com/zed-industries/zed/issues/5812)
    cx.simulate_keystrokes("1 shift-g");
    cx.assert_editor_state("aˇa\nbb\ncc");
}

#[perf]
#[gpui::test]
async fn test_end_of_line_with_times(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // goes to current line end
    cx.set_state(indoc! {"ˇaa\nbb\ncc"}, Mode::Normal);
    cx.simulate_keystrokes("$");
    cx.assert_editor_state("aˇa\nbb\ncc");

    // goes to next line end
    cx.simulate_keystrokes("2 $");
    cx.assert_editor_state("aa\nbˇb\ncc");

    // try to exceed the final line.
    cx.simulate_keystrokes("4 $");
    cx.assert_editor_state("aa\nbb\ncˇc");
}

#[perf]
#[gpui::test]
async fn test_indent_outdent(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // works in normal mode
    cx.set_state(indoc! {"aa\nbˇb\ncc"}, Mode::Normal);
    cx.simulate_keystrokes("> >");
    cx.assert_editor_state("aa\n    bˇb\ncc");
    cx.simulate_keystrokes("< <");
    cx.assert_editor_state("aa\nbˇb\ncc");

    // works in visual mode
    cx.simulate_keystrokes("shift-v down >");
    cx.assert_editor_state("aa\n    bˇb\n    cc");

    // works as operator
    cx.set_state("aa\nbˇb\ncc\n", Mode::Normal);
    cx.simulate_keystrokes("> j");
    cx.assert_editor_state("aa\n    bˇb\n    cc\n");
    cx.simulate_keystrokes("< k");
    cx.assert_editor_state("aa\nbˇb\n    cc\n");
    cx.simulate_keystrokes("> i p");
    cx.assert_editor_state("    aa\n    bˇb\n        cc\n");
    cx.simulate_keystrokes("< i p");
    cx.assert_editor_state("aa\nbˇb\n    cc\n");
    cx.simulate_keystrokes("< i p");
    cx.assert_editor_state("aa\nbˇb\ncc\n");

    cx.set_state("ˇaa\nbb\ncc\n", Mode::Normal);
    cx.simulate_keystrokes("> 2 j");
    cx.assert_editor_state("    ˇaa\n    bb\n    cc\n");

    cx.set_state("aa\nbb\nˇcc\n", Mode::Normal);
    cx.simulate_keystrokes("> 2 k");
    cx.assert_editor_state("    aa\n    bb\n    ˇcc\n");

    // works with repeat
    cx.set_state("a\nb\nccˇc\n", Mode::Normal);
    cx.simulate_keystrokes("> 2 k");
    cx.assert_editor_state("    a\n    b\n    ccˇc\n");
    cx.simulate_keystrokes(".");
    cx.assert_editor_state("        a\n        b\n        ccˇc\n");
    cx.simulate_keystrokes("v k <");
    cx.assert_editor_state("        a\n    bˇ\n    ccc\n");
    cx.simulate_keystrokes(".");
    cx.assert_editor_state("        a\nbˇ\nccc\n");
}

#[perf]
#[gpui::test]
async fn test_escape_command_palette(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("aˇbc\n", Mode::Normal);
    cx.simulate_keystrokes("i cmd-shift-p");

    assert!(
        cx.workspace(|workspace, _, cx| workspace.active_modal::<CommandPalette>(cx).is_some())
    );
    cx.simulate_keystrokes("escape");
    cx.run_until_parked();
    assert!(
        !cx.workspace(|workspace, _, cx| workspace.active_modal::<CommandPalette>(cx).is_some())
    );
    cx.assert_state("aˇbc\n", Mode::Insert);
}

#[perf]
#[gpui::test]
async fn test_escape_cancels(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("aˇbˇc", Mode::Normal);
    cx.simulate_keystrokes("escape");

    cx.assert_state("aˇbc", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_selection_on_search(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(indoc! {"aa\nbˇb\ncc\ncc\ncc\n"}, Mode::Normal);
    cx.simulate_keystrokes("/ c c");

    let search_bar = cx.workspace(|workspace, _, cx| {
        workspace
            .active_pane()
            .read(cx)
            .toolbar()
            .read(cx)
            .item_of_type::<BufferSearchBar>()
            .expect("Buffer search bar should be deployed")
    });

    cx.update_entity(search_bar, |bar, _, cx| {
        assert_eq!(bar.query(cx), "cc");
    });

    cx.update_editor(|editor, window, cx| {
        let highlights = editor.all_text_background_highlights(window, cx);
        assert_eq!(3, highlights.len());
        assert_eq!(
            DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(2), 2),
            highlights[0].0
        )
    });
    cx.simulate_keystrokes("enter");

    cx.assert_state(indoc! {"aa\nbb\nˇcc\ncc\ncc\n"}, Mode::Normal);
    cx.simulate_keystrokes("n");
    cx.assert_state(indoc! {"aa\nbb\ncc\nˇcc\ncc\n"}, Mode::Normal);
    cx.simulate_keystrokes("shift-n");
    cx.assert_state(indoc! {"aa\nbb\nˇcc\ncc\ncc\n"}, Mode::Normal);
}

#[perf]
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
    cx.simulate_keystrokes("v i w");
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

#[perf]
#[gpui::test]
async fn test_kebab_case(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new_html(cx).await;
    cx.set_state(
        indoc! { r#"
            <div><a class="bg-rˇed"></a></div>
            "#},
        Mode::Normal,
    );
    cx.simulate_keystrokes("v i w");
    cx.assert_state(
        indoc! { r#"
        <div><a class="bg-«redˇ»"></a></div>
        "#
        },
        Mode::Visual,
    )
}

#[perf]
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
    cx.simulate_shared_keystrokes("shift-j").await;
    cx.shared_state().await.assert_eq(indoc! {"
          oneˇ two
          three
          four
          five
          six
          "});
    cx.simulate_shared_keystrokes("3 shift-j").await;
    cx.shared_state().await.assert_eq(indoc! {"
          one two threeˇ four
          five
          six
          "});

    cx.set_shared_state(indoc! {"
      ˇone
      two
      three
      four
      five
      six
      "})
        .await;
    cx.simulate_shared_keystrokes("j v 3 j shift-j").await;
    cx.shared_state().await.assert_eq(indoc! {"
      one
      two three fourˇ five
      six
      "});

    cx.set_shared_state(indoc! {"
      ˇone
      two
      three
      four
      five
      six
      "})
        .await;
    cx.simulate_shared_keystrokes("g shift-j").await;
    cx.shared_state().await.assert_eq(indoc! {"
          oneˇtwo
          three
          four
          five
          six
          "});
    cx.simulate_shared_keystrokes("3 g shift-j").await;
    cx.shared_state().await.assert_eq(indoc! {"
          onetwothreeˇfour
          five
          six
          "});

    cx.set_shared_state(indoc! {"
      ˇone
      two
      three
      four
      five
      six
      "})
        .await;
    cx.simulate_shared_keystrokes("j v 3 j g shift-j").await;
    cx.shared_state().await.assert_eq(indoc! {"
      one
      twothreefourˇfive
      six
      "});
}

#[cfg(target_os = "macos")]
#[perf]
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
    cx.simulate_shared_keystrokes("j").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char twelve char
        tˇwelve char
    "});
    cx.simulate_shared_keystrokes("k").await;
    cx.shared_state().await.assert_eq(indoc! {"
        tˇwelve char twelve char
        twelve char
    "});
    cx.simulate_shared_keystrokes("g j").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char tˇwelve char
        twelve char
    "});
    cx.simulate_shared_keystrokes("g j").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char twelve char
        tˇwelve char
    "});

    cx.simulate_shared_keystrokes("g k").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char tˇwelve char
        twelve char
    "});

    cx.simulate_shared_keystrokes("g ^").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char ˇtwelve char
        twelve char
    "});

    cx.simulate_shared_keystrokes("^").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ˇtwelve char twelve char
        twelve char
    "});

    cx.simulate_shared_keystrokes("g $").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve charˇ twelve char
        twelve char
    "});
    cx.simulate_shared_keystrokes("$").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char twelve chaˇr
        twelve char
    "});

    cx.set_shared_state(indoc! { "
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes("enter").await;
    cx.shared_state().await.assert_eq(indoc! {"
            twelve char twelve char
            ˇtwelve char
        "});

    cx.set_shared_state(indoc! { "
        twelve char
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes("o o escape").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char
        twelve char twelve char
        ˇo
        twelve char
    "});

    cx.set_shared_state(indoc! { "
        twelve char
        tˇwelve char twelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes("shift-a a escape").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char
        twelve char twelve charˇa
        twelve char
    "});
    cx.simulate_shared_keystrokes("shift-i i escape").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char
        ˇitwelve char twelve chara
        twelve char
    "});
    cx.simulate_shared_keystrokes("shift-d").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char
        ˇ
        twelve char
    "});

    cx.set_shared_state(indoc! { "
        twelve char
        twelve char tˇwelve char
        twelve char
    "})
        .await;
    cx.simulate_shared_keystrokes("shift-o o escape").await;
    cx.shared_state().await.assert_eq(indoc! {"
        twelve char
        ˇo
        twelve char twelve char
        twelve char
    "});

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

    cx.simulate_shared_keystrokes("d i w").await;
    cx.shared_state().await.assert_eq(indoc! {"
        fourteenˇ•
        fourteen char
    "});
    cx.simulate_shared_keystrokes("j shift-f e f r").await;
    cx.shared_state().await.assert_eq(indoc! {"
        fourteen•
        fourteen chaˇr
    "});
}

#[perf]
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
    cx.simulate_shared_keystrokes("shift-v j z f").await;

    // visual display is now:
    // fn boop () {
    //  [FOLDED]
    // }

    // TODO: this should not be needed but currently zf does not
    // return to normal mode.
    cx.simulate_shared_keystrokes("escape").await;

    // skip over fold downward
    cx.simulate_shared_keystrokes("g g").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ˇfn boop() {
          barp()
          bazp()
        }
    "});

    cx.simulate_shared_keystrokes("j j").await;
    cx.shared_state().await.assert_eq(indoc! {"
        fn boop() {
          barp()
          bazp()
        ˇ}
    "});

    // skip over fold upward
    cx.simulate_shared_keystrokes("2 k").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ˇfn boop() {
          barp()
          bazp()
        }
    "});

    // yank the fold
    cx.simulate_shared_keystrokes("down y y").await;
    cx.shared_clipboard()
        .await
        .assert_eq("  barp()\n  bazp()\n");

    // re-open
    cx.simulate_shared_keystrokes("z o").await;
    cx.shared_state().await.assert_eq(indoc! {"
        fn boop() {
        ˇ  barp()
          bazp()
        }
    "});
}

#[perf]
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
    cx.simulate_shared_keystrokes("shift-v j z f").await;
    cx.simulate_shared_keystrokes("escape").await;
    cx.simulate_shared_keystrokes("g g").await;
    cx.simulate_shared_keystrokes("5 d j").await;
    cx.shared_state().await.assert_eq("ˇ");
    cx.set_shared_state(indoc! {"
        fn boop() {
          ˇbarp()
          bazp()
        }
    "})
        .await;
    cx.simulate_shared_keystrokes("shift-v j j z f").await;
    cx.simulate_shared_keystrokes("escape").await;
    cx.simulate_shared_keystrokes("shift-g shift-v").await;
    cx.shared_state().await.assert_eq(indoc! {"
        fn boop() {
          barp()
          bazp()
        }
        ˇ"});
}

#[perf]
#[gpui::test]
async fn test_clear_counts(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        The quick brown
        fox juˇmps over
        the lazy dog"})
        .await;

    cx.simulate_shared_keystrokes("4 escape 3 d l").await;
    cx.shared_state().await.assert_eq(indoc! {"
        The quick brown
        fox juˇ over
        the lazy dog"});
}

#[perf]
#[gpui::test]
async fn test_zero(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        The quˇick brown
        fox jumps over
        the lazy dog"})
        .await;

    cx.simulate_shared_keystrokes("0").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ˇThe quick brown
        fox jumps over
        the lazy dog"});

    cx.simulate_shared_keystrokes("1 0 l").await;
    cx.shared_state().await.assert_eq(indoc! {"
        The quick ˇbrown
        fox jumps over
        the lazy dog"});
}

#[perf]
#[gpui::test]
async fn test_selection_goal(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        ;;ˇ;
        Lorem Ipsum"})
        .await;

    cx.simulate_shared_keystrokes("a down up ; down up").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ;;;;ˇ
        Lorem Ipsum"});
}

#[cfg(target_os = "macos")]
#[perf]
#[gpui::test]
async fn test_wrapped_motions(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_wrap(12).await;

    cx.set_shared_state(indoc! {"
                aaˇaa
                😃😃"
    })
    .await;
    cx.simulate_shared_keystrokes("j").await;
    cx.shared_state().await.assert_eq(indoc! {"
                aaaa
                😃ˇ😃"
    });

    cx.set_shared_state(indoc! {"
                123456789012aaˇaa
                123456789012😃😃"
    })
    .await;
    cx.simulate_shared_keystrokes("j").await;
    cx.shared_state().await.assert_eq(indoc! {"
        123456789012aaaa
        123456789012😃ˇ😃"
    });

    cx.set_shared_state(indoc! {"
                123456789012aaˇaa
                123456789012😃😃"
    })
    .await;
    cx.simulate_shared_keystrokes("j").await;
    cx.shared_state().await.assert_eq(indoc! {"
        123456789012aaaa
        123456789012😃ˇ😃"
    });

    cx.set_shared_state(indoc! {"
        123456789012aaaaˇaaaaaaaa123456789012
        wow
        123456789012😃😃😃😃😃😃123456789012"
    })
    .await;
    cx.simulate_shared_keystrokes("j j").await;
    cx.shared_state().await.assert_eq(indoc! {"
        123456789012aaaaaaaaaaaa123456789012
        wow
        123456789012😃😃ˇ😃😃😃😃123456789012"
    });
}

#[perf]
#[gpui::test]
async fn test_wrapped_delete_end_document(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_wrap(12).await;

    cx.set_shared_state(indoc! {"
                aaˇaaaaaaaaaaaaaaaaaa
                bbbbbbbbbbbbbbbbbbbb
                cccccccccccccccccccc"
    })
    .await;
    cx.simulate_shared_keystrokes("d shift-g i z z z").await;
    cx.shared_state().await.assert_eq(indoc! {"
                zzzˇ"
    });
}

#[perf]
#[gpui::test]
async fn test_paragraphs_dont_wrap(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {"
        one
        ˇ
        two"})
        .await;

    cx.simulate_shared_keystrokes("} }").await;
    cx.shared_state().await.assert_eq(indoc! {"
        one

        twˇo"});

    cx.simulate_shared_keystrokes("{ { {").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ˇone

        two"});
}

#[perf]
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
    cx.simulate_keystrokes("g a");
    cx.assert_state(
        indoc! {"
        defmodule Test do
            def test(a, «[ˇ»_, _] = b), do: IO.puts('hi')
        end
    "},
        Mode::Visual,
    );
}

#[perf]
#[gpui::test]
async fn test_jk(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "j k",
            NormalBefore,
            Some("vim_mode == insert"),
        )])
    });
    cx.neovim.exec("imap jk <esc>").await;

    cx.set_shared_state("ˇhello").await;
    cx.simulate_shared_keystrokes("i j o j k").await;
    cx.shared_state().await.assert_eq("jˇohello");
}

fn assert_pending_input(cx: &mut VimTestContext, expected: &str) {
    cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let highlights = editor
            .text_highlights::<editor::PendingInput>(cx)
            .unwrap()
            .1;
        let (_, ranges) = marked_text_ranges(expected, false);

        assert_eq!(
            highlights
                .iter()
                .map(|highlight| highlight.to_offset(&snapshot.buffer_snapshot()))
                .collect::<Vec<_>>(),
            ranges
                .iter()
                .map(|range| MultiBufferOffset(range.start)..MultiBufferOffset(range.end))
                .collect::<Vec<_>>()
        )
    });
}

#[perf]
#[gpui::test]
async fn test_jk_multi(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "j k l",
            NormalBefore,
            Some("vim_mode == insert"),
        )])
    });

    cx.set_state("ˇone ˇone ˇone", Mode::Normal);
    cx.simulate_keystrokes("i j");
    cx.simulate_keystrokes("k");
    cx.assert_state("ˇjkone ˇjkone ˇjkone", Mode::Insert);
    assert_pending_input(&mut cx, "«jk»one «jk»one «jk»one");
    cx.simulate_keystrokes("o j k");
    cx.assert_state("jkoˇjkone jkoˇjkone jkoˇjkone", Mode::Insert);
    assert_pending_input(&mut cx, "jko«jk»one jko«jk»one jko«jk»one");
    cx.simulate_keystrokes("l");
    cx.assert_state("jkˇoone jkˇoone jkˇoone", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_jk_delay(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "j k",
            NormalBefore,
            Some("vim_mode == insert"),
        )])
    });

    cx.set_state("ˇhello", Mode::Normal);
    cx.simulate_keystrokes("i j");
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.assert_state("ˇjhello", Mode::Insert);
    cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let highlights = editor
            .text_highlights::<editor::PendingInput>(cx)
            .unwrap()
            .1;

        assert_eq!(
            highlights
                .iter()
                .map(|highlight| highlight.to_offset(&snapshot.buffer_snapshot()))
                .collect::<Vec<_>>(),
            vec![MultiBufferOffset(0)..MultiBufferOffset(1)]
        )
    });
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.assert_state("jˇhello", Mode::Insert);
    cx.simulate_keystrokes("k j k");
    cx.assert_state("jˇkhello", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_jk_max_count(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("1\nˇ2\n3").await;
    cx.simulate_shared_keystrokes("9 9 9 9 9 9 9 9 9 9 9 9 9 9 9 9 9 9 9 9 j")
        .await;
    cx.shared_state().await.assert_eq("1\n2\nˇ3");

    let number: String = usize::MAX.to_string().split("").join(" ");
    cx.simulate_shared_keystrokes(&format!("{number} k")).await;
    cx.shared_state().await.assert_eq("ˇ1\n2\n3");
}

#[perf]
#[gpui::test]
async fn test_comma_w(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.update(|_, cx| {
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
    cx.simulate_shared_keystrokes("f o ; , w").await;
    cx.shared_state()
        .await
        .assert_eq("hello hello\nhello hellˇo");

    cx.set_shared_state("ˇhello hello\nhello hello").await;
    cx.simulate_shared_keystrokes("f o ; , i").await;
    cx.shared_state()
        .await
        .assert_eq("hellˇo hello\nhello hello");
}

#[perf]
#[gpui::test]
async fn test_completion_menu_scroll_aside(cx: &mut TestAppContext) {
    let mut cx = VimTestContext::new_typescript(cx).await;

    cx.lsp
        .set_request_handler::<lsp::request::Completion, _, _>(move |_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "Test Item".to_string(),
                    documentation: Some(lsp::Documentation::String(
                        "This is some very long documentation content that will be displayed in the aside panel for scrolling.\n".repeat(50)
                    )),
                    ..Default::default()
                },
            ])))
        });

    cx.set_state("variableˇ", Mode::Insert);
    cx.simulate_keystroke(".");
    cx.executor().run_until_parked();

    let mut initial_offset: Pixels = px(0.0);

    cx.update_editor(|editor, _, _| {
        let binding = editor.context_menu().borrow();
        let Some(CodeContextMenu::Completions(menu)) = binding.as_ref() else {
            panic!("Should have completions menu open");
        };

        initial_offset = menu.scroll_handle_aside.offset().y;
    });

    // The `ctrl-e` shortcut should scroll the completion menu's aside content
    // down, so the updated offset should be lower than the initial offset.
    cx.simulate_keystroke("ctrl-e");
    cx.update_editor(|editor, _, _| {
        let binding = editor.context_menu().borrow();
        let Some(CodeContextMenu::Completions(menu)) = binding.as_ref() else {
            panic!("Should have completions menu open");
        };

        assert!(menu.scroll_handle_aside.offset().y < initial_offset);
    });

    // The `ctrl-y` shortcut should do the inverse scrolling as `ctrl-e`, so the
    // offset should now be the same as the initial offset.
    cx.simulate_keystroke("ctrl-y");
    cx.update_editor(|editor, _, _| {
        let binding = editor.context_menu().borrow();
        let Some(CodeContextMenu::Completions(menu)) = binding.as_ref() else {
            panic!("Should have completions menu open");
        };

        assert_eq!(menu.scroll_handle_aside.offset().y, initial_offset);
    });

    // The `ctrl-d` shortcut should scroll the completion menu's aside content
    // down, so the updated offset should be lower than the initial offset.
    cx.simulate_keystroke("ctrl-d");
    cx.update_editor(|editor, _, _| {
        let binding = editor.context_menu().borrow();
        let Some(CodeContextMenu::Completions(menu)) = binding.as_ref() else {
            panic!("Should have completions menu open");
        };

        assert!(menu.scroll_handle_aside.offset().y < initial_offset);
    });

    // The `ctrl-u` shortcut should do the inverse scrolling as `ctrl-u`, so the
    // offset should now be the same as the initial offset.
    cx.simulate_keystroke("ctrl-u");
    cx.update_editor(|editor, _, _| {
        let binding = editor.context_menu().borrow();
        let Some(CodeContextMenu::Completions(menu)) = binding.as_ref() else {
            panic!("Should have completions menu open");
        };

        assert_eq!(menu.scroll_handle_aside.offset().y, initial_offset);
    });
}

#[perf]
#[gpui::test]
async fn test_rename(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new_typescript(cx).await;

    cx.set_state("const beˇfore = 2; console.log(before)", Mode::Normal);
    let def_range = cx.lsp_range("const «beforeˇ» = 2; console.log(before)");
    let tgt_range = cx.lsp_range("const before = 2; console.log(«beforeˇ»)");
    let mut prepare_request = cx.set_request_handler::<lsp::request::PrepareRenameRequest, _, _>(
        move |_, _, _| async move { Ok(Some(lsp::PrepareRenameResponse::Range(def_range))) },
    );
    let mut rename_request =
        cx.set_request_handler::<lsp::request::Rename, _, _>(move |url, params, _| async move {
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

    cx.simulate_keystrokes("c d");
    prepare_request.next().await.unwrap();
    cx.simulate_input("after");
    cx.simulate_keystrokes("enter");
    rename_request.next().await.unwrap();
    cx.assert_state("const afterˇ = 2; console.log(after)", Mode::Normal)
}

#[gpui::test]
async fn test_go_to_definition(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new_typescript(cx).await;

    cx.set_state("const before = 2; console.log(beforˇe)", Mode::Normal);
    let def_range = cx.lsp_range("const «beforeˇ» = 2; console.log(before)");
    let mut go_to_request =
        cx.set_request_handler::<lsp::request::GotoDefinition, _, _>(move |url, _, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(url.clone(), def_range),
            )))
        });

    cx.simulate_keystrokes("g d");
    go_to_request.next().await.unwrap();
    cx.run_until_parked();

    cx.assert_state("const ˇbefore = 2; console.log(before)", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_remap(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // test moving the cursor
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g z",
            workspace::SendKeystrokes("l l l l".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ123456789", Mode::Normal);
    cx.simulate_keystrokes("g z");
    cx.assert_state("1234ˇ56789", Mode::Normal);

    // test switching modes
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g y",
            workspace::SendKeystrokes("i f o o escape l".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ123456789", Mode::Normal);
    cx.simulate_keystrokes("g y");
    cx.assert_state("fooˇ123456789", Mode::Normal);

    // test recursion
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g x",
            workspace::SendKeystrokes("g z g y".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ123456789", Mode::Normal);
    cx.simulate_keystrokes("g x");
    cx.assert_state("1234fooˇ56789", Mode::Normal);

    // test command
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g w",
            workspace::SendKeystrokes(": j enter".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ1234\n56789", Mode::Normal);
    cx.simulate_keystrokes("g w");
    cx.assert_state("1234ˇ 56789", Mode::Normal);

    // test leaving command
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g u",
            workspace::SendKeystrokes("g w g z".to_string()),
            None,
        )])
    });
    cx.set_state("ˇ1234\n56789", Mode::Normal);
    cx.simulate_keystrokes("g u");
    cx.assert_state("1234 567ˇ89", Mode::Normal);

    // test leaving command
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g t",
            workspace::SendKeystrokes("i space escape".to_string()),
            None,
        )])
    });
    cx.set_state("12ˇ34", Mode::Normal);
    cx.simulate_keystrokes("g t");
    cx.assert_state("12ˇ 34", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_undo(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("hello quˇoel world").await;
    cx.simulate_shared_keystrokes("v i w s c o escape u").await;
    cx.shared_state().await.assert_eq("hello ˇquoel world");
    cx.simulate_shared_keystrokes("ctrl-r").await;
    cx.shared_state().await.assert_eq("hello ˇco world");
    cx.simulate_shared_keystrokes("a o right l escape").await;
    cx.shared_state().await.assert_eq("hello cooˇl world");
    cx.simulate_shared_keystrokes("u").await;
    cx.shared_state().await.assert_eq("hello cooˇ world");
    cx.simulate_shared_keystrokes("u").await;
    cx.shared_state().await.assert_eq("hello cˇo world");
    cx.simulate_shared_keystrokes("u").await;
    cx.shared_state().await.assert_eq("hello ˇquoel world");

    cx.set_shared_state("hello quˇoel world").await;
    cx.simulate_shared_keystrokes("v i w ~ u").await;
    cx.shared_state().await.assert_eq("hello ˇquoel world");

    cx.set_shared_state("\nhello quˇoel world\n").await;
    cx.simulate_shared_keystrokes("shift-v s c escape u").await;
    cx.shared_state().await.assert_eq("\nˇhello quoel world\n");

    cx.set_shared_state(indoc! {"
        ˇ1
        2
        3"})
        .await;

    cx.simulate_shared_keystrokes("ctrl-v shift-g ctrl-a").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ˇ2
        3
        4"});

    cx.simulate_shared_keystrokes("u").await;
    cx.shared_state().await.assert_eq(indoc! {"
        ˇ1
        2
        3"});
}

#[perf]
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

#[gpui::test]
async fn test_mouse_drag_across_anchor_does_not_drift(cx: &mut TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("ˇone two three four", Mode::Normal);

    let click_pos = cx.pixel_position("one ˇtwo three four");
    let drag_left = cx.pixel_position("ˇone two three four");
    let anchor_pos = cx.pixel_position("one tˇwo three four");

    cx.simulate_mouse_down(click_pos, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();

    cx.simulate_mouse_move(drag_left, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    cx.assert_state("«ˇone t»wo three four", Mode::Visual);

    cx.simulate_mouse_move(anchor_pos, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();

    cx.simulate_mouse_move(drag_left, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    cx.assert_state("«ˇone t»wo three four", Mode::Visual);

    cx.simulate_mouse_move(anchor_pos, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    cx.simulate_mouse_move(drag_left, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    cx.assert_state("«ˇone t»wo three four", Mode::Visual);

    cx.simulate_mouse_up(drag_left, MouseButton::Left, Modifiers::none());
}

#[perf]
#[gpui::test]
async fn test_lowercase_marks(cx: &mut TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("line one\nline ˇtwo\nline three").await;
    cx.simulate_shared_keystrokes("m a l ' a").await;
    cx.shared_state()
        .await
        .assert_eq("line one\nˇline two\nline three");
    cx.simulate_shared_keystrokes("` a").await;
    cx.shared_state()
        .await
        .assert_eq("line one\nline ˇtwo\nline three");

    cx.simulate_shared_keystrokes("^ d ` a").await;
    cx.shared_state()
        .await
        .assert_eq("line one\nˇtwo\nline three");
}

#[perf]
#[gpui::test]
async fn test_lt_gt_marks(cx: &mut TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc!(
        "
        Line one
        Line two
        Line ˇthree
        Line four
        Line five
    "
    ))
    .await;

    cx.simulate_shared_keystrokes("v j escape k k").await;

    cx.simulate_shared_keystrokes("' <").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        ˇLine three
        Line four
        Line five
    "});

    cx.simulate_shared_keystrokes("` <").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line ˇthree
        Line four
        Line five
    "});

    cx.simulate_shared_keystrokes("' >").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line three
        ˇLine four
        Line five
    "
    });

    cx.simulate_shared_keystrokes("` >").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line three
        Line ˇfour
        Line five
    "
    });

    cx.simulate_shared_keystrokes("v i w o escape").await;
    cx.simulate_shared_keystrokes("` >").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line three
        Line fouˇr
        Line five
    "
    });
    cx.simulate_shared_keystrokes("` <").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line three
        Line ˇfour
        Line five
    "
    });
}

#[perf]
#[gpui::test]
async fn test_caret_mark(cx: &mut TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc!(
        "
        Line one
        Line two
        Line three
        ˇLine four
        Line five
    "
    ))
    .await;

    cx.simulate_shared_keystrokes("c w shift-s t r a i g h t space t h i n g escape j j")
        .await;

    cx.simulate_shared_keystrokes("' ^").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line three
        ˇStraight thing four
        Line five
    "
    });

    cx.simulate_shared_keystrokes("` ^").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line three
        Straight thingˇ four
        Line five
    "
    });

    cx.simulate_shared_keystrokes("k a ! escape k g i ?").await;
    cx.shared_state().await.assert_eq(indoc! {"
        Line one
        Line two
        Line three!?ˇ
        Straight thing four
        Line five
    "
    });
}

#[cfg(target_os = "macos")]
#[perf]
#[gpui::test]
async fn test_dw_eol(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_wrap(12).await;
    cx.set_shared_state("twelve ˇchar twelve char\ntwelve char")
        .await;
    cx.simulate_shared_keystrokes("d w").await;
    cx.shared_state()
        .await
        .assert_eq("twelve ˇtwelve char\ntwelve char");
}

#[perf]
#[gpui::test]
async fn test_toggle_comments(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    let language = std::sync::Arc::new(language::Language::new(
        language::LanguageConfig {
            line_comments: vec!["// ".into(), "//! ".into(), "/// ".into()],
            ..Default::default()
        },
        Some(language::tree_sitter_rust::LANGUAGE.into()),
    ));
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    // works in normal model
    cx.set_state(
        indoc! {"
      ˇone
      two
      three
      "},
        Mode::Normal,
    );
    cx.simulate_keystrokes("g c c");
    cx.assert_state(
        indoc! {"
          // ˇone
          two
          three
          "},
        Mode::Normal,
    );

    // works in visual mode
    cx.simulate_keystrokes("v j g c");
    cx.assert_state(
        indoc! {"
          // // ˇone
          // two
          three
          "},
        Mode::Normal,
    );

    // works in visual line mode
    cx.simulate_keystrokes("shift-v j g c");
    cx.assert_state(
        indoc! {"
          // ˇone
          two
          three
          "},
        Mode::Normal,
    );

    // works with count
    cx.simulate_keystrokes("g c 2 j");
    cx.assert_state(
        indoc! {"
            // // ˇone
            // two
            // three
            "},
        Mode::Normal,
    );

    // works with motion object
    cx.simulate_keystrokes("shift-g");
    cx.simulate_keystrokes("g c g g");
    cx.assert_state(
        indoc! {"
            // one
            two
            three
            ˇ"},
        Mode::Normal,
    );
}

#[perf]
#[gpui::test]
async fn test_find_multibyte(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(r#"<label for="guests">ˇPočet hostů</label>"#)
        .await;

    cx.simulate_shared_keystrokes("c t < o escape").await;
    cx.shared_state()
        .await
        .assert_eq(r#"<label for="guests">ˇo</label>"#);
}

#[perf]
#[gpui::test]
async fn test_sneak(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.update(|_window, cx| {
        cx.bind_keys([
            KeyBinding::new(
                "s",
                PushSneak { first_char: None },
                Some("vim_mode == normal"),
            ),
            KeyBinding::new(
                "shift-s",
                PushSneakBackward { first_char: None },
                Some("vim_mode == normal"),
            ),
            KeyBinding::new(
                "shift-s",
                PushSneakBackward { first_char: None },
                Some("vim_mode == visual"),
            ),
        ])
    });

    // Sneak forwards multibyte & multiline
    cx.set_state(
        indoc! {
            r#"<labelˇ for="guests">
                    Počet hostů
                </label>"#
        },
        Mode::Normal,
    );
    cx.simulate_keystrokes("s t ů");
    cx.assert_state(
        indoc! {
            r#"<label for="guests">
                Počet hosˇtů
            </label>"#
        },
        Mode::Normal,
    );

    // Visual sneak backwards multibyte & multiline
    cx.simulate_keystrokes("v S < l");
    cx.assert_state(
        indoc! {
            r#"«ˇ<label for="guests">
                Počet host»ů
            </label>"#
        },
        Mode::Visual,
    );

    // Sneak backwards repeated
    cx.set_state(r#"11 12 13 ˇ14"#, Mode::Normal);
    cx.simulate_keystrokes("S space 1");
    cx.assert_state(r#"11 12ˇ 13 14"#, Mode::Normal);

    cx.simulate_keystrokes(";");
    cx.assert_state(r#"11ˇ 12 13 14"#, Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_sneak_unchanged_when_beam_jump_disabled(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(false);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([
            KeyBinding::new(
                "s",
                PushSneak { first_char: None },
                Some("vim_mode == normal"),
            ),
            KeyBinding::new(
                "shift-s",
                PushSneakBackward { first_char: None },
                Some("vim_mode == normal"),
            ),
        ])
    });

    cx.set_state("ˇxxabxxabyyab", Mode::Normal);
    cx.simulate_keystrokes("s a b");
    cx.assert_state("xxˇabxxabyyab", Mode::Normal);

    cx.simulate_keystrokes(";");
    cx.assert_state("xxabxxˇabyyab", Mode::Normal);

    cx.set_state("abxxabxxabˇxxab", Mode::Normal);
    cx.simulate_keystrokes("S a b");
    cx.assert_state("abxxabxxˇabxxab", Mode::Normal);

    cx.simulate_keystrokes(",");
    cx.assert_state("abxxabxxabxxˇab", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_labels_forward(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("ˇxxabxxabyyab", Mode::Normal);

    cx.simulate_keystrokes("s a");
    cx.assert_state("ˇxxabxxabyyab", Mode::Normal);

    let first_highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    assert!(
        !first_highlights.is_empty(),
        "beam jump should show first-char previews. {}",
        cx.assertion_context()
    );
    assert!(
        first_highlights
            .iter()
            .all(|highlight| highlight.label.is_none()),
        "labels should not show after first char. {}",
        cx.assertion_context()
    );

    cx.simulate_keystrokes("b");
    cx.assert_state("ˇxxabxxabyyab", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    assert!(
        highlights.len() > 1,
        "expected multiple matches for label selection. {}",
        cx.assertion_context()
    );
    assert!(
        highlights.iter().all(|highlight| highlight.label.is_some()),
        "labels should be visible once pattern length >= 2. {}",
        cx.assertion_context()
    );

    let label = highlights
        .last()
        .and_then(|highlight| highlight.label.clone())
        .expect("missing label for last highlight");
    let keystrokes = label.chars().map(|ch| ch.to_string()).join(" ");
    cx.simulate_keystrokes(&keystrokes);

    cx.assert_state("xxabxxabyyˇab", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());
}

#[perf]
#[gpui::test]
async fn test_beam_jump_filters_overlapping_viewport_matches(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    // Regression for overlapping matches (e.g. `aa` in `aaaaa`): viewport candidates must match the
    // non-overlapping enumeration used by the counted Beam Jump motion.
    cx.set_state("ˇaaaaa", Mode::Normal);
    cx.simulate_keystrokes("s a a");
    cx.assert_state("ˇaaaaa", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    assert_eq!(
        highlights.len(),
        2,
        "expected overlapping matches to be filtered out. {}",
        cx.assertion_context()
    );
    assert!(
        highlights.iter().all(|highlight| highlight.label.is_some()),
        "expected labels to be visible once pattern length >= 2. {}",
        cx.assertion_context()
    );
    assert!(
        highlights[0].range.end <= highlights[1].range.start,
        "expected non-overlapping matches in start order. {}",
        cx.assertion_context()
    );
    assert_eq!(
        highlights[0].range,
        MultiBufferOffset(1)..MultiBufferOffset(3),
        "unexpected first match. {}",
        cx.assertion_context()
    );
    assert_eq!(
        highlights[1].range,
        MultiBufferOffset(3)..MultiBufferOffset(5),
        "unexpected second match. {}",
        cx.assertion_context()
    );

    let label = highlights[1]
        .label
        .clone()
        .expect("missing label for second match");
    let keystrokes = label.chars().map(|ch| ch.to_string()).join(" ");
    cx.simulate_keystrokes(&keystrokes);

    cx.assert_state("aaaˇaa", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    // Jumping to the first match is still correct.
    cx.set_state("ˇaaaaa", Mode::Normal);
    cx.simulate_keystrokes("s a a");
    cx.assert_state("ˇaaaaa", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    let label = highlights[0]
        .label
        .clone()
        .expect("missing label for first match");
    let keystrokes = label.chars().map(|ch| ch.to_string()).join(" ");
    cx.simulate_keystrokes(&keystrokes);

    cx.assert_state("aˇaaaa", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());
}

#[perf]
#[gpui::test]
async fn test_beam_jump_filters_overlapping_viewport_matches_backward(
    cx: &mut gpui::TestAppContext,
) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    // Regression for overlapping matches on the left of the cursor (e.g. `aa` in `aaaaa`).
    cx.set_state("aaaaˇa", Mode::Normal);
    cx.simulate_keystrokes("s a a");
    cx.assert_state("aaaaˇa", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    assert_eq!(
        highlights.len(),
        2,
        "expected overlapping matches to be filtered out. {}",
        cx.assertion_context()
    );
    assert!(
        highlights.iter().all(|highlight| highlight.label.is_some()),
        "expected labels to be visible once pattern length >= 2. {}",
        cx.assertion_context()
    );
    assert!(
        highlights[0].range.end <= highlights[1].range.start,
        "expected non-overlapping matches in start order. {}",
        cx.assertion_context()
    );
    assert_eq!(
        highlights[0].range,
        MultiBufferOffset(0)..MultiBufferOffset(2),
        "unexpected first match. {}",
        cx.assertion_context()
    );
    assert_eq!(
        highlights[1].range,
        MultiBufferOffset(2)..MultiBufferOffset(4),
        "unexpected second match. {}",
        cx.assertion_context()
    );

    let label = highlights[0]
        .label
        .clone()
        .expect("missing label for first match");
    let keystrokes = label.chars().map(|ch| ch.to_string()).join(" ");
    cx.simulate_keystrokes(&keystrokes);

    cx.assert_state("ˇaaaaa", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    // Jumping to the second match is still correct.
    cx.set_state("aaaaˇa", Mode::Normal);
    cx.simulate_keystrokes("s a a");
    cx.assert_state("aaaaˇa", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    let label = highlights[1]
        .label
        .clone()
        .expect("missing label for second match");
    let keystrokes = label.chars().map(|ch| ch.to_string()).join(" ");
    cx.simulate_keystrokes(&keystrokes);

    cx.assert_state("aaˇaaa", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());
}

#[perf]
#[gpui::test]
async fn test_beam_jump_viewport_range_includes_before_and_after_cursor(
    cx: &mut gpui::TestAppContext,
) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let text = "abxxabyyˇzzabxxab";
    cx.set_state(text, Mode::Normal);
    cx.simulate_keystrokes("s a b");
    cx.assert_state(text, Mode::Normal);

    let (cursor_offset, highlights) = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let cursor = editor
            .selections
            .newest_anchor()
            .head()
            .to_display_point(&snapshot.display_snapshot);
        let cursor_offset = cursor.to_offset(&snapshot.display_snapshot, Bias::Left);

        let len = snapshot.display_snapshot.buffer_snapshot().len();
        let highlights = editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec();

        (cursor_offset, highlights)
    });

    assert!(
        highlights.iter().any(|h| h.range.start < cursor_offset),
        "expected Beam Jump highlights before cursor. {}",
        cx.assertion_context()
    );
    assert!(
        highlights.iter().any(|h| h.range.start > cursor_offset),
        "expected Beam Jump highlights after cursor. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_labels_prioritize_cursor_proximity(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("abˇxxab", Mode::Normal);
    cx.simulate_keystrokes("s a b");
    cx.assert_state("abˇxxab", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    assert_eq!(
        highlights.len(),
        2,
        "expected 2 visible matches. {}",
        cx.assertion_context()
    );

    let first = highlights
        .iter()
        .find(|h| h.range.start == MultiBufferOffset(0))
        .and_then(|h| h.label.as_ref())
        .map(|label| label.as_str().to_string())
        .expect("expected a label for the first match");
    let second = highlights
        .iter()
        .find(|h| h.range.start == MultiBufferOffset(4))
        .and_then(|h| h.label.as_ref())
        .map(|label| label.as_str().to_string())
        .expect("expected a label for the second match");

    assert_eq!(
        first,
        "f",
        "closest match (tie-break by offset) should receive the first label. {}",
        cx.assertion_context()
    );
    assert_eq!(
        second,
        "j",
        "second-priority match should receive the next label. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_defers_global_search_until_repeat_keys(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let mut lines = vec!["xx".to_string(); 2000];
    lines[0] = "ˇxx".to_string();
    lines[1500] = "ab".to_string();
    let text = lines.join("\n");
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state(&text, Mode::Normal);

    cx.simulate_keystrokes(";");

    let mut expected_lines = vec!["xx".to_string(); 2000];
    expected_lines[1500] = "ˇab".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());
}

#[perf]
#[gpui::test]
async fn test_beam_jump_labels_avoid_suffix_chars(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("ˇ seaf seaf", Mode::Normal);
    cx.simulate_keystrokes("s s e");
    cx.assert_state("ˇ seaf seaf", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    assert!(
        highlights.len() > 1,
        "expected multiple matches for label selection. {}",
        cx.assertion_context()
    );
    assert!(
        highlights.iter().all(|highlight| highlight.label.is_some()),
        "labels should be visible once pattern length >= 2. {}",
        cx.assertion_context()
    );

    let uses_unsafe_char = highlights
        .iter()
        .filter_map(|highlight| highlight.label.as_ref())
        .any(|label| label.chars().any(|ch| ch == 'a' || ch == 'f'));
    assert!(
        !uses_unsafe_char,
        "labels should avoid characters that appear later in matched words. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_passthrough_key(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let mut lines = vec!["xx".to_string(); 2000];
    lines[0] = "abc".to_string();
    lines[100] = "abc".to_string();
    lines[1999] = "ˇxxabxxabyyab".to_string();
    let text = lines.join("\n");
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state(&text, Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        highlights.len() > 1 && highlights.iter().any(|highlight| highlight.label.is_some()),
        "beam jump should be active with labels visible. {}",
        cx.assertion_context()
    );

    // Extending the pattern to `abc` yields V == 0 in the viewport,
    // so Beam Jump should auto-trigger global navigation.
    cx.simulate_keystrokes("c");

    let mut expected_lines = lines.clone();
    expected_lines[0] = "ˇabc".to_string();
    expected_lines[1999] = "xxabxxabyyab".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after auto-global jump. {}",
        cx.assertion_context()
    );

    // Subsequent repeats continue using the extended pattern.
    cx.simulate_keystrokes(";");

    expected_lines[0] = "abc".to_string();
    expected_lines[100] = "ˇabc".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_auto_global_search_on_viewport_miss(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let mut lines = vec!["xx".to_string(); 2000];
    lines[0] = "ˇxxabxxabyyab".to_string();
    lines[1500] = "abc".to_string();
    lines[1600] = "abc".to_string();
    let text = lines.join("\n");
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state(&text, Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        highlights.len() > 1 && highlights.iter().any(|highlight| highlight.label.is_some()),
        "beam jump should be active with labels visible. {}",
        cx.assertion_context()
    );

    // Viewport has `ab` candidates, but no `abc`, so extending triggers auto-global navigation.
    cx.simulate_keystrokes("c");

    let mut expected_lines = lines.clone();
    expected_lines[0] = "xxabxxabyyab".to_string();
    expected_lines[1500] = "ˇabc".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after auto-global jump. {}",
        cx.assertion_context()
    );

    let cursor_is_visible = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let layout = editor.text_layout_details(window);
        let buffer = snapshot.display_snapshot.buffer_snapshot();

        let cursor = editor
            .selections
            .newest_anchor()
            .head()
            .to_display_point(&snapshot.display_snapshot);
        let cursor_offset = cursor.to_offset(&snapshot.display_snapshot, Bias::Left);

        let (visible_start, visible_end) = if let Some(visible_rows) = layout.visible_rows {
            let visible_rows = visible_rows.ceil() as u32;
            let visible_start_point = layout.scroll_anchor.anchor.to_point(&buffer);
            let visible_end_point = buffer.clip_point(
                visible_start_point + Point::new(visible_rows, 0),
                Bias::Left,
            );
            (
                visible_start_point.to_offset(&buffer),
                visible_end_point.to_offset(&buffer),
            )
        } else {
            (MultiBufferOffset(0), buffer.len())
        };

        cursor_offset >= visible_start && cursor_offset <= visible_end
    });
    assert!(
        cursor_is_visible,
        "expected cursor to be visible after jump. {}",
        cx.assertion_context()
    );

    // Subsequent repeats continue using the extended pattern.
    cx.simulate_keystrokes(";");

    expected_lines[1500] = "abc".to_string();
    expected_lines[1600] = "ˇabc".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_auto_global_no_matches_cancels_and_consumes_key(
    cx: &mut gpui::TestAppContext,
) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("ˇxxabxxabyyab", Mode::Normal);

    // Seed `last_find` so we can confirm it is restored.
    cx.simulate_keystrokes("f x");
    cx.assert_state("xˇxabxxabyyab", Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state("xˇxabxxabyyab", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        highlights.len() > 1 && highlights.iter().any(|highlight| highlight.label.is_some()),
        "beam jump should be active with labels visible. {}",
        cx.assertion_context()
    );

    // Extending to a pattern with no global matches cancels Beam Jump and consumes the key.
    cx.simulate_keystrokes("l");
    cx.run_until_parked();

    cx.assert_state("xˇxabxxabyyab", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after cancellation. {}",
        cx.assertion_context()
    );

    // Repeat should still use the pre-existing `last_find` motion.
    cx.simulate_keystrokes(";");
    cx.assert_state("xxabˇxxabyyab", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_cancel_restores_last_find(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("ˇa xabx abx", Mode::Normal);
    cx.simulate_keystrokes("f x");
    cx.assert_state("a ˇxabx abx", Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state("a ˇxabx abx", Mode::Normal);

    cx.simulate_keystrokes("escape");
    cx.assert_state("a ˇxabx abx", Mode::Normal);

    cx.simulate_keystrokes(";");
    cx.assert_state("a xabˇx abx", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_hides_labels_when_units_unsafe(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let base_labels = [
        'f', 'j', 'd', 'k', 's', 'l', 'a', 'g', 'h', 'r', 'u', 'e', 'i', 'o', 'w', 'm', 'n', 'c',
        'v', 'x', 'z', 'p', 'q', 'y', 't', 'b',
    ];
    let mut text = String::from("ˇx");
    for ch in base_labels {
        text.push('a');
        text.push('a');
        text.push(ch);
        text.push('x');
    }

    cx.set_state(&text, Mode::Normal);
    cx.simulate_keystrokes("s a a");
    cx.assert_state(&text, Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        highlights.len() > 1,
        "expected multiple matches for label assignment. {}",
        cx.assertion_context()
    );
    assert!(
        highlights.iter().all(|highlight| highlight.label.is_none()),
        "expected labels to stay hidden when all units are unsafe. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_excludes_cross_cursor_match_from_pending_commit(
    cx: &mut gpui::TestAppContext,
) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("aˇb", Mode::Normal);

    cx.simulate_keystrokes("s a");
    cx.assert_state("aˇb", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert_eq!(
        highlights.len(),
        1,
        "expected a single 1-char match highlight. {}",
        cx.assertion_context()
    );
    assert!(
        highlights.iter().all(|highlight| highlight.label.is_none()),
        "expected labels to be hidden until pattern length >= 2. {}",
        cx.assertion_context()
    );

    // Typing `b` would normally produce an `ab` match whose range crosses the cursor.
    // Such matches cannot be reached by the existing jump logic and must not be rendered.
    cx.simulate_keystrokes("b");
    cx.assert_state("aˇb", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        highlights.is_empty(),
        "expected cross-cursor matches to be excluded from viewport candidates. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_extension_drops_cross_cursor_match(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("abˇc abc", Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state("abˇc abc", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        highlights.len() > 1 && highlights.iter().any(|highlight| highlight.label.is_some()),
        "expected multi-candidate mode before extension. {}",
        cx.assertion_context()
    );

    // Extending to `abc` should drop the match that would cross the cursor.
    cx.simulate_keystrokes("c");
    cx.assert_state("abˇc abc", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    assert_eq!(
        highlights.len(),
        1,
        "expected the cross-cursor candidate to be removed. {}",
        cx.assertion_context()
    );
    assert_eq!(
        highlights
            .first()
            .and_then(|highlight| highlight.label.as_ref())
            .map(|label| label.as_ref()),
        Some(";"),
        "expected pending-commit on the remaining candidate. {}",
        cx.assertion_context()
    );
    assert_eq!(
        highlights[0].range,
        MultiBufferOffset(4)..MultiBufferOffset(7),
        "expected the remaining match to be after the cursor. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_pending_commit_does_not_jump_until_semicolon(
    cx: &mut gpui::TestAppContext,
) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let mut lines = vec!["xx".to_string(); 2000];
    lines[0] = "coxxˇyy".to_string();
    // Ensure `;` in pending-commit is not treated as global navigation.
    lines[1500] = "co".to_string();
    let text = lines.join("\n");
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s c o");
    cx.assert_state(&text, Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert_eq!(
        highlights.len(),
        1,
        "expected a single pending-commit candidate. {}",
        cx.assertion_context()
    );
    assert_eq!(
        highlights
            .first()
            .and_then(|highlight| highlight.label.as_ref())
            .map(|label| label.as_ref()),
        Some(";"),
        "expected `;` to be the pending-commit label key. {}",
        cx.assertion_context()
    );

    cx.simulate_keystrokes(";");

    let mut expected_lines = vec!["xx".to_string(); 2000];
    expected_lines[0] = "ˇcoxxyy".to_string();
    expected_lines[1500] = "co".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after pending-commit jump. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_pending_commit_auto_commits_after_delay(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let mut lines = vec!["xx".to_string(); 2000];
    lines[0] = "coxxˇyy".to_string();
    lines[1500] = "co".to_string();
    let text = lines.join("\n");
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s c o");
    cx.assert_state(&text, Mode::Normal);

    cx.executor()
        .advance_clock(BEAM_JUMP_PENDING_COMMIT_TIMEOUT);
    cx.run_until_parked();

    let mut expected_lines = vec!["xx".to_string(); 2000];
    expected_lines[0] = "ˇcoxxyy".to_string();
    expected_lines[1500] = "co".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after timed pending-commit jump. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_pending_commit_treats_fast_typing_as_pattern_extension(
    cx: &mut gpui::TestAppContext,
) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let mut lines = vec!["xx".to_string(); 2000];
    lines[0] = "cozyxxˇyy".to_string();
    lines[1500] = "codex".to_string();
    let text = lines.join("\n");
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s c o");
    cx.assert_state(&text, Mode::Normal);

    // `co` is pending-commit, but typing another character should be treated as pattern extension.
    // Since `cod` has V==0 in the viewport, Beam Jump should auto-trigger global navigation.
    cx.simulate_keystrokes("d");

    let mut expected_lines = vec!["xx".to_string(); 2000];
    expected_lines[0] = "cozyxxyy".to_string();
    expected_lines[1500] = "ˇcodex".to_string();
    let expected = expected_lines.join("\n");
    cx.assert_state(&expected, Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after auto-global jump. {}",
        cx.assertion_context()
    );

    // The timer from pending-commit should not fire after the session exits.
    cx.executor().advance_clock(Duration::from_millis(200));
    cx.run_until_parked();
    cx.assert_state(&expected, Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_semicolon_jumps_global_and_exits(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("ˇxxabxxabyyab", Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state("ˇxxabxxabyyab", Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        highlights.iter().any(|highlight| highlight.label.is_some()),
        "expected labels to be visible in multi-candidate mode. {}",
        cx.assertion_context()
    );
    assert!(
        highlights
            .iter()
            .filter_map(|highlight| highlight.label.as_ref())
            .all(|label| !label.as_ref().contains(';')),
        "expected `;` to remain reserved outside pending-commit. {}",
        cx.assertion_context()
    );

    cx.simulate_keystrokes(";");
    cx.assert_state("xxˇabxxabyyab", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after in-session ;. {}",
        cx.assertion_context()
    );

    // Subsequent repeats continue using the last Beam Jump pattern.
    cx.simulate_keystrokes(";");
    cx.assert_state("xxabxxˇabyyab", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_repeat_uses_arbitrary_length_pattern(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("ˇxxabcxxabcxxabc", Mode::Normal);

    cx.simulate_keystrokes("s a b c");
    cx.assert_state("ˇxxabcxxabcxxabc", Mode::Normal);

    cx.simulate_keystrokes(";");
    cx.assert_state("xxˇabcxxabcxxabc", Mode::Normal);

    cx.simulate_keystrokes(";");
    cx.assert_state("xxabcxxˇabcxxabc", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_comma_jumps_global_and_exits(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("abxxabxxabxxˇab", Mode::Normal);
    cx.simulate_keystrokes("s a b");
    cx.assert_state("abxxabxxabxxˇab", Mode::Normal);

    cx.simulate_keystrokes(",");
    cx.assert_state("abxxabxxˇabxxab", Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after in-session ,. {}",
        cx.assertion_context()
    );

    // Subsequent repeats continue using the last Beam Jump pattern, with `;` forward.
    cx.simulate_keystrokes(";");
    cx.assert_state("abxxabxxabxxˇab", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_beam_jump_shift_s_preserves_substitute_line(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    // Simulate the Sneak docs keybinding even though Beam Jump is enabled.
    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "shift-s",
            PushSneakBackward { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state(
        indoc! {"
        one
        ˇtwo
        three
    "},
        Mode::Normal,
    );

    cx.simulate_keystrokes("S");
    cx.assert_state(
        indoc! {"
        one
        ˇ
        three
    "},
        Mode::Insert,
    );

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "expected `S` to not enter Beam Jump when enabled. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_invalid_label_sequence_cancels(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let text = format!("ˇx{}", "ab".repeat(30));
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state(&text, Mode::Normal);

    // `b b` are valid label keys but the pair doesn't match any target.
    cx.simulate_keystrokes("b b");
    cx.assert_state(&text, Mode::Normal);
    assert_eq!(cx.active_operator(), None, "{}", cx.assertion_context());

    let remaining = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        remaining.is_empty(),
        "beam jump highlights should be cleared after invalid label. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_overflow_leaves_unlabeled_matches(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let text = format!("ˇx{}", "ab".repeat(700));
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state(&text, Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });

    let labeled = highlights.iter().filter(|h| h.label.is_some()).count();
    let unlabeled = highlights.len() - labeled;
    assert!(
        unlabeled > 0,
        "expected overflow to leave unlabeled matches. {}",
        cx.assertion_context()
    );
    assert!(
        highlights
            .iter()
            .filter_map(|h| h.label.as_ref())
            .all(|label| label.chars().count() == 2),
        "expected only 2-char labels. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_backfills_labels_after_narrowing(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    let text = format!("ˇx{}{}", "ab".repeat(570), "abc".repeat(10));
    cx.set_state(&text, Mode::Normal);

    cx.simulate_keystrokes("s a b");
    cx.assert_state(&text, Mode::Normal);

    let initial_highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(
        initial_highlights.iter().any(|h| h.label.is_none()),
        "expected overflow to leave unlabeled matches. {}",
        cx.assertion_context()
    );

    cx.simulate_keystrokes("c");
    cx.assert_state(&text, Mode::Normal);

    let highlights = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.display_snapshot.buffer_snapshot().len();
        editor
            .beam_jump_highlights_in_range(MultiBufferOffset(0)..len)
            .to_vec()
    });
    assert!(highlights.len() > 1, "{}", cx.assertion_context());
    assert!(
        highlights.iter().all(|h| h.label.is_some()),
        "expected labels to be backfilled after narrowing. {}",
        cx.assertion_context()
    );
}

#[perf]
#[gpui::test]
async fn test_beam_jump_v0_does_not_jump_while_typing(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.set_state("ˇxxabxx", Mode::Normal);

    let action = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let buffer = snapshot.display_snapshot.buffer_snapshot();

        let mut state = BeamJumpState::new(
            false,
            MultiBufferOffset(0),
            MultiBufferOffset(0),
            MultiBufferOffset(2),
            None,
        );
        let _ = state.on_typed_char('a', &buffer);
        state.on_typed_char('b', &buffer)
    });

    match action {
        BeamJumpAction::Continue => {}
        other => panic!("expected Continue, got {other:?}"),
    }
}

#[perf]
#[gpui::test]
async fn test_beam_jump_moves_primary_cursor_only(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            s.vim.get_or_insert_with(Default::default).beam_jump = Some(true);
        });
    });

    cx.update(|_window, cx| {
        cx.bind_keys([KeyBinding::new(
            "s",
            PushSneak { first_char: None },
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state("ˇxxab xxab ˇxxab", Mode::Normal);
    cx.simulate_keystrokes("s a b ;");
    cx.assert_state("ˇxxab xxab xxˇab", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_plus_minus(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state(indoc! {
        "one
           two
        thrˇee
    "})
        .await;

    cx.simulate_shared_keystrokes("-").await;
    cx.shared_state().await.assert_matches();
    cx.simulate_shared_keystrokes("-").await;
    cx.shared_state().await.assert_matches();
    cx.simulate_shared_keystrokes("+").await;
    cx.shared_state().await.assert_matches();
}

#[perf]
#[gpui::test]
async fn test_command_alias(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |s| {
            let mut aliases = HashMap::default();
            aliases.insert("Q".to_string(), "upper".to_string());
            s.workspace.command_aliases = aliases
        });
    });

    cx.set_state("ˇhello world", Mode::Normal);
    cx.simulate_keystrokes(": Q");
    cx.set_state("ˇHello world", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_remap_adjacent_dog_cat(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.update(|_, cx| {
        cx.bind_keys([
            KeyBinding::new(
                "d o g",
                workspace::SendKeystrokes("🐶".to_string()),
                Some("vim_mode == insert"),
            ),
            KeyBinding::new(
                "c a t",
                workspace::SendKeystrokes("🐱".to_string()),
                Some("vim_mode == insert"),
            ),
        ])
    });
    cx.neovim.exec("imap dog 🐶").await;
    cx.neovim.exec("imap cat 🐱").await;

    cx.set_shared_state("ˇ").await;
    cx.simulate_shared_keystrokes("i d o g").await;
    cx.shared_state().await.assert_eq("🐶ˇ");

    cx.set_shared_state("ˇ").await;
    cx.simulate_shared_keystrokes("i d o d o g").await;
    cx.shared_state().await.assert_eq("do🐶ˇ");

    cx.set_shared_state("ˇ").await;
    cx.simulate_shared_keystrokes("i d o c a t").await;
    cx.shared_state().await.assert_eq("do🐱ˇ");
}

#[perf]
#[gpui::test]
async fn test_remap_nested_pineapple(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.update(|_, cx| {
        cx.bind_keys([
            KeyBinding::new(
                "p i n",
                workspace::SendKeystrokes("📌".to_string()),
                Some("vim_mode == insert"),
            ),
            KeyBinding::new(
                "p i n e",
                workspace::SendKeystrokes("🌲".to_string()),
                Some("vim_mode == insert"),
            ),
            KeyBinding::new(
                "p i n e a p p l e",
                workspace::SendKeystrokes("🍍".to_string()),
                Some("vim_mode == insert"),
            ),
        ])
    });
    cx.neovim.exec("imap pin 📌").await;
    cx.neovim.exec("imap pine 🌲").await;
    cx.neovim.exec("imap pineapple 🍍").await;

    cx.set_shared_state("ˇ").await;
    cx.simulate_shared_keystrokes("i p i n").await;
    cx.executor().advance_clock(Duration::from_millis(1000));
    cx.run_until_parked();
    cx.shared_state().await.assert_eq("📌ˇ");

    cx.set_shared_state("ˇ").await;
    cx.simulate_shared_keystrokes("i p i n e").await;
    cx.executor().advance_clock(Duration::from_millis(1000));
    cx.run_until_parked();
    cx.shared_state().await.assert_eq("🌲ˇ");

    cx.set_shared_state("ˇ").await;
    cx.simulate_shared_keystrokes("i p i n e a p p l e").await;
    cx.shared_state().await.assert_eq("🍍ˇ");
}

#[perf]
#[gpui::test]
async fn test_remap_recursion(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "x",
            workspace::SendKeystrokes("\" _ x".to_string()),
            Some("VimControl"),
        )]);
        cx.bind_keys([KeyBinding::new(
            "y",
            workspace::SendKeystrokes("2 x".to_string()),
            Some("VimControl"),
        )])
    });
    cx.neovim.exec("noremap x \"_x").await;
    cx.neovim.exec("map y 2x").await;

    cx.set_shared_state("ˇhello").await;
    cx.simulate_shared_keystrokes("d l").await;
    cx.shared_clipboard().await.assert_eq("h");
    cx.simulate_shared_keystrokes("y").await;
    cx.shared_clipboard().await.assert_eq("h");
    cx.shared_state().await.assert_eq("ˇlo");
}

#[perf]
#[gpui::test]
async fn test_escape_while_waiting(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_shared_state("ˇhi").await;
    cx.simulate_shared_keystrokes("\" + escape x").await;
    cx.shared_state().await.assert_eq("ˇi");
}

#[perf]
#[gpui::test]
async fn test_ctrl_w_override(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new("ctrl-w", DeleteLine, None)]);
    });
    cx.neovim.exec("map <c-w> D").await;
    cx.set_shared_state("ˇhi").await;
    cx.simulate_shared_keystrokes("ctrl-w").await;
    cx.shared_state().await.assert_eq("ˇ");
}

#[perf]
#[gpui::test]
async fn test_visual_indent_count(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.set_state("ˇhi", Mode::Normal);
    cx.simulate_keystrokes("shift-v 3 >");
    cx.assert_state("            ˇhi", Mode::Normal);
    cx.simulate_keystrokes("shift-v 2 <");
    cx.assert_state("    ˇhi", Mode::Normal);
}

#[perf]
#[gpui::test]
async fn test_record_replay_recursion(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("ˇhello world").await;
    cx.simulate_shared_keystrokes(">").await;
    cx.simulate_shared_keystrokes(".").await;
    cx.simulate_shared_keystrokes(".").await;
    cx.simulate_shared_keystrokes(".").await;
    cx.shared_state().await.assert_eq("ˇhello world");
}

#[perf]
#[gpui::test]
async fn test_blackhole_register(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("ˇhello world").await;
    cx.simulate_shared_keystrokes("d i w \" _ d a w").await;
    cx.simulate_shared_keystrokes("p").await;
    cx.shared_state().await.assert_eq("hellˇo");
}

#[perf]
#[gpui::test]
async fn test_sentence_backwards(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("one\n\ntwo\nthree\nˇ\nfour").await;
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state()
        .await
        .assert_eq("one\n\nˇtwo\nthree\n\nfour");

    cx.set_shared_state("hello.\n\n\nworˇld.").await;
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq("hello.\n\n\nˇworld.");
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq("hello.\n\nˇ\nworld.");
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq("ˇhello.\n\n\nworld.");

    cx.set_shared_state("hello. worlˇd.").await;
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq("hello. ˇworld.");
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq("ˇhello. world.");

    cx.set_shared_state(". helˇlo.").await;
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq(". ˇhello.");
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq(". ˇhello.");

    cx.set_shared_state(indoc! {
        "{
            hello_world();
        ˇ}"
    })
    .await;
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq(indoc! {
        "ˇ{
            hello_world();
        }"
    });

    cx.set_shared_state(indoc! {
        "Hello! World..?

        \tHello! World... ˇ"
    })
    .await;
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq(indoc! {
        "Hello! World..?

        \tHello! ˇWorld... "
    });
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq(indoc! {
        "Hello! World..?

        \tˇHello! World... "
    });
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq(indoc! {
        "Hello! World..?
        ˇ
        \tHello! World... "
    });
    cx.simulate_shared_keystrokes("(").await;
    cx.shared_state().await.assert_eq(indoc! {
        "Hello! ˇWorld..?

        \tHello! World... "
    });
}

#[perf]
#[gpui::test]
async fn test_sentence_forwards(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("helˇlo.\n\n\nworld.").await;
    cx.simulate_shared_keystrokes(")").await;
    cx.shared_state().await.assert_eq("hello.\nˇ\n\nworld.");
    cx.simulate_shared_keystrokes(")").await;
    cx.shared_state().await.assert_eq("hello.\n\n\nˇworld.");
    cx.simulate_shared_keystrokes(")").await;
    cx.shared_state().await.assert_eq("hello.\n\n\nworldˇ.");

    cx.set_shared_state("helˇlo.\n\n\nworld.").await;
}

#[perf]
#[gpui::test]
async fn test_ctrl_o_visual(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("helloˇ world.").await;
    cx.simulate_shared_keystrokes("i ctrl-o v b r l").await;
    cx.shared_state().await.assert_eq("ˇllllllworld.");
    cx.simulate_shared_keystrokes("ctrl-o v f w d").await;
    cx.shared_state().await.assert_eq("ˇorld.");
}

#[perf]
#[gpui::test]
async fn test_ctrl_o_position(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("helˇlo world.").await;
    cx.simulate_shared_keystrokes("i ctrl-o d i w").await;
    cx.shared_state().await.assert_eq("ˇ world.");
    cx.simulate_shared_keystrokes("ctrl-o p").await;
    cx.shared_state().await.assert_eq(" helloˇworld.");
}

#[perf]
#[gpui::test]
async fn test_ctrl_o_dot(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("heˇllo world.").await;
    cx.simulate_shared_keystrokes("x i ctrl-o .").await;
    cx.shared_state().await.assert_eq("heˇo world.");
    cx.simulate_shared_keystrokes("l l escape .").await;
    cx.shared_state().await.assert_eq("hellˇllo world.");
}

#[perf(iterations = 1)]
#[gpui::test]
async fn test_folded_multibuffer_excerpts(cx: &mut gpui::TestAppContext) {
    VimTestContext::init(cx);
    cx.update(|cx| {
        VimTestContext::init_keybindings(true, cx);
    });
    let (editor, cx) = cx.add_window_view(|window, cx| {
        let multi_buffer = MultiBuffer::build_multi(
            [
                ("111\n222\n333\n444\n", vec![Point::row_range(0..2)]),
                ("aaa\nbbb\nccc\nddd\n", vec![Point::row_range(0..2)]),
                ("AAA\nBBB\nCCC\nDDD\n", vec![Point::row_range(0..2)]),
                ("one\ntwo\nthr\nfou\n", vec![Point::row_range(0..2)]),
            ],
            cx,
        );
        let mut editor = Editor::new(EditorMode::full(), multi_buffer.clone(), None, window, cx);

        let buffer_ids = multi_buffer.read(cx).excerpt_buffer_ids();
        // fold all but the second buffer, so that we test navigating between two
        // adjacent folded buffers, as well as folded buffers at the start and
        // end the multibuffer
        editor.fold_buffer(buffer_ids[0], cx);
        editor.fold_buffer(buffer_ids[2], cx);
        editor.fold_buffer(buffer_ids[3], cx);

        editor
    });
    let mut cx = EditorTestContext::for_editor_in(editor.clone(), cx).await;

    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        aaa
        bbb
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("j");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        ˇaaa
        bbb
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("j");
    cx.simulate_keystroke("j");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        aaa
        bbb
        ˇ[EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("j");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        aaa
        bbb
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("j");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        aaa
        bbb
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        ˇ[FOLDED]
        "
    });
    cx.simulate_keystroke("k");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        aaa
        bbb
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("k");
    cx.simulate_keystroke("k");
    cx.simulate_keystroke("k");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        ˇaaa
        bbb
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("k");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        aaa
        bbb
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("shift-g");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        aaa
        bbb
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        ˇ[FOLDED]
        "
    });
    cx.simulate_keystrokes("g g");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        aaa
        bbb
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.update_editor(|editor, _, cx| {
        let buffer_ids = editor.buffer().read(cx).excerpt_buffer_ids();
        editor.fold_buffer(buffer_ids[1], cx);
    });

    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystrokes("2 j");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
}

#[perf]
#[gpui::test]
async fn test_delete_paragraph_motion(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_shared_state(indoc! {
        "ˇhello world.

        hello world.
        "
    })
    .await;
    cx.simulate_shared_keystrokes("y }").await;
    cx.shared_clipboard().await.assert_eq("hello world.\n");
    cx.simulate_shared_keystrokes("d }").await;
    cx.shared_state().await.assert_eq("ˇ\nhello world.\n");
    cx.shared_clipboard().await.assert_eq("hello world.\n");

    cx.set_shared_state(indoc! {
        "helˇlo world.

            hello world.
            "
    })
    .await;
    cx.simulate_shared_keystrokes("y }").await;
    cx.shared_clipboard().await.assert_eq("lo world.");
    cx.simulate_shared_keystrokes("d }").await;
    cx.shared_state().await.assert_eq("heˇl\n\nhello world.\n");
    cx.shared_clipboard().await.assert_eq("lo world.");
}

#[perf]
#[gpui::test]
async fn test_delete_unmatched_brace(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_shared_state(indoc! {
        "fn o(wow: i32) {
          othˇ(wow)
          oth(wow)
        }
        "
    })
    .await;
    cx.simulate_shared_keystrokes("d ] }").await;
    cx.shared_state().await.assert_eq(indoc! {
        "fn o(wow: i32) {
          otˇh
        }
        "
    });
    cx.shared_clipboard().await.assert_eq("(wow)\n  oth(wow)");
    cx.set_shared_state(indoc! {
        "fn o(wow: i32) {
          ˇoth(wow)
          oth(wow)
        }
        "
    })
    .await;
    cx.simulate_shared_keystrokes("d ] }").await;
    cx.shared_state().await.assert_eq(indoc! {
        "fn o(wow: i32) {
         ˇ}
        "
    });
    cx.shared_clipboard()
        .await
        .assert_eq("  oth(wow)\n  oth(wow)\n");
}

#[perf]
#[gpui::test]
async fn test_paragraph_multi_delete(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_shared_state(indoc! {
        "
        Emacs is
        ˇa great

        operating system

        all it lacks
        is a

        decent text editor
        "
    })
    .await;

    cx.simulate_shared_keystrokes("2 d a p").await;
    cx.shared_state().await.assert_eq(indoc! {
        "
        ˇall it lacks
        is a

        decent text editor
        "
    });

    cx.simulate_shared_keystrokes("d a p").await;
    cx.shared_clipboard()
        .await
        .assert_eq("all it lacks\nis a\n\n");

    //reset to initial state
    cx.simulate_shared_keystrokes("2 u").await;

    cx.simulate_shared_keystrokes("4 d a p").await;
    cx.shared_state().await.assert_eq(indoc! {"ˇ"});
}

#[perf]
#[gpui::test]
async fn test_yank_paragraph_with_paste(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_shared_state(indoc! {
        "
        first paragraph
        ˇstill first

        second paragraph
        still second

        third paragraph
        "
    })
    .await;

    cx.simulate_shared_keystrokes("y a p").await;
    cx.shared_clipboard()
        .await
        .assert_eq("first paragraph\nstill first\n\n");

    cx.simulate_shared_keystrokes("j j p").await;
    cx.shared_state().await.assert_eq(indoc! {
        "
        first paragraph
        still first

        ˇfirst paragraph
        still first

        second paragraph
        still second

        third paragraph
        "
    });
}

#[perf]
#[gpui::test]
async fn test_change_paragraph(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_shared_state(indoc! {
        "
        first paragraph
        ˇstill first

        second paragraph
        still second

        third paragraph
        "
    })
    .await;

    cx.simulate_shared_keystrokes("c a p").await;
    cx.shared_clipboard()
        .await
        .assert_eq("first paragraph\nstill first\n\n");

    cx.simulate_shared_keystrokes("escape").await;
    cx.shared_state().await.assert_eq(indoc! {
        "
        ˇ
        second paragraph
        still second

        third paragraph
        "
    });
}

#[perf]
#[gpui::test]
async fn test_multi_cursor_replay(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.set_state(
        indoc! {
            "
        oˇne one one

        two two two
        "
        },
        Mode::Normal,
    );

    cx.simulate_keystrokes("3 g l s wow escape escape");
    cx.assert_state(
        indoc! {
            "
        woˇw wow wow

        two two two
        "
        },
        Mode::Normal,
    );

    cx.simulate_keystrokes("2 j 3 g l .");
    cx.assert_state(
        indoc! {
            "
        wow wow wow

        woˇw woˇw woˇw
        "
        },
        Mode::Normal,
    );
}

#[gpui::test]
async fn test_clipping_on_mode_change(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(
        indoc! {
        "
        ˇverylongline
        andsomelinebelow
        "
        },
        Mode::Normal,
    );

    cx.simulate_keystrokes("v e");
    cx.assert_state(
        indoc! {
        "
        «verylonglineˇ»
        andsomelinebelow
        "
        },
        Mode::Visual,
    );

    let mut pixel_position = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let current_head = editor
            .selections
            .newest_display(&snapshot.display_snapshot)
            .end;
        editor.last_bounds().unwrap().origin
            + editor
                .display_to_pixel_point(current_head, &snapshot, window, cx)
                .unwrap()
    });
    pixel_position.x += px(100.);
    // click beyond end of the line
    cx.simulate_click(pixel_position, Modifiers::default());
    cx.run_until_parked();

    cx.assert_state(
        indoc! {
        "
        verylonglinˇe
        andsomelinebelow
        "
        },
        Mode::Normal,
    );
}

#[gpui::test]
async fn test_wrap_selections_in_tag_line_mode(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    let js_language = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            wrap_characters: Some(language::WrapCharactersConfig {
                start_prefix: "<".into(),
                start_suffix: ">".into(),
                end_prefix: "</".into(),
                end_suffix: ">".into(),
            }),
            ..LanguageConfig::default()
        },
        None,
    ));

    cx.update_buffer(|buffer, cx| buffer.set_language(Some(js_language), cx));

    cx.set_state(
        indoc! {
        "
        ˇaaaaa
        bbbbb
        "
        },
        Mode::Normal,
    );

    cx.simulate_keystrokes("shift-v j");
    cx.dispatch_action(WrapSelectionsInTag);

    cx.assert_state(
        indoc! {
            "
            <ˇ>aaaaa
            bbbbb</ˇ>
            "
        },
        Mode::VisualLine,
    );
}

#[gpui::test]
async fn test_repeat_grouping_41735(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    // typically transaction gropuing is disabled in tests, but here we need to test it.
    cx.update_buffer(|buffer, _cx| buffer.set_group_interval(Duration::from_millis(300)));

    cx.set_shared_state("ˇ").await;

    cx.simulate_shared_keystrokes("i a escape").await;
    cx.simulate_shared_keystrokes(". . .").await;
    cx.shared_state().await.assert_eq("ˇaaaa");
    cx.simulate_shared_keystrokes("u").await;
    cx.shared_state().await.assert_eq("ˇaaa");
}

#[gpui::test]
async fn test_deactivate(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |settings| {
            settings.editor.cursor_shape = Some(settings::CursorShape::Underline);
        });
    });

    // Assert that, while in `Normal` mode, the cursor shape is `Block` but,
    // after deactivating vim mode, it should revert to the one specified in the
    // user's settings, if set.
    cx.update_editor(|editor, _window, _cx| {
        assert_eq!(editor.cursor_shape(), CursorShape::Block);
    });

    cx.disable_vim();

    cx.update_editor(|editor, _window, _cx| {
        assert_eq!(editor.cursor_shape(), CursorShape::Underline);
    });
}

// workspace::SendKeystrokes should pass literal keystrokes without triggering vim motions.
// When sending `" _ x`, the `_` should select the blackhole register, not trigger
// vim::StartOfLineDownward.
#[gpui::test]
async fn test_send_keystrokes_underscore_is_literal_46509(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    // Bind a key to send `" _ x` which should:
    // `"` - start register selection
    // `_` - select blackhole register (NOT vim::StartOfLineDownward)
    // `x` - delete character into blackhole register
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g x",
            workspace::SendKeystrokes("\" _ x".to_string()),
            Some("VimControl"),
        )])
    });

    cx.set_state("helˇlo", Mode::Normal);

    cx.simulate_keystrokes("g x");
    cx.run_until_parked();

    cx.assert_state("helˇo", Mode::Normal);
}

#[gpui::test]
async fn test_send_keystrokes_no_key_equivalent_mapping_46509(cx: &mut gpui::TestAppContext) {
    use collections::HashMap;
    use gpui::{KeybindingKeystroke, Keystroke, PlatformKeyboardMapper};

    // create a mock Danish keyboard mapper
    // on Danish keyboards, the macOS key equivalents mapping includes: '{' -> 'Æ' and '}' -> 'Ø'
    // this means the `{` character is produced by the key labeled `Æ` (with shift modifier)
    struct DanishKeyboardMapper;
    impl PlatformKeyboardMapper for DanishKeyboardMapper {
        fn map_key_equivalent(
            &self,
            mut keystroke: Keystroke,
            use_key_equivalents: bool,
        ) -> KeybindingKeystroke {
            if use_key_equivalents {
                if keystroke.key == "{" {
                    keystroke.key = "Æ".to_string();
                }
                if keystroke.key == "}" {
                    keystroke.key = "Ø".to_string();
                }
            }
            KeybindingKeystroke::from_keystroke(keystroke)
        }

        fn get_key_equivalents(&self) -> Option<&HashMap<char, char>> {
            None
        }
    }

    let mapper = DanishKeyboardMapper;

    let keystroke_brace = Keystroke::parse("{").unwrap();
    let mapped_with_bug = mapper.map_key_equivalent(keystroke_brace.clone(), true);
    assert_eq!(
        mapped_with_bug.key(),
        "Æ",
        "BUG: With use_key_equivalents=true, {{ is mapped to Æ on Danish keyboard"
    );

    // Fixed behavior, where the literal `{` character is preserved
    let mapped_fixed = mapper.map_key_equivalent(keystroke_brace.clone(), false);
    assert_eq!(
        mapped_fixed.key(),
        "{",
        "FIX: With use_key_equivalents=false, {{ stays as {{"
    );

    // Same applies to }
    let keystroke_close = Keystroke::parse("}").unwrap();
    let mapped_close_bug = mapper.map_key_equivalent(keystroke_close.clone(), true);
    assert_eq!(mapped_close_bug.key(), "Ø");
    let mapped_close_fixed = mapper.map_key_equivalent(keystroke_close.clone(), false);
    assert_eq!(mapped_close_fixed.key(), "}");

    let mut cx = VimTestContext::new(cx, true).await;

    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "g p",
            workspace::SendKeystrokes("{".to_string()),
            Some("vim_mode == normal"),
        )])
    });

    cx.set_state(
        indoc! {"
            first paragraph

            second paragraphˇ

            third paragraph
        "},
        Mode::Normal,
    );

    cx.simulate_keystrokes("g p");
    cx.run_until_parked();

    cx.assert_state(
        indoc! {"
            first paragraph
            ˇ
            second paragraph

            third paragraph
        "},
        Mode::Normal,
    );
}

#[gpui::test]
async fn test_project_search_opens_in_normal_mode(cx: &mut gpui::TestAppContext) {
    VimTestContext::init(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "file_a.rs": "// File A.",
            "file_b.rs": "// File B.",
        }),
    )
    .await;

    let project = project::Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
    let workspace =
        cx.add_window(|window, cx| workspace::Workspace::test_new(project.clone(), window, cx));

    cx.update(|cx| {
        VimTestContext::init_keybindings(true, cx);
    });

    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    workspace
        .update(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(workspace, &DeploySearch::default(), window, cx)
        })
        .unwrap();

    let search_view = workspace
        .update(cx, |workspace, _, cx| {
            workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<ProjectSearchView>())
                .expect("Project search view should be active")
        })
        .unwrap();

    project_search::perform_project_search(&search_view, "File A", cx);

    search_view.update(cx, |search_view, cx| {
        let vim_mode = search_view
            .results_editor()
            .read(cx)
            .addon::<VimAddon>()
            .map(|addon| addon.entity.read(cx).mode);

        assert_eq!(vim_mode, Some(Mode::Normal));
    });
}
