#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCInboundRTPStreamStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcInboundRtpStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub type RtcInboundRtpStreamStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcInboundRtpStreamStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcInboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcInboundRtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcInboundRtpStreamStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcInboundRtpStreamStats, val: RtcStatsType);
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateMean")]
    pub fn get_bitrate_mean(this: &RtcInboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bitrateMean")]
    pub fn set_bitrate_mean(this: &RtcInboundRtpStreamStats, val: f64);
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateStdDev")]
    pub fn get_bitrate_std_dev(this: &RtcInboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    pub fn set_bitrate_std_dev(this: &RtcInboundRtpStreamStats, val: f64);
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "codecId")]
    pub fn get_codec_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "codecId")]
    pub fn set_codec_id(this: &RtcInboundRtpStreamStats, val: &str);
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "firCount")]
    pub fn get_fir_count(this: &RtcInboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "firCount")]
    pub fn set_fir_count(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateMean")]
    pub fn get_framerate_mean(this: &RtcInboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "framerateMean")]
    pub fn set_framerate_mean(this: &RtcInboundRtpStreamStats, val: f64);
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateStdDev")]
    pub fn get_framerate_std_dev(this: &RtcInboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    pub fn set_framerate_std_dev(this: &RtcInboundRtpStreamStats, val: f64);
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "isRemote")]
    pub fn get_is_remote(this: &RtcInboundRtpStreamStats) -> Option<bool>;
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "isRemote")]
    pub fn set_is_remote(this: &RtcInboundRtpStreamStats, val: bool);
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaTrackId")]
    pub fn get_media_track_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    pub fn set_media_track_id(this: &RtcInboundRtpStreamStats, val: &str);
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaType")]
    pub fn get_media_type(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "mediaType")]
    pub fn set_media_type(this: &RtcInboundRtpStreamStats, val: &str);
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "nackCount")]
    pub fn get_nack_count(this: &RtcInboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "nackCount")]
    pub fn set_nack_count(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "pliCount")]
    pub fn get_pli_count(this: &RtcInboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "pliCount")]
    pub fn set_pli_count(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "remoteId")]
    pub fn get_remote_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "remoteId")]
    pub fn set_remote_id(this: &RtcInboundRtpStreamStats, val: &str);
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "ssrc")]
    pub fn get_ssrc(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "ssrc")]
    pub fn set_ssrc(this: &RtcInboundRtpStreamStats, val: &str);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "transportId")]
    pub fn set_transport_id(this: &RtcInboundRtpStreamStats, val: &str);
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &RtcInboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received(this: &RtcInboundRtpStreamStats, val: f64);
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received_u32(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received_f64(this: &RtcInboundRtpStreamStats, val: f64);
    #[doc = "Get the `discardedPackets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "discardedPackets")]
    pub fn get_discarded_packets(this: &RtcInboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `discardedPackets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "discardedPackets")]
    pub fn set_discarded_packets(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framesDecoded")]
    pub fn get_frames_decoded(this: &RtcInboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "framesDecoded")]
    pub fn set_frames_decoded(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Get the `jitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "jitter")]
    pub fn get_jitter(this: &RtcInboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `jitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "jitter")]
    pub fn set_jitter(this: &RtcInboundRtpStreamStats, val: f64);
    #[doc = "Get the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "packetsLost")]
    pub fn get_packets_lost(this: &RtcInboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "packetsLost")]
    pub fn set_packets_lost(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Get the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "packetsReceived")]
    pub fn get_packets_received(this: &RtcInboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "packetsReceived")]
    pub fn set_packets_received(this: &RtcInboundRtpStreamStats, val: u32);
    #[doc = "Get the `roundTripTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "roundTripTime")]
    pub fn get_round_trip_time(this: &RtcInboundRtpStreamStats) -> Option<i32>;
    #[doc = "Change the `roundTripTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "roundTripTime")]
    pub fn set_round_trip_time(this: &RtcInboundRtpStreamStats, val: i32);
}
impl RtcInboundRtpStreamStats {
    #[doc = "Construct a new `RtcInboundRtpStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
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
    #[deprecated = "Use `set_bitrate_mean()` instead."]
    pub fn bitrate_mean(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_mean(val);
        self
    }
    #[deprecated = "Use `set_bitrate_std_dev()` instead."]
    pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_std_dev(val);
        self
    }
    #[deprecated = "Use `set_codec_id()` instead."]
    pub fn codec_id(&mut self, val: &str) -> &mut Self {
        self.set_codec_id(val);
        self
    }
    #[deprecated = "Use `set_fir_count()` instead."]
    pub fn fir_count(&mut self, val: u32) -> &mut Self {
        self.set_fir_count(val);
        self
    }
    #[deprecated = "Use `set_framerate_mean()` instead."]
    pub fn framerate_mean(&mut self, val: f64) -> &mut Self {
        self.set_framerate_mean(val);
        self
    }
    #[deprecated = "Use `set_framerate_std_dev()` instead."]
    pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_framerate_std_dev(val);
        self
    }
    #[deprecated = "Use `set_is_remote()` instead."]
    pub fn is_remote(&mut self, val: bool) -> &mut Self {
        self.set_is_remote(val);
        self
    }
    #[deprecated = "Use `set_media_track_id()` instead."]
    pub fn media_track_id(&mut self, val: &str) -> &mut Self {
        self.set_media_track_id(val);
        self
    }
    #[deprecated = "Use `set_media_type()` instead."]
    pub fn media_type(&mut self, val: &str) -> &mut Self {
        self.set_media_type(val);
        self
    }
    #[deprecated = "Use `set_nack_count()` instead."]
    pub fn nack_count(&mut self, val: u32) -> &mut Self {
        self.set_nack_count(val);
        self
    }
    #[deprecated = "Use `set_pli_count()` instead."]
    pub fn pli_count(&mut self, val: u32) -> &mut Self {
        self.set_pli_count(val);
        self
    }
    #[deprecated = "Use `set_remote_id()` instead."]
    pub fn remote_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_id(val);
        self
    }
    #[deprecated = "Use `set_ssrc()` instead."]
    pub fn ssrc(&mut self, val: &str) -> &mut Self {
        self.set_ssrc(val);
        self
    }
    #[deprecated = "Use `set_transport_id()` instead."]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id(val);
        self
    }
    #[deprecated = "Use `set_bytes_received()` instead."]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received(val);
        self
    }
    #[deprecated = "Use `set_discarded_packets()` instead."]
    pub fn discarded_packets(&mut self, val: u32) -> &mut Self {
        self.set_discarded_packets(val);
        self
    }
    #[deprecated = "Use `set_frames_decoded()` instead."]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_decoded(val);
        self
    }
    #[deprecated = "Use `set_jitter()` instead."]
    pub fn jitter(&mut self, val: f64) -> &mut Self {
        self.set_jitter(val);
        self
    }
    #[deprecated = "Use `set_packets_lost()` instead."]
    pub fn packets_lost(&mut self, val: u32) -> &mut Self {
        self.set_packets_lost(val);
        self
    }
    #[deprecated = "Use `set_packets_received()` instead."]
    pub fn packets_received(&mut self, val: u32) -> &mut Self {
        self.set_packets_received(val);
        self
    }
    #[deprecated = "Use `set_round_trip_time()` instead."]
    pub fn round_trip_time(&mut self, val: i32) -> &mut Self {
        self.set_round_trip_time(val);
        self
    }
}
impl Default for RtcInboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
