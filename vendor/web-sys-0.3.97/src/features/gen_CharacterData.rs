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
        js_name = "CharacterData",
        typescript_type = "CharacterData"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CharacterData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub type CharacterData;
    #[wasm_bindgen(method, getter, js_class = "CharacterData", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn data(this: &CharacterData) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CharacterData", js_name = "data")]
    #[doc = "Setter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn set_data(this: &CharacterData, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CharacterData", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn length(this: &CharacterData) -> u32;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CharacterData",
        js_name = "previousElementSibling"
    )]
    #[doc = "Getter for the `previousElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/previousElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
    pub fn previous_element_sibling(this: &CharacterData) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CharacterData",
        js_name = "nextElementSibling"
    )]
    #[doc = "Getter for the `nextElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/nextElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
    pub fn next_element_sibling(this: &CharacterData) -> Option<Element>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "appendData")]
    #[doc = "The `appendData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/appendData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn append_data(this: &CharacterData, data: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "deleteData")]
    #[doc = "The `deleteData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/deleteData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn delete_data(this: &CharacterData, offset: u32, count: u32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "insertData")]
    #[doc = "The `insertData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/insertData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn insert_data(this: &CharacterData, offset: u32, data: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceData")]
    #[doc = "The `replaceData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_data(
        this: &CharacterData,
        offset: u32,
        count: u32,
        data: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "substringData")]
    #[doc = "The `substringData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/substringData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn substring_data(
        this: &CharacterData,
        offset: u32,
        count: u32,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node(this: &CharacterData, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_0(this: &CharacterData) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_1(this: &CharacterData, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_2(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_3(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_4(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_5(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_6(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_7(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str(this: &CharacterData, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_0(this: &CharacterData) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_1(this: &CharacterData, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_2(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_3(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_4(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_5(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_6(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "after")]
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_7(
        this: &CharacterData,
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
        js_class = "CharacterData",
        js_name = "before"
    )]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node(this: &CharacterData, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_0(this: &CharacterData) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_1(this: &CharacterData, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_2(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_3(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_4(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_5(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_6(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_7(
        this: &CharacterData,
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
        js_class = "CharacterData",
        js_name = "before"
    )]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str(this: &CharacterData, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_0(this: &CharacterData) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_1(this: &CharacterData, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_2(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_3(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_4(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_5(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_6(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "before")]
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_7(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "CharacterData")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn remove(this: &CharacterData);
    #[wasm_bindgen(
        catch,
        method,
        variadic,
        js_class = "CharacterData",
        js_name = "replaceWith"
    )]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node(
        this: &CharacterData,
        nodes: &::js_sys::Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_0(this: &CharacterData) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_1(this: &CharacterData, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_2(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_3(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_4(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_5(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_6(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_7(
        this: &CharacterData,
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
        js_class = "CharacterData",
        js_name = "replaceWith"
    )]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str(
        this: &CharacterData,
        nodes: &::js_sys::Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_0(this: &CharacterData) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_1(this: &CharacterData, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_2(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_3(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_4(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_5(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_6(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CharacterData", js_name = "replaceWith")]
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_7(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
}
