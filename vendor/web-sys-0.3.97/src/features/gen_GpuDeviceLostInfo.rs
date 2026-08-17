#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUDeviceLostInfo",
        typescript_type = "GPUDeviceLostInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuDeviceLostInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDeviceLostInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuDeviceLostInfo;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDeviceLostReason")]
    #[wasm_bindgen(method, getter, js_class = "GPUDeviceLostInfo", js_name = "reason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDeviceLostInfo`, `GpuDeviceLostReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reason(this: &GpuDeviceLostInfo) -> GpuDeviceLostReason;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUDeviceLostInfo", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDeviceLostInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn message(this: &GpuDeviceLostInfo) -> ::alloc::string::String;
}
