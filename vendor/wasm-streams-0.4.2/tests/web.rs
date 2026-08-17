#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod js;
mod tests;
mod util;
