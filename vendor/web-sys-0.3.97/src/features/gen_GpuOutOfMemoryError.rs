#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "GpuError",
        extends = "::js_sys::Object",
        js_name = "GPUOutOfMemoryError",
        typescript_type = "GPUOutOfMemoryError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuOutOfMemoryError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUOutOfMemoryError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuOutOfMemoryError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuOutOfMemoryError;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "GPUOutOfMemoryError")]
    #[doc = "The `new GpuOutOfMemoryError(..)` constructor, creating a new instance of `GpuOutOfMemoryError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUOutOfMemoryError/GPUOutOfMemoryError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuOutOfMemoryError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(message: &str) -> Result<GpuOutOfMemoryError, JsValue>;
}
