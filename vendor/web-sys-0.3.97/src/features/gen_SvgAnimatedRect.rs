#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedRect",
        typescript_type = "SVGAnimatedRect"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedRect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`*"]
    pub type SvgAnimatedRect;
    #[cfg(feature = "SvgRect")]
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedRect", js_name = "baseVal")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgRect`*"]
    pub fn base_val(this: &SvgAnimatedRect) -> Option<SvgRect>;
    #[cfg(feature = "SvgRect")]
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedRect", js_name = "animVal")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgRect`*"]
    pub fn anim_val(this: &SvgAnimatedRect) -> Option<SvgRect>;
}
