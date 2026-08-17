#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUVertexAttribute")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuVertexAttribute` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuVertexAttribute;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "format")]
    pub fn get_format(this: &GpuVertexAttribute) -> GpuVertexFormat;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "format")]
    pub fn set_format(this: &GpuVertexAttribute, val: GpuVertexFormat);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &GpuVertexAttribute) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &GpuVertexAttribute, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset_f64(this: &GpuVertexAttribute, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `shaderLocation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "shaderLocation")]
    pub fn get_shader_location(this: &GpuVertexAttribute) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `shaderLocation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "shaderLocation")]
    pub fn set_shader_location(this: &GpuVertexAttribute, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl GpuVertexAttribute {
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Construct a new `GpuVertexAttribute`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: GpuVertexFormat, offset: u32, shader_location: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_format(format);
        ret.set_offset(offset);
        ret.set_shader_location(shader_location);
        ret
    }
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Construct a new `GpuVertexAttribute`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_f64(format: GpuVertexFormat, offset: f64, shader_location: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_format(format);
        ret.set_offset_f64(offset);
        ret.set_shader_location(shader_location);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexFormat")]
    #[deprecated = "Use `set_format()` instead."]
    pub fn format(&mut self, val: GpuVertexFormat) -> &mut Self {
        self.set_format(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: u32) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_shader_location()` instead."]
    pub fn shader_location(&mut self, val: u32) -> &mut Self {
        self.set_shader_location(val);
        self
    }
}
