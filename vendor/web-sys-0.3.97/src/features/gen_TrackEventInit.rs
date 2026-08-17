#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "TrackEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub type TrackEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &TrackEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &TrackEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &TrackEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &TrackEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &TrackEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &TrackEventInit, val: bool);
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "track")]
    pub fn get_track(this: &TrackEventInit) -> Option<::js_sys::Object>;
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, setter = "track")]
    pub fn set_track(this: &TrackEventInit, val: Option<&::js_sys::Object>);
    #[cfg(feature = "VideoTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`, `VideoTrack`*"]
    #[wasm_bindgen(method, setter = "track")]
    pub fn set_track_opt_video_track(this: &TrackEventInit, val: Option<&VideoTrack>);
    #[cfg(feature = "AudioTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`, `TrackEventInit`*"]
    #[wasm_bindgen(method, setter = "track")]
    pub fn set_track_opt_audio_track(this: &TrackEventInit, val: Option<&AudioTrack>);
    #[cfg(feature = "TextTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TrackEventInit`*"]
    #[wasm_bindgen(method, setter = "track")]
    pub fn set_track_opt_text_track(this: &TrackEventInit, val: Option<&TextTrack>);
}
impl TrackEventInit {
    #[doc = "Construct a new `TrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_track()` instead."]
    pub fn track(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_track(val);
        self
    }
}
impl Default for TrackEventInit {
    fn default() -> Self {
        Self::new()
    }
}
