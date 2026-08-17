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
        js_name = "HTMLFieldSetElement",
        typescript_type = "HTMLFieldSetElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFieldSetElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub type HtmlFieldSetElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLFieldSetElement", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn disabled(this: &HtmlFieldSetElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLFieldSetElement", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn set_disabled(this: &HtmlFieldSetElement, value: bool);
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(method, getter, js_class = "HTMLFieldSetElement", js_name = "form")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `HtmlFormElement`*"]
    pub fn form(this: &HtmlFieldSetElement) -> Option<HtmlFormElement>;
    #[wasm_bindgen(method, getter, js_class = "HTMLFieldSetElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn name(this: &HtmlFieldSetElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFieldSetElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn set_name(this: &HtmlFieldSetElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFieldSetElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn type_(this: &HtmlFieldSetElement) -> ::alloc::string::String;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "HTMLFieldSetElement", js_name = "elements")]
    #[doc = "Getter for the `elements` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/elements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFieldSetElement`*"]
    pub fn elements(this: &HtmlFieldSetElement) -> HtmlCollection;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFieldSetElement",
        js_name = "willValidate"
    )]
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn will_validate(this: &HtmlFieldSetElement) -> bool;
    #[cfg(feature = "ValidityState")]
    #[wasm_bindgen(method, getter, js_class = "HTMLFieldSetElement", js_name = "validity")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlFieldSetElement) -> ValidityState;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLFieldSetElement",
        js_name = "validationMessage"
    )]
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn validation_message(
        this: &HtmlFieldSetElement,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, js_class = "HTMLFieldSetElement", js_name = "checkValidity")]
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn check_validity(this: &HtmlFieldSetElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLFieldSetElement", js_name = "reportValidity")]
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn report_validity(this: &HtmlFieldSetElement) -> bool;
    #[wasm_bindgen(
        method,
        js_class = "HTMLFieldSetElement",
        js_name = "setCustomValidity"
    )]
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn set_custom_validity(this: &HtmlFieldSetElement, error: &str);
}
