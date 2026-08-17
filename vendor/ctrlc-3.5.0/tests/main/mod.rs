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

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn test_set_handler() {
    let flag = Arc::new(AtomicBool::new(false));
    let flag_handler = Arc::clone(&flag);
    ctrlc::set_handler(move || {
        flag_handler.store(true, Ordering::SeqCst);
    })
    .unwrap();

    unsafe {
        platform::raise_ctrl_c();
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(flag.load(Ordering::SeqCst));

    match ctrlc::set_handler(|| {}) {
        Err(ctrlc::Error::MultipleHandlers) => {}
        ret => panic!("{:?}", ret),
    }
}

fn tests() {
    run_tests!(test_set_handler);
}

fn main() {
    run_harness(tests);
}
