#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XRHand",
        typescript_type = "XRHand"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrHand` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRHand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrHand`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrHand;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRHand", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRHand/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrHand`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(this: &XrHand) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrHandJoint", feature = "XrJointSpace",))]
    #[wasm_bindgen(method, js_class = "XRHand")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRHand/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrHand`, `XrHandJoint`, `XrJointSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get(this: &XrHand, key: XrHandJoint) -> XrJointSpace;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "XRHand")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRHand/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrHand`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn keys(this: &XrHand) -> ::js_sys::Iterator<::js_sys::JsString>;
}
