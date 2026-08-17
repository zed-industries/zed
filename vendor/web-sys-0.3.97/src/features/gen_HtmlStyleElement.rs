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
        js_name = "HTMLStyleElement",
        typescript_type = "HTMLStyleElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlStyleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub type HtmlStyleElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLStyleElement", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn disabled(this: &HtmlStyleElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLStyleElement", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn set_disabled(this: &HtmlStyleElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLStyleElement", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn media(this: &HtmlStyleElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLStyleElement", js_name = "media")]
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn set_media(this: &HtmlStyleElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLStyleElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn type_(this: &HtmlStyleElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLStyleElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn set_type(this: &HtmlStyleElement, value: &str);
    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, getter, js_class = "HTMLStyleElement", js_name = "sheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`, `StyleSheet`*"]
    pub fn sheet(this: &HtmlStyleElement) -> Option<StyleSheet>;
}
