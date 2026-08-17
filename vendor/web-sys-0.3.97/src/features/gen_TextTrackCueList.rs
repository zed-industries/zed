#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TextTrackCueList",
        typescript_type = "TextTrackCueList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackCueList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`*"]
    pub type TextTrackCueList;
    #[wasm_bindgen(method, getter, js_class = "TextTrackCueList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`*"]
    pub fn length(this: &TextTrackCueList) -> u32;
    #[cfg(feature = "VttCue")]
    #[wasm_bindgen(method, js_class = "TextTrackCueList", js_name = "getCueById")]
    #[doc = "The `getCueById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/getCueById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*"]
    pub fn get_cue_by_id(this: &TextTrackCueList, id: &str) -> Option<VttCue>;
    #[cfg(feature = "VttCue")]
    #[wasm_bindgen(method, js_class = "TextTrackCueList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*"]
    pub fn get(this: &TextTrackCueList, index: u32) -> Option<VttCue>;
}
