#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGMatrix",
        typescript_type = "SVGMatrix"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMatrix` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub type SvgMatrix;
    #[wasm_bindgen(method, getter, js_class = "SVGMatrix", js_name = "a")]
    #[doc = "Getter for the `a` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn a(this: &SvgMatrix) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGMatrix", js_name = "a")]
    #[doc = "Setter for the `a` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_a(this: &SvgMatrix, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGMatrix", js_name = "b")]
    #[doc = "Getter for the `b` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn b(this: &SvgMatrix) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGMatrix", js_name = "b")]
    #[doc = "Setter for the `b` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_b(this: &SvgMatrix, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGMatrix", js_name = "c")]
    #[doc = "Getter for the `c` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn c(this: &SvgMatrix) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGMatrix", js_name = "c")]
    #[doc = "Setter for the `c` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_c(this: &SvgMatrix, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGMatrix", js_name = "d")]
    #[doc = "Getter for the `d` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn d(this: &SvgMatrix) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGMatrix", js_name = "d")]
    #[doc = "Setter for the `d` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_d(this: &SvgMatrix, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGMatrix", js_name = "e")]
    #[doc = "Getter for the `e` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn e(this: &SvgMatrix) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGMatrix", js_name = "e")]
    #[doc = "Setter for the `e` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_e(this: &SvgMatrix, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGMatrix", js_name = "f")]
    #[doc = "Getter for the `f` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn f(this: &SvgMatrix) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGMatrix", js_name = "f")]
    #[doc = "Setter for the `f` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_f(this: &SvgMatrix, value: f32);
    #[wasm_bindgen(method, js_class = "SVGMatrix", js_name = "flipX")]
    #[doc = "The `flipX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn flip_x(this: &SvgMatrix) -> SvgMatrix;
    #[wasm_bindgen(method, js_class = "SVGMatrix", js_name = "flipY")]
    #[doc = "The `flipY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn flip_y(this: &SvgMatrix) -> SvgMatrix;
    #[wasm_bindgen(catch, method, js_class = "SVGMatrix")]
    #[doc = "The `inverse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/inverse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn inverse(this: &SvgMatrix) -> Result<SvgMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGMatrix")]
    #[doc = "The `multiply()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/multiply)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn multiply(this: &SvgMatrix, second_matrix: &SvgMatrix) -> SvgMatrix;
    #[wasm_bindgen(method, js_class = "SVGMatrix")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn rotate(this: &SvgMatrix, angle: f32) -> SvgMatrix;
    #[wasm_bindgen(catch, method, js_class = "SVGMatrix", js_name = "rotateFromVector")]
    #[doc = "The `rotateFromVector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotateFromVector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn rotate_from_vector(this: &SvgMatrix, x: f32, y: f32) -> Result<SvgMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGMatrix")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn scale(this: &SvgMatrix, scale_factor: f32) -> SvgMatrix;
    #[wasm_bindgen(method, js_class = "SVGMatrix", js_name = "scaleNonUniform")]
    #[doc = "The `scaleNonUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scaleNonUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn scale_non_uniform(
        this: &SvgMatrix,
        scale_factor_x: f32,
        scale_factor_y: f32,
    ) -> SvgMatrix;
    #[wasm_bindgen(catch, method, js_class = "SVGMatrix", js_name = "skewX")]
    #[doc = "The `skewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn skew_x(this: &SvgMatrix, angle: f32) -> Result<SvgMatrix, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGMatrix", js_name = "skewY")]
    #[doc = "The `skewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn skew_y(this: &SvgMatrix, angle: f32) -> Result<SvgMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGMatrix")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn translate(this: &SvgMatrix, x: f32, y: f32) -> SvgMatrix;
}
