#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedString",
        typescript_type = "SVGAnimatedString"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedString` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub type SvgAnimatedString;
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedString", js_name = "baseVal")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub fn base_val(this: &SvgAnimatedString) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAnimatedString", js_name = "baseVal")]
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub fn set_base_val(this: &SvgAnimatedString, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedString", js_name = "animVal")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub fn anim_val(this: &SvgAnimatedString) -> ::alloc::string::String;
}
