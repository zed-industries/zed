#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "BlobEvent",
        typescript_type = "BlobEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEvent`*"]
    pub type BlobEvent;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(method, getter, js_class = "BlobEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobEvent`*"]
    pub fn data(this: &BlobEvent) -> Option<Blob>;
    #[wasm_bindgen(catch, constructor, js_class = "BlobEvent")]
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEvent`*"]
    pub fn new(type_: &str) -> Result<BlobEvent, JsValue>;
    #[cfg(feature = "BlobEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "BlobEvent")]
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEvent`, `BlobEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &BlobEventInit,
    ) -> Result<BlobEvent, JsValue>;
}
