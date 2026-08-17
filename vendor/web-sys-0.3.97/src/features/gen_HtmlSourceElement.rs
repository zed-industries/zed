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
        js_name = "HTMLSourceElement",
        typescript_type = "HTMLSourceElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSourceElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub type HtmlSourceElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn src(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_src(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn type_(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_type(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "srcset")]
    #[doc = "Getter for the `srcset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn srcset(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "srcset")]
    #[doc = "Setter for the `srcset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_srcset(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "sizes")]
    #[doc = "Getter for the `sizes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn sizes(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "sizes")]
    #[doc = "Setter for the `sizes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_sizes(this: &HtmlSourceElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSourceElement", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn media(this: &HtmlSourceElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSourceElement", js_name = "media")]
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_media(this: &HtmlSourceElement, value: &str);
}
