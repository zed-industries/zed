#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MutationRecord",
        typescript_type = "MutationRecord"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationRecord` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub type MutationRecord;
    #[wasm_bindgen(method, getter, js_class = "MutationRecord", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn type_(this: &MutationRecord) -> ::alloc::string::String;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "MutationRecord", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    pub fn target(this: &MutationRecord) -> Option<Node>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "MutationRecord", js_name = "addedNodes")]
    #[doc = "Getter for the `addedNodes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/addedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
    pub fn added_nodes(this: &MutationRecord) -> NodeList;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "MutationRecord", js_name = "removedNodes")]
    #[doc = "Getter for the `removedNodes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/removedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
    pub fn removed_nodes(this: &MutationRecord) -> NodeList;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MutationRecord",
        js_name = "previousSibling"
    )]
    #[doc = "Getter for the `previousSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/previousSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    pub fn previous_sibling(this: &MutationRecord) -> Option<Node>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "MutationRecord", js_name = "nextSibling")]
    #[doc = "Getter for the `nextSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/nextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    pub fn next_sibling(this: &MutationRecord) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "MutationRecord", js_name = "attributeName")]
    #[doc = "Getter for the `attributeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn attribute_name(this: &MutationRecord) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MutationRecord",
        js_name = "attributeNamespace"
    )]
    #[doc = "Getter for the `attributeNamespace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeNamespace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn attribute_namespace(this: &MutationRecord) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "MutationRecord", js_name = "oldValue")]
    #[doc = "Getter for the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/oldValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn old_value(this: &MutationRecord) -> Option<::alloc::string::String>;
}
