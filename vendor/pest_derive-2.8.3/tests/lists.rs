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
use alloc::{format, vec::Vec};

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "../tests/lists.pest"]
struct ListsParser;

#[test]
fn item() {
    parses_to! {
        parser: ListsParser,
        input: "- a",
        rule: Rule::lists,
        tokens: [
            item(2, 3)
        ]
    };
}

#[test]
fn items() {
    parses_to! {
        parser: ListsParser,
        input: "- a\n- b",
        rule: Rule::lists,
        tokens: [
            item(2, 3),
            item(6, 7)
        ]
    };
}

#[test]
fn children() {
    parses_to! {
        parser: ListsParser,
        input: "  - b",
        rule: Rule::children,
        tokens: [
            children(0, 5, [
                item(4, 5)
            ])
        ]
    };
}

#[test]
fn nested_item() {
    parses_to! {
        parser: ListsParser,
        input: "- a\n  - b",
        rule: Rule::lists,
        tokens: [
            item(2, 3),
            children(4, 9, [
                item(8, 9)
            ])
        ]
    };
}

#[test]
fn nested_items() {
    parses_to! {
        parser: ListsParser,
        input: "- a\n  - b\n  - c",
        rule: Rule::lists,
        tokens: [
            item(2, 3),
            children(4, 15, [
                item(8, 9),
                item(14, 15)
            ])
        ]
    };
}

#[test]
fn nested_two_levels() {
    parses_to! {
        parser: ListsParser,
        input: "- a\n  - b\n    - c",
        rule: Rule::lists,
        tokens: [
            item(2, 3),
            children(4, 17, [
                item(8, 9),
                children(10, 17, [
                    item(16, 17)
                ])
            ])
        ]
    };
}

#[test]
fn nested_then_not() {
    parses_to! {
        parser: ListsParser,
        input: "- a\n  - b\n- c",
        rule: Rule::lists,
        tokens: [
            item(2, 3),
            children(4, 9, [
                item(8, 9)
            ]),
            item(12, 13)
        ]
    };
}
