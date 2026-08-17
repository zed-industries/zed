#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSKeyframeRule",
        typescript_type = "CSSKeyframeRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssKeyframeRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    pub type CssKeyframeRule;
    #[wasm_bindgen(method, getter, js_class = "CSSKeyframeRule", js_name = "keyText")]
    #[doc = "Getter for the `keyText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    pub fn key_text(this: &CssKeyframeRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSKeyframeRule", js_name = "keyText")]
    #[doc = "Setter for the `keyText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    pub fn set_key_text(this: &CssKeyframeRule, value: &str);
    #[cfg(feature = "CssStyleDeclaration")]
    #[wasm_bindgen(method, getter, js_class = "CSSKeyframeRule", js_name = "style")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssStyleDeclaration`*"]
    pub fn style(this: &CssKeyframeRule) -> CssStyleDeclaration;
}
