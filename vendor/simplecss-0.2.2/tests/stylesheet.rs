// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Stylesheet

use simplecss::*;

#[test]
fn style_01() {
    let style = StyleSheet::parse("");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_02() {
    let style = StyleSheet::parse("a {}");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_03() {
    let style = StyleSheet::parse("a { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_04() {
    let style = StyleSheet::parse("/**/");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_05() {
    let style = StyleSheet::parse("a { color:red } /**/");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_06() {
    let style = StyleSheet::parse("a, b { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }\nb { color:red; }");
}

#[test]
fn style_07() {
    let style = StyleSheet::parse("a, { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_08() {
    let style = StyleSheet::parse("a,, { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_09() {
    let style = StyleSheet::parse("a,,b { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }\nb { color:red; }");
}

#[test]
fn style_10() {
    let style = StyleSheet::parse(",a { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_11() {
    let style = StyleSheet::parse("@import \"subs.css\";\na { color:red }");
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_12() {
    let style = StyleSheet::parse(
        "\
@media screen {
    p:before { content: 'Hello'; }
}
a { color:red }",
    );
    assert_eq!(style.to_string(), "a { color:red; }");
}

#[test]
fn style_13() {
    let style = StyleSheet::parse("a > { color:red }");
    assert_eq!(style.to_string(), "");
}

#[test]
fn style_14() {
    let style = StyleSheet::parse("p { color:green; color }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_15() {
    let style = StyleSheet::parse("p { color; color:green }");
    assert_eq!(style.to_string(), ""); // TODO: should be 'p { color:green; }'
}

#[test]
fn style_16() {
    let style = StyleSheet::parse("p { color:green; color: }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_17() {
    let style = StyleSheet::parse("p { color:green; color:; color:red; }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_18() {
    let style = StyleSheet::parse("p { color:green; color{;color:maroon} }");
    assert_eq!(style.to_string(), "p { color:green; }");
}

#[test]
fn style_19() {
    let style = StyleSheet::parse("p { color{;color:maroon} color:green; }");
    assert_eq!(style.to_string(), ""); // TODO: should be 'p { color:green; }'
}

#[test]
fn style_20() {
    let style = StyleSheet::parse(
        "\
        h1 { color: green }
        h2 & h3 { color: red }
        h4 { color: black }
    ",
    );
    assert_eq!(
        style.to_string(),
        "h1 { color:green; }\nh4 { color:black; }"
    );
}

#[test]
fn style_21() {
    let style = StyleSheet::parse(":le>*");
    assert_eq!(style.to_string(), "");
}
