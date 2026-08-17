// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Selector Tokenizer

use simplecss::*;

macro_rules! tokenize {
    ($name:ident, $text:expr, $( $token:expr ),*) => (
        #[test]
        fn $name() {
            let mut t = SelectorTokenizer::from($text);
            $(
                assert_eq!(t.next().unwrap().unwrap(), $token);
            )*

            assert!(t.next().is_none());
        }
    )
}

tokenize!(tokenize_01, "*", SelectorToken::UniversalSelector);

tokenize!(tokenize_02, "div", SelectorToken::TypeSelector("div"));

tokenize!(tokenize_03, "#div", SelectorToken::IdSelector("div"));

tokenize!(tokenize_04, ".div", SelectorToken::ClassSelector("div"));

tokenize!(
    tokenize_05,
    "[id]",
    SelectorToken::AttributeSelector("id", AttributeOperator::Exists)
);

tokenize!(
    tokenize_06,
    "[id=test]",
    SelectorToken::AttributeSelector("id", AttributeOperator::Matches("test"))
);

tokenize!(
    tokenize_07,
    "[id~=test]",
    SelectorToken::AttributeSelector("id", AttributeOperator::Contains("test"))
);

tokenize!(
    tokenize_08,
    "[id|=test]",
    SelectorToken::AttributeSelector("id", AttributeOperator::StartsWith("test"))
);

tokenize!(
    tokenize_09,
    "[id='test']",
    SelectorToken::AttributeSelector("id", AttributeOperator::Matches("test"))
);

tokenize!(
    tokenize_10,
    "[id=\"test\"]",
    SelectorToken::AttributeSelector("id", AttributeOperator::Matches("test"))
);

tokenize!(
    tokenize_11,
    "[id='te\\'st']",
    SelectorToken::AttributeSelector("id", AttributeOperator::Matches("te\\'st"))
);

tokenize!(
    tokenize_12,
    "[id=\"te\\\"st\"]",
    SelectorToken::AttributeSelector("id", AttributeOperator::Matches("te\\\"st"))
);

tokenize!(
    tokenize_13,
    "div:first-child",
    SelectorToken::TypeSelector("div"),
    SelectorToken::PseudoClass("first-child")
);

tokenize!(
    tokenize_14,
    ":first-child",
    SelectorToken::PseudoClass("first-child")
);

tokenize!(
    tokenize_15,
    "div p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_16,
    "div p a",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("p"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("a")
);

tokenize!(
    tokenize_17,
    "div>p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::ChildCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_18,
    "div >p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::ChildCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_19,
    "div> p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::ChildCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_20,
    "div > p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::ChildCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_21,
    "div .p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::ClassSelector("p")
);

tokenize!(
    tokenize_22,
    "div *",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::UniversalSelector
);

tokenize!(
    tokenize_23,
    "div #p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::IdSelector("p")
);

tokenize!(
    tokenize_24,
    "div [id]",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::AttributeSelector("id", AttributeOperator::Exists)
);

tokenize!(
    tokenize_25,
    "div :link",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::PseudoClass("link")
);

tokenize!(
    tokenize_26,
    "div+p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::AdjacentCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_27,
    "div +p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::AdjacentCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_28,
    "div+ p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::AdjacentCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_29,
    "div + p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::AdjacentCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(tokenize_30, "div {", SelectorToken::TypeSelector("div"));

tokenize!(tokenize_31, "div,", SelectorToken::TypeSelector("div"));

tokenize!(tokenize_32, "div{", SelectorToken::TypeSelector("div"));

tokenize!(tokenize_33, "div ,", SelectorToken::TypeSelector("div"));

tokenize!(
    tokenize_34,
    "div.test",
    SelectorToken::TypeSelector("div"),
    SelectorToken::ClassSelector("test")
);

tokenize!(
    tokenize_35,
    "div.test.warn",
    SelectorToken::TypeSelector("div"),
    SelectorToken::ClassSelector("test"),
    SelectorToken::ClassSelector("warn")
);

tokenize!(
    tokenize_36,
    "div#id",
    SelectorToken::TypeSelector("div"),
    SelectorToken::IdSelector("id")
);

tokenize!(
    tokenize_37,
    "*[id]",
    SelectorToken::UniversalSelector,
    SelectorToken::AttributeSelector("id", AttributeOperator::Exists)
);

tokenize!(
    tokenize_38,
    "*.test",
    SelectorToken::UniversalSelector,
    SelectorToken::ClassSelector("test")
);

tokenize!(
    tokenize_39,
    "*#id",
    SelectorToken::UniversalSelector,
    SelectorToken::IdSelector("id")
);

tokenize!(
    tokenize_40,
    "div * p",
    SelectorToken::TypeSelector("div"),
    SelectorToken::DescendantCombinator,
    SelectorToken::UniversalSelector,
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("p")
);

tokenize!(
    tokenize_41,
    "div[id=test][color=red]",
    SelectorToken::TypeSelector("div"),
    SelectorToken::AttributeSelector("id", AttributeOperator::Matches("test")),
    SelectorToken::AttributeSelector("color", AttributeOperator::Matches("red"))
);

tokenize!(
    tokenize_42,
    "a.external:visited",
    SelectorToken::TypeSelector("a"),
    SelectorToken::ClassSelector("external"),
    SelectorToken::PseudoClass("visited")
);

tokenize!(
    tokenize_43,
    ":lang(en)",
    SelectorToken::LangPseudoClass("en")
);

tokenize!(
    tokenize_44,
    "a\nb",
    SelectorToken::TypeSelector("a"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("b")
);

tokenize!(
    tokenize_45,
    ".warn :first-child",
    SelectorToken::ClassSelector("warn"),
    SelectorToken::DescendantCombinator,
    SelectorToken::PseudoClass("first-child")
);

macro_rules! malformed {
    ($name:ident, $text:expr, $err_str:expr) => {
        #[test]
        fn $name() {
            for token in SelectorTokenizer::from($text) {
                match token {
                    Ok(_) => {}
                    Err(e) => {
                        assert_eq!(e.to_string(), $err_str);
                        return;
                    }
                }
            }

            unreachable!()
        }
    };
}

malformed!(malformed_01, ">", "unexpected combinator");

malformed!(malformed_02, "+", "unexpected combinator");

malformed!(malformed_03, "> a", "unexpected combinator");

malformed!(malformed_04, "a >", "selector missing");

malformed!(malformed_05, "*a", "unexpected selector");

malformed!(malformed_06, "a*", "unexpected selector");

malformed!(malformed_07, "a > ,", "selector missing");

malformed!(malformed_08, "a > >", "unexpected combinator");

malformed!(malformed_09, "a > {", "selector missing");

malformed!(malformed_10, "a/**/b", "unexpected selector");

malformed!(malformed_11, "a < b", "invalid ident at 1:3");

malformed!(malformed_12, ":lang()", "invalid language pseudo-class");

malformed!(malformed_13, ":lang( )", "invalid language pseudo-class");

malformed!(malformed_14, "::first-child", "invalid ident at 1:2");

malformed!(
    malformed_15,
    "[olor:red",
    "invalid or unsupported attribute selector"
);

malformed!(malformed_16, "", "selector missing");

malformed!(malformed_17, " ", "selector missing");

malformed!(malformed_18, "/**/", "selector missing");

tokenize!(comment_01, "/**/a", SelectorToken::TypeSelector("a"));

tokenize!(comment_02, "/* */a", SelectorToken::TypeSelector("a"));

tokenize!(
    comment_03,
    "/* comment */a",
    SelectorToken::TypeSelector("a")
);

tokenize!(comment_04, "/**/ /**/a", SelectorToken::TypeSelector("a"));

tokenize!(comment_05, "/**/ a /**/", SelectorToken::TypeSelector("a"));

tokenize!(
    comment_06,
    "a /**/ b",
    SelectorToken::TypeSelector("a"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("b")
);

tokenize!(
    comment_08,
    "a /**/b",
    SelectorToken::TypeSelector("a"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("b")
);

tokenize!(
    comment_09,
    "a/**/ b",
    SelectorToken::TypeSelector("a"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("b")
);

tokenize!(
    comment_10,
    "a/**/ /**/b",
    SelectorToken::TypeSelector("a"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("b")
);

tokenize!(
    comment_11,
    "a /**/ /**/ b",
    SelectorToken::TypeSelector("a"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("b")
);

tokenize!(
    comment_12,
    "a /**//**/ b",
    SelectorToken::TypeSelector("a"),
    SelectorToken::DescendantCombinator,
    SelectorToken::TypeSelector("b")
);
