#![warn(rust_2018_idioms)]
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
#[should_panic]
fn instant_now_panics() {
    let _ = tokio::time::Instant::now();
}

#[cfg(all(feature = "rt", not(feature = "time")))]
#[wasm_bindgen_test]
fn runtime_without_time_does_not_panic() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(async {});
}

#[cfg(all(feature = "rt", feature = "time"))]
#[wasm_bindgen_test]
#[should_panic] // should remove this once time is supported
fn runtime_with_time_does_not_panic() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(async {});
}

#[cfg(all(feature = "rt", feature = "time"))]
#[wasm_bindgen_test]
#[should_panic]
fn sleep_panics_on_unknown_unknown() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(async { tokio::time::sleep(core::time::Duration::from_millis(1)).await });
}
