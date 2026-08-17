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
        js_name = "HTMLSelectElement",
        typescript_type = "HTMLSelectElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSelectElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub type HtmlSelectElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "autofocus")]
    #[doc = "Getter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
    pub fn autofocus(this: &HtmlSelectElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "autofocus")]
    #[doc = "Setter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
    pub fn set_autofocus(this: &HtmlSelectElement, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLSelectElement",
        js_name = "autocomplete"
    )]
    #[doc = "Getter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn autocomplete(this: &HtmlSelectElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLSelectElement",
        js_name = "autocomplete"
    )]
    #[doc = "Setter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_autocomplete(this: &HtmlSelectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn disabled(this: &HtmlSelectElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_disabled(this: &HtmlSelectElement, value: bool);
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "form")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlSelectElement`*"]
    pub fn form(this: &HtmlSelectElement) -> Option<HtmlFormElement>;
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "multiple")]
    #[doc = "Getter for the `multiple` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn multiple(this: &HtmlSelectElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "multiple")]
    #[doc = "Setter for the `multiple` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_multiple(this: &HtmlSelectElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn name(this: &HtmlSelectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_name(this: &HtmlSelectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "required")]
    #[doc = "Getter for the `required` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn required(this: &HtmlSelectElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "required")]
    #[doc = "Setter for the `required` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_required(this: &HtmlSelectElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn size(this: &HtmlSelectElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "size")]
    #[doc = "Setter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_size(this: &HtmlSelectElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn type_(this: &HtmlSelectElement) -> ::alloc::string::String;
    #[cfg(feature = "HtmlOptionsCollection")]
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "options")]
    #[doc = "Getter for the `options` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/options)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`, `HtmlSelectElement`*"]
    pub fn options(this: &HtmlSelectElement) -> HtmlOptionsCollection;
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn length(this: &HtmlSelectElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "length")]
    #[doc = "Setter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_length(this: &HtmlSelectElement, value: u32);
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLSelectElement",
        js_name = "selectedOptions"
    )]
    #[doc = "Getter for the `selectedOptions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedOptions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlSelectElement`*"]
    pub fn selected_options(this: &HtmlSelectElement) -> HtmlCollection;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLSelectElement",
        js_name = "selectedIndex"
    )]
    #[doc = "Getter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn selected_index(this: &HtmlSelectElement) -> i32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLSelectElement",
        js_name = "selectedIndex"
    )]
    #[doc = "Setter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_selected_index(this: &HtmlSelectElement, value: i32);
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn value(this: &HtmlSelectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSelectElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_value(this: &HtmlSelectElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLSelectElement",
        js_name = "willValidate"
    )]
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn will_validate(this: &HtmlSelectElement) -> bool;
    #[cfg(feature = "ValidityState")]
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "validity")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlSelectElement) -> ValidityState;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLSelectElement",
        js_name = "validationMessage"
    )]
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn validation_message(this: &HtmlSelectElement)
        -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLSelectElement", js_name = "labels")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`, `NodeList`*"]
    pub fn labels(this: &HtmlSelectElement) -> NodeList;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLSelectElement", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_option_element(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLSelectElement", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_opt_group_element(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLSelectElement", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_option_element_and_opt_html_element(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLSelectElement", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_opt_group_element_and_opt_html_element(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLSelectElement", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_option_element_and_opt_i32(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLSelectElement", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_opt_group_element_and_opt_i32(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "HTMLSelectElement", js_name = "checkValidity")]
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn check_validity(this: &HtmlSelectElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLSelectElement")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn item(this: &HtmlSelectElement, index: u32) -> Option<Element>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(method, js_class = "HTMLSelectElement", js_name = "namedItem")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn named_item(this: &HtmlSelectElement, name: &str) -> Option<HtmlOptionElement>;
    #[wasm_bindgen(method, js_class = "HTMLSelectElement", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn remove_with_index(this: &HtmlSelectElement, index: i32);
    #[wasm_bindgen(method, js_class = "HTMLSelectElement")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn remove(this: &HtmlSelectElement);
    #[wasm_bindgen(method, js_class = "HTMLSelectElement", js_name = "reportValidity")]
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn report_validity(this: &HtmlSelectElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLSelectElement", js_name = "setCustomValidity")]
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_custom_validity(this: &HtmlSelectElement, error: &str);
    #[wasm_bindgen(method, js_class = "HTMLSelectElement", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn get(this: &HtmlSelectElement, index: u32) -> Option<Element>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLSelectElement", indexing_setter)]
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn set(
        this: &HtmlSelectElement,
        index: u32,
        option: Option<&HtmlOptionElement>,
    ) -> Result<(), JsValue>;
}
