#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XRView",
        typescript_type = "XRView"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrView` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrView;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrEye")]
    #[wasm_bindgen(method, getter, js_class = "XRView", js_name = "eye")]
    #[doc = "Getter for the `eye` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRView/eye)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrEye`, `XrView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn eye(this: &XrView) -> XrEye;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRView", js_name = "projectionMatrix")]
    #[doc = "Getter for the `projectionMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRView/projectionMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn projection_matrix(this: &XrView) -> ::alloc::vec::Vec<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrRigidTransform")]
    #[wasm_bindgen(method, getter, js_class = "XRView", js_name = "transform")]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRView/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRigidTransform`, `XrView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transform(this: &XrView) -> XrRigidTransform;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRView",
        js_name = "recommendedViewportScale"
    )]
    #[doc = "Getter for the `recommendedViewportScale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRView/recommendedViewportScale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn recommended_viewport_scale(this: &XrView) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "XRView", js_name = "requestViewportScale")]
    #[doc = "The `requestViewportScale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRView/requestViewportScale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_viewport_scale(this: &XrView, scale: Option<f64>);
}
