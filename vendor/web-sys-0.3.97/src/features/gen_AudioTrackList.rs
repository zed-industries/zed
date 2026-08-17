#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "AudioTrackList",
        typescript_type = "AudioTrackList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioTrackList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub type AudioTrackList;
    #[wasm_bindgen(method, getter, js_class = "AudioTrackList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn length(this: &AudioTrackList) -> u32;
    #[wasm_bindgen(method, getter, js_class = "AudioTrackList", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn onchange(this: &AudioTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "AudioTrackList", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn set_onchange(this: &AudioTrackList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "AudioTrackList", js_name = "onaddtrack")]
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn onaddtrack(this: &AudioTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "AudioTrackList", js_name = "onaddtrack")]
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn set_onaddtrack(this: &AudioTrackList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "AudioTrackList", js_name = "onremovetrack")]
    #[doc = "Getter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn onremovetrack(this: &AudioTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "AudioTrackList", js_name = "onremovetrack")]
    #[doc = "Setter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn set_onremovetrack(this: &AudioTrackList, value: Option<&::js_sys::Function>);
    #[cfg(feature = "AudioTrack")]
    #[wasm_bindgen(method, js_class = "AudioTrackList", js_name = "getTrackById")]
    #[doc = "The `getTrackById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/getTrackById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`, `AudioTrackList`*"]
    pub fn get_track_by_id(this: &AudioTrackList, id: &str) -> Option<AudioTrack>;
    #[cfg(feature = "AudioTrack")]
    #[wasm_bindgen(method, js_class = "AudioTrackList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`, `AudioTrackList`*"]
    pub fn get(this: &AudioTrackList, index: u32) -> Option<AudioTrack>;
}
