#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PluginArray",
        typescript_type = "PluginArray"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PluginArray` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
    pub type PluginArray;
    #[wasm_bindgen(method, getter, js_class = "PluginArray", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
    pub fn length(this: &PluginArray) -> u32;
    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, js_class = "PluginArray")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn item(this: &PluginArray, index: u32) -> Option<Plugin>;
    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, js_class = "PluginArray", js_name = "namedItem")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn named_item(this: &PluginArray, name: &str) -> Option<Plugin>;
    #[wasm_bindgen(method, js_class = "PluginArray")]
    #[doc = "The `refresh()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
    pub fn refresh(this: &PluginArray);
    #[wasm_bindgen(method, js_class = "PluginArray", js_name = "refresh")]
    #[doc = "The `refresh()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
    pub fn refresh_with_reload_documents(this: &PluginArray, reload_documents: bool);
    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, js_class = "PluginArray", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn get_with_index(this: &PluginArray, index: u32) -> Option<Plugin>;
    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, js_class = "PluginArray", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn get_with_name(this: &PluginArray, name: &str) -> Option<Plugin>;
}
