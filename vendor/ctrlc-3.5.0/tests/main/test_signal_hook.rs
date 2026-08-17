// Copyright (c) 2023 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub fn run_signal_hook() {
    let hook = Arc::new(AtomicBool::new(false));

    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&hook)).unwrap();

    unsafe {
        super::platform::raise_ctrl_c();
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(hook.load(Ordering::SeqCst));
}
