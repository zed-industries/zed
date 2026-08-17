#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VideoTrack",
        typescript_type = "VideoTrack"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub type VideoTrack;
    #[wasm_bindgen(method, getter, js_class = "VideoTrack", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn id(this: &VideoTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "VideoTrack", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn kind(this: &VideoTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "VideoTrack", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn label(this: &VideoTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "VideoTrack", js_name = "language")]
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn language(this: &VideoTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "VideoTrack", js_name = "selected")]
    #[doc = "Getter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn selected(this: &VideoTrack) -> bool;
    #[wasm_bindgen(method, setter, js_class = "VideoTrack", js_name = "selected")]
    #[doc = "Setter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
    pub fn set_selected(this: &VideoTrack, value: bool);
    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(method, getter, js_class = "VideoTrack", js_name = "sourceBuffer")]
    #[doc = "Getter for the `sourceBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/sourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `VideoTrack`*"]
    pub fn source_buffer(this: &VideoTrack) -> Option<SourceBuffer>;
}
