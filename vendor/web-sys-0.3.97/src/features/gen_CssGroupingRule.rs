#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSGroupingRule",
        typescript_type = "CSSGroupingRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssGroupingRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub type CssGroupingRule;
    #[cfg(feature = "CssRuleList")]
    #[wasm_bindgen(method, getter, js_class = "CSSGroupingRule", js_name = "cssRules")]
    #[doc = "Getter for the `cssRules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/cssRules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`, `CssRuleList`*"]
    pub fn css_rules(this: &CssGroupingRule) -> CssRuleList;
    #[wasm_bindgen(catch, method, js_class = "CSSGroupingRule", js_name = "deleteRule")]
    #[doc = "The `deleteRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/deleteRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub fn delete_rule(this: &CssGroupingRule, index: u32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CSSGroupingRule", js_name = "insertRule")]
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub fn insert_rule(this: &CssGroupingRule, rule: &str) -> Result<u32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CSSGroupingRule", js_name = "insertRule")]
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub fn insert_rule_with_index(
        this: &CssGroupingRule,
        rule: &str,
        index: u32,
    ) -> Result<u32, JsValue>;
}
