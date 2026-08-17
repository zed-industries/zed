#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "DomException",
        extends = "::js_sys::Object",
        js_name = "GPUPipelineError",
        typescript_type = "GPUPipelineError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPipelineError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuPipelineError;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[wasm_bindgen(method, getter, js_class = "GPUPipelineError", js_name = "reason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reason(this: &GpuPipelineError) -> GpuPipelineErrorReason;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorInit")]
    #[wasm_bindgen(catch, constructor, js_class = "GPUPipelineError")]
    #[doc = "The `new GpuPipelineError(..)` constructor, creating a new instance of `GpuPipelineError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError/GPUPipelineError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`, `GpuPipelineErrorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(options: &GpuPipelineErrorInit) -> Result<GpuPipelineError, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorInit")]
    #[wasm_bindgen(catch, constructor, js_class = "GPUPipelineError")]
    #[doc = "The `new GpuPipelineError(..)` constructor, creating a new instance of `GpuPipelineError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError/GPUPipelineError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`, `GpuPipelineErrorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_message(
        message: &str,
        options: &GpuPipelineErrorInit,
    ) -> Result<GpuPipelineError, JsValue>;
}
