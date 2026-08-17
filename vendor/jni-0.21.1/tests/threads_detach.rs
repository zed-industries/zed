#![cfg(feature = "invocation")]

use std::thread::spawn;

mod util;
use util::{attach_current_thread_permanently, call_java_abs, jvm};

#[test]
fn thread_detaches_when_finished() {
    let thread = spawn(|| {
        let mut env = attach_current_thread_permanently();
        let val = call_java_abs(&mut env, -2);
        assert_eq!(val, 2);
        assert_eq!(jvm().threads_attached(), 1);
    });

    thread.join().unwrap();
    assert_eq!(jvm().threads_attached(), 0);
}
