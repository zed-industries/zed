#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCMediaStreamTrackStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcMediaStreamTrackStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub type RtcMediaStreamTrackStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcMediaStreamTrackStats) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcMediaStreamTrackStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcMediaStreamTrackStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcMediaStreamTrackStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RtcMediaStreamTrackStats, val: RtcStatsType);
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "audioLevel")]
    pub fn get_audio_level(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "audioLevel")]
    pub fn set_audio_level(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "echoReturnLoss")]
    pub fn get_echo_return_loss(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[doc = "Change the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "echoReturnLoss")]
    pub fn set_echo_return_loss(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "echoReturnLossEnhancement")]
    pub fn get_echo_return_loss_enhancement(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[doc = "Change the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "echoReturnLossEnhancement")]
    pub fn set_echo_return_loss_enhancement(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "frameHeight")]
    pub fn get_frame_height(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[doc = "Change the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "frameHeight")]
    pub fn set_frame_height(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "frameWidth")]
    pub fn get_frame_width(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[doc = "Change the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "frameWidth")]
    pub fn set_frame_width(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesCorrupted")]
    pub fn get_frames_corrupted(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[doc = "Change the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "framesCorrupted")]
    pub fn set_frames_corrupted(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesDecoded")]
    pub fn get_frames_decoded(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "framesDecoded")]
    pub fn set_frames_decoded(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesDropped")]
    pub fn get_frames_dropped(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[doc = "Change the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "framesDropped")]
    pub fn set_frames_dropped(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesPerSecond")]
    pub fn get_frames_per_second(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[doc = "Change the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "framesPerSecond")]
    pub fn set_frames_per_second(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesReceived")]
    pub fn get_frames_received(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[doc = "Change the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "framesReceived")]
    pub fn set_frames_received(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesSent")]
    pub fn get_frames_sent(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[doc = "Change the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "framesSent")]
    pub fn set_frames_sent(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "remoteSource")]
    pub fn get_remote_source(this: &RtcMediaStreamTrackStats) -> Option<bool>;
    #[doc = "Change the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "remoteSource")]
    pub fn set_remote_source(this: &RtcMediaStreamTrackStats, val: bool);
    #[doc = "Get the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "ssrcIds")]
    pub fn get_ssrc_ids(this: &RtcMediaStreamTrackStats) -> Option<::js_sys::Array>;
    #[doc = "Change the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "ssrcIds")]
    pub fn set_ssrc_ids(this: &RtcMediaStreamTrackStats, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "trackIdentifier")]
    pub fn get_track_identifier(this: &RtcMediaStreamTrackStats)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, setter = "trackIdentifier")]
    pub fn set_track_identifier(this: &RtcMediaStreamTrackStats, val: &str);
}
impl RtcMediaStreamTrackStats {
    #[doc = "Construct a new `RtcMediaStreamTrackStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
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
    #[deprecated = "Use `set_audio_level()` instead."]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level(val);
        self
    }
    #[deprecated = "Use `set_echo_return_loss()` instead."]
    pub fn echo_return_loss(&mut self, val: f64) -> &mut Self {
        self.set_echo_return_loss(val);
        self
    }
    #[deprecated = "Use `set_echo_return_loss_enhancement()` instead."]
    pub fn echo_return_loss_enhancement(&mut self, val: f64) -> &mut Self {
        self.set_echo_return_loss_enhancement(val);
        self
    }
    #[deprecated = "Use `set_frame_height()` instead."]
    pub fn frame_height(&mut self, val: u32) -> &mut Self {
        self.set_frame_height(val);
        self
    }
    #[deprecated = "Use `set_frame_width()` instead."]
    pub fn frame_width(&mut self, val: u32) -> &mut Self {
        self.set_frame_width(val);
        self
    }
    #[deprecated = "Use `set_frames_corrupted()` instead."]
    pub fn frames_corrupted(&mut self, val: u32) -> &mut Self {
        self.set_frames_corrupted(val);
        self
    }
    #[deprecated = "Use `set_frames_decoded()` instead."]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_decoded(val);
        self
    }
    #[deprecated = "Use `set_frames_dropped()` instead."]
    pub fn frames_dropped(&mut self, val: u32) -> &mut Self {
        self.set_frames_dropped(val);
        self
    }
    #[deprecated = "Use `set_frames_per_second()` instead."]
    pub fn frames_per_second(&mut self, val: f64) -> &mut Self {
        self.set_frames_per_second(val);
        self
    }
    #[deprecated = "Use `set_frames_received()` instead."]
    pub fn frames_received(&mut self, val: u32) -> &mut Self {
        self.set_frames_received(val);
        self
    }
    #[deprecated = "Use `set_frames_sent()` instead."]
    pub fn frames_sent(&mut self, val: u32) -> &mut Self {
        self.set_frames_sent(val);
        self
    }
    #[deprecated = "Use `set_remote_source()` instead."]
    pub fn remote_source(&mut self, val: bool) -> &mut Self {
        self.set_remote_source(val);
        self
    }
    #[deprecated = "Use `set_ssrc_ids()` instead."]
    pub fn ssrc_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ssrc_ids(val);
        self
    }
    #[deprecated = "Use `set_track_identifier()` instead."]
    pub fn track_identifier(&mut self, val: &str) -> &mut Self {
        self.set_track_identifier(val);
        self
    }
}
impl Default for RtcMediaStreamTrackStats {
    fn default() -> Self {
        Self::new()
    }
}
