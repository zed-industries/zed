#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCRTPContributingSourceStats"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcrtpContributingSourceStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub type RtcrtpContributingSourceStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcrtpContributingSourceStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcrtpContributingSourceStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcrtpContributingSourceStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcrtpContributingSourceStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcrtpContributingSourceStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcrtpContributingSourceStats, val: RtcStatsType);
    #[doc = "Get the `contributorSsrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, getter = "contributorSsrc")]
    pub fn get_contributor_ssrc(this: &RtcrtpContributingSourceStats) -> Option<u32>;
    #[doc = "Change the `contributorSsrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, setter = "contributorSsrc")]
    pub fn set_contributor_ssrc(this: &RtcrtpContributingSourceStats, val: u32);
    #[doc = "Get the `inboundRtpStreamId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, getter = "inboundRtpStreamId")]
    pub fn get_inbound_rtp_stream_id(
        this: &RtcrtpContributingSourceStats,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `inboundRtpStreamId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    #[wasm_bindgen(method, setter = "inboundRtpStreamId")]
    pub fn set_inbound_rtp_stream_id(this: &RtcrtpContributingSourceStats, val: &str);
}
impl RtcrtpContributingSourceStats {
    #[doc = "Construct a new `RtcrtpContributingSourceStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
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
    #[deprecated = "Use `set_contributor_ssrc()` instead."]
    pub fn contributor_ssrc(&mut self, val: u32) -> &mut Self {
        self.set_contributor_ssrc(val);
        self
    }
    #[deprecated = "Use `set_inbound_rtp_stream_id()` instead."]
    pub fn inbound_rtp_stream_id(&mut self, val: &str) -> &mut Self {
        self.set_inbound_rtp_stream_id(val);
        self
    }
}
impl Default for RtcrtpContributingSourceStats {
    fn default() -> Self {
        Self::new()
    }
}
