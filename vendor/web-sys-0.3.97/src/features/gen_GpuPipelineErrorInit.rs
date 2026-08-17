#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUPipelineErrorInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPipelineErrorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuPipelineErrorInit;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &GpuPipelineErrorInit) -> GpuPipelineErrorReason;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &GpuPipelineErrorInit, val: GpuPipelineErrorReason);
}
#[cfg(web_sys_unstable_apis)]
impl GpuPipelineErrorInit {
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Construct a new `GpuPipelineErrorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(reason: GpuPipelineErrorReason) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_reason(reason);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: GpuPipelineErrorReason) -> &mut Self {
        self.set_reason(val);
        self
    }
}
