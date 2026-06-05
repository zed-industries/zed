use crate::actions::{BracketDelimiter, ChangeBracketsTo, ChangeQuotesTo, QuoteDelimiter};
use crate::editor_tests::init_test;
use crate::test::editor_test_context::EditorTestContext;
use crate::{
    RemoveBrackets, RemoveQuotes, SelectBracketContent, SelectQuoteContent, SwapBrackets,
    SwapQuotes,
};
use gpui::TestAppContext;
use language::{Language, LanguageConfig};
use std::sync::Arc;

#[gpui::test]
async fn test_bracketeer_bracket_actions(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state("foo(bar[baˇz])");
    cx.update_editor(|editor, window, cx| editor.swap_brackets(&SwapBrackets, window, cx));
    cx.assert_editor_state("foo(bar{baˇz})");

    cx.set_state("(alˇpha)");
    cx.update_editor(|editor, window, cx| editor.remove_brackets(&RemoveBrackets, window, cx));
    cx.assert_editor_state("alˇpha");

    cx.set_state("(alˇpha)");
    cx.update_editor(|editor, window, cx| {
        editor.change_brackets_to(
            &ChangeBracketsTo {
                delimiter: BracketDelimiter::Square,
            },
            window,
            cx,
        )
    });
    cx.assert_editor_state("[alˇpha]");

    cx.set_state("(alˇpha)");
    cx.update_editor(|editor, window, cx| {
        editor.change_brackets_to(
            &ChangeBracketsTo {
                delimiter: BracketDelimiter::Angle,
            },
            window,
            cx,
        )
    });
    cx.assert_editor_state("<alˇpha>");

    cx.set_state("(alˇpha)");
    cx.update_editor(|editor, window, cx| {
        editor.select_bracket_content(&SelectBracketContent, window, cx)
    });
    cx.assert_editor_state("(«alphaˇ»)");

    cx.update_editor(|editor, window, cx| {
        editor.select_bracket_content(&SelectBracketContent, window, cx)
    });
    cx.assert_editor_state("«(alpha)ˇ»");
}

#[gpui::test]
async fn test_bracketeer_bracket_actions_do_not_match_after_caret(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state("fooˇ(bar)");
    cx.update_editor(|editor, window, cx| editor.swap_brackets(&SwapBrackets, window, cx));
    cx.assert_editor_state("fooˇ(bar)");

    cx.set_state("ˇ(alpha)");
    cx.update_editor(|editor, window, cx| {
        editor.select_bracket_content(&SelectBracketContent, window, cx)
    });
    cx.assert_editor_state("ˇ(alpha)");
}

#[gpui::test]
async fn test_bracketeer_bracket_actions_deduplicate_matches(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state("(foˇo baˇr)");
    cx.update_editor(|editor, window, cx| editor.swap_brackets(&SwapBrackets, window, cx));
    cx.assert_editor_state("[foˇo baˇr]");

    cx.set_state("(foˇo baˇr)");
    cx.update_editor(|editor, window, cx| {
        editor.change_brackets_to(
            &ChangeBracketsTo {
                delimiter: BracketDelimiter::Curly,
            },
            window,
            cx,
        )
    });
    cx.assert_editor_state("{foˇo baˇr}");

    cx.set_state("[(foˇo baˇr)]");
    cx.update_editor(|editor, window, cx| editor.remove_brackets(&RemoveBrackets, window, cx));
    cx.assert_editor_state("[foˇo baˇr]");

    cx.set_state("(foˇo(baˇr))");
    cx.update_editor(|editor, window, cx| editor.swap_brackets(&SwapBrackets, window, cx));
    cx.assert_editor_state("[foˇo[baˇr]]");

    cx.set_state("(foˇo(baˇr))");
    cx.update_editor(|editor, window, cx| editor.remove_brackets(&RemoveBrackets, window, cx));
    cx.assert_editor_state("foˇobaˇr");

    cx.set_state("(foˇo baˇr)");
    cx.update_editor(|editor, window, cx| {
        editor.select_bracket_content(&SelectBracketContent, window, cx)
    });
    cx.assert_editor_state("(«foo barˇ»)");

    cx.set_state("(foˇo(baˇr))");
    cx.update_editor(|editor, window, cx| {
        editor.select_bracket_content(&SelectBracketContent, window, cx)
    });
    cx.assert_editor_state("(«foo(bar)ˇ»)");
}

#[gpui::test]
async fn test_bracketeer_quote_actions(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let js_language = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            ..LanguageConfig::default()
        },
        None,
    ));

    cx.update_buffer(|buffer, cx| buffer.set_language(Some(js_language), cx));

    cx.set_state("const value = 'alˇpha';");
    cx.update_editor(|editor, window, cx| editor.swap_quotes(&SwapQuotes, window, cx));
    cx.assert_editor_state("const value = \"alˇpha\";");

    cx.set_state("const value = \"alˇpha\";");
    cx.update_editor(|editor, window, cx| {
        editor.change_quotes_to(
            &ChangeQuotesTo {
                delimiter: QuoteDelimiter::Backtick,
            },
            window,
            cx,
        )
    });
    cx.assert_editor_state("const value = `alˇpha`;");

    cx.set_state("const value = 'alˇpha';");
    cx.update_editor(|editor, window, cx| editor.remove_quotes(&RemoveQuotes, window, cx));
    cx.assert_editor_state("const value = alˇpha;");

    cx.set_state("const value = 'alˇpha';");
    cx.update_editor(|editor, window, cx| {
        editor.select_quote_content(&SelectQuoteContent, window, cx)
    });
    cx.assert_editor_state("const value = '«alphaˇ»';");

    cx.update_editor(|editor, window, cx| {
        editor.select_quote_content(&SelectQuoteContent, window, cx)
    });
    cx.assert_editor_state("const value = «'alpha'ˇ»;");
}

#[gpui::test]
async fn test_bracketeer_quote_actions_do_not_match_after_caret(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let js_language = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            ..LanguageConfig::default()
        },
        None,
    ));

    cx.update_buffer(|buffer, cx| buffer.set_language(Some(js_language), cx));

    cx.set_state("const value = ˇ'alpha';");
    cx.update_editor(|editor, window, cx| editor.swap_quotes(&SwapQuotes, window, cx));
    cx.assert_editor_state("const value = ˇ'alpha';");

    cx.set_state("const value = ˇ'alpha';");
    cx.update_editor(|editor, window, cx| {
        editor.select_quote_content(&SelectQuoteContent, window, cx)
    });
    cx.assert_editor_state("const value = ˇ'alpha';");
}

#[gpui::test]
async fn test_bracketeer_quote_actions_do_not_match_before_caret(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let js_language = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            ..LanguageConfig::default()
        },
        None,
    ));

    cx.update_buffer(|buffer, cx| buffer.set_language(Some(js_language), cx));

    cx.set_state("const value = 'alpha'ˇ;");
    cx.update_editor(|editor, window, cx| editor.swap_quotes(&SwapQuotes, window, cx));
    cx.assert_editor_state("const value = 'alpha'ˇ;");

    cx.set_state("const value = 'alpha'ˇ;");
    cx.update_editor(|editor, window, cx| {
        editor.select_quote_content(&SelectQuoteContent, window, cx)
    });
    cx.assert_editor_state("const value = 'alpha'ˇ;");
}

#[gpui::test]
async fn test_bracketeer_quote_actions_deduplicate_matches(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let js_language = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            ..LanguageConfig::default()
        },
        None,
    ));

    cx.update_buffer(|buffer, cx| buffer.set_language(Some(js_language), cx));

    cx.set_state("const value = 'foˇo baˇr';");
    cx.update_editor(|editor, window, cx| editor.swap_quotes(&SwapQuotes, window, cx));
    cx.assert_editor_state("const value = \"foˇo baˇr\";");

    cx.set_state("const value = 'foˇo baˇr';");
    cx.update_editor(|editor, window, cx| editor.remove_quotes(&RemoveQuotes, window, cx));
    cx.assert_editor_state("const value = foˇo baˇr;");

    cx.set_state("const value = 'foˇo baˇr';");
    cx.update_editor(|editor, window, cx| {
        editor.select_quote_content(&SelectQuoteContent, window, cx)
    });
    cx.assert_editor_state("const value = '«foo barˇ»';");
}

#[gpui::test]
async fn test_bracketeer_does_not_infer_angle_brackets(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state("const compare = left < riˇght > fallback;");
    cx.update_editor(|editor, window, cx| editor.swap_brackets(&SwapBrackets, window, cx));
    cx.assert_editor_state("const compare = left < riˇght > fallback;");
}
