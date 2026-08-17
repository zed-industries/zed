#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCCertificate",
        typescript_type = "RTCCertificate"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCertificate` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificate`*"]
    pub type RtcCertificate;
    #[wasm_bindgen(method, getter, js_class = "RTCCertificate", js_name = "expires")]
    #[doc = "Getter for the `expires` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate/expires)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificate`*"]
    pub fn expires(this: &RtcCertificate) -> f64;
}
