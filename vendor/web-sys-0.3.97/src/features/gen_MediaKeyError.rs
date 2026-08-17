#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MediaKeyError",
        typescript_type = "MediaKeyError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyError`*"]
    pub type MediaKeyError;
    #[wasm_bindgen(method, getter, js_class = "MediaKeyError", js_name = "systemCode")]
    #[doc = "Getter for the `systemCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError/systemCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyError`*"]
    pub fn system_code(this: &MediaKeyError) -> u32;
}
