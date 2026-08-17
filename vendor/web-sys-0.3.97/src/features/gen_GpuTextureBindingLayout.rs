#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUTextureBindingLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTextureBindingLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuTextureBindingLayout;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `multisampled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "multisampled")]
    pub fn get_multisampled(this: &GpuTextureBindingLayout) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `multisampled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "multisampled")]
    pub fn set_multisampled(this: &GpuTextureBindingLayout, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureSampleType")]
    #[doc = "Get the `sampleType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureSampleType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "sampleType")]
    pub fn get_sample_type(this: &GpuTextureBindingLayout) -> Option<GpuTextureSampleType>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureSampleType")]
    #[doc = "Change the `sampleType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureSampleType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "sampleType")]
    pub fn set_sample_type(this: &GpuTextureBindingLayout, val: GpuTextureSampleType);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Get the `viewDimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "viewDimension")]
    pub fn get_view_dimension(this: &GpuTextureBindingLayout) -> Option<GpuTextureViewDimension>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Change the `viewDimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "viewDimension")]
    pub fn set_view_dimension(this: &GpuTextureBindingLayout, val: GpuTextureViewDimension);
}
#[cfg(web_sys_unstable_apis)]
impl GpuTextureBindingLayout {
    #[doc = "Construct a new `GpuTextureBindingLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_multisampled()` instead."]
    pub fn multisampled(&mut self, val: bool) -> &mut Self {
        self.set_multisampled(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureSampleType")]
    #[deprecated = "Use `set_sample_type()` instead."]
    pub fn sample_type(&mut self, val: GpuTextureSampleType) -> &mut Self {
        self.set_sample_type(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[deprecated = "Use `set_view_dimension()` instead."]
    pub fn view_dimension(&mut self, val: GpuTextureViewDimension) -> &mut Self {
        self.set_view_dimension(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuTextureBindingLayout {
    fn default() -> Self {
        Self::new()
    }
}
