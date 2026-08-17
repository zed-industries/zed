#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCCertificateExpiration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCertificateExpiration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    pub type RtcCertificateExpiration;
    #[doc = "Get the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    #[wasm_bindgen(method, getter = "expires")]
    pub fn get_expires(this: &RtcCertificateExpiration) -> Option<f64>;
    #[doc = "Change the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    #[wasm_bindgen(method, setter = "expires")]
    pub fn set_expires(this: &RtcCertificateExpiration, val: f64);
    #[doc = "Change the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    #[wasm_bindgen(method, setter = "expires")]
    pub fn set_expires_u32(this: &RtcCertificateExpiration, val: u32);
    #[doc = "Change the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    #[wasm_bindgen(method, setter = "expires")]
    pub fn set_expires_f64(this: &RtcCertificateExpiration, val: f64);
}
impl RtcCertificateExpiration {
    #[doc = "Construct a new `RtcCertificateExpiration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_expires()` instead."]
    pub fn expires(&mut self, val: f64) -> &mut Self {
        self.set_expires(val);
        self
    }
}
impl Default for RtcCertificateExpiration {
    fn default() -> Self {
        Self::new()
    }
}
