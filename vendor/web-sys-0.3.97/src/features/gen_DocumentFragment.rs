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
        js_name = "DocumentFragment",
        typescript_type = "DocumentFragment"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentFragment` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub type DocumentFragment;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "DocumentFragment", js_name = "children")]
    #[doc = "Getter for the `children` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/children)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `HtmlCollection`*"]
    pub fn children(this: &DocumentFragment) -> HtmlCollection;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DocumentFragment",
        js_name = "firstElementChild"
    )]
    #[doc = "Getter for the `firstElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/firstElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    pub fn first_element_child(this: &DocumentFragment) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DocumentFragment",
        js_name = "lastElementChild"
    )]
    #[doc = "Getter for the `lastElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/lastElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    pub fn last_element_child(this: &DocumentFragment) -> Option<Element>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DocumentFragment",
        js_name = "childElementCount"
    )]
    #[doc = "Getter for the `childElementCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/childElementCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn child_element_count(this: &DocumentFragment) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "DocumentFragment")]
    #[doc = "The `new DocumentFragment(..)` constructor, creating a new instance of `DocumentFragment`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/DocumentFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn new() -> Result<DocumentFragment, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "getElementById")]
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    pub fn get_element_by_id(this: &DocumentFragment, element_id: &str) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DocumentFragment",
        js_name = "querySelector"
    )]
    #[doc = "The `querySelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    pub fn query_selector(
        this: &DocumentFragment,
        selectors: &str,
    ) -> Result<Option<Element>, JsValue>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DocumentFragment",
        js_name = "querySelectorAll"
    )]
    #[doc = "The `querySelectorAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelectorAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `NodeList`*"]
    pub fn query_selector_all(
        this: &DocumentFragment,
        selectors: &str,
    ) -> Result<NodeList, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        variadic,
        js_class = "DocumentFragment",
        js_name = "append"
    )]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node(
        this: &DocumentFragment,
        nodes: &::js_sys::Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_0(this: &DocumentFragment) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_1(this: &DocumentFragment, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_2(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_3(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_4(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_5(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_6(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_node_7(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        variadic,
        js_class = "DocumentFragment",
        js_name = "append"
    )]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str(this: &DocumentFragment, nodes: &::js_sys::Array)
        -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_0(this: &DocumentFragment) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_1(this: &DocumentFragment, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_2(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_3(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_4(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_5(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_6(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn append_with_str_7(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        variadic,
        js_class = "DocumentFragment",
        js_name = "prepend"
    )]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node(
        this: &DocumentFragment,
        nodes: &::js_sys::Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_0(this: &DocumentFragment) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_1(this: &DocumentFragment, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_2(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_3(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_4(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_5(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_6(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_node_7(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        variadic,
        js_class = "DocumentFragment",
        js_name = "prepend"
    )]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str(
        this: &DocumentFragment,
        nodes: &::js_sys::Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_0(this: &DocumentFragment) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_1(this: &DocumentFragment, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_2(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_3(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_4(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_5(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_6(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DocumentFragment", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn prepend_with_str_7(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        variadic,
        js_class = "DocumentFragment",
        js_name = "replaceChildren"
    )]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node(this: &DocumentFragment, nodes: &::js_sys::Array);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_0(this: &DocumentFragment);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_1(this: &DocumentFragment, nodes_1: &Node);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_2(this: &DocumentFragment, nodes_1: &Node, nodes_2: &Node);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_3(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_4(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_5(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_6(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_node_7(
        this: &DocumentFragment,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    );
    #[wasm_bindgen(
        method,
        variadic,
        js_class = "DocumentFragment",
        js_name = "replaceChildren"
    )]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str(this: &DocumentFragment, nodes: &::js_sys::Array);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_0(this: &DocumentFragment);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_1(this: &DocumentFragment, nodes_1: &str);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_2(this: &DocumentFragment, nodes_1: &str, nodes_2: &str);
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_3(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_4(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_5(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_6(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    );
    #[wasm_bindgen(method, js_class = "DocumentFragment", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
    pub fn replace_children_with_str_7(
        this: &DocumentFragment,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    );
}
