// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use alloc::vec;

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "../tests/reporting.pest"]
struct ReportingParser;

#[test]
fn choices() {
    fails_with! {
        parser: ReportingParser,
        input: "x",
        rule: Rule::choices,
        positives: vec![Rule::a, Rule::b, Rule::c],
        negatives: vec![],
        pos: 0
    };
}

#[test]
fn choices_no_progress() {
    fails_with! {
        parser: ReportingParser,
        input: "x",
        rule: Rule::choices_no_progress,
        positives: vec![Rule::choices_no_progress],
        negatives: vec![],
        pos: 0
    };
}

#[test]
fn choices_a_progress() {
    fails_with! {
        parser: ReportingParser,
        input: "a",
        rule: Rule::choices_a_progress,
        positives: vec![Rule::a],
        negatives: vec![],
        pos: 1
    };
}

#[test]
fn choices_b_progress() {
    fails_with! {
        parser: ReportingParser,
        input: "b",
        rule: Rule::choices_b_progress,
        positives: vec![Rule::b],
        negatives: vec![],
        pos: 1
    };
}

#[test]
fn nested() {
    fails_with! {
        parser: ReportingParser,
        input: "x",
        rule: Rule::level1,
        positives: vec![Rule::a, Rule::b, Rule::c],
        negatives: vec![],
        pos: 0
    };
}

#[test]
fn negative() {
    fails_with! {
        parser: ReportingParser,
        input: "x",
        rule: Rule::negative,
        positives: vec![],
        negatives: vec![Rule::d],
        pos: 0
    };
}

#[test]
fn negative_match() {
    fails_with! {
        parser: ReportingParser,
        input: "x",
        rule: Rule::negative_match,
        positives: vec![Rule::b],
        negatives: vec![],
        pos: 0
    };
}

#[test]
fn mixed() {
    fails_with! {
        parser: ReportingParser,
        input: "x",
        rule: Rule::mixed,
        positives: vec![Rule::a],
        negatives: vec![Rule::d],
        pos: 0
    };
}

#[test]
fn mixed_progress() {
    fails_with! {
        parser: ReportingParser,
        input: "b",
        rule: Rule::mixed_progress,
        positives: vec![Rule::a],
        negatives: vec![],
        pos: 1
    };
}
