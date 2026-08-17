#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSFontFeatureValuesRule",
        typescript_type = "CSSFontFeatureValuesRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssFontFeatureValuesRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub type CssFontFeatureValuesRule;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSFontFeatureValuesRule",
        js_name = "fontFamily"
    )]
    #[doc = "Getter for the `fontFamily` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn font_family(this: &CssFontFeatureValuesRule) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CSSFontFeatureValuesRule",
        js_name = "fontFamily"
    )]
    #[doc = "Setter for the `fontFamily` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn set_font_family(this: &CssFontFeatureValuesRule, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSFontFeatureValuesRule",
        js_name = "valueText"
    )]
    #[doc = "Getter for the `valueText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn value_text(this: &CssFontFeatureValuesRule) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CSSFontFeatureValuesRule",
        js_name = "valueText"
    )]
    #[doc = "Setter for the `valueText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn set_value_text(this: &CssFontFeatureValuesRule, value: &str);
}
