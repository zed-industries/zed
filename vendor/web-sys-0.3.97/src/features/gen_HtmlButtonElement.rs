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
        js_name = "HTMLButtonElement",
        typescript_type = "HTMLButtonElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlButtonElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub type HtmlButtonElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "autofocus")]
    #[doc = "Getter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
    pub fn autofocus(this: &HtmlButtonElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "autofocus")]
    #[doc = "Setter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
    pub fn set_autofocus(this: &HtmlButtonElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn disabled(this: &HtmlButtonElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_disabled(this: &HtmlButtonElement, value: bool);
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "form")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`, `HtmlFormElement`*"]
    pub fn form(this: &HtmlButtonElement) -> Option<HtmlFormElement>;
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "formAction")]
    #[doc = "Getter for the `formAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_action(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "formAction")]
    #[doc = "Setter for the `formAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_action(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLButtonElement",
        js_name = "formEnctype"
    )]
    #[doc = "Getter for the `formEnctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_enctype(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLButtonElement",
        js_name = "formEnctype"
    )]
    #[doc = "Setter for the `formEnctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_enctype(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "formMethod")]
    #[doc = "Getter for the `formMethod` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_method(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "formMethod")]
    #[doc = "Setter for the `formMethod` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_method(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLButtonElement",
        js_name = "formNoValidate"
    )]
    #[doc = "Getter for the `formNoValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_no_validate(this: &HtmlButtonElement) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLButtonElement",
        js_name = "formNoValidate"
    )]
    #[doc = "Setter for the `formNoValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_no_validate(this: &HtmlButtonElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "formTarget")]
    #[doc = "Getter for the `formTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_target(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "formTarget")]
    #[doc = "Setter for the `formTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_target(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn name(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_name(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn type_(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_type(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn value(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLButtonElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_value(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLButtonElement",
        js_name = "willValidate"
    )]
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn will_validate(this: &HtmlButtonElement) -> bool;
    #[cfg(feature = "ValidityState")]
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "validity")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlButtonElement) -> ValidityState;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLButtonElement",
        js_name = "validationMessage"
    )]
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn validation_message(this: &HtmlButtonElement)
        -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLButtonElement", js_name = "labels")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`, `NodeList`*"]
    pub fn labels(this: &HtmlButtonElement) -> NodeList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLButtonElement",
        js_name = "popoverTargetElement"
    )]
    #[doc = "Getter for the `popoverTargetElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn popover_target_element(this: &HtmlButtonElement) -> Option<Element>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLButtonElement",
        js_name = "popoverTargetElement"
    )]
    #[doc = "Setter for the `popoverTargetElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_popover_target_element(this: &HtmlButtonElement, value: Option<&Element>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLButtonElement",
        js_name = "popoverTargetAction"
    )]
    #[doc = "Getter for the `popoverTargetAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn popover_target_action(this: &HtmlButtonElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLButtonElement",
        js_name = "popoverTargetAction"
    )]
    #[doc = "Setter for the `popoverTargetAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_popover_target_action(this: &HtmlButtonElement, value: &str);
    #[wasm_bindgen(method, js_class = "HTMLButtonElement", js_name = "checkValidity")]
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn check_validity(this: &HtmlButtonElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLButtonElement", js_name = "reportValidity")]
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn report_validity(this: &HtmlButtonElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLButtonElement", js_name = "setCustomValidity")]
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_custom_validity(this: &HtmlButtonElement, error: &str);
}
