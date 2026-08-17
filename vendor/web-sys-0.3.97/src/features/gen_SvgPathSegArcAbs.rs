#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegArcAbs" , typescript_type = "SVGPathSegArcAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegArcAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub type SvgPathSegArcAbs;
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcAbs", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn x(this: &SvgPathSegArcAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcAbs", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn set_x(this: &SvgPathSegArcAbs, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcAbs", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn y(this: &SvgPathSegArcAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcAbs", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn set_y(this: &SvgPathSegArcAbs, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcAbs", js_name = "r1")]
    #[doc = "Getter for the `r1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn r1(this: &SvgPathSegArcAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcAbs", js_name = "r1")]
    #[doc = "Setter for the `r1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn set_r1(this: &SvgPathSegArcAbs, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcAbs", js_name = "r2")]
    #[doc = "Getter for the `r2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn r2(this: &SvgPathSegArcAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcAbs", js_name = "r2")]
    #[doc = "Setter for the `r2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn set_r2(this: &SvgPathSegArcAbs, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcAbs", js_name = "angle")]
    #[doc = "Getter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn angle(this: &SvgPathSegArcAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcAbs", js_name = "angle")]
    #[doc = "Setter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn set_angle(this: &SvgPathSegArcAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegArcAbs",
        js_name = "largeArcFlag"
    )]
    #[doc = "Getter for the `largeArcFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/largeArcFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn large_arc_flag(this: &SvgPathSegArcAbs) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegArcAbs",
        js_name = "largeArcFlag"
    )]
    #[doc = "Setter for the `largeArcFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/largeArcFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn set_large_arc_flag(this: &SvgPathSegArcAbs, value: bool);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegArcAbs", js_name = "sweepFlag")]
    #[doc = "Getter for the `sweepFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/sweepFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn sweep_flag(this: &SvgPathSegArcAbs) -> bool;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegArcAbs", js_name = "sweepFlag")]
    #[doc = "Setter for the `sweepFlag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/sweepFlag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*"]
    pub fn set_sweep_flag(this: &SvgPathSegArcAbs, value: bool);
}
