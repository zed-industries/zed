#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XRRenderState",
        typescript_type = "XRRenderState"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrRenderState` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrRenderState;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRRenderState", js_name = "depthNear")]
    #[doc = "Getter for the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/depthNear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_near(this: &XrRenderState) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRRenderState", js_name = "depthFar")]
    #[doc = "Getter for the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/depthFar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_far(this: &XrRenderState) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRRenderState",
        js_name = "inlineVerticalFieldOfView"
    )]
    #[doc = "Getter for the `inlineVerticalFieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/inlineVerticalFieldOfView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn inline_vertical_field_of_view(this: &XrRenderState) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[wasm_bindgen(method, getter, js_class = "XRRenderState", js_name = "baseLayer")]
    #[doc = "Getter for the `baseLayer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRenderState/baseLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderState`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn base_layer(this: &XrRenderState) -> Option<XrWebGlLayer>;
}
