#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCStatsReportInternal")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcStatsReportInternal` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub type RtcStatsReportInternal;
    #[doc = "Get the `closed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "closed")]
    pub fn get_closed(this: &RtcStatsReportInternal) -> Option<bool>;
    #[doc = "Change the `closed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "closed")]
    pub fn set_closed(this: &RtcStatsReportInternal, val: bool);
    #[doc = "Get the `codecStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "codecStats")]
    pub fn get_codec_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `codecStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "codecStats")]
    pub fn set_codec_stats(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `iceCandidatePairStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "iceCandidatePairStats")]
    pub fn get_ice_candidate_pair_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `iceCandidatePairStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "iceCandidatePairStats")]
    pub fn set_ice_candidate_pair_stats(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Get the `iceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "iceCandidateStats")]
    pub fn get_ice_candidate_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `iceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "iceCandidateStats")]
    pub fn set_ice_candidate_stats(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `iceComponentStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "iceComponentStats")]
    pub fn get_ice_component_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `iceComponentStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "iceComponentStats")]
    pub fn set_ice_component_stats(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `iceRestarts` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "iceRestarts")]
    pub fn get_ice_restarts(this: &RtcStatsReportInternal) -> Option<u32>;
    #[doc = "Change the `iceRestarts` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "iceRestarts")]
    pub fn set_ice_restarts(this: &RtcStatsReportInternal, val: u32);
    #[doc = "Get the `iceRollbacks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "iceRollbacks")]
    pub fn get_ice_rollbacks(this: &RtcStatsReportInternal) -> Option<u32>;
    #[doc = "Change the `iceRollbacks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "iceRollbacks")]
    pub fn set_ice_rollbacks(this: &RtcStatsReportInternal, val: u32);
    #[doc = "Get the `inboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "inboundRTPStreamStats")]
    pub fn get_inbound_rtp_stream_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `inboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "inboundRTPStreamStats")]
    pub fn set_inbound_rtp_stream_stats(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Get the `localSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "localSdp")]
    pub fn get_local_sdp(this: &RtcStatsReportInternal) -> Option<::alloc::string::String>;
    #[doc = "Change the `localSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "localSdp")]
    pub fn set_local_sdp(this: &RtcStatsReportInternal, val: &str);
    #[doc = "Get the `mediaStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "mediaStreamStats")]
    pub fn get_media_stream_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `mediaStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "mediaStreamStats")]
    pub fn set_media_stream_stats(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `mediaStreamTrackStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "mediaStreamTrackStats")]
    pub fn get_media_stream_track_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `mediaStreamTrackStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "mediaStreamTrackStats")]
    pub fn set_media_stream_track_stats(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Get the `offerer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "offerer")]
    pub fn get_offerer(this: &RtcStatsReportInternal) -> Option<bool>;
    #[doc = "Change the `offerer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "offerer")]
    pub fn set_offerer(this: &RtcStatsReportInternal, val: bool);
    #[doc = "Get the `outboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "outboundRTPStreamStats")]
    pub fn get_outbound_rtp_stream_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `outboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "outboundRTPStreamStats")]
    pub fn set_outbound_rtp_stream_stats(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Get the `pcid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "pcid")]
    pub fn get_pcid(this: &RtcStatsReportInternal) -> Option<::alloc::string::String>;
    #[doc = "Change the `pcid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "pcid")]
    pub fn set_pcid(this: &RtcStatsReportInternal, val: &str);
    #[doc = "Get the `rawLocalCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "rawLocalCandidates")]
    pub fn get_raw_local_candidates(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `rawLocalCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "rawLocalCandidates")]
    pub fn set_raw_local_candidates(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `rawRemoteCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "rawRemoteCandidates")]
    pub fn get_raw_remote_candidates(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `rawRemoteCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "rawRemoteCandidates")]
    pub fn set_raw_remote_candidates(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `remoteSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "remoteSdp")]
    pub fn get_remote_sdp(this: &RtcStatsReportInternal) -> Option<::alloc::string::String>;
    #[doc = "Change the `remoteSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "remoteSdp")]
    pub fn set_remote_sdp(this: &RtcStatsReportInternal, val: &str);
    #[doc = "Get the `rtpContributingSourceStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "rtpContributingSourceStats")]
    pub fn get_rtp_contributing_source_stats(
        this: &RtcStatsReportInternal,
    ) -> Option<::js_sys::Array>;
    #[doc = "Change the `rtpContributingSourceStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "rtpContributingSourceStats")]
    pub fn set_rtp_contributing_source_stats(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcStatsReportInternal) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcStatsReportInternal, val: f64);
    #[doc = "Get the `transportStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "transportStats")]
    pub fn get_transport_stats(this: &RtcStatsReportInternal) -> Option<::js_sys::Array>;
    #[doc = "Change the `transportStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "transportStats")]
    pub fn set_transport_stats(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `trickledIceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, getter = "trickledIceCandidateStats")]
    pub fn get_trickled_ice_candidate_stats(
        this: &RtcStatsReportInternal,
    ) -> Option<::js_sys::Array>;
    #[doc = "Change the `trickledIceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    #[wasm_bindgen(method, setter = "trickledIceCandidateStats")]
    pub fn set_trickled_ice_candidate_stats(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
}
impl RtcStatsReportInternal {
    #[doc = "Construct a new `RtcStatsReportInternal`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_closed()` instead."]
    pub fn closed(&mut self, val: bool) -> &mut Self {
        self.set_closed(val);
        self
    }
    #[deprecated = "Use `set_codec_stats()` instead."]
    pub fn codec_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_codec_stats(val);
        self
    }
    #[deprecated = "Use `set_ice_candidate_pair_stats()` instead."]
    pub fn ice_candidate_pair_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_candidate_pair_stats(val);
        self
    }
    #[deprecated = "Use `set_ice_candidate_stats()` instead."]
    pub fn ice_candidate_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_candidate_stats(val);
        self
    }
    #[deprecated = "Use `set_ice_component_stats()` instead."]
    pub fn ice_component_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_component_stats(val);
        self
    }
    #[deprecated = "Use `set_ice_restarts()` instead."]
    pub fn ice_restarts(&mut self, val: u32) -> &mut Self {
        self.set_ice_restarts(val);
        self
    }
    #[deprecated = "Use `set_ice_rollbacks()` instead."]
    pub fn ice_rollbacks(&mut self, val: u32) -> &mut Self {
        self.set_ice_rollbacks(val);
        self
    }
    #[deprecated = "Use `set_inbound_rtp_stream_stats()` instead."]
    pub fn inbound_rtp_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_inbound_rtp_stream_stats(val);
        self
    }
    #[deprecated = "Use `set_local_sdp()` instead."]
    pub fn local_sdp(&mut self, val: &str) -> &mut Self {
        self.set_local_sdp(val);
        self
    }
    #[deprecated = "Use `set_media_stream_stats()` instead."]
    pub fn media_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_media_stream_stats(val);
        self
    }
    #[deprecated = "Use `set_media_stream_track_stats()` instead."]
    pub fn media_stream_track_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_media_stream_track_stats(val);
        self
    }
    #[deprecated = "Use `set_offerer()` instead."]
    pub fn offerer(&mut self, val: bool) -> &mut Self {
        self.set_offerer(val);
        self
    }
    #[deprecated = "Use `set_outbound_rtp_stream_stats()` instead."]
    pub fn outbound_rtp_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_outbound_rtp_stream_stats(val);
        self
    }
    #[deprecated = "Use `set_pcid()` instead."]
    pub fn pcid(&mut self, val: &str) -> &mut Self {
        self.set_pcid(val);
        self
    }
    #[deprecated = "Use `set_raw_local_candidates()` instead."]
    pub fn raw_local_candidates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_raw_local_candidates(val);
        self
    }
    #[deprecated = "Use `set_raw_remote_candidates()` instead."]
    pub fn raw_remote_candidates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_raw_remote_candidates(val);
        self
    }
    #[deprecated = "Use `set_remote_sdp()` instead."]
    pub fn remote_sdp(&mut self, val: &str) -> &mut Self {
        self.set_remote_sdp(val);
        self
    }
    #[deprecated = "Use `set_rtp_contributing_source_stats()` instead."]
    pub fn rtp_contributing_source_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_rtp_contributing_source_stats(val);
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[deprecated = "Use `set_transport_stats()` instead."]
    pub fn transport_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_transport_stats(val);
        self
    }
    #[deprecated = "Use `set_trickled_ice_candidate_stats()` instead."]
    pub fn trickled_ice_candidate_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_trickled_ice_candidate_stats(val);
        self
    }
}
impl Default for RtcStatsReportInternal {
    fn default() -> Self {
        Self::new()
    }
}
