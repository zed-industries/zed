#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCRtpHeaderExtensionParameters"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpHeaderExtensionParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub type RtcRtpHeaderExtensionParameters;
    #[doc = "Get the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    #[wasm_bindgen(method, getter = "encrypted")]
    pub fn get_encrypted(this: &RtcRtpHeaderExtensionParameters) -> Option<bool>;
    #[doc = "Change the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    #[wasm_bindgen(method, setter = "encrypted")]
    pub fn set_encrypted(this: &RtcRtpHeaderExtensionParameters, val: bool);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcRtpHeaderExtensionParameters) -> Option<u16>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcRtpHeaderExtensionParameters, val: u16);
    #[doc = "Get the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    #[wasm_bindgen(method, getter = "uri")]
    pub fn get_uri(this: &RtcRtpHeaderExtensionParameters) -> Option<::alloc::string::String>;
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    #[wasm_bindgen(method, setter = "uri")]
    pub fn set_uri(this: &RtcRtpHeaderExtensionParameters, val: &str);
}
impl RtcRtpHeaderExtensionParameters {
    #[doc = "Construct a new `RtcRtpHeaderExtensionParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_encrypted()` instead."]
    pub fn encrypted(&mut self, val: bool) -> &mut Self {
        self.set_encrypted(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: u16) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_uri()` instead."]
    pub fn uri(&mut self, val: &str) -> &mut Self {
        self.set_uri(val);
        self
    }
}
impl Default for RtcRtpHeaderExtensionParameters {
    fn default() -> Self {
        Self::new()
    }
}
