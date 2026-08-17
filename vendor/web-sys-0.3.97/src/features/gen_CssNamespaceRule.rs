#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSNamespaceRule",
        typescript_type = "CSSNamespaceRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssNamespaceRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    pub type CssNamespaceRule;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSNamespaceRule",
        js_name = "namespaceURI"
    )]
    #[doc = "Getter for the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/namespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    pub fn namespace_uri(this: &CssNamespaceRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "CSSNamespaceRule", js_name = "prefix")]
    #[doc = "Getter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    pub fn prefix(this: &CssNamespaceRule) -> ::alloc::string::String;
}
