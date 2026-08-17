#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegArcRel" , typescript_type = "SVGPathSegArcRel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegArcRel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub type SvgPathSegArcRel;
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcRel", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn x(this: &SvgPathSegArcRel) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcRel", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_x(this: &SvgPathSegArcRel, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcRel", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn y(this: &SvgPathSegArcRel) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcRel", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_y(this: &SvgPathSegArcRel, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcRel", js_name = "r1")]
    #[doc = "Getter for the `r1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn r1(this: &SvgPathSegArcRel) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcRel", js_name = "r1")]
    #[doc = "Setter for the `r1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_r1(this: &SvgPathSegArcRel, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcRel", js_name = "r2")]
    #[doc = "Getter for the `r2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn r2(this: &SvgPathSegArcRel) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcRel", js_name = "r2")]
    #[doc = "Setter for the `r2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_r2(this: &SvgPathSegArcRel, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcRel", js_name = "angle")]
    #[doc = "Getter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn angle(this: &SvgPathSegArcRel) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcRel", js_name = "angle")]
    #[doc = "Setter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_angle(this: &SvgPathSegArcRel, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegArcRel",
        js_name = "largeArcFlag"
    )]
    #[doc = "Getter for the `largeArcFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn large_arc_flag(this: &SvgPathSegArcRel) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegArcRel",
        js_name = "largeArcFlag"
    )]
    #[doc = "Setter for the `largeArcFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_large_arc_flag(this: &SvgPathSegArcRel, value: bool);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcRel", js_name = "sweepFlag")]
    #[doc = "Getter for the `sweepFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn sweep_flag(this: &SvgPathSegArcRel) -> bool;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcRel", js_name = "sweepFlag")]
    #[doc = "Setter for the `sweepFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_sweep_flag(this: &SvgPathSegArcRel, value: bool);
}
