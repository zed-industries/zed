#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCSessionDescriptionInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcSessionDescriptionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
    pub type RtcSessionDescriptionInit;
    #[doc = "Get the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
    #[wasm_bindgen(method, getter = "sdp")]
    pub fn get_sdp(this: &RtcSessionDescriptionInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
    #[wasm_bindgen(method, setter = "sdp")]
    pub fn set_sdp(this: &RtcSessionDescriptionInit, val: &str);
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcSessionDescriptionInit) -> RtcSdpType;
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcSessionDescriptionInit, val: RtcSdpType);
}
impl RtcSessionDescriptionInit {
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Construct a new `RtcSessionDescriptionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
    pub fn new(type_: RtcSdpType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_sdp()` instead."]
    pub fn sdp(&mut self, val: &str) -> &mut Self {
        self.set_sdp(val);
        self
    }
    #[cfg(feature = "RtcSdpType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: RtcSdpType) -> &mut Self {
        self.set_type(val);
        self
    }
}
