#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMPointReadOnly",
        typescript_type = "DOMPointReadOnly"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPointReadOnly` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub type DomPointReadOnly;
    #[wasm_bindgen(method, getter, js_class = "DOMPointReadOnly", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn x(this: &DomPointReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMPointReadOnly", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn y(this: &DomPointReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMPointReadOnly", js_name = "z")]
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn z(this: &DomPointReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMPointReadOnly", js_name = "w")]
    #[doc = "Getter for the `w` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/w)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn w(this: &DomPointReadOnly) -> f64;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new() -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x(x: f64) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x_and_y_and_z(x: f64, y: f64, z: f64) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x_and_y_and_z_and_w(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(
        static_method_of = "DomPointReadOnly",
        js_class = "DOMPointReadOnly",
        js_name = "fromPoint"
    )]
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn from_point() -> DomPointReadOnly;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(
        static_method_of = "DomPointReadOnly",
        js_class = "DOMPointReadOnly",
        js_name = "fromPoint"
    )]
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomPointReadOnly`*"]
    pub fn from_point_with_other(other: &DomPointInit) -> DomPointReadOnly;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMPointReadOnly",
        js_name = "matrixTransform"
    )]
    #[doc = "The `matrixTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/matrixTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointReadOnly`*"]
    pub fn matrix_transform(this: &DomPointReadOnly) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomMatrixInit", feature = "DomPoint",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMPointReadOnly",
        js_name = "matrixTransform"
    )]
    #[doc = "The `matrixTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/matrixTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`, `DomPoint`, `DomPointReadOnly`*"]
    pub fn matrix_transform_with_matrix(
        this: &DomPointReadOnly,
        matrix: &DomMatrixInit,
    ) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(method, js_class = "DOMPointReadOnly", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn to_json(this: &DomPointReadOnly) -> ::js_sys::Object;
}
