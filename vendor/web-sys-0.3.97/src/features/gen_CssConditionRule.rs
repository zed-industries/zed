#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssGroupingRule",
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSConditionRule",
        typescript_type = "CSSConditionRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssConditionRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
    pub type CssConditionRule;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSConditionRule",
        js_name = "conditionText"
    )]
    #[doc = "Getter for the `conditionText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
    pub fn condition_text(this: &CssConditionRule) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CSSConditionRule",
        js_name = "conditionText"
    )]
    #[doc = "Setter for the `conditionText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
    pub fn set_condition_text(this: &CssConditionRule, value: &str);
}
