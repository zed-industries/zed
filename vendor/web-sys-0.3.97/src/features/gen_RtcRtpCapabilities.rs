#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtpCapabilities")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpCapabilities` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    pub type RtcRtpCapabilities;
    #[doc = "Get the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    #[wasm_bindgen(method, getter = "codecs")]
    pub fn get_codecs(this: &RtcRtpCapabilities) -> ::js_sys::Array;
    #[doc = "Change the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    #[wasm_bindgen(method, setter = "codecs")]
    pub fn set_codecs(this: &RtcRtpCapabilities, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    #[wasm_bindgen(method, getter = "headerExtensions")]
    pub fn get_header_extensions(this: &RtcRtpCapabilities) -> ::js_sys::Array;
    #[doc = "Change the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    #[wasm_bindgen(method, setter = "headerExtensions")]
    pub fn set_header_extensions(this: &RtcRtpCapabilities, val: &::wasm_bindgen::JsValue);
}
impl RtcRtpCapabilities {
    #[doc = "Construct a new `RtcRtpCapabilities`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    pub fn new(
        codecs: &::wasm_bindgen::JsValue,
        header_extensions: &::wasm_bindgen::JsValue,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_codecs(codecs);
        ret.set_header_extensions(header_extensions);
        ret
    }
    #[deprecated = "Use `set_codecs()` instead."]
    pub fn codecs(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_codecs(val);
        self
    }
    #[deprecated = "Use `set_header_extensions()` instead."]
    pub fn header_extensions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_header_extensions(val);
        self
    }
}
