#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMError",
        typescript_type = "DOMError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub type DomError;
    #[wasm_bindgen(method, getter, js_class = "DOMError", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn name(this: &DomError) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "DOMError", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn message(this: &DomError) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "DOMError")]
    #[doc = "The `new DomError(..)` constructor, creating a new instance of `DomError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn new(name: &str) -> Result<DomError, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMError")]
    #[doc = "The `new DomError(..)` constructor, creating a new instance of `DomError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn new_with_message(name: &str, message: &str) -> Result<DomError, JsValue>;
}
