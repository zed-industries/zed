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
        js_name = "HTMLFrameElement",
        typescript_type = "HTMLFrameElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFrameElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub type HtmlFrameElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn name(this: &HtmlFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_name(this: &HtmlFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameElement", js_name = "scrolling")]
    #[doc = "Getter for the `scrolling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn scrolling(this: &HtmlFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameElement", js_name = "scrolling")]
    #[doc = "Setter for the `scrolling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_scrolling(this: &HtmlFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn src(this: &HtmlFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_src(this: &HtmlFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameElement", js_name = "frameBorder")]
    #[doc = "Getter for the `frameBorder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn frame_border(this: &HtmlFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameElement", js_name = "frameBorder")]
    #[doc = "Setter for the `frameBorder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_frame_border(this: &HtmlFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameElement", js_name = "longDesc")]
    #[doc = "Getter for the `longDesc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn long_desc(this: &HtmlFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameElement", js_name = "longDesc")]
    #[doc = "Setter for the `longDesc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_long_desc(this: &HtmlFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameElement", js_name = "noResize")]
    #[doc = "Getter for the `noResize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn no_resize(this: &HtmlFrameElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameElement", js_name = "noResize")]
    #[doc = "Setter for the `noResize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_no_resize(this: &HtmlFrameElement, value: bool);
    #[cfg(feature = "Document")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameElement",
        js_name = "contentDocument"
    )]
    #[doc = "Getter for the `contentDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlFrameElement`*"]
    pub fn content_document(this: &HtmlFrameElement) -> Option<Document>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameElement",
        js_name = "contentWindow"
    )]
    #[doc = "Getter for the `contentWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`, `Window`*"]
    pub fn content_window(this: &HtmlFrameElement) -> Option<Window>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameElement",
        js_name = "marginHeight"
    )]
    #[doc = "Getter for the `marginHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn margin_height(this: &HtmlFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameElement",
        js_name = "marginHeight"
    )]
    #[doc = "Setter for the `marginHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_margin_height(this: &HtmlFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameElement", js_name = "marginWidth")]
    #[doc = "Getter for the `marginWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn margin_width(this: &HtmlFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameElement", js_name = "marginWidth")]
    #[doc = "Setter for the `marginWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    pub fn set_margin_width(this: &HtmlFrameElement, value: &str);
}
