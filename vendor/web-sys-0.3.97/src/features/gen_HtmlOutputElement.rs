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
        js_name = "HTMLOutputElement",
        typescript_type = "HTMLOutputElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOutputElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub type HtmlOutputElement;
    #[cfg(feature = "DomTokenList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLOutputElement", js_name = "htmlFor")]
    #[doc = "Getter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlOutputElement`*"]
    pub fn html_for(this: &HtmlOutputElement) -> DomTokenList;
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(method, getter, js_class = "HTMLOutputElement", js_name = "form")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOutputElement`*"]
    pub fn form(this: &HtmlOutputElement) -> Option<HtmlFormElement>;
    #[wasm_bindgen(method, getter, js_class = "HTMLOutputElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn name(this: &HtmlOutputElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLOutputElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_name(this: &HtmlOutputElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLOutputElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn type_(this: &HtmlOutputElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLOutputElement",
        js_name = "defaultValue"
    )]
    #[doc = "Getter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn default_value(this: &HtmlOutputElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLOutputElement",
        js_name = "defaultValue"
    )]
    #[doc = "Setter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_default_value(this: &HtmlOutputElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLOutputElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn value(this: &HtmlOutputElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLOutputElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_value(this: &HtmlOutputElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLOutputElement",
        js_name = "willValidate"
    )]
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn will_validate(this: &HtmlOutputElement) -> bool;
    #[cfg(feature = "ValidityState")]
    #[wasm_bindgen(method, getter, js_class = "HTMLOutputElement", js_name = "validity")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlOutputElement) -> ValidityState;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLOutputElement",
        js_name = "validationMessage"
    )]
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn validation_message(this: &HtmlOutputElement)
        -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLOutputElement", js_name = "labels")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`, `NodeList`*"]
    pub fn labels(this: &HtmlOutputElement) -> NodeList;
    #[wasm_bindgen(method, js_class = "HTMLOutputElement", js_name = "checkValidity")]
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn check_validity(this: &HtmlOutputElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLOutputElement", js_name = "reportValidity")]
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn report_validity(this: &HtmlOutputElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLOutputElement", js_name = "setCustomValidity")]
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_custom_validity(this: &HtmlOutputElement, error: &str);
}
