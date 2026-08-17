#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssConditionRule",
        extends = "CssGroupingRule",
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSSupportsRule",
        typescript_type = "CSSSupportsRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssSupportsRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSSupportsRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssSupportsRule`*"]
    pub type CssSupportsRule;
}
