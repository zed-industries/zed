#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUBlendComponent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBlendComponent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBlendComponent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Get the `dstFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "dstFactor")]
    pub fn get_dst_factor(this: &GpuBlendComponent) -> Option<GpuBlendFactor>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Change the `dstFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dstFactor")]
    pub fn set_dst_factor(this: &GpuBlendComponent, val: GpuBlendFactor);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendOperation")]
    #[doc = "Get the `operation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "operation")]
    pub fn get_operation(this: &GpuBlendComponent) -> Option<GpuBlendOperation>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendOperation")]
    #[doc = "Change the `operation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "operation")]
    pub fn set_operation(this: &GpuBlendComponent, val: GpuBlendOperation);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Get the `srcFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "srcFactor")]
    pub fn get_src_factor(this: &GpuBlendComponent) -> Option<GpuBlendFactor>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[doc = "Change the `srcFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendFactor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "srcFactor")]
    pub fn set_src_factor(this: &GpuBlendComponent, val: GpuBlendFactor);
}
#[cfg(web_sys_unstable_apis)]
impl GpuBlendComponent {
    #[doc = "Construct a new `GpuBlendComponent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[deprecated = "Use `set_dst_factor()` instead."]
    pub fn dst_factor(&mut self, val: GpuBlendFactor) -> &mut Self {
        self.set_dst_factor(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendOperation")]
    #[deprecated = "Use `set_operation()` instead."]
    pub fn operation(&mut self, val: GpuBlendOperation) -> &mut Self {
        self.set_operation(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendFactor")]
    #[deprecated = "Use `set_src_factor()` instead."]
    pub fn src_factor(&mut self, val: GpuBlendFactor) -> &mut Self {
        self.set_src_factor(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuBlendComponent {
    fn default() -> Self {
        Self::new()
    }
}
