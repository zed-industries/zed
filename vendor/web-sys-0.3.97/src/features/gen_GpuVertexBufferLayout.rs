#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUVertexBufferLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuVertexBufferLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuVertexBufferLayout;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `arrayStride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "arrayStride")]
    pub fn get_array_stride(this: &GpuVertexBufferLayout) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `arrayStride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "arrayStride")]
    pub fn set_array_stride(this: &GpuVertexBufferLayout, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `arrayStride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "arrayStride")]
    pub fn set_array_stride_f64(this: &GpuVertexBufferLayout, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexAttribute")]
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "attributes")]
    pub fn get_attributes(this: &GpuVertexBufferLayout) -> ::js_sys::Array<GpuVertexAttribute>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexAttribute")]
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "attributes")]
    pub fn set_attributes(this: &GpuVertexBufferLayout, val: &[GpuVertexAttribute]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexStepMode")]
    #[doc = "Get the `stepMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`, `GpuVertexStepMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "stepMode")]
    pub fn get_step_mode(this: &GpuVertexBufferLayout) -> Option<GpuVertexStepMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexStepMode")]
    #[doc = "Change the `stepMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`, `GpuVertexStepMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "stepMode")]
    pub fn set_step_mode(this: &GpuVertexBufferLayout, val: GpuVertexStepMode);
}
#[cfg(web_sys_unstable_apis)]
impl GpuVertexBufferLayout {
    #[cfg(feature = "GpuVertexAttribute")]
    #[doc = "Construct a new `GpuVertexBufferLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(array_stride: u32, attributes: &[GpuVertexAttribute]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_array_stride(array_stride);
        ret.set_attributes(attributes);
        ret
    }
    #[cfg(feature = "GpuVertexAttribute")]
    #[doc = "Construct a new `GpuVertexBufferLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_f64(array_stride: f64, attributes: &[GpuVertexAttribute]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_array_stride_f64(array_stride);
        ret.set_attributes(attributes);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_array_stride()` instead."]
    pub fn array_stride(&mut self, val: u32) -> &mut Self {
        self.set_array_stride(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexAttribute")]
    #[deprecated = "Use `set_attributes()` instead."]
    pub fn attributes(&mut self, val: &[GpuVertexAttribute]) -> &mut Self {
        self.set_attributes(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexStepMode")]
    #[deprecated = "Use `set_step_mode()` instead."]
    pub fn step_mode(&mut self, val: GpuVertexStepMode) -> &mut Self {
        self.set_step_mode(val);
        self
    }
}
