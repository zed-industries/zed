#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegCurvetoQuadraticRel" , typescript_type = "SVGPathSegCurvetoQuadraticRel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoQuadraticRel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub type SvgPathSegCurvetoQuadraticRel;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn x(this: &SvgPathSegCurvetoQuadraticRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "x"
    )]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn set_x(this: &SvgPathSegCurvetoQuadraticRel, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn y(this: &SvgPathSegCurvetoQuadraticRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "y"
    )]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn set_y(this: &SvgPathSegCurvetoQuadraticRel, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "x1"
    )]
    #[doc = "Getter for the `x1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/x1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn x1(this: &SvgPathSegCurvetoQuadraticRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "x1"
    )]
    #[doc = "Setter for the `x1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/x1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn set_x1(this: &SvgPathSegCurvetoQuadraticRel, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "y1"
    )]
    #[doc = "Getter for the `y1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/y1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn y1(this: &SvgPathSegCurvetoQuadraticRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoQuadraticRel",
        js_name = "y1"
    )]
    #[doc = "Setter for the `y1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticRel/y1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticRel`*"]
    pub fn set_y1(this: &SvgPathSegCurvetoQuadraticRel, value: f32);
}
