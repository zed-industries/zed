#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CSSRuleList",
        typescript_type = "CSSRuleList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssRuleList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRuleList`*"]
    pub type CssRuleList;
    #[wasm_bindgen(method, getter, js_class = "CSSRuleList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRuleList`*"]
    pub fn length(this: &CssRuleList) -> u32;
    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(method, js_class = "CSSRuleList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `CssRuleList`*"]
    pub fn item(this: &CssRuleList, index: u32) -> Option<CssRule>;
    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(method, js_class = "CSSRuleList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `CssRuleList`*"]
    pub fn get(this: &CssRuleList, index: u32) -> Option<CssRule>;
}
