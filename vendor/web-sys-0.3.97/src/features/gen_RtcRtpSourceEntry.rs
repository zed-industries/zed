#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtpSourceEntry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSourceEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    pub type RtcRtpSourceEntry;
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, getter = "audioLevel")]
    pub fn get_audio_level(this: &RtcRtpSourceEntry) -> Option<f64>;
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, setter = "audioLevel")]
    pub fn set_audio_level(this: &RtcRtpSourceEntry, val: f64);
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &RtcRtpSourceEntry) -> u32;
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &RtcRtpSourceEntry, val: u32);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcRtpSourceEntry) -> f64;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcRtpSourceEntry, val: f64);
    #[doc = "Get the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, getter = "voiceActivityFlag")]
    pub fn get_voice_activity_flag(this: &RtcRtpSourceEntry) -> Option<bool>;
    #[doc = "Change the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    #[wasm_bindgen(method, setter = "voiceActivityFlag")]
    pub fn set_voice_activity_flag(this: &RtcRtpSourceEntry, val: Option<bool>);
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Get the `sourceType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
    #[wasm_bindgen(method, getter = "sourceType")]
    pub fn get_source_type(this: &RtcRtpSourceEntry) -> RtcRtpSourceEntryType;
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Change the `sourceType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
    #[wasm_bindgen(method, setter = "sourceType")]
    pub fn set_source_type(this: &RtcRtpSourceEntry, val: RtcRtpSourceEntryType);
}
impl RtcRtpSourceEntry {
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Construct a new `RtcRtpSourceEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
    pub fn new(source: u32, timestamp: f64, source_type: RtcRtpSourceEntryType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source(source);
        ret.set_timestamp(timestamp);
        ret.set_source_type(source_type);
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
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[deprecated = "Use `set_source_type()` instead."]
    pub fn source_type(&mut self, val: RtcRtpSourceEntryType) -> &mut Self {
        self.set_source_type(val);
        self
    }
}
