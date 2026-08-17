#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioTrack",
        typescript_type = "AudioTrack"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub type AudioTrack;
    #[wasm_bindgen(method, getter, js_class = "AudioTrack", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn id(this: &AudioTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "AudioTrack", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn kind(this: &AudioTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "AudioTrack", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn label(this: &AudioTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "AudioTrack", js_name = "language")]
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn language(this: &AudioTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "AudioTrack", js_name = "enabled")]
    #[doc = "Getter for the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn enabled(this: &AudioTrack) -> bool;
    #[wasm_bindgen(method, setter, js_class = "AudioTrack", js_name = "enabled")]
    #[doc = "Setter for the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn set_enabled(this: &AudioTrack, value: bool);
    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(method, getter, js_class = "AudioTrack", js_name = "sourceBuffer")]
    #[doc = "Getter for the `sourceBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/sourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`, `SourceBuffer`*"]
    pub fn source_buffer(this: &AudioTrack) -> Option<SourceBuffer>;
}
