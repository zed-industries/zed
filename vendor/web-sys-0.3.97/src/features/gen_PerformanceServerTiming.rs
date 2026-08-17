#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PerformanceServerTiming",
        typescript_type = "PerformanceServerTiming"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceServerTiming` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub type PerformanceServerTiming;
    #[wasm_bindgen(method, getter, js_class = "PerformanceServerTiming", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn name(this: &PerformanceServerTiming) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceServerTiming",
        js_name = "duration"
    )]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn duration(this: &PerformanceServerTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceServerTiming",
        js_name = "description"
    )]
    #[doc = "Getter for the `description` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/description)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn description(this: &PerformanceServerTiming) -> ::alloc::string::String;
    #[wasm_bindgen(method, js_class = "PerformanceServerTiming", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn to_json(this: &PerformanceServerTiming) -> ::js_sys::Object;
}
