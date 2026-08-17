#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedInteger",
        typescript_type = "SVGAnimatedInteger"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedInteger` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub type SvgAnimatedInteger;
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedInteger", js_name = "baseVal")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub fn base_val(this: &SvgAnimatedInteger) -> i32;
    #[wasm_bindgen(method, setter, js_class = "SVGAnimatedInteger", js_name = "baseVal")]
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub fn set_base_val(this: &SvgAnimatedInteger, value: i32);
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedInteger", js_name = "animVal")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub fn anim_val(this: &SvgAnimatedInteger) -> i32;
}
