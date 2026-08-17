#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IntersectionObserver",
        typescript_type = "IntersectionObserver"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub type IntersectionObserver;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "IntersectionObserver", js_name = "root")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    pub fn root(this: &IntersectionObserver) -> Option<Element>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserver",
        js_name = "rootMargin"
    )]
    #[doc = "Getter for the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/rootMargin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn root_margin(this: &IntersectionObserver) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserver",
        js_name = "thresholds"
    )]
    #[doc = "Getter for the `thresholds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/thresholds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn thresholds(this: &IntersectionObserver) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "IntersectionObserver")]
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn new(intersection_callback: &::js_sys::Function)
        -> Result<IntersectionObserver, JsValue>;
    #[cfg(feature = "IntersectionObserverInit")]
    #[wasm_bindgen(catch, constructor, js_class = "IntersectionObserver")]
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`, `IntersectionObserverInit`*"]
    pub fn new_with_options(
        intersection_callback: &::js_sys::Function,
        options: &IntersectionObserverInit,
    ) -> Result<IntersectionObserver, JsValue>;
    #[wasm_bindgen(method, js_class = "IntersectionObserver")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn disconnect(this: &IntersectionObserver);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "IntersectionObserver")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    pub fn observe(this: &IntersectionObserver, target: &Element);
    #[wasm_bindgen(method, js_class = "IntersectionObserver", js_name = "takeRecords")]
    #[doc = "The `takeRecords()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/takeRecords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn take_records(this: &IntersectionObserver) -> ::js_sys::Array;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "IntersectionObserver")]
    #[doc = "The `unobserve()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/unobserve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    pub fn unobserve(this: &IntersectionObserver, target: &Element);
}
