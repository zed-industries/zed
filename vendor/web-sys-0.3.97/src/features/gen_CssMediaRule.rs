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
        js_name = "CSSMediaRule",
        typescript_type = "CSSMediaRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssMediaRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssMediaRule`*"]
    pub type CssMediaRule;
    #[cfg(feature = "MediaList")]
    #[wasm_bindgen(method, getter, js_class = "CSSMediaRule", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssMediaRule`, `MediaList`*"]
    pub fn media(this: &CssMediaRule) -> MediaList;
}
