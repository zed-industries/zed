#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUSamplerBindingLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSamplerBindingLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuSamplerBindingLayout;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSamplerBindingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerBindingLayout`, `GpuSamplerBindingType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &GpuSamplerBindingLayout) -> Option<GpuSamplerBindingType>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSamplerBindingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerBindingLayout`, `GpuSamplerBindingType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &GpuSamplerBindingLayout, val: GpuSamplerBindingType);
}
#[cfg(web_sys_unstable_apis)]
impl GpuSamplerBindingLayout {
    #[doc = "Construct a new `GpuSamplerBindingLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSamplerBindingType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: GpuSamplerBindingType) -> &mut Self {
        self.set_type(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuSamplerBindingLayout {
    fn default() -> Self {
        Self::new()
    }
}
