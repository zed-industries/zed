#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedPreserveAspectRatio",
        typescript_type = "SVGAnimatedPreserveAspectRatio"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedPreserveAspectRatio` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`*"]
    pub type SvgAnimatedPreserveAspectRatio;
    #[cfg(feature = "SvgPreserveAspectRatio")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedPreserveAspectRatio",
        js_name = "baseVal"
    )]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*"]
    pub fn base_val(this: &SvgAnimatedPreserveAspectRatio) -> SvgPreserveAspectRatio;
    #[cfg(feature = "SvgPreserveAspectRatio")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedPreserveAspectRatio",
        js_name = "animVal"
    )]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*"]
    pub fn anim_val(this: &SvgAnimatedPreserveAspectRatio) -> SvgPreserveAspectRatio;
}
