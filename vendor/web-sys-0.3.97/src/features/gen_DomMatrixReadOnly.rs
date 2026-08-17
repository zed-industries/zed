#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMMatrixReadOnly",
        typescript_type = "DOMMatrixReadOnly"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomMatrixReadOnly` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub type DomMatrixReadOnly;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "a")]
    #[doc = "Getter for the `a` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/a)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn a(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "b")]
    #[doc = "Getter for the `b` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/b)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn b(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "c")]
    #[doc = "Getter for the `c` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/c)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn c(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "d")]
    #[doc = "Getter for the `d` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn d(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "e")]
    #[doc = "Getter for the `e` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/e)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn e(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "f")]
    #[doc = "Getter for the `f` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn f(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m11")]
    #[doc = "Getter for the `m11` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m11)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m11(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m12")]
    #[doc = "Getter for the `m12` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m12)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m12(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m13")]
    #[doc = "Getter for the `m13` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m13)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m13(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m14")]
    #[doc = "Getter for the `m14` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m14)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m14(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m21")]
    #[doc = "Getter for the `m21` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m21)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m21(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m22")]
    #[doc = "Getter for the `m22` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m22)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m22(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m23")]
    #[doc = "Getter for the `m23` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m23)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m23(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m24")]
    #[doc = "Getter for the `m24` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m24)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m24(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m31")]
    #[doc = "Getter for the `m31` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m31)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m31(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m32")]
    #[doc = "Getter for the `m32` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m32(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m33")]
    #[doc = "Getter for the `m33` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m33)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m33(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m34")]
    #[doc = "Getter for the `m34` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m34)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m34(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m41")]
    #[doc = "Getter for the `m41` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m41)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m41(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m42")]
    #[doc = "Getter for the `m42` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m42)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m42(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m43")]
    #[doc = "Getter for the `m43` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m43)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m43(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "m44")]
    #[doc = "Getter for the `m44` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m44)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn m44(this: &DomMatrixReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "is2D")]
    #[doc = "Getter for the `is2D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/is2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn is_2d(this: &DomMatrixReadOnly) -> bool;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrixReadOnly", js_name = "isIdentity")]
    #[doc = "Getter for the `isIdentity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/isIdentity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn is_identity(this: &DomMatrixReadOnly) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `new DomMatrixReadOnly(..)` constructor, creating a new instance of `DomMatrixReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn new() -> Result<DomMatrixReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `new DomMatrixReadOnly(..)` constructor, creating a new instance of `DomMatrixReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn new_with_str(init: &str) -> Result<DomMatrixReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `new DomMatrixReadOnly(..)` constructor, creating a new instance of `DomMatrixReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn new_with_f64_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<DomMatrixReadOnly, JsValue>;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "flipX")]
    #[doc = "The `flipX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn flip_x(this: &DomMatrixReadOnly) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "flipY")]
    #[doc = "The `flipY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn flip_y(this: &DomMatrixReadOnly) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `inverse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/inverse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn inverse(this: &DomMatrixReadOnly) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `multiply()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/multiply)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn multiply(this: &DomMatrixReadOnly, other: &DomMatrix) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn rotate(this: &DomMatrixReadOnly, angle: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "rotate")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn rotate_with_origin_x(this: &DomMatrixReadOnly, angle: f64, origin_x: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "rotate")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn rotate_with_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        angle: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "rotateAxisAngle")]
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn rotate_axis_angle(
        this: &DomMatrixReadOnly,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "rotateFromVector")]
    #[doc = "The `rotateFromVector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateFromVector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn rotate_from_vector(this: &DomMatrixReadOnly, x: f64, y: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale(this: &DomMatrixReadOnly, scale: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scale")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_with_origin_x(this: &DomMatrixReadOnly, scale: f64, origin_x: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scale")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_with_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `scale3d()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale3d(this: &DomMatrixReadOnly, scale: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scale3d")]
    #[doc = "The `scale3d()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale3d_with_origin_x(this: &DomMatrixReadOnly, scale: f64, origin_x: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scale3d")]
    #[doc = "The `scale3d()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale3d_with_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scale3d")]
    #[doc = "The `scale3d()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale3d_with_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrixReadOnly,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scaleNonUniform")]
    #[doc = "The `scaleNonUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_non_uniform(this: &DomMatrixReadOnly, scale_x: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scaleNonUniform")]
    #[doc = "The `scaleNonUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_non_uniform_with_scale_y(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scaleNonUniform")]
    #[doc = "The `scaleNonUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_non_uniform_with_scale_y_and_scale_z(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scaleNonUniform")]
    #[doc = "The `scaleNonUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scaleNonUniform")]
    #[doc = "The `scaleNonUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "scaleNonUniform")]
    #[doc = "The `scaleNonUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "skewX")]
    #[doc = "The `skewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn skew_x(this: &DomMatrixReadOnly, sx: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "skewY")]
    #[doc = "The `skewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn skew_y(this: &DomMatrixReadOnly, sy: f64) -> DomMatrix;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMMatrixReadOnly",
        js_name = "toFloat32Array"
    )]
    #[doc = "The `toFloat32Array()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat32Array)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn to_float32_array(this: &DomMatrixReadOnly) -> Result<::alloc::vec::Vec<f32>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMMatrixReadOnly",
        js_name = "toFloat64Array"
    )]
    #[doc = "The `toFloat64Array()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat64Array)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn to_float64_array(this: &DomMatrixReadOnly) -> Result<::alloc::vec::Vec<f64>, JsValue>;
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    pub fn to_json(this: &DomMatrixReadOnly) -> ::js_sys::Object;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "transformPoint")]
    #[doc = "The `transformPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`, `DomPoint`*"]
    pub fn transform_point(this: &DomMatrixReadOnly) -> DomPoint;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "transformPoint")]
    #[doc = "The `transformPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixReadOnly`, `DomPoint`, `DomPointInit`*"]
    pub fn transform_point_with_point(this: &DomMatrixReadOnly, point: &DomPointInit) -> DomPoint;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn translate(this: &DomMatrixReadOnly, tx: f64, ty: f64) -> DomMatrix;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(method, js_class = "DOMMatrixReadOnly", js_name = "translate")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    pub fn translate_with_tz(this: &DomMatrixReadOnly, tx: f64, ty: f64, tz: f64) -> DomMatrix;
}
