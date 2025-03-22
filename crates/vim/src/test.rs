mod neovim_backed_test_context;
mod neovim_connection;
mod vim_test_context;

use std::time::Duration;

use collections::HashMap;
use command_palette::CommandPalette;
use editor::{
    actions::DeleteLine, display_map::DisplayRow, test::editor_test_context::EditorTestContext,
    DisplayPoint, Editor, EditorMode, MultiBuffer,
};
use futures::StreamExt;
use gpui::{KeyBinding, Modifiers, MouseButton, TestAppContext};
use language::Point;
pub use neovim_backed_test_context::*;
use settings::SettingsStore;
pub use vim_test_context::*;

use indoc::indoc;
use search::BufferSearchBar;
use workspace::WorkspaceSettings;

use crate::{insert::NormalBefore, motion, state::Mode, PushSneak, PushSneakBackward};

#[gpui::test]
async fn test_initially_disabled(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, false).await;
    cx.simulate_keystrokes("h j k l");
    cx.assert_editor_state("hjklˇ");
}

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

#[gpui::test]
async fn test_count_down(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state(indoc! {"aˇa\nbb\ncc\ndd\nee"}, Mode::Normal);
    cx.simulate_keystrokes("2 down");
    cx.assert_editor_state("aa\nbb\ncˇc\ndd\nee");
    cx.simulate_keystrokes("9 down");
    cx.assert_editor_state("aa\nbb\ncc\ndd\neˇe");
}

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

#[gpui::test]
async fn test_escape_command_palette(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("aˇbc\n", Mode::Normal);
    cx.simulate_keystrokes("i cmd-shift-p");

    assert!(cx.workspace(|workspace, _, cx| workspace.active_modal::<CommandPalette>(cx).is_some()));
    cx.simulate_keystrokes("escape");
    cx.run_until_parked();
    assert!(
        !cx.workspace(|workspace, _, cx| workspace.active_modal::<CommandPalette>(cx).is_some())
    );
    cx.assert_state("aˇbc\n", Mode::Insert);
}

#[gpui::test]
async fn test_escape_cancels(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;

    cx.set_state("aˇbˇc", Mode::Normal);
    cx.simulate_keystrokes("escape");

    cx.assert_state("aˇbc", Mode::Normal);
}

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
    cx.assert_state("ˇhello", Mode::Insert);
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.assert_state("jˇhello", Mode::Insert);
    cx.simulate_keystrokes("k j k");
    cx.assert_state("jˇkhello", Mode::Normal);
}

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

    cx.simulate_keystrokes("c d");
    prepare_request.next().await.unwrap();
    cx.simulate_input("after");
    cx.simulate_keystrokes("enter");
    rename_request.next().await.unwrap();
    cx.assert_state("const afterˇ = 2; console.log(after)", Mode::Normal)
}

// TODO: this test is flaky on our linux CI machines
#[cfg(target_os = "macos")]
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

    cx.executor().allow_parking();

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
    cx.simulate_keystrokes("v shift-s < l");
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
    cx.simulate_keystrokes("shift-s space 1");
    cx.assert_state(r#"11 12ˇ 13 14"#, Mode::Normal);
    cx.simulate_keystrokes(";");
    cx.assert_state(r#"11ˇ 12 13 14"#, Mode::Normal);
}

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

#[gpui::test]
async fn test_command_alias(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings::<WorkspaceSettings>(cx, |s| {
            let mut aliases = HashMap::default();
            aliases.insert("Q".to_string(), "upper".to_string());
            s.command_aliases = Some(aliases)
        });
    });

    cx.set_state("ˇhello world", Mode::Normal);
    cx.simulate_keystrokes(": Q");
    cx.set_state("ˇHello world", Mode::Normal);
}

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

#[gpui::test]
async fn test_escape_while_waiting(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;
    cx.set_shared_state("ˇhi").await;
    cx.simulate_shared_keystrokes("\" + escape x").await;
    cx.shared_state().await.assert_eq("ˇi");
}

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

#[gpui::test]
async fn test_visual_indent_count(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    cx.set_state("ˇhi", Mode::Normal);
    cx.simulate_keystrokes("shift-v 3 >");
    cx.assert_state("            ˇhi", Mode::Normal);
    cx.simulate_keystrokes("shift-v 2 <");
    cx.assert_state("    ˇhi", Mode::Normal);
}

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

#[gpui::test]
async fn test_blackhole_register(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("ˇhello world").await;
    cx.simulate_shared_keystrokes("d i w \" _ d a w").await;
    cx.simulate_shared_keystrokes("p").await;
    cx.shared_state().await.assert_eq("hellˇo");
}

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

#[gpui::test]
async fn test_ctrl_o_visual(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("helloˇ world.").await;
    cx.simulate_shared_keystrokes("i ctrl-o v b r l").await;
    cx.shared_state().await.assert_eq("ˇllllllworld.");
    cx.simulate_shared_keystrokes("ctrl-o v f w d").await;
    cx.shared_state().await.assert_eq("ˇorld.");
}

#[gpui::test]
async fn test_ctrl_o_position(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("helˇlo world.").await;
    cx.simulate_shared_keystrokes("i ctrl-o d i w").await;
    cx.shared_state().await.assert_eq("ˇ world.");
    cx.simulate_shared_keystrokes("ctrl-o p").await;
    cx.shared_state().await.assert_eq(" helloˇworld.");
}

#[gpui::test]
async fn test_ctrl_o_dot(cx: &mut gpui::TestAppContext) {
    let mut cx = NeovimBackedTestContext::new(cx).await;

    cx.set_shared_state("heˇllo world.").await;
    cx.simulate_shared_keystrokes("x i ctrl-o .").await;
    cx.shared_state().await.assert_eq("heˇo world.");
    cx.simulate_shared_keystrokes("l l escape .").await;
    cx.shared_state().await.assert_eq("hellˇllo world.");
}

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
        let mut editor = Editor::new(EditorMode::Full, multi_buffer.clone(), None, window, cx);

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
