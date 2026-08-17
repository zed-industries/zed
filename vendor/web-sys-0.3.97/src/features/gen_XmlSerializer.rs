#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XMLSerializer",
        typescript_type = "XMLSerializer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlSerializer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlSerializer`*"]
    pub type XmlSerializer;
    #[wasm_bindgen(catch, constructor, js_class = "XMLSerializer")]
    #[doc = "The `new XmlSerializer(..)` constructor, creating a new instance of `XmlSerializer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/XMLSerializer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlSerializer`*"]
    pub fn new() -> Result<XmlSerializer, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "XMLSerializer",
        js_name = "serializeToString"
    )]
    #[doc = "The `serializeToString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/serializeToString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XmlSerializer`*"]
    pub fn serialize_to_string(
        this: &XmlSerializer,
        root: &Node,
    ) -> Result<::alloc::string::String, JsValue>;
}
