#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUCompilationInfo",
        typescript_type = "GPUCompilationInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCompilationInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCompilationInfo;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompilationMessage")]
    #[wasm_bindgen(method, getter, js_class = "GPUCompilationInfo", js_name = "messages")]
    #[doc = "Getter for the `messages` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationInfo/messages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationInfo`, `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn messages(this: &GpuCompilationInfo) -> ::js_sys::Array<GpuCompilationMessage>;
}
