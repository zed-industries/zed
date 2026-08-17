#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedNumberList",
        typescript_type = "SVGAnimatedNumberList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedNumberList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`*"]
    pub type SvgAnimatedNumberList;
    #[cfg(feature = "SvgNumberList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedNumberList",
        js_name = "baseVal"
    )]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
    pub fn base_val(this: &SvgAnimatedNumberList) -> SvgNumberList;
    #[cfg(feature = "SvgNumberList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimatedNumberList",
        js_name = "animVal"
    )]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
    pub fn anim_val(this: &SvgAnimatedNumberList) -> SvgNumberList;
}
