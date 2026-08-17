#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtpParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    pub type RtcRtpParameters;
    #[doc = "Get the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    #[wasm_bindgen(method, getter = "codecs")]
    pub fn get_codecs(this: &RtcRtpParameters) -> Option<::js_sys::Array>;
    #[doc = "Change the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    #[wasm_bindgen(method, setter = "codecs")]
    pub fn set_codecs(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `encodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    #[wasm_bindgen(method, getter = "encodings")]
    pub fn get_encodings(this: &RtcRtpParameters) -> Option<::js_sys::Array>;
    #[doc = "Change the `encodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    #[wasm_bindgen(method, setter = "encodings")]
    pub fn set_encodings(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    #[wasm_bindgen(method, getter = "headerExtensions")]
    pub fn get_header_extensions(this: &RtcRtpParameters) -> Option<::js_sys::Array>;
    #[doc = "Change the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    #[wasm_bindgen(method, setter = "headerExtensions")]
    pub fn set_header_extensions(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcRtcpParameters")]
    #[doc = "Get the `rtcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`, `RtcRtpParameters`*"]
    #[wasm_bindgen(method, getter = "rtcp")]
    pub fn get_rtcp(this: &RtcRtpParameters) -> Option<RtcRtcpParameters>;
    #[cfg(feature = "RtcRtcpParameters")]
    #[doc = "Change the `rtcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`, `RtcRtpParameters`*"]
    #[wasm_bindgen(method, setter = "rtcp")]
    pub fn set_rtcp(this: &RtcRtpParameters, val: &RtcRtcpParameters);
}
impl RtcRtpParameters {
    #[doc = "Construct a new `RtcRtpParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_codecs()` instead."]
    pub fn codecs(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_codecs(val);
        self
    }
    #[deprecated = "Use `set_encodings()` instead."]
    pub fn encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_encodings(val);
        self
    }
    #[deprecated = "Use `set_header_extensions()` instead."]
    pub fn header_extensions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_header_extensions(val);
        self
    }
    #[cfg(feature = "RtcRtcpParameters")]
    #[deprecated = "Use `set_rtcp()` instead."]
    pub fn rtcp(&mut self, val: &RtcRtcpParameters) -> &mut Self {
        self.set_rtcp(val);
        self
    }
}
impl Default for RtcRtpParameters {
    fn default() -> Self {
        Self::new()
    }
}
