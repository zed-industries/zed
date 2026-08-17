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
        js_name = "HTMLDialogElement",
        typescript_type = "HTMLDialogElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDialogElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub type HtmlDialogElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLDialogElement", js_name = "open")]
    #[doc = "Getter for the `open` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn open(this: &HtmlDialogElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLDialogElement", js_name = "open")]
    #[doc = "Setter for the `open` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn set_open(this: &HtmlDialogElement, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLDialogElement",
        js_name = "returnValue"
    )]
    #[doc = "Getter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn return_value(this: &HtmlDialogElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLDialogElement",
        js_name = "returnValue"
    )]
    #[doc = "Setter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn set_return_value(this: &HtmlDialogElement, value: &str);
    #[wasm_bindgen(method, js_class = "HTMLDialogElement")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn close(this: &HtmlDialogElement);
    #[wasm_bindgen(method, js_class = "HTMLDialogElement", js_name = "close")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn close_with_return_value(this: &HtmlDialogElement, return_value: &str);
    #[wasm_bindgen(method, js_class = "HTMLDialogElement")]
    #[doc = "The `show()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/show)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn show(this: &HtmlDialogElement);
    #[wasm_bindgen(catch, method, js_class = "HTMLDialogElement", js_name = "showModal")]
    #[doc = "The `showModal()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/showModal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    pub fn show_modal(this: &HtmlDialogElement) -> Result<(), JsValue>;
}
