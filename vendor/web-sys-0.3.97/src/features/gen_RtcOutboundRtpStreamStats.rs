#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCOutboundRTPStreamStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOutboundRtpStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub type RtcOutboundRtpStreamStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcOutboundRtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcOutboundRtpStreamStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcOutboundRtpStreamStats, val: RtcStatsType);
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateMean")]
    pub fn get_bitrate_mean(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bitrateMean")]
    pub fn set_bitrate_mean(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateStdDev")]
    pub fn get_bitrate_std_dev(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    pub fn set_bitrate_std_dev(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "codecId")]
    pub fn get_codec_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "codecId")]
    pub fn set_codec_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "firCount")]
    pub fn get_fir_count(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "firCount")]
    pub fn set_fir_count(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateMean")]
    pub fn get_framerate_mean(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "framerateMean")]
    pub fn set_framerate_mean(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateStdDev")]
    pub fn get_framerate_std_dev(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    pub fn set_framerate_std_dev(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "isRemote")]
    pub fn get_is_remote(this: &RtcOutboundRtpStreamStats) -> Option<bool>;
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "isRemote")]
    pub fn set_is_remote(this: &RtcOutboundRtpStreamStats, val: bool);
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaTrackId")]
    pub fn get_media_track_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    pub fn set_media_track_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaType")]
    pub fn get_media_type(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "mediaType")]
    pub fn set_media_type(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "nackCount")]
    pub fn get_nack_count(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "nackCount")]
    pub fn set_nack_count(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "pliCount")]
    pub fn get_pli_count(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "pliCount")]
    pub fn set_pli_count(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "remoteId")]
    pub fn get_remote_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "remoteId")]
    pub fn set_remote_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "ssrc")]
    pub fn get_ssrc(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "ssrc")]
    pub fn set_ssrc(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "transportId")]
    pub fn set_transport_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent_u32(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent_f64(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "droppedFrames")]
    pub fn get_dropped_frames(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "droppedFrames")]
    pub fn set_dropped_frames(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framesEncoded")]
    pub fn get_frames_encoded(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "framesEncoded")]
    pub fn set_frames_encoded(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "packetsSent")]
    pub fn get_packets_sent(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[doc = "Change the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "packetsSent")]
    pub fn set_packets_sent(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "targetBitrate")]
    pub fn get_target_bitrate(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[doc = "Change the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, setter = "targetBitrate")]
    pub fn set_target_bitrate(this: &RtcOutboundRtpStreamStats, val: f64);
}
impl RtcOutboundRtpStreamStats {
    #[doc = "Construct a new `RtcOutboundRtpStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
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
    #[deprecated = "Use `set_bytes_sent()` instead."]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
    #[deprecated = "Use `set_dropped_frames()` instead."]
    pub fn dropped_frames(&mut self, val: u32) -> &mut Self {
        self.set_dropped_frames(val);
        self
    }
    #[deprecated = "Use `set_frames_encoded()` instead."]
    pub fn frames_encoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_encoded(val);
        self
    }
    #[deprecated = "Use `set_packets_sent()` instead."]
    pub fn packets_sent(&mut self, val: u32) -> &mut Self {
        self.set_packets_sent(val);
        self
    }
    #[deprecated = "Use `set_target_bitrate()` instead."]
    pub fn target_bitrate(&mut self, val: f64) -> &mut Self {
        self.set_target_bitrate(val);
        self
    }
}
impl Default for RtcOutboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
