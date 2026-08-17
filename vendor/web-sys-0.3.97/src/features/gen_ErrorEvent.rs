#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "ErrorEvent",
        typescript_type = "ErrorEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
    pub type ErrorEvent;
    #[wasm_bindgen(method, getter, js_class = "ErrorEvent", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
    pub fn message(this: &ErrorEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "ErrorEvent", js_name = "filename")]
    #[doc = "Getter for the `filename` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/filename)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
    pub fn filename(this: &ErrorEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "ErrorEvent", js_name = "lineno")]
    #[doc = "Getter for the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/lineno)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
    pub fn lineno(this: &ErrorEvent) -> u32;
    #[wasm_bindgen(method, getter, js_class = "ErrorEvent", js_name = "colno")]
    #[doc = "Getter for the `colno` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/colno)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
    pub fn colno(this: &ErrorEvent) -> u32;
    #[wasm_bindgen(method, getter, js_class = "ErrorEvent", js_name = "error")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
    pub fn error(this: &ErrorEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, constructor, js_class = "ErrorEvent")]
    #[doc = "The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
    pub fn new(type_: &str) -> Result<ErrorEvent, JsValue>;
    #[cfg(feature = "ErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ErrorEvent")]
    #[doc = "The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`, `ErrorEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ErrorEventInit,
    ) -> Result<ErrorEvent, JsValue>;
}
