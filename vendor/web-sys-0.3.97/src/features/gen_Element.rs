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
        js_name = "Element",
        typescript_type = "Element"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Element` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub type Element;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "namespaceURI")]
    #[doc = "Getter for the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn namespace_uri(this: &Element) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "prefix")]
    #[doc = "Getter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prefix(this: &Element) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "localName")]
    #[doc = "Getter for the `localName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/localName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn local_name(this: &Element) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "tagName")]
    #[doc = "Getter for the `tagName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn tag_name(this: &Element) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn id(this: &Element) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "id")]
    #[doc = "Setter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_id(this: &Element, value: &str);
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "className")]
    #[doc = "Getter for the `className` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn class_name(this: &Element) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "className")]
    #[doc = "Setter for the `className` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_class_name(this: &Element, value: &str);
    #[cfg(feature = "DomTokenList")]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "classList")]
    #[doc = "Getter for the `classList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `Element`*"]
    pub fn class_list(this: &Element) -> DomTokenList;
    #[cfg(feature = "NamedNodeMap")]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "attributes")]
    #[doc = "Getter for the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `NamedNodeMap`*"]
    pub fn attributes(this: &Element) -> NamedNodeMap;
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "scrollTop")]
    #[doc = "Getter for the `scrollTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_top(this: &Element) -> i32;
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "scrollTop")]
    #[doc = "Setter for the `scrollTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_scroll_top(this: &Element, value: i32);
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "scrollLeft")]
    #[doc = "Getter for the `scrollLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_left(this: &Element) -> i32;
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "scrollLeft")]
    #[doc = "Setter for the `scrollLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_scroll_left(this: &Element, value: i32);
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "scrollWidth")]
    #[doc = "Getter for the `scrollWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_width(this: &Element) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "scrollHeight")]
    #[doc = "Getter for the `scrollHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_height(this: &Element) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "clientTop")]
    #[doc = "Getter for the `clientTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_top(this: &Element) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "clientLeft")]
    #[doc = "Getter for the `clientLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_left(this: &Element) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "clientWidth")]
    #[doc = "Getter for the `clientWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_width(this: &Element) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "clientHeight")]
    #[doc = "Getter for the `clientHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_height(this: &Element) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "innerHTML")]
    #[doc = "Getter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn inner_html(this: &Element) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "innerHTML")]
    #[doc = "Setter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_inner_html(this: &Element, value: &str);
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "outerHTML")]
    #[doc = "Getter for the `outerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn outer_html(this: &Element) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "outerHTML")]
    #[doc = "Setter for the `outerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_outer_html(this: &Element, value: &str);
    #[cfg(feature = "ShadowRoot")]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "shadowRoot")]
    #[doc = "Getter for the `shadowRoot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn shadow_root(this: &Element) -> Option<ShadowRoot>;
    #[cfg(feature = "HtmlSlotElement")]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "assignedSlot")]
    #[doc = "Getter for the `assignedSlot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/assignedSlot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlSlotElement`*"]
    pub fn assigned_slot(this: &Element) -> Option<HtmlSlotElement>;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "slot")]
    #[doc = "Getter for the `slot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn slot(this: &Element) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "slot")]
    #[doc = "Setter for the `slot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_slot(this: &Element, value: &str);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "scrollTop")]
    #[doc = "Getter for the `scrollTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn scroll_top(this: &Element) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "scrollTop")]
    #[doc = "Setter for the `scrollTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_scroll_top(this: &Element, value: f64);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "scrollLeft")]
    #[doc = "Getter for the `scrollLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn scroll_left(this: &Element) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "Element", js_name = "scrollLeft")]
    #[doc = "Setter for the `scrollLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_scroll_left(this: &Element, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Element",
        js_name = "previousElementSibling"
    )]
    #[doc = "Getter for the `previousElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/previousElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn previous_element_sibling(this: &Element) -> Option<Element>;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "nextElementSibling")]
    #[doc = "Getter for the `nextElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/nextElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn next_element_sibling(this: &Element) -> Option<Element>;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "children")]
    #[doc = "Getter for the `children` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/children)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn children(this: &Element) -> HtmlCollection;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "firstElementChild")]
    #[doc = "Getter for the `firstElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/firstElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn first_element_child(this: &Element) -> Option<Element>;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "lastElementChild")]
    #[doc = "Getter for the `lastElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/lastElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn last_element_child(this: &Element) -> Option<Element>;
    #[wasm_bindgen(method, getter, js_class = "Element", js_name = "childElementCount")]
    #[doc = "Getter for the `childElementCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/childElementCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn child_element_count(this: &Element) -> u32;
    #[cfg(all(feature = "ShadowRoot", feature = "ShadowRootInit",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "attachShadow")]
    #[doc = "The `attachShadow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`, `ShadowRootInit`*"]
    pub fn attach_shadow(
        this: &Element,
        shadow_root_init_dict: &ShadowRootInit,
    ) -> Result<ShadowRoot, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element")]
    #[doc = "The `closest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn closest(this: &Element, selector: &str) -> Result<Option<Element>, JsValue>;
    #[wasm_bindgen(method, js_class = "Element", js_name = "getAttribute")]
    #[doc = "The `getAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_attribute(this: &Element, name: &str) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "Element", js_name = "getAttributeNS")]
    #[doc = "The `getAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "Element", js_name = "getAttributeNames")]
    #[doc = "The `getAttributeNames()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_attribute_names(this: &Element) -> ::js_sys::Array;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getAttributeNode")]
    #[doc = "The `getAttributeNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn get_attribute_node(this: &Element, name: &str) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getAttributeNodeNS")]
    #[doc = "The `getAttributeNodeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn get_attribute_node_ns(
        this: &Element,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Option<Attr>;
    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getBoundingClientRect")]
    #[doc = "The `getBoundingClientRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `Element`*"]
    pub fn get_bounding_client_rect(this: &Element) -> DomRect;
    #[cfg(feature = "DomRectList")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getClientRects")]
    #[doc = "The `getClientRects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectList`, `Element`*"]
    pub fn get_client_rects(this: &Element) -> DomRectList;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getElementsByClassName")]
    #[doc = "The `getElementsByClassName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_elements_by_class_name(this: &Element, class_names: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getElementsByTagName")]
    #[doc = "The `getElementsByTagName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_elements_by_tag_name(this: &Element, local_name: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Element",
        js_name = "getElementsByTagNameNS"
    )]
    #[doc = "The `getElementsByTagNameNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagNameNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_elements_by_tag_name_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<HtmlCollection, JsValue>;
    #[wasm_bindgen(method, js_class = "Element", js_name = "hasAttribute")]
    #[doc = "The `hasAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_attribute(this: &Element, name: &str) -> bool;
    #[wasm_bindgen(method, js_class = "Element", js_name = "hasAttributeNS")]
    #[doc = "The `hasAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_attribute_ns(this: &Element, namespace: Option<&str>, local_name: &str) -> bool;
    #[wasm_bindgen(method, js_class = "Element", js_name = "hasAttributes")]
    #[doc = "The `hasAttributes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_attributes(this: &Element) -> bool;
    #[wasm_bindgen(method, js_class = "Element", js_name = "hasPointerCapture")]
    #[doc = "The `hasPointerCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_pointer_capture(this: &Element, pointer_id: i32) -> bool;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "insertAdjacentElement")]
    #[doc = "The `insertAdjacentElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn insert_adjacent_element(
        this: &Element,
        where_: &str,
        element: &Element,
    ) -> Result<Option<Element>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "insertAdjacentHTML")]
    #[doc = "The `insertAdjacentHTML()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn insert_adjacent_html(this: &Element, position: &str, text: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "insertAdjacentText")]
    #[doc = "The `insertAdjacentText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn insert_adjacent_text(this: &Element, where_: &str, data: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element")]
    #[doc = "The `matches()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/matches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn matches(this: &Element, selector: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "querySelector")]
    #[doc = "The `querySelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn query_selector(this: &Element, selectors: &str) -> Result<Option<Element>, JsValue>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "querySelectorAll")]
    #[doc = "The `querySelectorAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `NodeList`*"]
    pub fn query_selector_all(this: &Element, selectors: &str) -> Result<NodeList, JsValue>;
    #[wasm_bindgen(method, js_class = "Element", js_name = "releaseCapture")]
    #[doc = "The `releaseCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releaseCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn release_capture(this: &Element);
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "releasePointerCapture")]
    #[doc = "The `releasePointerCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn release_pointer_capture(this: &Element, pointer_id: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "removeAttribute")]
    #[doc = "The `removeAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn remove_attribute(this: &Element, name: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "removeAttributeNS")]
    #[doc = "The `removeAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn remove_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "removeAttributeNode")]
    #[doc = "The `removeAttributeNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn remove_attribute_node(this: &Element, old_attr: &Attr) -> Result<Option<Attr>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "requestFullscreen")]
    #[doc = "The `requestFullscreen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn request_fullscreen(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Element", js_name = "requestPointerLock")]
    #[doc = "The `requestPointerLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn request_pointer_lock(this: &Element);
    #[wasm_bindgen(method, js_class = "Element", js_name = "scroll")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_with_x_and_y(this: &Element, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Element")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll(this: &Element);
    #[cfg(feature = "ScrollToOptions")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "scroll")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    pub fn scroll_with_scroll_to_options(this: &Element, options: &ScrollToOptions);
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollBy")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_by_with_x_and_y(this: &Element, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollBy")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_by(this: &Element);
    #[cfg(feature = "ScrollToOptions")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollBy")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    pub fn scroll_by_with_scroll_to_options(this: &Element, options: &ScrollToOptions);
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollIntoView")]
    #[doc = "The `scrollIntoView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_into_view(this: &Element);
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollIntoView")]
    #[doc = "The `scrollIntoView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_into_view_with_bool(this: &Element, arg: bool);
    #[cfg(feature = "ScrollIntoViewOptions")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollIntoView")]
    #[doc = "The `scrollIntoView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollIntoViewOptions`*"]
    pub fn scroll_into_view_with_scroll_into_view_options(
        this: &Element,
        arg: &ScrollIntoViewOptions,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollTo")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_to_with_x_and_y(this: &Element, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollTo")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_to(this: &Element);
    #[cfg(feature = "ScrollToOptions")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "scrollTo")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    pub fn scroll_to_with_scroll_to_options(this: &Element, options: &ScrollToOptions);
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "setAttribute")]
    #[doc = "The `setAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_attribute(this: &Element, name: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "setAttributeNS")]
    #[doc = "The `setAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        name: &str,
        value: &str,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "setAttributeNode")]
    #[doc = "The `setAttributeNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn set_attribute_node(this: &Element, new_attr: &Attr) -> Result<Option<Attr>, JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "setAttributeNodeNS")]
    #[doc = "The `setAttributeNodeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNodeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn set_attribute_node_ns(this: &Element, new_attr: &Attr) -> Result<Option<Attr>, JsValue>;
    #[wasm_bindgen(method, js_class = "Element", js_name = "setCapture")]
    #[doc = "The `setCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_capture(this: &Element);
    #[wasm_bindgen(method, js_class = "Element", js_name = "setCapture")]
    #[doc = "The `setCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_capture_with_retarget_to_element(this: &Element, retarget_to_element: bool);
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "setPointerCapture")]
    #[doc = "The `setPointerCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_pointer_capture(this: &Element, pointer_id: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "toggleAttribute")]
    #[doc = "The `toggleAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn toggle_attribute(this: &Element, name: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "toggleAttribute")]
    #[doc = "The `toggleAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn toggle_attribute_with_force(
        this: &Element,
        name: &str,
        force: bool,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "webkitMatchesSelector")]
    #[doc = "The `webkitMatchesSelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/webkitMatchesSelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn webkit_matches_selector(this: &Element, selector: &str) -> Result<bool, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[wasm_bindgen(method, js_class = "Element")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn animate(this: &Element, keyframes: Option<&::js_sys::Object>) -> Animation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "animate")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn animate_with_f64(
        this: &Element,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Animation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "Animation", feature = "KeyframeAnimationOptions",))]
    #[wasm_bindgen(method, js_class = "Element", js_name = "animate")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn animate_with_keyframe_animation_options(
        this: &Element,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeAnimationOptions,
    ) -> Animation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getAnimations")]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_animations(this: &Element) -> ::js_sys::Array<Animation>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "Animation", feature = "GetAnimationsOptions",))]
    #[wasm_bindgen(method, js_class = "Element", js_name = "getAnimations")]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`, `GetAnimationsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_animations_with_options(
        this: &Element,
        options: &GetAnimationsOptions,
    ) -> ::js_sys::Array<Animation>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_2(this: &Element, nodes_1: &Node, nodes_2: &Node)
        -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Element")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn remove(this: &Element);
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_2(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    pub fn convert_point_from_node_with_text(
        this: &Element,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_element(
        this: &Element,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_document(
        this: &Element,
        point: &DomPointInit,
        from: &Document,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    pub fn convert_point_from_node_with_text_and_options(
        this: &Element,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_element_and_options(
        this: &Element,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_document_and_options(
        this: &Element,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*"]
    pub fn convert_quad_from_node_with_text(
        this: &Element,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomQuad")]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_element(
        this: &Element,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomQuad",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_document(
        this: &Element,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Text",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*"]
    pub fn convert_quad_from_node_with_text_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_element_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_document_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    pub fn convert_rect_from_node_with_text(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_element(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_document(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    pub fn convert_rect_from_node_with_text_and_options(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_element_and_options(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_document_and_options(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "getBoxQuads")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_box_quads(this: &Element) -> Result<::js_sys::Array, JsValue>;
    #[cfg(feature = "BoxQuadOptions")]
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "getBoxQuads")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Element`*"]
    pub fn get_box_quads_with_options(
        this: &Element,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_0(this: &Element) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Element", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, variadic, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node(this: &Element, nodes: &::js_sys::Array);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_0(this: &Element);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_1(this: &Element, nodes_1: &Node);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_2(this: &Element, nodes_1: &Node, nodes_2: &Node);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    );
    #[wasm_bindgen(method, variadic, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str(this: &Element, nodes: &::js_sys::Array);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_0(this: &Element);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_1(this: &Element, nodes_1: &str);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_3(this: &Element, nodes_1: &str, nodes_2: &str, nodes_3: &str);
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    );
    #[wasm_bindgen(method, js_class = "Element", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_children_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    );
}
