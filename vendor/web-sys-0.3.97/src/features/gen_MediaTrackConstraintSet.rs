#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaTrackConstraintSet")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackConstraintSet` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub type MediaTrackConstraintSet;
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "autoGainControl")]
    pub fn get_auto_gain_control(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "autoGainControl")]
    pub fn set_auto_gain_control(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "autoGainControl")]
    pub fn set_auto_gain_control_bool(this: &MediaTrackConstraintSet, val: bool);
    #[cfg(feature = "ConstrainBooleanParameters")]
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "autoGainControl")]
    pub fn set_auto_gain_control_constrain_boolean_parameters(
        this: &MediaTrackConstraintSet,
        val: &ConstrainBooleanParameters,
    );
    #[doc = "Get the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "browserWindow")]
    pub fn get_browser_window(this: &MediaTrackConstraintSet) -> Option<f64>;
    #[doc = "Change the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "browserWindow")]
    pub fn set_browser_window(this: &MediaTrackConstraintSet, val: f64);
    #[doc = "Change the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "browserWindow")]
    pub fn set_browser_window_i32(this: &MediaTrackConstraintSet, val: i32);
    #[doc = "Change the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "browserWindow")]
    pub fn set_browser_window_f64(this: &MediaTrackConstraintSet, val: f64);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count_i32(this: &MediaTrackConstraintSet, val: i32);
    #[cfg(feature = "ConstrainLongRange")]
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count_constrain_long_range(
        this: &MediaTrackConstraintSet,
        val: &ConstrainLongRange,
    );
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id_str(this: &MediaTrackConstraintSet, val: &str);
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id_str_sequence(
        this: &MediaTrackConstraintSet,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "ConstrainDomStringParameters")]
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id_constrain_dom_string_parameters(
        this: &MediaTrackConstraintSet,
        val: &ConstrainDomStringParameters,
    );
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "echoCancellation")]
    pub fn get_echo_cancellation(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "echoCancellation")]
    pub fn set_echo_cancellation(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "echoCancellation")]
    pub fn set_echo_cancellation_bool(this: &MediaTrackConstraintSet, val: bool);
    #[cfg(feature = "ConstrainBooleanParameters")]
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "echoCancellation")]
    pub fn set_echo_cancellation_constrain_boolean_parameters(
        this: &MediaTrackConstraintSet,
        val: &ConstrainBooleanParameters,
    );
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "facingMode")]
    pub fn get_facing_mode(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "facingMode")]
    pub fn set_facing_mode(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "facingMode")]
    pub fn set_facing_mode_str(this: &MediaTrackConstraintSet, val: &str);
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "facingMode")]
    pub fn set_facing_mode_str_sequence(
        this: &MediaTrackConstraintSet,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "ConstrainDomStringParameters")]
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "facingMode")]
    pub fn set_facing_mode_constrain_dom_string_parameters(
        this: &MediaTrackConstraintSet,
        val: &ConstrainDomStringParameters,
    );
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "frameRate")]
    pub fn get_frame_rate(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "frameRate")]
    pub fn set_frame_rate(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "frameRate")]
    pub fn set_frame_rate_f64(this: &MediaTrackConstraintSet, val: f64);
    #[cfg(feature = "ConstrainDoubleRange")]
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "frameRate")]
    pub fn set_frame_rate_constrain_double_range(
        this: &MediaTrackConstraintSet,
        val: &ConstrainDoubleRange,
    );
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height_i32(this: &MediaTrackConstraintSet, val: i32);
    #[cfg(feature = "ConstrainLongRange")]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height_constrain_long_range(
        this: &MediaTrackConstraintSet,
        val: &ConstrainLongRange,
    );
    #[doc = "Get the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "mediaSource")]
    pub fn get_media_source(this: &MediaTrackConstraintSet) -> Option<::alloc::string::String>;
    #[doc = "Change the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "mediaSource")]
    pub fn set_media_source(this: &MediaTrackConstraintSet, val: &str);
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    pub fn get_noise_suppression(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    pub fn set_noise_suppression(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    pub fn set_noise_suppression_bool(this: &MediaTrackConstraintSet, val: bool);
    #[cfg(feature = "ConstrainBooleanParameters")]
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    pub fn set_noise_suppression_constrain_boolean_parameters(
        this: &MediaTrackConstraintSet,
        val: &ConstrainBooleanParameters,
    );
    #[doc = "Get the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "scrollWithPage")]
    pub fn get_scroll_with_page(this: &MediaTrackConstraintSet) -> Option<bool>;
    #[doc = "Change the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "scrollWithPage")]
    pub fn set_scroll_with_page(this: &MediaTrackConstraintSet, val: bool);
    #[doc = "Get the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportHeight")]
    pub fn get_viewport_height(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportHeight")]
    pub fn set_viewport_height(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportHeight")]
    pub fn set_viewport_height_i32(this: &MediaTrackConstraintSet, val: i32);
    #[cfg(feature = "ConstrainLongRange")]
    #[doc = "Change the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportHeight")]
    pub fn set_viewport_height_constrain_long_range(
        this: &MediaTrackConstraintSet,
        val: &ConstrainLongRange,
    );
    #[doc = "Get the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportOffsetX")]
    pub fn get_viewport_offset_x(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportOffsetX")]
    pub fn set_viewport_offset_x(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportOffsetX")]
    pub fn set_viewport_offset_x_i32(this: &MediaTrackConstraintSet, val: i32);
    #[cfg(feature = "ConstrainLongRange")]
    #[doc = "Change the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportOffsetX")]
    pub fn set_viewport_offset_x_constrain_long_range(
        this: &MediaTrackConstraintSet,
        val: &ConstrainLongRange,
    );
    #[doc = "Get the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportOffsetY")]
    pub fn get_viewport_offset_y(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportOffsetY")]
    pub fn set_viewport_offset_y(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportOffsetY")]
    pub fn set_viewport_offset_y_i32(this: &MediaTrackConstraintSet, val: i32);
    #[cfg(feature = "ConstrainLongRange")]
    #[doc = "Change the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportOffsetY")]
    pub fn set_viewport_offset_y_constrain_long_range(
        this: &MediaTrackConstraintSet,
        val: &ConstrainLongRange,
    );
    #[doc = "Get the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportWidth")]
    pub fn get_viewport_width(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportWidth")]
    pub fn set_viewport_width(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportWidth")]
    pub fn set_viewport_width_i32(this: &MediaTrackConstraintSet, val: i32);
    #[cfg(feature = "ConstrainLongRange")]
    #[doc = "Change the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "viewportWidth")]
    pub fn set_viewport_width_constrain_long_range(
        this: &MediaTrackConstraintSet,
        val: &ConstrainLongRange,
    );
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width_i32(this: &MediaTrackConstraintSet, val: i32);
    #[cfg(feature = "ConstrainLongRange")]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`, `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width_constrain_long_range(this: &MediaTrackConstraintSet, val: &ConstrainLongRange);
}
impl MediaTrackConstraintSet {
    #[doc = "Construct a new `MediaTrackConstraintSet`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_gain_control()` instead."]
    pub fn auto_gain_control(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_auto_gain_control(val);
        self
    }
    #[deprecated = "Use `set_browser_window()` instead."]
    pub fn browser_window(&mut self, val: f64) -> &mut Self {
        self.set_browser_window(val);
        self
    }
    #[deprecated = "Use `set_channel_count()` instead."]
    pub fn channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[deprecated = "Use `set_echo_cancellation()` instead."]
    pub fn echo_cancellation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_echo_cancellation(val);
        self
    }
    #[deprecated = "Use `set_facing_mode()` instead."]
    pub fn facing_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_facing_mode(val);
        self
    }
    #[deprecated = "Use `set_frame_rate()` instead."]
    pub fn frame_rate(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_frame_rate(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_media_source()` instead."]
    pub fn media_source(&mut self, val: &str) -> &mut Self {
        self.set_media_source(val);
        self
    }
    #[deprecated = "Use `set_noise_suppression()` instead."]
    pub fn noise_suppression(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_noise_suppression(val);
        self
    }
    #[deprecated = "Use `set_scroll_with_page()` instead."]
    pub fn scroll_with_page(&mut self, val: bool) -> &mut Self {
        self.set_scroll_with_page(val);
        self
    }
    #[deprecated = "Use `set_viewport_height()` instead."]
    pub fn viewport_height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_height(val);
        self
    }
    #[deprecated = "Use `set_viewport_offset_x()` instead."]
    pub fn viewport_offset_x(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_offset_x(val);
        self
    }
    #[deprecated = "Use `set_viewport_offset_y()` instead."]
    pub fn viewport_offset_y(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_offset_y(val);
        self
    }
    #[deprecated = "Use `set_viewport_width()` instead."]
    pub fn viewport_width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_width(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for MediaTrackConstraintSet {
    fn default() -> Self {
        Self::new()
    }
}
