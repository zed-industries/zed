#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSImportRule",
        typescript_type = "CSSImportRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssImportRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`*"]
    pub type CssImportRule;
    #[wasm_bindgen(method, getter, js_class = "CSSImportRule", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`*"]
    pub fn href(this: &CssImportRule) -> ::alloc::string::String;
    #[cfg(feature = "MediaList")]
    #[wasm_bindgen(method, getter, js_class = "CSSImportRule", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`, `MediaList`*"]
    pub fn media(this: &CssImportRule) -> Option<MediaList>;
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, getter, js_class = "CSSImportRule", js_name = "styleSheet")]
    #[doc = "Getter for the `styleSheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/styleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`, `CssStyleSheet`*"]
    pub fn style_sheet(this: &CssImportRule) -> Option<CssStyleSheet>;
}
