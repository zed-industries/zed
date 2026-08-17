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
        js_name = "HTMLEmbedElement",
        typescript_type = "HTMLEmbedElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlEmbedElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub type HtmlEmbedElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLEmbedElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn src(this: &HtmlEmbedElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLEmbedElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn set_src(this: &HtmlEmbedElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLEmbedElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn type_(this: &HtmlEmbedElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLEmbedElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn set_type(this: &HtmlEmbedElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLEmbedElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn width(this: &HtmlEmbedElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLEmbedElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn set_width(this: &HtmlEmbedElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLEmbedElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn height(this: &HtmlEmbedElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLEmbedElement", js_name = "height")]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn set_height(this: &HtmlEmbedElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLEmbedElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn align(this: &HtmlEmbedElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLEmbedElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn set_align(this: &HtmlEmbedElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLEmbedElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn name(this: &HtmlEmbedElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLEmbedElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlEmbedElement`*"]
    pub fn set_name(this: &HtmlEmbedElement, value: &str);
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, js_class = "HTMLEmbedElement", js_name = "getSVGDocument")]
    #[doc = "The `getSVGDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/getSVGDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlEmbedElement`*"]
    pub fn get_svg_document(this: &HtmlEmbedElement) -> Option<Document>;
}
