#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "DomMatrixReadOnly",
        extends = "::js_sys::Object",
        js_name = "DOMMatrix",
        typescript_type = "DOMMatrix"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomMatrix` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub type DomMatrix;
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "a")]
    #[doc = "Getter for the `a` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn a(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "a")]
    #[doc = "Setter for the `a` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_a(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "b")]
    #[doc = "Getter for the `b` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn b(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "b")]
    #[doc = "Setter for the `b` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_b(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "c")]
    #[doc = "Getter for the `c` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn c(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "c")]
    #[doc = "Setter for the `c` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_c(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "d")]
    #[doc = "Getter for the `d` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn d(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "d")]
    #[doc = "Setter for the `d` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_d(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "e")]
    #[doc = "Getter for the `e` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn e(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "e")]
    #[doc = "Setter for the `e` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_e(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "f")]
    #[doc = "Getter for the `f` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn f(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "f")]
    #[doc = "Setter for the `f` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_f(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m11")]
    #[doc = "Getter for the `m11` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m11(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m11")]
    #[doc = "Setter for the `m11` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m11(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m12")]
    #[doc = "Getter for the `m12` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m12(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m12")]
    #[doc = "Setter for the `m12` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m12(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m13")]
    #[doc = "Getter for the `m13` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m13(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m13")]
    #[doc = "Setter for the `m13` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m13(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m14")]
    #[doc = "Getter for the `m14` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m14(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m14")]
    #[doc = "Setter for the `m14` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m14(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m21")]
    #[doc = "Getter for the `m21` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m21(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m21")]
    #[doc = "Setter for the `m21` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m21(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m22")]
    #[doc = "Getter for the `m22` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m22(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m22")]
    #[doc = "Setter for the `m22` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m22(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m23")]
    #[doc = "Getter for the `m23` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m23(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m23")]
    #[doc = "Setter for the `m23` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m23(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m24")]
    #[doc = "Getter for the `m24` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m24(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m24")]
    #[doc = "Setter for the `m24` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m24(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m31")]
    #[doc = "Getter for the `m31` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m31(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m31")]
    #[doc = "Setter for the `m31` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m31(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m32")]
    #[doc = "Getter for the `m32` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m32(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m32")]
    #[doc = "Setter for the `m32` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m32(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m33")]
    #[doc = "Getter for the `m33` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m33(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m33")]
    #[doc = "Setter for the `m33` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m33(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m34")]
    #[doc = "Getter for the `m34` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m34(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m34")]
    #[doc = "Setter for the `m34` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m34(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m41")]
    #[doc = "Getter for the `m41` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m41(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m41")]
    #[doc = "Setter for the `m41` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m41(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m42")]
    #[doc = "Getter for the `m42` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m42(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m42")]
    #[doc = "Setter for the `m42` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m42(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m43")]
    #[doc = "Getter for the `m43` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m43(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m43")]
    #[doc = "Setter for the `m43` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m43(this: &DomMatrix, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMMatrix", js_name = "m44")]
    #[doc = "Getter for the `m44` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn m44(this: &DomMatrix) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMMatrix", js_name = "m44")]
    #[doc = "Setter for the `m44` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_m44(this: &DomMatrix, value: f64);
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new() -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new_with_transform_list(transform_list: &str) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new_with_other(other: &DomMatrixReadOnly) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new_with_array32(array32: &mut [f32]) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new_with_f32_array(array32: &::js_sys::Float32Array) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new_with_array64(array64: &mut [f64]) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new_with_f64_array(array64: &::js_sys::Float64Array) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    #[doc = "The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn new_with_number_sequence(
        number_sequence: &::wasm_bindgen::JsValue,
    ) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "invertSelf")]
    #[doc = "The `invertSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/invertSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn invert_self(this: &DomMatrix) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "multiplySelf")]
    #[doc = "The `multiplySelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/multiplySelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn multiply_self(this: &DomMatrix, other: &DomMatrix) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "preMultiplySelf")]
    #[doc = "The `preMultiplySelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/preMultiplySelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn pre_multiply_self(this: &DomMatrix, other: &DomMatrix) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "rotateAxisAngleSelf")]
    #[doc = "The `rotateAxisAngleSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn rotate_axis_angle_self(
        this: &DomMatrix,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "rotateFromVectorSelf")]
    #[doc = "The `rotateFromVectorSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateFromVectorSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn rotate_from_vector_self(this: &DomMatrix, x: f64, y: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "rotateSelf")]
    #[doc = "The `rotateSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn rotate_self(this: &DomMatrix, angle: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "rotateSelf")]
    #[doc = "The `rotateSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn rotate_self_with_origin_x(this: &DomMatrix, angle: f64, origin_x: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "rotateSelf")]
    #[doc = "The `rotateSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn rotate_self_with_origin_x_and_origin_y(
        this: &DomMatrix,
        angle: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scale3dSelf")]
    #[doc = "The `scale3dSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale3d_self(this: &DomMatrix, scale: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scale3dSelf")]
    #[doc = "The `scale3dSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale3d_self_with_origin_x(this: &DomMatrix, scale: f64, origin_x: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scale3dSelf")]
    #[doc = "The `scale3dSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale3d_self_with_origin_x_and_origin_y(
        this: &DomMatrix,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scale3dSelf")]
    #[doc = "The `scale3dSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale3d_self_with_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrix,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleNonUniformSelf")]
    #[doc = "The `scaleNonUniformSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_non_uniform_self(this: &DomMatrix, scale_x: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleNonUniformSelf")]
    #[doc = "The `scaleNonUniformSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_non_uniform_self_with_scale_y(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleNonUniformSelf")]
    #[doc = "The `scaleNonUniformSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleNonUniformSelf")]
    #[doc = "The `scaleNonUniformSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleNonUniformSelf")]
    #[doc = "The `scaleNonUniformSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleNonUniformSelf")]
    #[doc = "The `scaleNonUniformSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleSelf")]
    #[doc = "The `scaleSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_self(this: &DomMatrix, scale: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleSelf")]
    #[doc = "The `scaleSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_self_with_origin_x(this: &DomMatrix, scale: f64, origin_x: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "scaleSelf")]
    #[doc = "The `scaleSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn scale_self_with_origin_x_and_origin_y(
        this: &DomMatrix,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;
    #[wasm_bindgen(catch, method, js_class = "DOMMatrix", js_name = "setMatrixValue")]
    #[doc = "The `setMatrixValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/setMatrixValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn set_matrix_value(this: &DomMatrix, transform_list: &str) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "skewXSelf")]
    #[doc = "The `skewXSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewXSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn skew_x_self(this: &DomMatrix, sx: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "skewYSelf")]
    #[doc = "The `skewYSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewYSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn skew_y_self(this: &DomMatrix, sy: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "translateSelf")]
    #[doc = "The `translateSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn translate_self(this: &DomMatrix, tx: f64, ty: f64) -> DomMatrix;
    #[wasm_bindgen(method, js_class = "DOMMatrix", js_name = "translateSelf")]
    #[doc = "The `translateSelf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`*"]
    pub fn translate_self_with_tz(this: &DomMatrix, tx: f64, ty: f64, tz: f64) -> DomMatrix;
}
