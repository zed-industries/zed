#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaTrackCapabilities")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackCapabilities` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaTrackCapabilities;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "aspectRatio")]
    pub fn get_aspect_ratio(this: &MediaTrackCapabilities) -> Option<DoubleRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "aspectRatio")]
    pub fn set_aspect_ratio(this: &MediaTrackCapabilities, val: &DoubleRange);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "autoGainControl")]
    pub fn get_auto_gain_control(
        this: &MediaTrackCapabilities,
    ) -> Option<::js_sys::Array<::js_sys::Boolean>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "autoGainControl")]
    pub fn set_auto_gain_control(this: &MediaTrackCapabilities, val: &[::js_sys::Boolean]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `backgroundBlur` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "backgroundBlur")]
    pub fn get_background_blur(
        this: &MediaTrackCapabilities,
    ) -> Option<::js_sys::Array<::js_sys::Boolean>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `backgroundBlur` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "backgroundBlur")]
    pub fn set_background_blur(this: &MediaTrackCapabilities, val: &[::js_sys::Boolean]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &MediaTrackCapabilities) -> Option<ULongRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &MediaTrackCapabilities, val: &ULongRange);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &MediaTrackCapabilities) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &MediaTrackCapabilities, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "echoCancellation")]
    pub fn get_echo_cancellation(
        this: &MediaTrackCapabilities,
    ) -> Option<::js_sys::Array<::js_sys::Boolean>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "echoCancellation")]
    pub fn set_echo_cancellation(this: &MediaTrackCapabilities, val: &[::js_sys::Boolean]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "facingMode")]
    pub fn get_facing_mode(
        this: &MediaTrackCapabilities,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "facingMode")]
    pub fn set_facing_mode(this: &MediaTrackCapabilities, val: &[::js_sys::JsString]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "frameRate")]
    pub fn get_frame_rate(this: &MediaTrackCapabilities) -> Option<DoubleRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "frameRate")]
    pub fn set_frame_rate(this: &MediaTrackCapabilities, val: &DoubleRange);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "groupId")]
    pub fn get_group_id(this: &MediaTrackCapabilities) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "groupId")]
    pub fn set_group_id(this: &MediaTrackCapabilities, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &MediaTrackCapabilities) -> Option<ULongRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &MediaTrackCapabilities, val: &ULongRange);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "latency")]
    pub fn get_latency(this: &MediaTrackCapabilities) -> Option<DoubleRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "latency")]
    pub fn set_latency(this: &MediaTrackCapabilities, val: &DoubleRange);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    pub fn get_noise_suppression(
        this: &MediaTrackCapabilities,
    ) -> Option<::js_sys::Array<::js_sys::Boolean>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    pub fn set_noise_suppression(this: &MediaTrackCapabilities, val: &[::js_sys::Boolean]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `resizeMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "resizeMode")]
    pub fn get_resize_mode(
        this: &MediaTrackCapabilities,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `resizeMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "resizeMode")]
    pub fn set_resize_mode(this: &MediaTrackCapabilities, val: &[::js_sys::JsString]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "sampleRate")]
    pub fn get_sample_rate(this: &MediaTrackCapabilities) -> Option<ULongRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "sampleRate")]
    pub fn set_sample_rate(this: &MediaTrackCapabilities, val: &ULongRange);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "sampleSize")]
    pub fn get_sample_size(this: &MediaTrackCapabilities) -> Option<ULongRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "sampleSize")]
    pub fn set_sample_size(this: &MediaTrackCapabilities, val: &ULongRange);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &MediaTrackCapabilities) -> Option<ULongRange>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &MediaTrackCapabilities, val: &ULongRange);
}
#[cfg(web_sys_unstable_apis)]
impl MediaTrackCapabilities {
    #[doc = "Construct a new `MediaTrackCapabilities`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[deprecated = "Use `set_aspect_ratio()` instead."]
    pub fn aspect_ratio(&mut self, val: &DoubleRange) -> &mut Self {
        self.set_aspect_ratio(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_auto_gain_control()` instead."]
    pub fn auto_gain_control(&mut self, val: &[::js_sys::Boolean]) -> &mut Self {
        self.set_auto_gain_control(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_background_blur()` instead."]
    pub fn background_blur(&mut self, val: &[::js_sys::Boolean]) -> &mut Self {
        self.set_background_blur(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_channel_count()` instead."]
    pub fn channel_count(&mut self, val: &ULongRange) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_echo_cancellation()` instead."]
    pub fn echo_cancellation(&mut self, val: &[::js_sys::Boolean]) -> &mut Self {
        self.set_echo_cancellation(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_facing_mode()` instead."]
    pub fn facing_mode(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_facing_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[deprecated = "Use `set_frame_rate()` instead."]
    pub fn frame_rate(&mut self, val: &DoubleRange) -> &mut Self {
        self.set_frame_rate(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_group_id()` instead."]
    pub fn group_id(&mut self, val: &str) -> &mut Self {
        self.set_group_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: &ULongRange) -> &mut Self {
        self.set_height(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[deprecated = "Use `set_latency()` instead."]
    pub fn latency(&mut self, val: &DoubleRange) -> &mut Self {
        self.set_latency(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_noise_suppression()` instead."]
    pub fn noise_suppression(&mut self, val: &[::js_sys::Boolean]) -> &mut Self {
        self.set_noise_suppression(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_resize_mode()` instead."]
    pub fn resize_mode(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_resize_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_sample_rate()` instead."]
    pub fn sample_rate(&mut self, val: &ULongRange) -> &mut Self {
        self.set_sample_rate(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_sample_size()` instead."]
    pub fn sample_size(&mut self, val: &ULongRange) -> &mut Self {
        self.set_sample_size(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: &ULongRange) -> &mut Self {
        self.set_width(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MediaTrackCapabilities {
    fn default() -> Self {
        Self::new()
    }
}
