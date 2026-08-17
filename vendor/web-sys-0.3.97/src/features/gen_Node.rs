#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Node",
        typescript_type = "Node"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Node` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub type Node;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "nodeType")]
    #[doc = "Getter for the `nodeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn node_type(this: &Node) -> u16;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "nodeName")]
    #[doc = "Getter for the `nodeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn node_name(this: &Node) -> ::alloc::string::String;
    #[wasm_bindgen(catch, method, getter, js_class = "Node", js_name = "baseURI")]
    #[doc = "Getter for the `baseURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn base_uri(this: &Node) -> Result<Option<::alloc::string::String>, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "isConnected")]
    #[doc = "Getter for the `isConnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_connected(this: &Node) -> bool;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "ownerDocument")]
    #[doc = "Getter for the `ownerDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Node`*"]
    pub fn owner_document(this: &Node) -> Option<Document>;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "parentNode")]
    #[doc = "Getter for the `parentNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn parent_node(this: &Node) -> Option<Node>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "parentElement")]
    #[doc = "Getter for the `parentElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn parent_element(this: &Node) -> Option<Element>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "childNodes")]
    #[doc = "Getter for the `childNodes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
    pub fn child_nodes(this: &Node) -> NodeList;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "firstChild")]
    #[doc = "Getter for the `firstChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn first_child(this: &Node) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "lastChild")]
    #[doc = "Getter for the `lastChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lastChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn last_child(this: &Node) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "previousSibling")]
    #[doc = "Getter for the `previousSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/previousSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn previous_sibling(this: &Node) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "nextSibling")]
    #[doc = "Getter for the `nextSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn next_sibling(this: &Node) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "nodeValue")]
    #[doc = "Getter for the `nodeValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn node_value(this: &Node) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, setter, js_class = "Node", js_name = "nodeValue")]
    #[doc = "Setter for the `nodeValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn set_node_value(this: &Node, value: Option<&str>);
    #[wasm_bindgen(method, getter, js_class = "Node", js_name = "textContent")]
    #[doc = "Getter for the `textContent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn text_content(this: &Node) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, setter, js_class = "Node", js_name = "textContent")]
    #[doc = "Setter for the `textContent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn set_text_content(this: &Node, value: Option<&str>);
    #[wasm_bindgen(catch, method, js_class = "Node", js_name = "appendChild")]
    #[doc = "The `appendChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn append_child(this: &Node, node: &Node) -> Result<Node, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Node", js_name = "cloneNode")]
    #[doc = "The `cloneNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn clone_node(this: &Node) -> Result<Node, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Node", js_name = "cloneNode")]
    #[doc = "The `cloneNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn clone_node_with_deep(this: &Node, deep: bool) -> Result<Node, JsValue>;
    #[wasm_bindgen(method, js_class = "Node", js_name = "compareDocumentPosition")]
    #[doc = "The `compareDocumentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/compareDocumentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn compare_document_position(this: &Node, other: &Node) -> u16;
    #[wasm_bindgen(method, js_class = "Node")]
    #[doc = "The `contains()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn contains(this: &Node, other: Option<&Node>) -> bool;
    #[wasm_bindgen(method, js_class = "Node", js_name = "getRootNode")]
    #[doc = "The `getRootNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn get_root_node(this: &Node) -> Node;
    #[cfg(feature = "GetRootNodeOptions")]
    #[wasm_bindgen(method, js_class = "Node", js_name = "getRootNode")]
    #[doc = "The `getRootNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`, `Node`*"]
    pub fn get_root_node_with_options(this: &Node, options: &GetRootNodeOptions) -> Node;
    #[wasm_bindgen(method, js_class = "Node", js_name = "hasChildNodes")]
    #[doc = "The `hasChildNodes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn has_child_nodes(this: &Node) -> bool;
    #[wasm_bindgen(catch, method, js_class = "Node", js_name = "insertBefore")]
    #[doc = "The `insertBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn insert_before(this: &Node, node: &Node, child: Option<&Node>) -> Result<Node, JsValue>;
    #[wasm_bindgen(method, js_class = "Node", js_name = "isDefaultNamespace")]
    #[doc = "The `isDefaultNamespace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_default_namespace(this: &Node, namespace: Option<&str>) -> bool;
    #[wasm_bindgen(method, js_class = "Node", js_name = "isEqualNode")]
    #[doc = "The `isEqualNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_equal_node(this: &Node, node: Option<&Node>) -> bool;
    #[wasm_bindgen(method, js_class = "Node", js_name = "isSameNode")]
    #[doc = "The `isSameNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_same_node(this: &Node, node: Option<&Node>) -> bool;
    #[wasm_bindgen(method, js_class = "Node", js_name = "lookupNamespaceURI")]
    #[doc = "The `lookupNamespaceURI()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn lookup_namespace_uri(
        this: &Node,
        prefix: Option<&str>,
    ) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "Node", js_name = "lookupPrefix")]
    #[doc = "The `lookupPrefix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn lookup_prefix(this: &Node, namespace: Option<&str>) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "Node")]
    #[doc = "The `normalize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn normalize(this: &Node);
    #[wasm_bindgen(catch, method, js_class = "Node", js_name = "removeChild")]
    #[doc = "The `removeChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn remove_child(this: &Node, child: &Node) -> Result<Node, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Node", js_name = "replaceChild")]
    #[doc = "The `replaceChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub fn replace_child(this: &Node, node: &Node, child: &Node) -> Result<Node, JsValue>;
}
impl Node {
    #[doc = "The `Node.ELEMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ELEMENT_NODE: u16 = 1u64 as u16;
    #[doc = "The `Node.ATTRIBUTE_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ATTRIBUTE_NODE: u16 = 2u64 as u16;
    #[doc = "The `Node.TEXT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const TEXT_NODE: u16 = 3u64 as u16;
    #[doc = "The `Node.CDATA_SECTION_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const CDATA_SECTION_NODE: u16 = 4u64 as u16;
    #[doc = "The `Node.ENTITY_REFERENCE_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ENTITY_REFERENCE_NODE: u16 = 5u64 as u16;
    #[doc = "The `Node.ENTITY_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ENTITY_NODE: u16 = 6u64 as u16;
    #[doc = "The `Node.PROCESSING_INSTRUCTION_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const PROCESSING_INSTRUCTION_NODE: u16 = 7u64 as u16;
    #[doc = "The `Node.COMMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const COMMENT_NODE: u16 = 8u64 as u16;
    #[doc = "The `Node.DOCUMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_NODE: u16 = 9u64 as u16;
    #[doc = "The `Node.DOCUMENT_TYPE_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_TYPE_NODE: u16 = 10u64 as u16;
    #[doc = "The `Node.DOCUMENT_FRAGMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_FRAGMENT_NODE: u16 = 11u64 as u16;
    #[doc = "The `Node.NOTATION_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const NOTATION_NODE: u16 = 12u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_DISCONNECTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_DISCONNECTED: u16 = 1u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_PRECEDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_PRECEDING: u16 = 2u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_FOLLOWING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_FOLLOWING: u16 = 4u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_CONTAINS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_CONTAINS: u16 = 8u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_CONTAINED_BY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_CONTAINED_BY: u16 = 16u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u16 = 32u64 as u16;
}
