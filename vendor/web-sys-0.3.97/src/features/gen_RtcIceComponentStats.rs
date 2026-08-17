#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIceComponentStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceComponentStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub type RtcIceComponentStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcIceComponentStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcIceComponentStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcIceComponentStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcIceComponentStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcIceComponentStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcIceComponentStats, val: RtcStatsType);
    #[doc = "Get the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "activeConnection")]
    pub fn get_active_connection(this: &RtcIceComponentStats) -> Option<bool>;
    #[doc = "Change the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, setter = "activeConnection")]
    pub fn set_active_connection(this: &RtcIceComponentStats, val: bool);
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &RtcIceComponentStats) -> Option<u32>;
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received(this: &RtcIceComponentStats, val: u32);
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &RtcIceComponentStats) -> Option<u32>;
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent(this: &RtcIceComponentStats, val: u32);
    #[doc = "Get the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "component")]
    pub fn get_component(this: &RtcIceComponentStats) -> Option<i32>;
    #[doc = "Change the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, setter = "component")]
    pub fn set_component(this: &RtcIceComponentStats, val: i32);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcIceComponentStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, setter = "transportId")]
    pub fn set_transport_id(this: &RtcIceComponentStats, val: &str);
}
impl RtcIceComponentStats {
    #[doc = "Construct a new `RtcIceComponentStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
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
    #[deprecated = "Use `set_active_connection()` instead."]
    pub fn active_connection(&mut self, val: bool) -> &mut Self {
        self.set_active_connection(val);
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
    #[deprecated = "Use `set_component()` instead."]
    pub fn component(&mut self, val: i32) -> &mut Self {
        self.set_component(val);
        self
    }
    #[deprecated = "Use `set_transport_id()` instead."]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id(val);
        self
    }
}
impl Default for RtcIceComponentStats {
    fn default() -> Self {
        Self::new()
    }
}
