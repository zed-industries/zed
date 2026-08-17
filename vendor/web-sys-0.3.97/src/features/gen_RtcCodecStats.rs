#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCCodecStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCodecStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub type RtcCodecStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcCodecStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcCodecStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcCodecStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcCodecStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcCodecStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcCodecStats, val: RtcStatsType);
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "channels")]
    pub fn get_channels(this: &RtcCodecStats) -> Option<u32>;
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, setter = "channels")]
    pub fn set_channels(this: &RtcCodecStats, val: u32);
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "clockRate")]
    pub fn get_clock_rate(this: &RtcCodecStats) -> Option<u32>;
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, setter = "clockRate")]
    pub fn set_clock_rate(this: &RtcCodecStats, val: u32);
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "codec")]
    pub fn get_codec(this: &RtcCodecStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, setter = "codec")]
    pub fn set_codec(this: &RtcCodecStats, val: &str);
    #[doc = "Get the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "parameters")]
    pub fn get_parameters(this: &RtcCodecStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, setter = "parameters")]
    pub fn set_parameters(this: &RtcCodecStats, val: &str);
    #[doc = "Get the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "payloadType")]
    pub fn get_payload_type(this: &RtcCodecStats) -> Option<u32>;
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, setter = "payloadType")]
    pub fn set_payload_type(this: &RtcCodecStats, val: u32);
}
impl RtcCodecStats {
    #[doc = "Construct a new `RtcCodecStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_channels()` instead."]
    pub fn channels(&mut self, val: u32) -> &mut Self {
        self.set_channels(val);
        self
    }
    #[deprecated = "Use `set_clock_rate()` instead."]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.set_clock_rate(val);
        self
    }
    #[deprecated = "Use `set_codec()` instead."]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec(val);
        self
    }
    #[deprecated = "Use `set_parameters()` instead."]
    pub fn parameters(&mut self, val: &str) -> &mut Self {
        self.set_parameters(val);
        self
    }
    #[deprecated = "Use `set_payload_type()` instead."]
    pub fn payload_type(&mut self, val: u32) -> &mut Self {
        self.set_payload_type(val);
        self
    }
}
impl Default for RtcCodecStats {
    fn default() -> Self {
        Self::new()
    }
}
