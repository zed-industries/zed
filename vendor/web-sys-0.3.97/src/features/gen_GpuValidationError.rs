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
        js_name = "GPUValidationError",
        typescript_type = "GPUValidationError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuValidationError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUValidationError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuValidationError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuValidationError;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "GPUValidationError")]
    #[doc = "The `new GpuValidationError(..)` constructor, creating a new instance of `GpuValidationError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUValidationError/GPUValidationError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuValidationError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(message: &str) -> Result<GpuValidationError, JsValue>;
}
