#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaConfiguration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"]
    pub type MediaConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaConfiguration`*"]
    #[wasm_bindgen(method, getter = "audio")]
    pub fn get_audio(this: &MediaConfiguration) -> Option<AudioConfiguration>;
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaConfiguration`*"]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio(this: &MediaConfiguration, val: &AudioConfiguration);
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`, `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "video")]
    pub fn get_video(this: &MediaConfiguration) -> Option<VideoConfiguration>;
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`, `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video(this: &MediaConfiguration, val: &VideoConfiguration);
}
impl MediaConfiguration {
    #[doc = "Construct a new `MediaConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "AudioConfiguration")]
    #[deprecated = "Use `set_audio()` instead."]
    pub fn audio(&mut self, val: &AudioConfiguration) -> &mut Self {
        self.set_audio(val);
        self
    }
    #[cfg(feature = "VideoConfiguration")]
    #[deprecated = "Use `set_video()` instead."]
    pub fn video(&mut self, val: &VideoConfiguration) -> &mut Self {
        self.set_video(val);
        self
    }
}
impl Default for MediaConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
