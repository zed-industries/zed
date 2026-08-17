#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaStreamConstraints")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub type MediaStreamConstraints;
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, getter = "audio")]
    pub fn get_audio(this: &MediaStreamConstraints) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio_bool(this: &MediaStreamConstraints, val: bool);
    #[cfg(feature = "MediaTrackConstraints")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio_media_track_constraints(
        this: &MediaStreamConstraints,
        val: &MediaTrackConstraints,
    );
    #[doc = "Get the `fake` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, getter = "fake")]
    pub fn get_fake(this: &MediaStreamConstraints) -> Option<bool>;
    #[doc = "Change the `fake` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, setter = "fake")]
    pub fn set_fake(this: &MediaStreamConstraints, val: bool);
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, getter = "peerIdentity")]
    pub fn get_peer_identity(this: &MediaStreamConstraints) -> Option<::alloc::string::String>;
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, setter = "peerIdentity")]
    pub fn set_peer_identity(this: &MediaStreamConstraints, val: Option<&str>);
    #[doc = "Get the `picture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, getter = "picture")]
    pub fn get_picture(this: &MediaStreamConstraints) -> Option<bool>;
    #[doc = "Change the `picture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, setter = "picture")]
    pub fn set_picture(this: &MediaStreamConstraints, val: bool);
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, getter = "video")]
    pub fn get_video(this: &MediaStreamConstraints) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video_bool(this: &MediaStreamConstraints, val: bool);
    #[cfg(feature = "MediaTrackConstraints")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`, `MediaTrackConstraints`*"]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video_media_track_constraints(
        this: &MediaStreamConstraints,
        val: &MediaTrackConstraints,
    );
}
impl MediaStreamConstraints {
    #[doc = "Construct a new `MediaStreamConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_audio()` instead."]
    pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_audio(val);
        self
    }
    #[deprecated = "Use `set_fake()` instead."]
    pub fn fake(&mut self, val: bool) -> &mut Self {
        self.set_fake(val);
        self
    }
    #[deprecated = "Use `set_peer_identity()` instead."]
    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {
        self.set_peer_identity(val);
        self
    }
    #[deprecated = "Use `set_picture()` instead."]
    pub fn picture(&mut self, val: bool) -> &mut Self {
        self.set_picture(val);
        self
    }
    #[deprecated = "Use `set_video()` instead."]
    pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_video(val);
        self
    }
}
impl Default for MediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}
