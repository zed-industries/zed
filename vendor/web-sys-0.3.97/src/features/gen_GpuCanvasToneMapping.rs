#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUCanvasToneMapping")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCanvasToneMapping` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasToneMapping`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCanvasToneMapping;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasToneMappingMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasToneMapping`, `GpuCanvasToneMappingMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &GpuCanvasToneMapping) -> Option<GpuCanvasToneMappingMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasToneMappingMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasToneMapping`, `GpuCanvasToneMappingMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &GpuCanvasToneMapping, val: GpuCanvasToneMappingMode);
}
#[cfg(web_sys_unstable_apis)]
impl GpuCanvasToneMapping {
    #[doc = "Construct a new `GpuCanvasToneMapping`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasToneMapping`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasToneMappingMode")]
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: GpuCanvasToneMappingMode) -> &mut Self {
        self.set_mode(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuCanvasToneMapping {
    fn default() -> Self {
        Self::new()
    }
}
