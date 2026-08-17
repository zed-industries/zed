#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtpSynchronizationSource")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSynchronizationSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub type RtcRtpSynchronizationSource;
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, getter = "audioLevel")]
    pub fn get_audio_level(this: &RtcRtpSynchronizationSource) -> Option<f64>;
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, setter = "audioLevel")]
    pub fn set_audio_level(this: &RtcRtpSynchronizationSource, val: f64);
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &RtcRtpSynchronizationSource) -> u32;
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &RtcRtpSynchronizationSource, val: u32);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcRtpSynchronizationSource) -> f64;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcRtpSynchronizationSource, val: f64);
    #[doc = "Get the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, getter = "voiceActivityFlag")]
    pub fn get_voice_activity_flag(this: &RtcRtpSynchronizationSource) -> Option<bool>;
    #[doc = "Change the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    #[wasm_bindgen(method, setter = "voiceActivityFlag")]
    pub fn set_voice_activity_flag(this: &RtcRtpSynchronizationSource, val: Option<bool>);
}
impl RtcRtpSynchronizationSource {
    #[doc = "Construct a new `RtcRtpSynchronizationSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn new(source: u32, timestamp: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source(source);
        ret.set_timestamp(timestamp);
        ret
    }
    #[deprecated = "Use `set_audio_level()` instead."]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: u32) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[deprecated = "Use `set_voice_activity_flag()` instead."]
    pub fn voice_activity_flag(&mut self, val: Option<bool>) -> &mut Self {
        self.set_voice_activity_flag(val);
        self
    }
}
