#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUBlendState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBlendState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBlendState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &GpuBlendState) -> GpuBlendComponent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "alpha")]
    pub fn set_alpha(this: &GpuBlendState, val: &GpuBlendComponent);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Get the `color` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &GpuBlendState) -> GpuBlendComponent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Change the `color` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &GpuBlendState, val: &GpuBlendComponent);
}
#[cfg(web_sys_unstable_apis)]
impl GpuBlendState {
    #[cfg(feature = "GpuBlendComponent")]
    #[doc = "Construct a new `GpuBlendState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(alpha: &GpuBlendComponent, color: &GpuBlendComponent) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_alpha(alpha);
        ret.set_color(color);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[deprecated = "Use `set_alpha()` instead."]
    pub fn alpha(&mut self, val: &GpuBlendComponent) -> &mut Self {
        self.set_alpha(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendComponent")]
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: &GpuBlendComponent) -> &mut Self {
        self.set_color(val);
        self
    }
}
