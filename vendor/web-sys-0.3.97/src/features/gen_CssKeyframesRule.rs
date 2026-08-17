#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSKeyframesRule",
        typescript_type = "CSSKeyframesRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssKeyframesRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub type CssKeyframesRule;
    #[wasm_bindgen(method, getter, js_class = "CSSKeyframesRule", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn name(this: &CssKeyframesRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSKeyframesRule", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn set_name(this: &CssKeyframesRule, value: &str);
    #[cfg(feature = "CssRuleList")]
    #[wasm_bindgen(method, getter, js_class = "CSSKeyframesRule", js_name = "cssRules")]
    #[doc = "Getter for the `cssRules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/cssRules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`, `CssRuleList`*"]
    pub fn css_rules(this: &CssKeyframesRule) -> CssRuleList;
    #[wasm_bindgen(method, js_class = "CSSKeyframesRule", js_name = "appendRule")]
    #[doc = "The `appendRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/appendRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn append_rule(this: &CssKeyframesRule, rule: &str);
    #[wasm_bindgen(method, js_class = "CSSKeyframesRule", js_name = "deleteRule")]
    #[doc = "The `deleteRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/deleteRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn delete_rule(this: &CssKeyframesRule, select: &str);
    #[cfg(feature = "CssKeyframeRule")]
    #[wasm_bindgen(method, js_class = "CSSKeyframesRule", js_name = "findRule")]
    #[doc = "The `findRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/findRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssKeyframesRule`*"]
    pub fn find_rule(this: &CssKeyframesRule, select: &str) -> Option<CssKeyframeRule>;
}
