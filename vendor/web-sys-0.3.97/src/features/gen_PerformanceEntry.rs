#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PerformanceEntry",
        typescript_type = "PerformanceEntry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
    pub type PerformanceEntry;
    #[wasm_bindgen(method, getter, js_class = "PerformanceEntry", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
    pub fn name(this: &PerformanceEntry) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PerformanceEntry", js_name = "entryType")]
    #[doc = "Getter for the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/entryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
    pub fn entry_type(this: &PerformanceEntry) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PerformanceEntry", js_name = "startTime")]
    #[doc = "Getter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
    pub fn start_time(this: &PerformanceEntry) -> f64;
    #[wasm_bindgen(method, getter, js_class = "PerformanceEntry", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
    pub fn duration(this: &PerformanceEntry) -> f64;
    #[wasm_bindgen(method, js_class = "PerformanceEntry", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
    pub fn to_json(this: &PerformanceEntry) -> ::js_sys::Object;
}
