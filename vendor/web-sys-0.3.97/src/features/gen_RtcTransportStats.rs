#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCTransportStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTransportStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub type RtcTransportStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcTransportStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcTransportStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcTransportStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcTransportStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcTransportStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcTransportStats, val: RtcStatsType);
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &RtcTransportStats) -> Option<u32>;
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received(this: &RtcTransportStats, val: u32);
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &RtcTransportStats) -> Option<u32>;
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent(this: &RtcTransportStats, val: u32);
}
impl RtcTransportStats {
    #[doc = "Construct a new `RtcTransportStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
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
    #[deprecated = "Use `set_bytes_received()` instead."]
    pub fn bytes_received(&mut self, val: u32) -> &mut Self {
        self.set_bytes_received(val);
        self
    }
    #[deprecated = "Use `set_bytes_sent()` instead."]
    pub fn bytes_sent(&mut self, val: u32) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
}
impl Default for RtcTransportStats {
    fn default() -> Self {
        Self::new()
    }
}
