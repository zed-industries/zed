#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedTransformList",
        typescript_type = "SVGAnimatedTransformList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedTransformList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`*"]
    pub type SvgAnimatedTransformList;
    #[cfg(feature = "SvgTransformList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedTransformList",
        js_name = "baseVal"
    )]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*"]
    pub fn base_val(this: &SvgAnimatedTransformList) -> SvgTransformList;
    #[cfg(feature = "SvgTransformList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedTransformList",
        js_name = "animVal"
    )]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*"]
    pub fn anim_val(this: &SvgAnimatedTransformList) -> SvgTransformList;
}
