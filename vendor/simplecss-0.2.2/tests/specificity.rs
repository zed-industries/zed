// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Specificity

use simplecss::*;

#[test]
fn spec_01() {
    let selectors = Selector::parse("*").unwrap();
    assert_eq!(selectors.specificity(), [0, 0, 0]);
}

#[test]
fn spec_02() {
    let selectors = Selector::parse("li").unwrap();
    assert_eq!(selectors.specificity(), [0, 0, 1]);
}

#[test]
fn spec_03() {
    let selectors = Selector::parse("ul li").unwrap();
    assert_eq!(selectors.specificity(), [0, 0, 2]);
}

#[test]
fn spec_04() {
    let selectors = Selector::parse("ul ol + li").unwrap();
    assert_eq!(selectors.specificity(), [0, 0, 3]);
}

#[test]
fn spec_05() {
    let selectors = Selector::parse("h1 + *[rel=up]").unwrap();
    assert_eq!(selectors.specificity(), [0, 1, 1]);
}

#[test]
fn spec_06() {
    let selectors = Selector::parse("ul ol li.red").unwrap();
    assert_eq!(selectors.specificity(), [0, 1, 3]);
}

#[test]
fn spec_07() {
    let selectors = Selector::parse("li.red.level").unwrap();
    assert_eq!(selectors.specificity(), [0, 2, 1]);
}

#[test]
fn spec_08() {
    let selectors = Selector::parse("#x34y").unwrap();
    assert_eq!(selectors.specificity(), [1, 0, 0]);
}
