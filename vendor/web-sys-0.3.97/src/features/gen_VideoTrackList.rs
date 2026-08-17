#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "VideoTrackList",
        typescript_type = "VideoTrackList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoTrackList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub type VideoTrackList;
    #[wasm_bindgen(method, getter, js_class = "VideoTrackList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn length(this: &VideoTrackList) -> u32;
    #[wasm_bindgen(method, getter, js_class = "VideoTrackList", js_name = "selectedIndex")]
    #[doc = "Getter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn selected_index(this: &VideoTrackList) -> i32;
    #[wasm_bindgen(method, getter, js_class = "VideoTrackList", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn onchange(this: &VideoTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "VideoTrackList", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn set_onchange(this: &VideoTrackList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "VideoTrackList", js_name = "onaddtrack")]
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn onaddtrack(this: &VideoTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "VideoTrackList", js_name = "onaddtrack")]
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn set_onaddtrack(this: &VideoTrackList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "VideoTrackList", js_name = "onremovetrack")]
    #[doc = "Getter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn onremovetrack(this: &VideoTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "VideoTrackList", js_name = "onremovetrack")]
    #[doc = "Setter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrackList`*"]
    pub fn set_onremovetrack(this: &VideoTrackList, value: Option<&::js_sys::Function>);
    #[cfg(feature = "VideoTrack")]
    #[wasm_bindgen(method, js_class = "VideoTrackList", js_name = "getTrackById")]
    #[doc = "The `getTrackById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/getTrackById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`, `VideoTrackList`*"]
    pub fn get_track_by_id(this: &VideoTrackList, id: &str) -> Option<VideoTrack>;
    #[cfg(feature = "VideoTrack")]
    #[wasm_bindgen(method, js_class = "VideoTrackList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`, `VideoTrackList`*"]
    pub fn get(this: &VideoTrackList, index: u32) -> Option<VideoTrack>;
}
