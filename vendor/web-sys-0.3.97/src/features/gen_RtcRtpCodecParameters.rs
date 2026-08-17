#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtpCodecParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpCodecParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub type RtcRtpCodecParameters;
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, getter = "channels")]
    pub fn get_channels(this: &RtcRtpCodecParameters) -> Option<u16>;
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, setter = "channels")]
    pub fn set_channels(this: &RtcRtpCodecParameters, val: u16);
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, getter = "clockRate")]
    pub fn get_clock_rate(this: &RtcRtpCodecParameters) -> Option<u32>;
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, setter = "clockRate")]
    pub fn set_clock_rate(this: &RtcRtpCodecParameters, val: u32);
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &RtcRtpCodecParameters) -> Option<::alloc::string::String>;
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &RtcRtpCodecParameters, val: &str);
    #[doc = "Get the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, getter = "payloadType")]
    pub fn get_payload_type(this: &RtcRtpCodecParameters) -> Option<u16>;
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, setter = "payloadType")]
    pub fn set_payload_type(this: &RtcRtpCodecParameters, val: u16);
    #[doc = "Get the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, getter = "sdpFmtpLine")]
    pub fn get_sdp_fmtp_line(this: &RtcRtpCodecParameters) -> Option<::alloc::string::String>;
    #[doc = "Change the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    #[wasm_bindgen(method, setter = "sdpFmtpLine")]
    pub fn set_sdp_fmtp_line(this: &RtcRtpCodecParameters, val: &str);
}
impl RtcRtpCodecParameters {
    #[doc = "Construct a new `RtcRtpCodecParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_channels()` instead."]
    pub fn channels(&mut self, val: u16) -> &mut Self {
        self.set_channels(val);
        self
    }
    #[deprecated = "Use `set_clock_rate()` instead."]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.set_clock_rate(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[deprecated = "Use `set_payload_type()` instead."]
    pub fn payload_type(&mut self, val: u16) -> &mut Self {
        self.set_payload_type(val);
        self
    }
    #[deprecated = "Use `set_sdp_fmtp_line()` instead."]
    pub fn sdp_fmtp_line(&mut self, val: &str) -> &mut Self {
        self.set_sdp_fmtp_line(val);
        self
    }
}
impl Default for RtcRtpCodecParameters {
    fn default() -> Self {
        Self::new()
    }
}
