#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Attr",
        typescript_type = "Attr"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Attr` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub type Attr;
    #[wasm_bindgen(method, getter, js_class = "Attr", js_name = "localName")]
    #[doc = "Getter for the `localName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/localName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub fn local_name(this: &Attr) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Attr", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub fn value(this: &Attr) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Attr", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub fn set_value(this: &Attr, value: &str);
    #[wasm_bindgen(method, getter, js_class = "Attr", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub fn name(this: &Attr) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Attr", js_name = "namespaceURI")]
    #[doc = "Getter for the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/namespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub fn namespace_uri(this: &Attr) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Attr", js_name = "prefix")]
    #[doc = "Getter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub fn prefix(this: &Attr) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Attr", js_name = "specified")]
    #[doc = "Getter for the `specified` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/specified)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
    pub fn specified(this: &Attr) -> bool;
}
