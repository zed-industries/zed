#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PerformanceObserver",
        typescript_type = "PerformanceObserver"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub type PerformanceObserver;
    #[wasm_bindgen(catch, constructor, js_class = "PerformanceObserver")]
    #[doc = "The `new PerformanceObserver(..)` constructor, creating a new instance of `PerformanceObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/PerformanceObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub fn new(callback: &::js_sys::Function) -> Result<PerformanceObserver, JsValue>;
    #[wasm_bindgen(method, js_class = "PerformanceObserver")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub fn disconnect(this: &PerformanceObserver);
    #[cfg(feature = "PerformanceObserverInit")]
    #[wasm_bindgen(method, js_class = "PerformanceObserver")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`, `PerformanceObserverInit`*"]
    pub fn observe(this: &PerformanceObserver, options: &PerformanceObserverInit);
    #[wasm_bindgen(method, js_class = "PerformanceObserver", js_name = "takeRecords")]
    #[doc = "The `takeRecords()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/takeRecords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub fn take_records(this: &PerformanceObserver) -> ::js_sys::Array;
}
