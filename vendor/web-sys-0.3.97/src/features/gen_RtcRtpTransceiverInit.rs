#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtpTransceiverInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpTransceiverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub type RtcRtpTransceiverInit;
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`, `RtcRtpTransceiverInit`*"]
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &RtcRtpTransceiverInit) -> Option<RtcRtpTransceiverDirection>;
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`, `RtcRtpTransceiverInit`*"]
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &RtcRtpTransceiverInit, val: RtcRtpTransceiverDirection);
    #[doc = "Get the `sendEncodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    #[wasm_bindgen(method, getter = "sendEncodings")]
    pub fn get_send_encodings(this: &RtcRtpTransceiverInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `sendEncodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    #[wasm_bindgen(method, setter = "sendEncodings")]
    pub fn set_send_encodings(this: &RtcRtpTransceiverInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    #[wasm_bindgen(method, getter = "streams")]
    pub fn get_streams(this: &RtcRtpTransceiverInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    #[wasm_bindgen(method, setter = "streams")]
    pub fn set_streams(this: &RtcRtpTransceiverInit, val: &::wasm_bindgen::JsValue);
}
impl RtcRtpTransceiverInit {
    #[doc = "Construct a new `RtcRtpTransceiverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[deprecated = "Use `set_direction()` instead."]
    pub fn direction(&mut self, val: RtcRtpTransceiverDirection) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[deprecated = "Use `set_send_encodings()` instead."]
    pub fn send_encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_send_encodings(val);
        self
    }
    #[deprecated = "Use `set_streams()` instead."]
    pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_streams(val);
        self
    }
}
impl Default for RtcRtpTransceiverInit {
    fn default() -> Self {
        Self::new()
    }
}
