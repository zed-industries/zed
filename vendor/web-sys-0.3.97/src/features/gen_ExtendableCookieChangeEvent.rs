#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "ExtendableEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "ExtendableCookieChangeEvent",
        typescript_type = "ExtendableCookieChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableCookieChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEvent`*"]
    pub type ExtendableCookieChangeEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ExtendableCookieChangeEvent",
        js_name = "changed"
    )]
    #[doc = "Getter for the `changed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent/changed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEvent`*"]
    pub fn changed(this: &ExtendableCookieChangeEvent) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ExtendableCookieChangeEvent",
        js_name = "deleted"
    )]
    #[doc = "Getter for the `deleted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent/deleted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEvent`*"]
    pub fn deleted(this: &ExtendableCookieChangeEvent) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "ExtendableCookieChangeEvent")]
    #[doc = "The `new ExtendableCookieChangeEvent(..)` constructor, creating a new instance of `ExtendableCookieChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent/ExtendableCookieChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEvent`*"]
    pub fn new(type_: &str) -> Result<ExtendableCookieChangeEvent, JsValue>;
    #[cfg(feature = "ExtendableCookieChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ExtendableCookieChangeEvent")]
    #[doc = "The `new ExtendableCookieChangeEvent(..)` constructor, creating a new instance of `ExtendableCookieChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent/ExtendableCookieChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEvent`, `ExtendableCookieChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ExtendableCookieChangeEventInit,
    ) -> Result<ExtendableCookieChangeEvent, JsValue>;
}
