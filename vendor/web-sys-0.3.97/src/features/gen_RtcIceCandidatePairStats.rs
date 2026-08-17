#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIceCandidatePairStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidatePairStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub type RtcIceCandidatePairStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcIceCandidatePairStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcIceCandidatePairStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcIceCandidatePairStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcIceCandidatePairStats, val: RtcStatsType);
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received_u32(this: &RtcIceCandidatePairStats, val: u32);
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received_f64(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent_u32(this: &RtcIceCandidatePairStats, val: u32);
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent_f64(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "componentId")]
    pub fn get_component_id(this: &RtcIceCandidatePairStats) -> Option<u32>;
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "componentId")]
    pub fn set_component_id(this: &RtcIceCandidatePairStats, val: u32);
    #[doc = "Get the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "lastPacketReceivedTimestamp")]
    pub fn get_last_packet_received_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[doc = "Change the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "lastPacketReceivedTimestamp")]
    pub fn set_last_packet_received_timestamp(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "lastPacketSentTimestamp")]
    pub fn get_last_packet_sent_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[doc = "Change the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "lastPacketSentTimestamp")]
    pub fn set_last_packet_sent_timestamp(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "localCandidateId")]
    pub fn get_local_candidate_id(
        this: &RtcIceCandidatePairStats,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "localCandidateId")]
    pub fn set_local_candidate_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "nominated")]
    pub fn get_nominated(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[doc = "Change the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "nominated")]
    pub fn set_nominated(this: &RtcIceCandidatePairStats, val: bool);
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "priority")]
    pub fn get_priority(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority_u32(this: &RtcIceCandidatePairStats, val: u32);
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority_f64(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "readable")]
    pub fn get_readable(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "readable")]
    pub fn set_readable(this: &RtcIceCandidatePairStats, val: bool);
    #[doc = "Get the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "remoteCandidateId")]
    pub fn get_remote_candidate_id(
        this: &RtcIceCandidatePairStats,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "remoteCandidateId")]
    pub fn set_remote_candidate_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "selected")]
    pub fn get_selected(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[doc = "Change the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "selected")]
    pub fn set_selected(this: &RtcIceCandidatePairStats, val: bool);
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &RtcIceCandidatePairStats) -> Option<RtcStatsIceCandidatePairState>;
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &RtcIceCandidatePairStats, val: RtcStatsIceCandidatePairState);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcIceCandidatePairStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "transportId")]
    pub fn set_transport_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "writable")]
    pub fn get_writable(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, setter = "writable")]
    pub fn set_writable(this: &RtcIceCandidatePairStats, val: bool);
}
impl RtcIceCandidatePairStats {
    #[doc = "Construct a new `RtcIceCandidatePairStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
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
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received(val);
        self
    }
    #[deprecated = "Use `set_bytes_sent()` instead."]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
    #[deprecated = "Use `set_component_id()` instead."]
    pub fn component_id(&mut self, val: u32) -> &mut Self {
        self.set_component_id(val);
        self
    }
    #[deprecated = "Use `set_last_packet_received_timestamp()` instead."]
    pub fn last_packet_received_timestamp(&mut self, val: f64) -> &mut Self {
        self.set_last_packet_received_timestamp(val);
        self
    }
    #[deprecated = "Use `set_last_packet_sent_timestamp()` instead."]
    pub fn last_packet_sent_timestamp(&mut self, val: f64) -> &mut Self {
        self.set_last_packet_sent_timestamp(val);
        self
    }
    #[deprecated = "Use `set_local_candidate_id()` instead."]
    pub fn local_candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_local_candidate_id(val);
        self
    }
    #[deprecated = "Use `set_nominated()` instead."]
    pub fn nominated(&mut self, val: bool) -> &mut Self {
        self.set_nominated(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: f64) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_readable()` instead."]
    pub fn readable(&mut self, val: bool) -> &mut Self {
        self.set_readable(val);
        self
    }
    #[deprecated = "Use `set_remote_candidate_id()` instead."]
    pub fn remote_candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_candidate_id(val);
        self
    }
    #[deprecated = "Use `set_selected()` instead."]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected(val);
        self
    }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: RtcStatsIceCandidatePairState) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_transport_id()` instead."]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id(val);
        self
    }
    #[deprecated = "Use `set_writable()` instead."]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.set_writable(val);
        self
    }
}
impl Default for RtcIceCandidatePairStats {
    fn default() -> Self {
        Self::new()
    }
}
