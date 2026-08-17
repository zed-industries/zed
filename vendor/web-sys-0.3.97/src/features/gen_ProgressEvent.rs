#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "ProgressEvent",
        typescript_type = "ProgressEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProgressEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub type ProgressEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ProgressEvent",
        js_name = "lengthComputable"
    )]
    #[doc = "Getter for the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn length_computable(this: &ProgressEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ProgressEvent", js_name = "loaded")]
    #[doc = "Getter for the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn loaded(this: &ProgressEvent) -> f64;
    #[wasm_bindgen(method, getter, js_class = "ProgressEvent", js_name = "total")]
    #[doc = "Getter for the `total` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn total(this: &ProgressEvent) -> f64;
    #[wasm_bindgen(catch, constructor, js_class = "ProgressEvent")]
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn new(type_: &str) -> Result<ProgressEvent, JsValue>;
    #[cfg(feature = "ProgressEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ProgressEvent")]
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`, `ProgressEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ProgressEventInit,
    ) -> Result<ProgressEvent, JsValue>;
}
