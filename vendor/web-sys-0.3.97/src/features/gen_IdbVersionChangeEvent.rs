#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "IDBVersionChangeEvent",
        typescript_type = "IDBVersionChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbVersionChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub type IdbVersionChangeEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IDBVersionChangeEvent",
        js_name = "oldVersion"
    )]
    #[doc = "Getter for the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/oldVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub fn old_version(this: &IdbVersionChangeEvent) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IDBVersionChangeEvent",
        js_name = "newVersion"
    )]
    #[doc = "Getter for the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/newVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub fn new_version(this: &IdbVersionChangeEvent) -> Option<f64>;
    #[wasm_bindgen(catch, constructor, js_class = "IDBVersionChangeEvent")]
    #[doc = "The `new IdbVersionChangeEvent(..)` constructor, creating a new instance of `IdbVersionChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub fn new(type_: &str) -> Result<IdbVersionChangeEvent, JsValue>;
    #[cfg(feature = "IdbVersionChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "IDBVersionChangeEvent")]
    #[doc = "The `new IdbVersionChangeEvent(..)` constructor, creating a new instance of `IdbVersionChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`, `IdbVersionChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &IdbVersionChangeEventInit,
    ) -> Result<IdbVersionChangeEvent, JsValue>;
}
