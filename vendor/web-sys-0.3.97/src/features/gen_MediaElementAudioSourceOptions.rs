#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaElementAudioSourceOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaElementAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*"]
    pub type MediaElementAudioSourceOptions;
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Get the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    #[wasm_bindgen(method, getter = "mediaElement")]
    pub fn get_media_element(this: &MediaElementAudioSourceOptions) -> HtmlMediaElement;
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Change the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    #[wasm_bindgen(method, setter = "mediaElement")]
    pub fn set_media_element(this: &MediaElementAudioSourceOptions, val: &HtmlMediaElement);
}
impl MediaElementAudioSourceOptions {
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Construct a new `MediaElementAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    pub fn new(media_element: &HtmlMediaElement) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_media_element(media_element);
        ret
    }
    #[cfg(feature = "HtmlMediaElement")]
    #[deprecated = "Use `set_media_element()` instead."]
    pub fn media_element(&mut self, val: &HtmlMediaElement) -> &mut Self {
        self.set_media_element(val);
        self
    }
}
