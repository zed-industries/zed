#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Plugin",
        typescript_type = "Plugin"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Plugin` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
    pub type Plugin;
    #[wasm_bindgen(method, getter, js_class = "Plugin", js_name = "description")]
    #[doc = "Getter for the `description` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/description)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
    pub fn description(this: &Plugin) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Plugin", js_name = "filename")]
    #[doc = "Getter for the `filename` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/filename)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
    pub fn filename(this: &Plugin) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Plugin", js_name = "version")]
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
    pub fn version(this: &Plugin) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Plugin", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
    pub fn name(this: &Plugin) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Plugin", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
    pub fn length(this: &Plugin) -> u32;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "Plugin")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    pub fn item(this: &Plugin, index: u32) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "Plugin", js_name = "namedItem")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    pub fn named_item(this: &Plugin, name: &str) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "Plugin", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    pub fn get_with_index(this: &Plugin, index: u32) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "Plugin", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    pub fn get_with_name(this: &Plugin, name: &str) -> Option<MimeType>;
}
