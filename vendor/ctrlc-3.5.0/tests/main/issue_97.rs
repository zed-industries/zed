// Copyright (c) 2023 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[macro_use]
mod harness;
use harness::{platform, run_harness};

mod test_signal_hook;
use test_signal_hook::run_signal_hook;

fn expect_multiple_handlers() {
    #[cfg(not(windows))]
    match ctrlc::try_set_handler(|| {}) {
        Err(ctrlc::Error::MultipleHandlers) => {}
        _ => panic!("Expected Error::MultipleHandlers"),
    }
}

fn tests() {
    run_tests!(run_signal_hook);
    run_tests!(expect_multiple_handlers);
}

fn main() {
    run_harness(tests);
}
