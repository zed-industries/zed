#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCRtpHeaderExtensionCapability"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpHeaderExtensionCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub type RtcRtpHeaderExtensionCapability;
    #[doc = "Get the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    #[wasm_bindgen(method, getter = "uri")]
    pub fn get_uri(this: &RtcRtpHeaderExtensionCapability) -> ::alloc::string::String;
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    #[wasm_bindgen(method, setter = "uri")]
    pub fn set_uri(this: &RtcRtpHeaderExtensionCapability, val: &str);
}
impl RtcRtpHeaderExtensionCapability {
    #[doc = "Construct a new `RtcRtpHeaderExtensionCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub fn new(uri: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_uri(uri);
        ret
    }
    #[deprecated = "Use `set_uri()` instead."]
    pub fn uri(&mut self, val: &str) -> &mut Self {
        self.set_uri(val);
        self
    }
}
