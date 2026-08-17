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
        js_name = "HTMLObjectElement",
        typescript_type = "HTMLObjectElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlObjectElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub type HtmlObjectElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn data(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "data")]
    #[doc = "Setter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_data(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn type_(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_type(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLObjectElement",
        js_name = "typeMustMatch"
    )]
    #[doc = "Getter for the `typeMustMatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn type_must_match(this: &HtmlObjectElement) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLObjectElement",
        js_name = "typeMustMatch"
    )]
    #[doc = "Setter for the `typeMustMatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_type_must_match(this: &HtmlObjectElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn name(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_name(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "useMap")]
    #[doc = "Getter for the `useMap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn use_map(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "useMap")]
    #[doc = "Setter for the `useMap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_use_map(this: &HtmlObjectElement, value: &str);
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "form")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlObjectElement`*"]
    pub fn form(this: &HtmlObjectElement) -> Option<HtmlFormElement>;
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn width(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_width(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn height(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "height")]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_height(this: &HtmlObjectElement, value: &str);
    #[cfg(feature = "Document")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLObjectElement",
        js_name = "contentDocument"
    )]
    #[doc = "Getter for the `contentDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
    pub fn content_document(this: &HtmlObjectElement) -> Option<Document>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLObjectElement",
        js_name = "contentWindow"
    )]
    #[doc = "Getter for the `contentWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`, `Window`*"]
    pub fn content_window(this: &HtmlObjectElement) -> Option<Window>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLObjectElement",
        js_name = "willValidate"
    )]
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn will_validate(this: &HtmlObjectElement) -> bool;
    #[cfg(feature = "ValidityState")]
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "validity")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlObjectElement) -> ValidityState;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLObjectElement",
        js_name = "validationMessage"
    )]
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn validation_message(this: &HtmlObjectElement)
        -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn align(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_align(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "archive")]
    #[doc = "Getter for the `archive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn archive(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "archive")]
    #[doc = "Setter for the `archive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_archive(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "code")]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn code(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "code")]
    #[doc = "Setter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_code(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "declare")]
    #[doc = "Getter for the `declare` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn declare(this: &HtmlObjectElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "declare")]
    #[doc = "Setter for the `declare` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_declare(this: &HtmlObjectElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "hspace")]
    #[doc = "Getter for the `hspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn hspace(this: &HtmlObjectElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "hspace")]
    #[doc = "Setter for the `hspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_hspace(this: &HtmlObjectElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "standby")]
    #[doc = "Getter for the `standby` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn standby(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "standby")]
    #[doc = "Setter for the `standby` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_standby(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "vspace")]
    #[doc = "Getter for the `vspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn vspace(this: &HtmlObjectElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "vspace")]
    #[doc = "Setter for the `vspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_vspace(this: &HtmlObjectElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "codeBase")]
    #[doc = "Getter for the `codeBase` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn code_base(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "codeBase")]
    #[doc = "Setter for the `codeBase` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_code_base(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "codeType")]
    #[doc = "Getter for the `codeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn code_type(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "codeType")]
    #[doc = "Setter for the `codeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_code_type(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLObjectElement", js_name = "border")]
    #[doc = "Getter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn border(this: &HtmlObjectElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLObjectElement", js_name = "border")]
    #[doc = "Setter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_border(this: &HtmlObjectElement, value: &str);
    #[wasm_bindgen(method, js_class = "HTMLObjectElement", js_name = "checkValidity")]
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn check_validity(this: &HtmlObjectElement) -> bool;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, js_class = "HTMLObjectElement", js_name = "getSVGDocument")]
    #[doc = "The `getSVGDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/getSVGDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
    pub fn get_svg_document(this: &HtmlObjectElement) -> Option<Document>;
    #[wasm_bindgen(method, js_class = "HTMLObjectElement", js_name = "reportValidity")]
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn report_validity(this: &HtmlObjectElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLObjectElement", js_name = "setCustomValidity")]
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_custom_validity(this: &HtmlObjectElement, error: &str);
}
