#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PaintRequest",
        typescript_type = "PaintRequest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaintRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequest`*"]
    pub type PaintRequest;
    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, getter, js_class = "PaintRequest", js_name = "clientRect")]
    #[doc = "Getter for the `clientRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/clientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `PaintRequest`*"]
    pub fn client_rect(this: &PaintRequest) -> DomRect;
    #[wasm_bindgen(method, getter, js_class = "PaintRequest", js_name = "reason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequest`*"]
    pub fn reason(this: &PaintRequest) -> ::alloc::string::String;
}
