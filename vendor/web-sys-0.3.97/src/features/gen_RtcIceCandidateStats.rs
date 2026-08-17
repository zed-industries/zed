#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIceCandidateStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidateStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub type RtcIceCandidateStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcIceCandidateStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcIceCandidateStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcIceCandidateStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcIceCandidateStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcIceCandidateStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcIceCandidateStats, val: RtcStatsType);
    #[doc = "Get the `candidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, getter = "candidateId")]
    pub fn get_candidate_id(this: &RtcIceCandidateStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `candidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, setter = "candidateId")]
    pub fn set_candidate_id(this: &RtcIceCandidateStats, val: &str);
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[doc = "Get the `candidateType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*"]
    #[wasm_bindgen(method, getter = "candidateType")]
    pub fn get_candidate_type(this: &RtcIceCandidateStats) -> Option<RtcStatsIceCandidateType>;
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[doc = "Change the `candidateType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*"]
    #[wasm_bindgen(method, setter = "candidateType")]
    pub fn set_candidate_type(this: &RtcIceCandidateStats, val: RtcStatsIceCandidateType);
    #[doc = "Get the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, getter = "componentId")]
    pub fn get_component_id(this: &RtcIceCandidateStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, setter = "componentId")]
    pub fn set_component_id(this: &RtcIceCandidateStats, val: &str);
    #[doc = "Get the `ipAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, getter = "ipAddress")]
    pub fn get_ip_address(this: &RtcIceCandidateStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `ipAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, setter = "ipAddress")]
    pub fn set_ip_address(this: &RtcIceCandidateStats, val: &str);
    #[doc = "Get the `portNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, getter = "portNumber")]
    pub fn get_port_number(this: &RtcIceCandidateStats) -> Option<i32>;
    #[doc = "Change the `portNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, setter = "portNumber")]
    pub fn set_port_number(this: &RtcIceCandidateStats, val: i32);
    #[doc = "Get the `transport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, getter = "transport")]
    pub fn get_transport(this: &RtcIceCandidateStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `transport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    #[wasm_bindgen(method, setter = "transport")]
    pub fn set_transport(this: &RtcIceCandidateStats, val: &str);
}
impl RtcIceCandidateStats {
    #[doc = "Construct a new `RtcIceCandidateStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
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
    #[deprecated = "Use `set_candidate_id()` instead."]
    pub fn candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_candidate_id(val);
        self
    }
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[deprecated = "Use `set_candidate_type()` instead."]
    pub fn candidate_type(&mut self, val: RtcStatsIceCandidateType) -> &mut Self {
        self.set_candidate_type(val);
        self
    }
    #[deprecated = "Use `set_component_id()` instead."]
    pub fn component_id(&mut self, val: &str) -> &mut Self {
        self.set_component_id(val);
        self
    }
    #[deprecated = "Use `set_ip_address()` instead."]
    pub fn ip_address(&mut self, val: &str) -> &mut Self {
        self.set_ip_address(val);
        self
    }
    #[deprecated = "Use `set_port_number()` instead."]
    pub fn port_number(&mut self, val: i32) -> &mut Self {
        self.set_port_number(val);
        self
    }
    #[deprecated = "Use `set_transport()` instead."]
    pub fn transport(&mut self, val: &str) -> &mut Self {
        self.set_transport(val);
        self
    }
}
impl Default for RtcIceCandidateStats {
    fn default() -> Self {
        Self::new()
    }
}
