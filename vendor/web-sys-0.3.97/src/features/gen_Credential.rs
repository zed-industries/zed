#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Credential",
        typescript_type = "Credential"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Credential` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub type Credential;
    #[wasm_bindgen(method, getter, js_class = "Credential", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub fn id(this: &Credential) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Credential", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
    pub fn type_(this: &Credential) -> ::alloc::string::String;
}
