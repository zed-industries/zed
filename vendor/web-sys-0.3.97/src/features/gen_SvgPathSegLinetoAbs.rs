#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegLinetoAbs" , typescript_type = "SVGPathSegLinetoAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*"]
    pub type SvgPathSegLinetoAbs;
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegLinetoAbs", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*"]
    pub fn x(this: &SvgPathSegLinetoAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegLinetoAbs", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*"]
    pub fn set_x(this: &SvgPathSegLinetoAbs, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegLinetoAbs", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*"]
    pub fn y(this: &SvgPathSegLinetoAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegLinetoAbs", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*"]
    pub fn set_y(this: &SvgPathSegLinetoAbs, value: f32);
}
