#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "XRRenderStateInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrRenderStateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrRenderStateInit;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[doc = "Get the `baseLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "baseLayer")]
    pub fn get_base_layer(this: &XrRenderStateInit) -> Option<XrWebGlLayer>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[doc = "Change the `baseLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "baseLayer")]
    pub fn set_base_layer(this: &XrRenderStateInit, val: Option<&XrWebGlLayer>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthFar")]
    pub fn get_depth_far(this: &XrRenderStateInit) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthFar")]
    pub fn set_depth_far(this: &XrRenderStateInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthNear")]
    pub fn get_depth_near(this: &XrRenderStateInit) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthNear")]
    pub fn set_depth_near(this: &XrRenderStateInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `inlineVerticalFieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "inlineVerticalFieldOfView")]
    pub fn get_inline_vertical_field_of_view(this: &XrRenderStateInit) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `inlineVerticalFieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "inlineVerticalFieldOfView")]
    pub fn set_inline_vertical_field_of_view(this: &XrRenderStateInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrLayer")]
    #[doc = "Get the `layers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrLayer`, `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "layers")]
    pub fn get_layers(this: &XrRenderStateInit) -> Option<::js_sys::Array<XrLayer>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrLayer")]
    #[doc = "Change the `layers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrLayer`, `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "layers")]
    pub fn set_layers(this: &XrRenderStateInit, val: Option<&[XrLayer]>);
}
#[cfg(web_sys_unstable_apis)]
impl XrRenderStateInit {
    #[doc = "Construct a new `XrRenderStateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[deprecated = "Use `set_base_layer()` instead."]
    pub fn base_layer(&mut self, val: Option<&XrWebGlLayer>) -> &mut Self {
        self.set_base_layer(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_far()` instead."]
    pub fn depth_far(&mut self, val: f64) -> &mut Self {
        self.set_depth_far(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_near()` instead."]
    pub fn depth_near(&mut self, val: f64) -> &mut Self {
        self.set_depth_near(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_inline_vertical_field_of_view()` instead."]
    pub fn inline_vertical_field_of_view(&mut self, val: f64) -> &mut Self {
        self.set_inline_vertical_field_of_view(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrLayer")]
    #[deprecated = "Use `set_layers()` instead."]
    pub fn layers(&mut self, val: Option<&[XrLayer]>) -> &mut Self {
        self.set_layers(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrRenderStateInit {
    fn default() -> Self {
        Self::new()
    }
}
