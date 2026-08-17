#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUPrimitiveState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPrimitiveState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuPrimitiveState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCullMode")]
    #[doc = "Get the `cullMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCullMode`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "cullMode")]
    pub fn get_cull_mode(this: &GpuPrimitiveState) -> Option<GpuCullMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCullMode")]
    #[doc = "Change the `cullMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCullMode`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "cullMode")]
    pub fn set_cull_mode(this: &GpuPrimitiveState, val: GpuCullMode);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFrontFace")]
    #[doc = "Get the `frontFace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFrontFace`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "frontFace")]
    pub fn get_front_face(this: &GpuPrimitiveState) -> Option<GpuFrontFace>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFrontFace")]
    #[doc = "Change the `frontFace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFrontFace`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "frontFace")]
    pub fn set_front_face(this: &GpuPrimitiveState, val: GpuFrontFace);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuIndexFormat")]
    #[doc = "Get the `stripIndexFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuIndexFormat`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "stripIndexFormat")]
    pub fn get_strip_index_format(this: &GpuPrimitiveState) -> Option<GpuIndexFormat>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuIndexFormat")]
    #[doc = "Change the `stripIndexFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuIndexFormat`, `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "stripIndexFormat")]
    pub fn set_strip_index_format(this: &GpuPrimitiveState, val: GpuIndexFormat);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[doc = "Get the `topology` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuPrimitiveTopology`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "topology")]
    pub fn get_topology(this: &GpuPrimitiveState) -> Option<GpuPrimitiveTopology>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[doc = "Change the `topology` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuPrimitiveTopology`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "topology")]
    pub fn set_topology(this: &GpuPrimitiveState, val: GpuPrimitiveTopology);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unclippedDepth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unclippedDepth")]
    pub fn get_unclipped_depth(this: &GpuPrimitiveState) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unclippedDepth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unclippedDepth")]
    pub fn set_unclipped_depth(this: &GpuPrimitiveState, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl GpuPrimitiveState {
    #[doc = "Construct a new `GpuPrimitiveState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCullMode")]
    #[deprecated = "Use `set_cull_mode()` instead."]
    pub fn cull_mode(&mut self, val: GpuCullMode) -> &mut Self {
        self.set_cull_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFrontFace")]
    #[deprecated = "Use `set_front_face()` instead."]
    pub fn front_face(&mut self, val: GpuFrontFace) -> &mut Self {
        self.set_front_face(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuIndexFormat")]
    #[deprecated = "Use `set_strip_index_format()` instead."]
    pub fn strip_index_format(&mut self, val: GpuIndexFormat) -> &mut Self {
        self.set_strip_index_format(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveTopology")]
    #[deprecated = "Use `set_topology()` instead."]
    pub fn topology(&mut self, val: GpuPrimitiveTopology) -> &mut Self {
        self.set_topology(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unclipped_depth()` instead."]
    pub fn unclipped_depth(&mut self, val: bool) -> &mut Self {
        self.set_unclipped_depth(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuPrimitiveState {
    fn default() -> Self {
        Self::new()
    }
}
