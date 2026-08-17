#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "StyleSheet",
        typescript_type = "StyleSheet"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub type StyleSheet;
    #[wasm_bindgen(method, getter, js_class = "StyleSheet", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn type_(this: &StyleSheet) -> ::alloc::string::String;
    #[wasm_bindgen(catch, method, getter, js_class = "StyleSheet", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn href(this: &StyleSheet) -> Result<Option<::alloc::string::String>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "StyleSheet", js_name = "ownerNode")]
    #[doc = "Getter for the `ownerNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/ownerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `StyleSheet`*"]
    pub fn owner_node(this: &StyleSheet) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "StyleSheet", js_name = "parentStyleSheet")]
    #[doc = "Getter for the `parentStyleSheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/parentStyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn parent_style_sheet(this: &StyleSheet) -> Option<StyleSheet>;
    #[wasm_bindgen(method, getter, js_class = "StyleSheet", js_name = "title")]
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn title(this: &StyleSheet) -> Option<::alloc::string::String>;
    #[cfg(feature = "MediaList")]
    #[wasm_bindgen(method, getter, js_class = "StyleSheet", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`, `StyleSheet`*"]
    pub fn media(this: &StyleSheet) -> MediaList;
    #[wasm_bindgen(method, getter, js_class = "StyleSheet", js_name = "disabled")]
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn disabled(this: &StyleSheet) -> bool;
    #[wasm_bindgen(method, setter, js_class = "StyleSheet", js_name = "disabled")]
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn set_disabled(this: &StyleSheet, value: bool);
}
