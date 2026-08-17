#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "UserActivation",
        typescript_type = "UserActivation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UserActivation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserActivation`*"]
    pub type UserActivation;
    #[wasm_bindgen(method, getter, js_class = "UserActivation", js_name = "hasBeenActive")]
    #[doc = "Getter for the `hasBeenActive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation/hasBeenActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserActivation`*"]
    pub fn has_been_active(this: &UserActivation) -> bool;
    #[wasm_bindgen(method, getter, js_class = "UserActivation", js_name = "isActive")]
    #[doc = "Getter for the `isActive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation/isActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserActivation`*"]
    pub fn is_active(this: &UserActivation) -> bool;
}
