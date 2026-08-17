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
        js_name = "HTMLHRElement",
        typescript_type = "HTMLHRElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlHrElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub type HtmlHrElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLHRElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn align(this: &HtmlHrElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLHRElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn set_align(this: &HtmlHrElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLHRElement", js_name = "color")]
    #[doc = "Getter for the `color` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn color(this: &HtmlHrElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLHRElement", js_name = "color")]
    #[doc = "Setter for the `color` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn set_color(this: &HtmlHrElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLHRElement", js_name = "noShade")]
    #[doc = "Getter for the `noShade` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn no_shade(this: &HtmlHrElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLHRElement", js_name = "noShade")]
    #[doc = "Setter for the `noShade` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn set_no_shade(this: &HtmlHrElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLHRElement", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn size(this: &HtmlHrElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLHRElement", js_name = "size")]
    #[doc = "Setter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn set_size(this: &HtmlHrElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLHRElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn width(this: &HtmlHrElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLHRElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    pub fn set_width(this: &HtmlHrElement, value: &str);
}
