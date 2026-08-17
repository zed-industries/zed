#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AnimationTimeline",
        extends = "::js_sys::Object",
        js_name = "DocumentTimeline",
        typescript_type = "DocumentTimeline"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentTimeline` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimeline`*"]
    pub type DocumentTimeline;
    #[wasm_bindgen(catch, constructor, js_class = "DocumentTimeline")]
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimeline`*"]
    pub fn new() -> Result<DocumentTimeline, JsValue>;
    #[cfg(feature = "DocumentTimelineOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "DocumentTimeline")]
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimeline`, `DocumentTimelineOptions`*"]
    pub fn new_with_options(options: &DocumentTimelineOptions)
        -> Result<DocumentTimeline, JsValue>;
}
