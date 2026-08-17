#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaDecodingConfiguration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDecodingConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`*"]
    pub type MediaDecodingConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaDecodingConfiguration`*"]
    #[wasm_bindgen(method, getter = "audio")]
    pub fn get_audio(this: &MediaDecodingConfiguration) -> Option<AudioConfiguration>;
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaDecodingConfiguration`*"]
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio(this: &MediaDecodingConfiguration, val: &AudioConfiguration);
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "video")]
    pub fn get_video(this: &MediaDecodingConfiguration) -> Option<VideoConfiguration>;
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video(this: &MediaDecodingConfiguration, val: &VideoConfiguration);
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &MediaDecodingConfiguration) -> MediaDecodingType;
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &MediaDecodingConfiguration, val: MediaDecodingType);
}
impl MediaDecodingConfiguration {
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Construct a new `MediaDecodingConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
    pub fn new(type_: MediaDecodingType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_type(type_);
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
    #[cfg(feature = "MediaDecodingType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: MediaDecodingType) -> &mut Self {
        self.set_type(val);
        self
    }
}
