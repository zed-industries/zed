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
        js_name = "GPUInternalError",
        typescript_type = "GPUInternalError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuInternalError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUInternalError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuInternalError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuInternalError;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "GPUInternalError")]
    #[doc = "The `new GpuInternalError(..)` constructor, creating a new instance of `GpuInternalError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUInternalError/GPUInternalError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuInternalError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(message: &str) -> Result<GpuInternalError, JsValue>;
}
