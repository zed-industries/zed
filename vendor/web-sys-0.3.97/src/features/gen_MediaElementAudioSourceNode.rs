#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MediaElementAudioSourceNode",
        typescript_type = "MediaElementAudioSourceNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaElementAudioSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceNode`*"]
    pub type MediaElementAudioSourceNode;
    #[cfg(all(feature = "AudioContext", feature = "MediaElementAudioSourceOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaElementAudioSourceNode")]
    #[doc = "The `new MediaElementAudioSourceNode(..)` constructor, creating a new instance of `MediaElementAudioSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode/MediaElementAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `MediaElementAudioSourceNode`, `MediaElementAudioSourceOptions`*"]
    pub fn new(
        context: &AudioContext,
        options: &MediaElementAudioSourceOptions,
    ) -> Result<MediaElementAudioSourceNode, JsValue>;
}
