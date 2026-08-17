#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CSSRule",
        typescript_type = "CSSRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub type CssRule;
    #[wasm_bindgen(method, getter, js_class = "CSSRule", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn type_(this: &CssRule) -> u16;
    #[wasm_bindgen(method, getter, js_class = "CSSRule", js_name = "cssText")]
    #[doc = "Getter for the `cssText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn css_text(this: &CssRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSRule", js_name = "cssText")]
    #[doc = "Setter for the `cssText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn set_css_text(this: &CssRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSRule", js_name = "parentRule")]
    #[doc = "Getter for the `parentRule` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn parent_rule(this: &CssRule) -> Option<CssRule>;
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, getter, js_class = "CSSRule", js_name = "parentStyleSheet")]
    #[doc = "Getter for the `parentStyleSheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentStyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*"]
    pub fn parent_style_sheet(this: &CssRule) -> Option<CssStyleSheet>;
}
impl CssRule {
    #[doc = "The `CSSRule.STYLE_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const STYLE_RULE: u16 = 1u64 as u16;
    #[doc = "The `CSSRule.CHARSET_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const CHARSET_RULE: u16 = 2u64 as u16;
    #[doc = "The `CSSRule.IMPORT_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const IMPORT_RULE: u16 = 3u64 as u16;
    #[doc = "The `CSSRule.MEDIA_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const MEDIA_RULE: u16 = 4u64 as u16;
    #[doc = "The `CSSRule.FONT_FACE_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const FONT_FACE_RULE: u16 = 5u64 as u16;
    #[doc = "The `CSSRule.PAGE_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const PAGE_RULE: u16 = 6u64 as u16;
    #[doc = "The `CSSRule.NAMESPACE_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const NAMESPACE_RULE: u16 = 10u64 as u16;
    #[doc = "The `CSSRule.KEYFRAMES_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const KEYFRAMES_RULE: u16 = 7u64 as u16;
    #[doc = "The `CSSRule.KEYFRAME_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const KEYFRAME_RULE: u16 = 8u64 as u16;
    #[doc = "The `CSSRule.COUNTER_STYLE_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const COUNTER_STYLE_RULE: u16 = 11u64 as u16;
    #[doc = "The `CSSRule.SUPPORTS_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const SUPPORTS_RULE: u16 = 12u64 as u16;
    #[doc = "The `CSSRule.FONT_FEATURE_VALUES_RULE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`*"]
    pub const FONT_FEATURE_VALUES_RULE: u16 = 14u64 as u16;
}
