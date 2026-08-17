// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate compiletest_rs as ct;

use std::env;

fn run_mode(src: &'static str, mode: &'static str) {
    let mut config = ct::Config::default();

    config.mode = mode.parse().expect("invalid mode");
    config.target_rustcflags =
        Some("-L ../target/debug/deps --extern proptest --edition=2021".to_owned());
    if let Ok(name) = env::var("TESTNAME") {
        config.filters = vec![name];
    }
    config.src_base = format!("tests/{}", src).into();
    config.clean_rmeta();

    ct::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail", "compile-fail");
}
