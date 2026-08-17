#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaRecorderOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub type MediaRecorderOptions;
    #[cfg(feature = "BitrateMode")]
    #[doc = "Get the `audioBitrateMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BitrateMode`, `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, getter = "audioBitrateMode")]
    pub fn get_audio_bitrate_mode(this: &MediaRecorderOptions) -> Option<BitrateMode>;
    #[cfg(feature = "BitrateMode")]
    #[doc = "Change the `audioBitrateMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BitrateMode`, `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, setter = "audioBitrateMode")]
    pub fn set_audio_bitrate_mode(this: &MediaRecorderOptions, val: BitrateMode);
    #[doc = "Get the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, getter = "audioBitsPerSecond")]
    pub fn get_audio_bits_per_second(this: &MediaRecorderOptions) -> Option<u32>;
    #[doc = "Change the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, setter = "audioBitsPerSecond")]
    pub fn set_audio_bits_per_second(this: &MediaRecorderOptions, val: u32);
    #[doc = "Get the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, getter = "bitsPerSecond")]
    pub fn get_bits_per_second(this: &MediaRecorderOptions) -> Option<u32>;
    #[doc = "Change the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, setter = "bitsPerSecond")]
    pub fn set_bits_per_second(this: &MediaRecorderOptions, val: u32);
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &MediaRecorderOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &MediaRecorderOptions, val: &str);
    #[doc = "Get the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, getter = "videoBitsPerSecond")]
    pub fn get_video_bits_per_second(this: &MediaRecorderOptions) -> Option<u32>;
    #[doc = "Change the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, setter = "videoBitsPerSecond")]
    pub fn set_video_bits_per_second(this: &MediaRecorderOptions, val: u32);
    #[doc = "Get the `videoKeyFrameIntervalCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, getter = "videoKeyFrameIntervalCount")]
    pub fn get_video_key_frame_interval_count(this: &MediaRecorderOptions) -> Option<u32>;
    #[doc = "Change the `videoKeyFrameIntervalCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, setter = "videoKeyFrameIntervalCount")]
    pub fn set_video_key_frame_interval_count(this: &MediaRecorderOptions, val: u32);
    #[doc = "Get the `videoKeyFrameIntervalDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, getter = "videoKeyFrameIntervalDuration")]
    pub fn get_video_key_frame_interval_duration(this: &MediaRecorderOptions) -> Option<f64>;
    #[doc = "Change the `videoKeyFrameIntervalDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    #[wasm_bindgen(method, setter = "videoKeyFrameIntervalDuration")]
    pub fn set_video_key_frame_interval_duration(this: &MediaRecorderOptions, val: f64);
}
impl MediaRecorderOptions {
    #[doc = "Construct a new `MediaRecorderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "BitrateMode")]
    #[deprecated = "Use `set_audio_bitrate_mode()` instead."]
    pub fn audio_bitrate_mode(&mut self, val: BitrateMode) -> &mut Self {
        self.set_audio_bitrate_mode(val);
        self
    }
    #[deprecated = "Use `set_audio_bits_per_second()` instead."]
    pub fn audio_bits_per_second(&mut self, val: u32) -> &mut Self {
        self.set_audio_bits_per_second(val);
        self
    }
    #[deprecated = "Use `set_bits_per_second()` instead."]
    pub fn bits_per_second(&mut self, val: u32) -> &mut Self {
        self.set_bits_per_second(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[deprecated = "Use `set_video_bits_per_second()` instead."]
    pub fn video_bits_per_second(&mut self, val: u32) -> &mut Self {
        self.set_video_bits_per_second(val);
        self
    }
    #[deprecated = "Use `set_video_key_frame_interval_count()` instead."]
    pub fn video_key_frame_interval_count(&mut self, val: u32) -> &mut Self {
        self.set_video_key_frame_interval_count(val);
        self
    }
    #[deprecated = "Use `set_video_key_frame_interval_duration()` instead."]
    pub fn video_key_frame_interval_duration(&mut self, val: f64) -> &mut Self {
        self.set_video_key_frame_interval_duration(val);
        self
    }
}
impl Default for MediaRecorderOptions {
    fn default() -> Self {
        Self::new()
    }
}
