#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedAngle",
        typescript_type = "SVGAnimatedAngle"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedAngle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedAngle`*"]
    pub type SvgAnimatedAngle;
    #[cfg(feature = "SvgAngle")]
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedAngle", js_name = "baseVal")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*"]
    pub fn base_val(this: &SvgAnimatedAngle) -> SvgAngle;
    #[cfg(feature = "SvgAngle")]
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedAngle", js_name = "animVal")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*"]
    pub fn anim_val(this: &SvgAnimatedAngle) -> SvgAngle;
}
