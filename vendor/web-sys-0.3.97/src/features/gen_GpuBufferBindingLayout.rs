#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUBufferBindingLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBufferBindingLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBufferBindingLayout;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hasDynamicOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "hasDynamicOffset")]
    pub fn get_has_dynamic_offset(this: &GpuBufferBindingLayout) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hasDynamicOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "hasDynamicOffset")]
    pub fn set_has_dynamic_offset(this: &GpuBufferBindingLayout, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `minBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "minBindingSize")]
    pub fn get_min_binding_size(this: &GpuBufferBindingLayout) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `minBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "minBindingSize")]
    pub fn set_min_binding_size(this: &GpuBufferBindingLayout, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `minBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "minBindingSize")]
    pub fn set_min_binding_size_f64(this: &GpuBufferBindingLayout, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`, `GpuBufferBindingType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &GpuBufferBindingLayout) -> Option<GpuBufferBindingType>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`, `GpuBufferBindingType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &GpuBufferBindingLayout, val: GpuBufferBindingType);
}
#[cfg(web_sys_unstable_apis)]
impl GpuBufferBindingLayout {
    #[doc = "Construct a new `GpuBufferBindingLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_has_dynamic_offset()` instead."]
    pub fn has_dynamic_offset(&mut self, val: bool) -> &mut Self {
        self.set_has_dynamic_offset(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_min_binding_size()` instead."]
    pub fn min_binding_size(&mut self, val: u32) -> &mut Self {
        self.set_min_binding_size(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: GpuBufferBindingType) -> &mut Self {
        self.set_type(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuBufferBindingLayout {
    fn default() -> Self {
        Self::new()
    }
}
