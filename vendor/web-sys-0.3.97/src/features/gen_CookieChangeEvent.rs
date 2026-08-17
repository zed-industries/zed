#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "CookieChangeEvent",
        typescript_type = "CookieChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CookieChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieChangeEvent`*"]
    pub type CookieChangeEvent;
    #[wasm_bindgen(method, getter, js_class = "CookieChangeEvent", js_name = "changed")]
    #[doc = "Getter for the `changed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent/changed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieChangeEvent`*"]
    pub fn changed(this: &CookieChangeEvent) -> ::js_sys::Array;
    #[wasm_bindgen(method, getter, js_class = "CookieChangeEvent", js_name = "deleted")]
    #[doc = "Getter for the `deleted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent/deleted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieChangeEvent`*"]
    pub fn deleted(this: &CookieChangeEvent) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "CookieChangeEvent")]
    #[doc = "The `new CookieChangeEvent(..)` constructor, creating a new instance of `CookieChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent/CookieChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieChangeEvent`*"]
    pub fn new(type_: &str) -> Result<CookieChangeEvent, JsValue>;
    #[cfg(feature = "CookieChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CookieChangeEvent")]
    #[doc = "The `new CookieChangeEvent(..)` constructor, creating a new instance of `CookieChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent/CookieChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieChangeEvent`, `CookieChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CookieChangeEventInit,
    ) -> Result<CookieChangeEvent, JsValue>;
}
