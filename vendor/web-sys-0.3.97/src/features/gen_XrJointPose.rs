#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "XrPose",
        extends = "::js_sys::Object",
        js_name = "XRJointPose",
        typescript_type = "XRJointPose"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrJointPose` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRJointPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrJointPose`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrJointPose;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRJointPose", js_name = "radius")]
    #[doc = "Getter for the `radius` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRJointPose/radius)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrJointPose`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn radius(this: &XrJointPose) -> f32;
}
