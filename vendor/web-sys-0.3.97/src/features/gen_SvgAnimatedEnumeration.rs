#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedEnumeration",
        typescript_type = "SVGAnimatedEnumeration"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedEnumeration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub type SvgAnimatedEnumeration;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedEnumeration",
        js_name = "baseVal"
    )]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub fn base_val(this: &SvgAnimatedEnumeration) -> u16;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGAnimatedEnumeration",
        js_name = "baseVal"
    )]
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub fn set_base_val(this: &SvgAnimatedEnumeration, value: u16);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedEnumeration",
        js_name = "animVal"
    )]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub fn anim_val(this: &SvgAnimatedEnumeration) -> u16;
}
