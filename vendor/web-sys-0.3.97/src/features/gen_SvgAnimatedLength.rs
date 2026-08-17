#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedLength",
        typescript_type = "SVGAnimatedLength"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedLength` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`*"]
    pub type SvgAnimatedLength;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedLength", js_name = "baseVal")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
    pub fn base_val(this: &SvgAnimatedLength) -> SvgLength;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedLength", js_name = "animVal")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
    pub fn anim_val(this: &SvgAnimatedLength) -> SvgLength;
}
