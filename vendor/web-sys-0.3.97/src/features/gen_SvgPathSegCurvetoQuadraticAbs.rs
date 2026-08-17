#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegCurvetoQuadraticAbs" , typescript_type = "SVGPathSegCurvetoQuadraticAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoQuadraticAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub type SvgPathSegCurvetoQuadraticAbs;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn x(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "x"
    )]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn set_x(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn y(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "y"
    )]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn set_y(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "x1"
    )]
    #[doc = "Getter for the `x1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn x1(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "x1"
    )]
    #[doc = "Setter for the `x1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn set_x1(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "y1"
    )]
    #[doc = "Getter for the `y1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn y1(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticAbs",
        js_name = "y1"
    )]
    #[doc = "Setter for the `y1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*"]
    pub fn set_y1(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);
}
