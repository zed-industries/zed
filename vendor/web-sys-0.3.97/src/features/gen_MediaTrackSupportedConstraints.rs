#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaTrackSupportedConstraints"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackSupportedConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub type MediaTrackSupportedConstraints;
    #[doc = "Get the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "aspectRatio")]
    pub fn get_aspect_ratio(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "aspectRatio")]
    pub fn set_aspect_ratio(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "autoGainControl")]
    pub fn get_auto_gain_control(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "autoGainControl")]
    pub fn set_auto_gain_control(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "echoCancellation")]
    pub fn get_echo_cancellation(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "echoCancellation")]
    pub fn set_echo_cancellation(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "facingMode")]
    pub fn get_facing_mode(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "facingMode")]
    pub fn set_facing_mode(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "frameRate")]
    pub fn get_frame_rate(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "frameRate")]
    pub fn set_frame_rate(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "groupId")]
    pub fn get_group_id(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "groupId")]
    pub fn set_group_id(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "latency")]
    pub fn get_latency(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "latency")]
    pub fn set_latency(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    pub fn get_noise_suppression(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    pub fn set_noise_suppression(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "sampleRate")]
    pub fn get_sample_rate(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "sampleRate")]
    pub fn set_sample_rate(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "sampleSize")]
    pub fn get_sample_size(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "sampleSize")]
    pub fn set_sample_size(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `volume` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "volume")]
    pub fn get_volume(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `volume` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "volume")]
    pub fn set_volume(this: &MediaTrackSupportedConstraints, val: bool);
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &MediaTrackSupportedConstraints) -> Option<bool>;
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &MediaTrackSupportedConstraints, val: bool);
}
impl MediaTrackSupportedConstraints {
    #[doc = "Construct a new `MediaTrackSupportedConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_aspect_ratio()` instead."]
    pub fn aspect_ratio(&mut self, val: bool) -> &mut Self {
        self.set_aspect_ratio(val);
        self
    }
    #[deprecated = "Use `set_auto_gain_control()` instead."]
    pub fn auto_gain_control(&mut self, val: bool) -> &mut Self {
        self.set_auto_gain_control(val);
        self
    }
    #[deprecated = "Use `set_channel_count()` instead."]
    pub fn channel_count(&mut self, val: bool) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: bool) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[deprecated = "Use `set_echo_cancellation()` instead."]
    pub fn echo_cancellation(&mut self, val: bool) -> &mut Self {
        self.set_echo_cancellation(val);
        self
    }
    #[deprecated = "Use `set_facing_mode()` instead."]
    pub fn facing_mode(&mut self, val: bool) -> &mut Self {
        self.set_facing_mode(val);
        self
    }
    #[deprecated = "Use `set_frame_rate()` instead."]
    pub fn frame_rate(&mut self, val: bool) -> &mut Self {
        self.set_frame_rate(val);
        self
    }
    #[deprecated = "Use `set_group_id()` instead."]
    pub fn group_id(&mut self, val: bool) -> &mut Self {
        self.set_group_id(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: bool) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_latency()` instead."]
    pub fn latency(&mut self, val: bool) -> &mut Self {
        self.set_latency(val);
        self
    }
    #[deprecated = "Use `set_noise_suppression()` instead."]
    pub fn noise_suppression(&mut self, val: bool) -> &mut Self {
        self.set_noise_suppression(val);
        self
    }
    #[deprecated = "Use `set_sample_rate()` instead."]
    pub fn sample_rate(&mut self, val: bool) -> &mut Self {
        self.set_sample_rate(val);
        self
    }
    #[deprecated = "Use `set_sample_size()` instead."]
    pub fn sample_size(&mut self, val: bool) -> &mut Self {
        self.set_sample_size(val);
        self
    }
    #[deprecated = "Use `set_volume()` instead."]
    pub fn volume(&mut self, val: bool) -> &mut Self {
        self.set_volume(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: bool) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for MediaTrackSupportedConstraints {
    fn default() -> Self {
        Self::new()
    }
}
