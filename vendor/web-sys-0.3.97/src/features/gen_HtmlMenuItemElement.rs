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
        js_name = "HTMLMenuItemElement",
        typescript_type = "HTMLMenuItemElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMenuItemElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub type HtmlMenuItemElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuItemElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn type_(this: &HtmlMenuItemElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuItemElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn set_type(this: &HtmlMenuItemElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuItemElement", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn label(this: &HtmlMenuItemElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuItemElement", js_name = "label")]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn set_label(this: &HtmlMenuItemElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuItemElement", js_name = "icon")]
    #[doc = "Getter for the `icon` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn icon(this: &HtmlMenuItemElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuItemElement", js_name = "icon")]
    #[doc = "Setter for the `icon` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn set_icon(this: &HtmlMenuItemElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuItemElement", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn disabled(this: &HtmlMenuItemElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuItemElement", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn set_disabled(this: &HtmlMenuItemElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuItemElement", js_name = "checked")]
    #[doc = "Getter for the `checked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn checked(this: &HtmlMenuItemElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuItemElement", js_name = "checked")]
    #[doc = "Setter for the `checked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn set_checked(this: &HtmlMenuItemElement, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLMenuItemElement",
        js_name = "radiogroup"
    )]
    #[doc = "Getter for the `radiogroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn radiogroup(this: &HtmlMenuItemElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLMenuItemElement",
        js_name = "radiogroup"
    )]
    #[doc = "Setter for the `radiogroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn set_radiogroup(this: &HtmlMenuItemElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLMenuItemElement",
        js_name = "defaultChecked"
    )]
    #[doc = "Getter for the `defaultChecked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn default_checked(this: &HtmlMenuItemElement) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLMenuItemElement",
        js_name = "defaultChecked"
    )]
    #[doc = "Setter for the `defaultChecked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[deprecated(note = "Absent in all major browsers")]
    pub fn set_default_checked(this: &HtmlMenuItemElement, value: bool);
}
