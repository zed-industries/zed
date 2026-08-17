#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSStyleRule",
        typescript_type = "CSSStyleRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleRule`*"]
    pub type CssStyleRule;
    #[wasm_bindgen(method, getter, js_class = "CSSStyleRule", js_name = "selectorText")]
    #[doc = "Getter for the `selectorText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleRule`*"]
    pub fn selector_text(this: &CssStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSStyleRule", js_name = "selectorText")]
    #[doc = "Setter for the `selectorText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleRule`*"]
    pub fn set_selector_text(this: &CssStyleRule, value: &str);
    #[cfg(feature = "CssStyleDeclaration")]
    #[wasm_bindgen(method, getter, js_class = "CSSStyleRule", js_name = "style")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `CssStyleRule`*"]
    pub fn style(this: &CssStyleRule) -> CssStyleDeclaration;
}
