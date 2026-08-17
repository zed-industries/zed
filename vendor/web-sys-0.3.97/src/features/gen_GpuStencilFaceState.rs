#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUStencilFaceState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuStencilFaceState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuStencilFaceState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Get the `compare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "compare")]
    pub fn get_compare(this: &GpuStencilFaceState) -> Option<GpuCompareFunction>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `compare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "compare")]
    pub fn set_compare(this: &GpuStencilFaceState, val: GpuCompareFunction);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Get the `depthFailOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthFailOp")]
    pub fn get_depth_fail_op(this: &GpuStencilFaceState) -> Option<GpuStencilOperation>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `depthFailOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthFailOp")]
    pub fn set_depth_fail_op(this: &GpuStencilFaceState, val: GpuStencilOperation);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Get the `failOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "failOp")]
    pub fn get_fail_op(this: &GpuStencilFaceState) -> Option<GpuStencilOperation>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `failOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "failOp")]
    pub fn set_fail_op(this: &GpuStencilFaceState, val: GpuStencilOperation);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Get the `passOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "passOp")]
    pub fn get_pass_op(this: &GpuStencilFaceState) -> Option<GpuStencilOperation>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `passOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`, `GpuStencilOperation`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "passOp")]
    pub fn set_pass_op(this: &GpuStencilFaceState, val: GpuStencilOperation);
}
#[cfg(web_sys_unstable_apis)]
impl GpuStencilFaceState {
    #[doc = "Construct a new `GpuStencilFaceState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[deprecated = "Use `set_compare()` instead."]
    pub fn compare(&mut self, val: GpuCompareFunction) -> &mut Self {
        self.set_compare(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[deprecated = "Use `set_depth_fail_op()` instead."]
    pub fn depth_fail_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        self.set_depth_fail_op(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[deprecated = "Use `set_fail_op()` instead."]
    pub fn fail_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        self.set_fail_op(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilOperation")]
    #[deprecated = "Use `set_pass_op()` instead."]
    pub fn pass_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        self.set_pass_op(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuStencilFaceState {
    fn default() -> Self {
        Self::new()
    }
}
