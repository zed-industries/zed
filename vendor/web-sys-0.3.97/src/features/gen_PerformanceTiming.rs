#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PerformanceTiming",
        typescript_type = "PerformanceTiming"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceTiming` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub type PerformanceTiming;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "navigationStart"
    )]
    #[doc = "Getter for the `navigationStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/navigationStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn navigation_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "unloadEventStart"
    )]
    #[doc = "Getter for the `unloadEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn unload_event_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "unloadEventEnd"
    )]
    #[doc = "Getter for the `unloadEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn unload_event_end(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "redirectStart"
    )]
    #[doc = "Getter for the `redirectStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn redirect_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "redirectEnd"
    )]
    #[doc = "Getter for the `redirectEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn redirect_end(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(method, getter, js_class = "PerformanceTiming", js_name = "fetchStart")]
    #[doc = "Getter for the `fetchStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/fetchStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn fetch_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "domainLookupStart"
    )]
    #[doc = "Getter for the `domainLookupStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn domain_lookup_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "domainLookupEnd"
    )]
    #[doc = "Getter for the `domainLookupEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn domain_lookup_end(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "connectStart"
    )]
    #[doc = "Getter for the `connectStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn connect_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(method, getter, js_class = "PerformanceTiming", js_name = "connectEnd")]
    #[doc = "Getter for the `connectEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn connect_end(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "secureConnectionStart"
    )]
    #[doc = "Getter for the `secureConnectionStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/secureConnectionStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn secure_connection_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "requestStart"
    )]
    #[doc = "Getter for the `requestStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/requestStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn request_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "responseStart"
    )]
    #[doc = "Getter for the `responseStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn response_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "responseEnd"
    )]
    #[doc = "Getter for the `responseEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn response_end(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(method, getter, js_class = "PerformanceTiming", js_name = "domLoading")]
    #[doc = "Getter for the `domLoading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domLoading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_loading(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "domInteractive"
    )]
    #[doc = "Getter for the `domInteractive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domInteractive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_interactive(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "domContentLoadedEventStart"
    )]
    #[doc = "Getter for the `domContentLoadedEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_content_loaded_event_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "domContentLoadedEventEnd"
    )]
    #[doc = "Getter for the `domContentLoadedEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_content_loaded_event_end(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "domComplete"
    )]
    #[doc = "Getter for the `domComplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domComplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_complete(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "loadEventStart"
    )]
    #[doc = "Getter for the `loadEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn load_event_start(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "loadEventEnd"
    )]
    #[doc = "Getter for the `loadEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn load_event_end(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "timeToNonBlankPaint"
    )]
    #[doc = "Getter for the `timeToNonBlankPaint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToNonBlankPaint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn time_to_non_blank_paint(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceTiming",
        js_name = "timeToDOMContentFlushed"
    )]
    #[doc = "Getter for the `timeToDOMContentFlushed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToDOMContentFlushed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn time_to_dom_content_flushed(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(method, js_class = "PerformanceTiming", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn to_json(this: &PerformanceTiming) -> ::js_sys::Object;
}
