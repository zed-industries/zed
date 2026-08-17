#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "StyleSheet",
        extends = "::js_sys::Object",
        js_name = "CSSStyleSheet",
        typescript_type = "CSSStyleSheet"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleSheet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub type CssStyleSheet;
    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(method, getter, js_class = "CSSStyleSheet", js_name = "ownerRule")]
    #[doc = "Getter for the `ownerRule` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/ownerRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*"]
    pub fn owner_rule(this: &CssStyleSheet) -> Option<CssRule>;
    #[cfg(feature = "CssRuleList")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "CSSStyleSheet",
        js_name = "cssRules"
    )]
    #[doc = "Getter for the `cssRules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/cssRules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRuleList`, `CssStyleSheet`*"]
    pub fn css_rules(this: &CssStyleSheet) -> Result<CssRuleList, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "CSSStyleSheet")]
    #[doc = "The `new CssStyleSheet(..)` constructor, creating a new instance of `CssStyleSheet`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/CSSStyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn new() -> Result<CssStyleSheet, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CSSStyleSheet", js_name = "deleteRule")]
    #[doc = "The `deleteRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/deleteRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn delete_rule(this: &CssStyleSheet, index: u32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CSSStyleSheet", js_name = "insertRule")]
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn insert_rule(this: &CssStyleSheet, rule: &str) -> Result<u32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CSSStyleSheet", js_name = "insertRule")]
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn insert_rule_with_index(
        this: &CssStyleSheet,
        rule: &str,
        index: u32,
    ) -> Result<u32, JsValue>;
    #[wasm_bindgen(method, js_class = "CSSStyleSheet")]
    #[doc = "The `replace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn replace(this: &CssStyleSheet, text: &str) -> ::js_sys::Promise;
    #[wasm_bindgen(catch, method, js_class = "CSSStyleSheet", js_name = "replaceSync")]
    #[doc = "The `replaceSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replaceSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn replace_sync(this: &CssStyleSheet, text: &str) -> Result<(), JsValue>;
}
