#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "XrReferenceSpace",
        extends = "XrSpace",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "XRBoundedReferenceSpace",
        typescript_type = "XRBoundedReferenceSpace"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrBoundedReferenceSpace` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRBoundedReferenceSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrBoundedReferenceSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrBoundedReferenceSpace;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomPointReadOnly")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRBoundedReferenceSpace",
        js_name = "boundsGeometry"
    )]
    #[doc = "Getter for the `boundsGeometry` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRBoundedReferenceSpace/boundsGeometry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`, `XrBoundedReferenceSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bounds_geometry(this: &XrBoundedReferenceSpace) -> ::js_sys::Array<DomPointReadOnly>;
}
