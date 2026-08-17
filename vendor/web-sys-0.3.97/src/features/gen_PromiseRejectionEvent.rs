#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PromiseRejectionEvent",
        typescript_type = "PromiseRejectionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PromiseRejectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
    pub type PromiseRejectionEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PromiseRejectionEvent",
        js_name = "promise"
    )]
    #[doc = "Getter for the `promise` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/promise)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
    pub fn promise(this: &PromiseRejectionEvent) -> ::js_sys::Promise;
    #[wasm_bindgen(method, getter, js_class = "PromiseRejectionEvent", js_name = "reason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
    pub fn reason(this: &PromiseRejectionEvent) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "PromiseRejectionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PromiseRejectionEvent")]
    #[doc = "The `new PromiseRejectionEvent(..)` constructor, creating a new instance of `PromiseRejectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/PromiseRejectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`, `PromiseRejectionEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &PromiseRejectionEventInit,
    ) -> Result<PromiseRejectionEvent, JsValue>;
}
