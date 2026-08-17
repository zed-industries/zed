#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSPageRule",
        typescript_type = "CSSPageRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssPageRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPageRule`*"]
    pub type CssPageRule;
    #[cfg(feature = "CssStyleDeclaration")]
    #[wasm_bindgen(method, getter, js_class = "CSSPageRule", js_name = "style")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPageRule`, `CssStyleDeclaration`*"]
    pub fn style(this: &CssPageRule) -> CssStyleDeclaration;
}
