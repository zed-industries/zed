//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Shows how to pick values from a strategy.
//
// This is *not* how proptest is normally used; it is simply used to play
// around with value generation.

use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner::TestRunner;

fn main() {
    let mut runner = TestRunner::default();
    let int_val = (0..100i32).new_tree(&mut runner).unwrap();
    let str_val = "[a-z]{1,4}\\p{Cyrillic}{1,4}\\p{Greek}{1,4}"
        .new_tree(&mut runner)
        .unwrap();
    println!(
        "int_val = {}, str_val = {}",
        int_val.current(),
        str_val.current()
    );
}
