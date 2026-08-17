#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TimeRanges",
        typescript_type = "TimeRanges"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TimeRanges` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub type TimeRanges;
    #[wasm_bindgen(method, getter, js_class = "TimeRanges", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub fn length(this: &TimeRanges) -> u32;
    #[wasm_bindgen(catch, method, js_class = "TimeRanges")]
    #[doc = "The `end()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/end)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub fn end(this: &TimeRanges, index: u32) -> Result<f64, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TimeRanges")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub fn start(this: &TimeRanges, index: u32) -> Result<f64, JsValue>;
}
