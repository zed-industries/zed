extern crate wasm_bindgen_test;

use instant::{Instant, SystemTime};
use std::time::Duration;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
// run these tests using: wasm-pack test --chrome --headless -- --features wasm-bindgen

#[wasm_bindgen_test]
fn test_instant_now() {
    let now = Instant::now();
    #[cfg(feature = "inaccurate")]
    while now.elapsed().as_millis() == 0 {}
    #[cfg(not(feature = "inaccurate"))]
    assert!(now.elapsed().as_nanos() > 0);
}

#[wasm_bindgen_test]
fn test_duration() {
    let now = Instant::now();
    let one_sec = Duration::from_secs(1);
    assert!(now.elapsed() < one_sec);
}

// Duration::new will overflow when you have u64::MAX seconds and one billion nanoseconds.
// <https://doc.rust-lang.org/std/time/struct.Duration.html#method.new>
const ONE_BILLION: u32 = 1_000_000_000;

#[wasm_bindgen_test]
fn test_checked_add() {
    let now = Instant::now();

    assert!(now.checked_add(Duration::from_millis(1)).is_some());
    assert_eq!(
        None,
        now.checked_add(Duration::new(u64::MAX, ONE_BILLION - 1))
    );
}

#[wasm_bindgen_test]
fn test_checked_sub() {
    let now = Instant::now();

    assert!(now.checked_sub(Duration::from_millis(1)).is_some());
    assert!(now
        .checked_sub(Duration::new(u64::MAX, ONE_BILLION - 1))
        .is_none());
}

#[wasm_bindgen_test]
fn test_system_time() {
    assert!(SystemTime::UNIX_EPOCH
        .duration_since(SystemTime::now())
        .is_err());
}

