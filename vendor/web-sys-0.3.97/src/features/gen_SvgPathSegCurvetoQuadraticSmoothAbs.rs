#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegCurvetoQuadraticSmoothAbs" , typescript_type = "SVGPathSegCurvetoQuadraticSmoothAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoQuadraticSmoothAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub type SvgPathSegCurvetoQuadraticSmoothAbs;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticSmoothAbs",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn x(this: &SvgPathSegCurvetoQuadraticSmoothAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticSmoothAbs",
        js_name = "x"
    )]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn set_x(this: &SvgPathSegCurvetoQuadraticSmoothAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticSmoothAbs",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn y(this: &SvgPathSegCurvetoQuadraticSmoothAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticSmoothAbs",
        js_name = "y"
    )]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn set_y(this: &SvgPathSegCurvetoQuadraticSmoothAbs, value: f32);
}
