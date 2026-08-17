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
        js_name = "HTMLOptionElement",
        typescript_type = "HTMLOptionElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOptionElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub type HtmlOptionElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionElement", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn disabled(this: &HtmlOptionElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLOptionElement", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn set_disabled(this: &HtmlOptionElement, value: bool);
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionElement", js_name = "form")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOptionElement`*"]
    pub fn form(this: &HtmlOptionElement) -> Option<HtmlFormElement>;
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionElement", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn label(this: &HtmlOptionElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLOptionElement", js_name = "label")]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn set_label(this: &HtmlOptionElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLOptionElement",
        js_name = "defaultSelected"
    )]
    #[doc = "Getter for the `defaultSelected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn default_selected(this: &HtmlOptionElement) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLOptionElement",
        js_name = "defaultSelected"
    )]
    #[doc = "Setter for the `defaultSelected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn set_default_selected(this: &HtmlOptionElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionElement", js_name = "selected")]
    #[doc = "Getter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn selected(this: &HtmlOptionElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLOptionElement", js_name = "selected")]
    #[doc = "Setter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn set_selected(this: &HtmlOptionElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn value(this: &HtmlOptionElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLOptionElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn set_value(this: &HtmlOptionElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionElement", js_name = "text")]
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn text(this: &HtmlOptionElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLOptionElement", js_name = "text")]
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn set_text(this: &HtmlOptionElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionElement", js_name = "index")]
    #[doc = "Getter for the `index` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn index(this: &HtmlOptionElement) -> i32;
    #[wasm_bindgen(catch, constructor, js_class = "Option")]
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn new() -> Result<HtmlOptionElement, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Option")]
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn new_with_text(text: &str) -> Result<HtmlOptionElement, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Option")]
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn new_with_text_and_value(text: &str, value: &str) -> Result<HtmlOptionElement, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Option")]
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn new_with_text_and_value_and_default_selected(
        text: &str,
        value: &str,
        default_selected: bool,
    ) -> Result<HtmlOptionElement, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Option")]
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    pub fn new_with_text_and_value_and_default_selected_and_selected(
        text: &str,
        value: &str,
        default_selected: bool,
        selected: bool,
    ) -> Result<HtmlOptionElement, JsValue>;
}
