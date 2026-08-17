#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUPipelineLayoutDescriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPipelineLayoutDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuPipelineLayoutDescriptor;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &GpuPipelineLayoutDescriptor) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &GpuPipelineLayoutDescriptor, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroupLayout")]
    #[doc = "Get the `bindGroupLayouts` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayout`, `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bindGroupLayouts")]
    pub fn get_bind_group_layouts(
        this: &GpuPipelineLayoutDescriptor,
    ) -> ::js_sys::Array<::js_sys::JsOption<GpuBindGroupLayout>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroupLayout")]
    #[doc = "Change the `bindGroupLayouts` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayout`, `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bindGroupLayouts")]
    pub fn set_bind_group_layouts(
        this: &GpuPipelineLayoutDescriptor,
        val: &[::js_sys::JsOption<GpuBindGroupLayout>],
    );
}
#[cfg(web_sys_unstable_apis)]
impl GpuPipelineLayoutDescriptor {
    #[cfg(feature = "GpuBindGroupLayout")]
    #[doc = "Construct a new `GpuPipelineLayoutDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayout`, `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(bind_group_layouts: &[::js_sys::JsOption<GpuBindGroupLayout>]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_bind_group_layouts(bind_group_layouts);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroupLayout")]
    #[deprecated = "Use `set_bind_group_layouts()` instead."]
    pub fn bind_group_layouts(
        &mut self,
        val: &[::js_sys::JsOption<GpuBindGroupLayout>],
    ) -> &mut Self {
        self.set_bind_group_layouts(val);
        self
    }
}
