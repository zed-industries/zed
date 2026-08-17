#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMParser",
        typescript_type = "DOMParser"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomParser` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomParser`*"]
    pub type DomParser;
    #[wasm_bindgen(catch, constructor, js_class = "DOMParser")]
    #[doc = "The `new DomParser(..)` constructor, creating a new instance of `DomParser`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/DOMParser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomParser`*"]
    pub fn new() -> Result<DomParser, JsValue>;
    #[cfg(all(feature = "Document", feature = "SupportedType",))]
    #[wasm_bindgen(catch, method, js_class = "DOMParser", js_name = "parseFromString")]
    #[doc = "The `parseFromString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/parseFromString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomParser`, `SupportedType`*"]
    pub fn parse_from_string(
        this: &DomParser,
        str: &str,
        type_: SupportedType,
    ) -> Result<Document, JsValue>;
}
