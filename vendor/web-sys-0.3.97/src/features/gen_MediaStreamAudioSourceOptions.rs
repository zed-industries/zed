#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaStreamAudioSourceOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceOptions`*"]
    pub type MediaStreamAudioSourceOptions;
    #[cfg(feature = "MediaStream")]
    #[doc = "Get the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    #[wasm_bindgen(method, getter = "mediaStream")]
    pub fn get_media_stream(this: &MediaStreamAudioSourceOptions) -> MediaStream;
    #[cfg(feature = "MediaStream")]
    #[doc = "Change the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    #[wasm_bindgen(method, setter = "mediaStream")]
    pub fn set_media_stream(this: &MediaStreamAudioSourceOptions, val: &MediaStream);
}
impl MediaStreamAudioSourceOptions {
    #[cfg(feature = "MediaStream")]
    #[doc = "Construct a new `MediaStreamAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    pub fn new(media_stream: &MediaStream) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_media_stream(media_stream);
        ret
    }
    #[cfg(feature = "MediaStream")]
    #[deprecated = "Use `set_media_stream()` instead."]
    pub fn media_stream(&mut self, val: &MediaStream) -> &mut Self {
        self.set_media_stream(val);
        self
    }
}
