#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NamedNodeMap",
        typescript_type = "NamedNodeMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NamedNodeMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NamedNodeMap`*"]
    pub type NamedNodeMap;
    #[wasm_bindgen(method, getter, js_class = "NamedNodeMap", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NamedNodeMap`*"]
    pub fn length(this: &NamedNodeMap) -> u32;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, js_class = "NamedNodeMap", js_name = "getNamedItem")]
    #[doc = "The `getNamedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_named_item(this: &NamedNodeMap, name: &str) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, js_class = "NamedNodeMap", js_name = "getNamedItemNS")]
    #[doc = "The `getNamedItemNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItemNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_named_item_ns(
        this: &NamedNodeMap,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, js_class = "NamedNodeMap")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn item(this: &NamedNodeMap, index: u32) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "NamedNodeMap", js_name = "removeNamedItem")]
    #[doc = "The `removeNamedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn remove_named_item(this: &NamedNodeMap, name: &str) -> Result<Attr, JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "NamedNodeMap",
        js_name = "removeNamedItemNS"
    )]
    #[doc = "The `removeNamedItemNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItemNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn remove_named_item_ns(
        this: &NamedNodeMap,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Result<Attr, JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "NamedNodeMap", js_name = "setNamedItem")]
    #[doc = "The `setNamedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn set_named_item(this: &NamedNodeMap, arg: &Attr) -> Result<Option<Attr>, JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "NamedNodeMap", js_name = "setNamedItemNS")]
    #[doc = "The `setNamedItemNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItemNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn set_named_item_ns(this: &NamedNodeMap, arg: &Attr) -> Result<Option<Attr>, JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, js_class = "NamedNodeMap", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_with_name(this: &NamedNodeMap, name: &str) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, js_class = "NamedNodeMap", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_with_index(this: &NamedNodeMap, index: u32) -> Option<Attr>;
}
