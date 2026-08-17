#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUCompilationMessage",
        typescript_type = "GPUCompilationMessage"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCompilationMessage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCompilationMessage;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUCompilationMessage",
        js_name = "message"
    )]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn message(this: &GpuCompilationMessage) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompilationMessageType")]
    #[wasm_bindgen(method, getter, js_class = "GPUCompilationMessage", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`, `GpuCompilationMessageType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(this: &GpuCompilationMessage) -> GpuCompilationMessageType;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUCompilationMessage",
        js_name = "lineNum"
    )]
    #[doc = "Getter for the `lineNum` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/lineNum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn line_num(this: &GpuCompilationMessage) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUCompilationMessage",
        js_name = "linePos"
    )]
    #[doc = "Getter for the `linePos` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/linePos)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn line_pos(this: &GpuCompilationMessage) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUCompilationMessage", js_name = "offset")]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn offset(this: &GpuCompilationMessage) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUCompilationMessage", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCompilationMessage/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompilationMessage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn length(this: &GpuCompilationMessage) -> f64;
}
