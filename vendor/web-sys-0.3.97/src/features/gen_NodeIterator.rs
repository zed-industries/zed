#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NodeIterator",
        typescript_type = "NodeIterator"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeIterator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub type NodeIterator;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "NodeIterator", js_name = "root")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn root(this: &NodeIterator) -> Node;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "NodeIterator", js_name = "referenceNode")]
    #[doc = "Getter for the `referenceNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/referenceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn reference_node(this: &NodeIterator) -> Option<Node>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NodeIterator",
        js_name = "pointerBeforeReferenceNode"
    )]
    #[doc = "Getter for the `pointerBeforeReferenceNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/pointerBeforeReferenceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub fn pointer_before_reference_node(this: &NodeIterator) -> bool;
    #[wasm_bindgen(method, getter, js_class = "NodeIterator", js_name = "whatToShow")]
    #[doc = "Getter for the `whatToShow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/whatToShow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub fn what_to_show(this: &NodeIterator) -> u32;
    #[cfg(feature = "NodeFilter")]
    #[wasm_bindgen(method, getter, js_class = "NodeIterator", js_name = "filter")]
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`, `NodeIterator`*"]
    pub fn filter(this: &NodeIterator) -> Option<NodeFilter>;
    #[wasm_bindgen(method, js_class = "NodeIterator")]
    #[doc = "The `detach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/detach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub fn detach(this: &NodeIterator);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "NodeIterator", js_name = "nextNode")]
    #[doc = "The `nextNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/nextNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn next_node(this: &NodeIterator) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "NodeIterator", js_name = "previousNode")]
    #[doc = "The `previousNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/previousNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn previous_node(this: &NodeIterator) -> Result<Option<Node>, JsValue>;
}
