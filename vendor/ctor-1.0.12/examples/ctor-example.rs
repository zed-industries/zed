//! Examples from the `README`.
// Ensure we don't blow a low recursion limit.
#![recursion_limit = "62"]

use ctor::ctor;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

static INITED: AtomicBool = AtomicBool::new(false);

#[ctor(unsafe)]
fn foo() {
    INITED.store(true, Ordering::SeqCst);
}

#[ctor(unsafe)]
/// This is an immutable static, evaluated at init time.
static STATIC_CTOR: HashMap<u32, &'static str> = {
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");
    m
};

fn main() {
    assert!(INITED.load(Ordering::SeqCst));
    assert_eq!(STATIC_CTOR.get(&1), Some(&"bar"));
}
