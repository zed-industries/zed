#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "TextTrack",
        typescript_type = "TextTrack"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
    pub type TextTrack;
    #[cfg(feature = "TextTrackKind")]
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackKind`*"]
    pub fn kind(this: &TextTrack) -> TextTrackKind;
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn label(this: &TextTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "language")]
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn language(this: &TextTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn id(this: &TextTrack) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TextTrack",
        js_name = "inBandMetadataTrackDispatchType"
    )]
    #[doc = "Getter for the `inBandMetadataTrackDispatchType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/inBandMetadataTrackDispatchType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn in_band_metadata_track_dispatch_type(this: &TextTrack) -> ::alloc::string::String;
    #[cfg(feature = "TextTrackMode")]
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "mode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
    pub fn mode(this: &TextTrack) -> TextTrackMode;
    #[cfg(feature = "TextTrackMode")]
    #[wasm_bindgen(method, setter, js_class = "TextTrack", js_name = "mode")]
    #[doc = "Setter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
    pub fn set_mode(this: &TextTrack, value: TextTrackMode);
    #[cfg(feature = "TextTrackCueList")]
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "cues")]
    #[doc = "Getter for the `cues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/cues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
    pub fn cues(this: &TextTrack) -> Option<TextTrackCueList>;
    #[cfg(feature = "TextTrackCueList")]
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "activeCues")]
    #[doc = "Getter for the `activeCues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/activeCues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
    pub fn active_cues(this: &TextTrack) -> Option<TextTrackCueList>;
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "oncuechange")]
    #[doc = "Getter for the `oncuechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn oncuechange(this: &TextTrack) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TextTrack", js_name = "oncuechange")]
    #[doc = "Setter for the `oncuechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
    pub fn set_oncuechange(this: &TextTrack, value: Option<&::js_sys::Function>);
    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(method, getter, js_class = "TextTrack", js_name = "sourceBuffer")]
    #[doc = "Getter for the `sourceBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/sourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `TextTrack`*"]
    pub fn source_buffer(this: &TextTrack) -> Option<SourceBuffer>;
    #[cfg(feature = "VttCue")]
    #[wasm_bindgen(method, js_class = "TextTrack", js_name = "addCue")]
    #[doc = "The `addCue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/addCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
    pub fn add_cue(this: &TextTrack, cue: &VttCue);
    #[cfg(feature = "VttCue")]
    #[wasm_bindgen(catch, method, js_class = "TextTrack", js_name = "removeCue")]
    #[doc = "The `removeCue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/removeCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
    pub fn remove_cue(this: &TextTrack, cue: &VttCue) -> Result<(), JsValue>;
}
