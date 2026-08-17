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
        js_name = "HTMLFontElement",
        typescript_type = "HTMLFontElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFontElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub type HtmlFontElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLFontElement", js_name = "color")]
    #[doc = "Getter for the `color` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn color(this: &HtmlFontElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFontElement", js_name = "color")]
    #[doc = "Setter for the `color` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn set_color(this: &HtmlFontElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFontElement", js_name = "face")]
    #[doc = "Getter for the `face` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn face(this: &HtmlFontElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFontElement", js_name = "face")]
    #[doc = "Setter for the `face` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn set_face(this: &HtmlFontElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFontElement", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn size(this: &HtmlFontElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFontElement", js_name = "size")]
    #[doc = "Setter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn set_size(this: &HtmlFontElement, value: &str);
}
