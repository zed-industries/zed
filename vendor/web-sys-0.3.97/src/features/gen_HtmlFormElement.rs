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
        js_name = "HTMLFormElement",
        typescript_type = "HTMLFormElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFormElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub type HtmlFormElement;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFormElement",
        js_name = "acceptCharset"
    )]
    #[doc = "Getter for the `acceptCharset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn accept_charset(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFormElement",
        js_name = "acceptCharset"
    )]
    #[doc = "Setter for the `acceptCharset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_accept_charset(this: &HtmlFormElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "action")]
    #[doc = "Getter for the `action` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn action(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "action")]
    #[doc = "Setter for the `action` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_action(this: &HtmlFormElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "autocomplete")]
    #[doc = "Getter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn autocomplete(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "autocomplete")]
    #[doc = "Setter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_autocomplete(this: &HtmlFormElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "enctype")]
    #[doc = "Getter for the `enctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn enctype(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "enctype")]
    #[doc = "Setter for the `enctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_enctype(this: &HtmlFormElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "encoding")]
    #[doc = "Getter for the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn encoding(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "encoding")]
    #[doc = "Setter for the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_encoding(this: &HtmlFormElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "method")]
    #[doc = "Getter for the `method` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn method(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "method")]
    #[doc = "Setter for the `method` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_method(this: &HtmlFormElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn name(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_name(this: &HtmlFormElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "noValidate")]
    #[doc = "Getter for the `noValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn no_validate(this: &HtmlFormElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "noValidate")]
    #[doc = "Setter for the `noValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_no_validate(this: &HtmlFormElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn target(this: &HtmlFormElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFormElement", js_name = "target")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn set_target(this: &HtmlFormElement, value: &str);
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "elements")]
    #[doc = "Getter for the `elements` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/elements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFormElement`*"]
    pub fn elements(this: &HtmlFormElement) -> HtmlCollection;
    #[wasm_bindgen(method, getter, js_class = "HTMLFormElement", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn length(this: &HtmlFormElement) -> i32;
    #[wasm_bindgen(method, js_class = "HTMLFormElement", js_name = "checkValidity")]
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn check_validity(this: &HtmlFormElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLFormElement", js_name = "reportValidity")]
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn report_validity(this: &HtmlFormElement) -> bool;
    #[wasm_bindgen(catch, method, js_class = "HTMLFormElement", js_name = "requestSubmit")]
    #[doc = "The `requestSubmit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/requestSubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn request_submit(this: &HtmlFormElement) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLFormElement", js_name = "requestSubmit")]
    #[doc = "The `requestSubmit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/requestSubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn request_submit_with_submitter(
        this: &HtmlFormElement,
        submitter: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "HTMLFormElement")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn reset(this: &HtmlFormElement);
    #[wasm_bindgen(catch, method, js_class = "HTMLFormElement")]
    #[doc = "The `submit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn submit(this: &HtmlFormElement) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "HTMLFormElement", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn get_with_index(this: &HtmlFormElement, index: u32) -> Option<Element>;
    #[wasm_bindgen(method, js_class = "HTMLFormElement", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    pub fn get_with_name(this: &HtmlFormElement, name: &str) -> Option<::js_sys::Object>;
}
