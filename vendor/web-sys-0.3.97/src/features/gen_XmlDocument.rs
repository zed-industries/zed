#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Document",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "XMLDocument",
        typescript_type = "XMLDocument"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlDocument` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
    pub type XmlDocument;
    #[wasm_bindgen(method, getter, js_class = "XMLDocument", js_name = "async")]
    #[doc = "Getter for the `async` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
    pub fn r#async(this: &XmlDocument) -> bool;
    #[wasm_bindgen(method, setter, js_class = "XMLDocument", js_name = "async")]
    #[doc = "Setter for the `async` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
    pub fn set_async(this: &XmlDocument, value: bool);
    #[wasm_bindgen(catch, method, js_class = "XMLDocument")]
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
    pub fn load(this: &XmlDocument, url: &str) -> Result<bool, JsValue>;
}
