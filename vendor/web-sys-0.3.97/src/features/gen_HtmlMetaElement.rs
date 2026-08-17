#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "HTMLMetaElement",
        typescript_type = "HTMLMetaElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMetaElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub type HtmlMetaElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLMetaElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn name(this: &HtmlMetaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMetaElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn set_name(this: &HtmlMetaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMetaElement", js_name = "httpEquiv")]
    #[doc = "Getter for the `httpEquiv` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn http_equiv(this: &HtmlMetaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMetaElement", js_name = "httpEquiv")]
    #[doc = "Setter for the `httpEquiv` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn set_http_equiv(this: &HtmlMetaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMetaElement", js_name = "content")]
    #[doc = "Getter for the `content` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn content(this: &HtmlMetaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMetaElement", js_name = "content")]
    #[doc = "Setter for the `content` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn set_content(this: &HtmlMetaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMetaElement", js_name = "scheme")]
    #[doc = "Getter for the `scheme` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn scheme(this: &HtmlMetaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMetaElement", js_name = "scheme")]
    #[doc = "Setter for the `scheme` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    pub fn set_scheme(this: &HtmlMetaElement, value: &str);
}
