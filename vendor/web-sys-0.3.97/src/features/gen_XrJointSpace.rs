#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "XrSpace",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "XRJointSpace",
        typescript_type = "XRJointSpace"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrJointSpace` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRJointSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrJointSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrJointSpace;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrHandJoint")]
    #[wasm_bindgen(method, getter, js_class = "XRJointSpace", js_name = "jointName")]
    #[doc = "Getter for the `jointName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRJointSpace/jointName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrHandJoint`, `XrJointSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn joint_name(this: &XrJointSpace) -> XrHandJoint;
}
