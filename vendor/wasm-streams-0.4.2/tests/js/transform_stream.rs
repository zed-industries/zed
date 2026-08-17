use wasm_bindgen::prelude::*;

use wasm_streams::transform::*;

#[wasm_bindgen(module = "/tests/js/transform_stream.js")]
extern "C" {
    pub fn new_noop_transform_stream() -> sys::TransformStream;
    pub fn new_uppercase_transform_stream() -> sys::TransformStream;
}
