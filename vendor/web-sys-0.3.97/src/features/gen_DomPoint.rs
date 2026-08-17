#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "DomPointReadOnly",
        extends = "::js_sys::Object",
        js_name = "DOMPoint",
        typescript_type = "DOMPoint"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPoint` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub type DomPoint;
    #[wasm_bindgen(method, getter, js_class = "DOMPoint", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn x(this: &DomPoint) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMPoint", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_x(this: &DomPoint, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMPoint", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn y(this: &DomPoint) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMPoint", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_y(this: &DomPoint, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMPoint", js_name = "z")]
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn z(this: &DomPoint) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMPoint", js_name = "z")]
    #[doc = "Setter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_z(this: &DomPoint, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMPoint", js_name = "w")]
    #[doc = "Getter for the `w` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn w(this: &DomPoint) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMPoint", js_name = "w")]
    #[doc = "Setter for the `w` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_w(this: &DomPoint, value: f64);
    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new() -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x(x: f64) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x_and_y_and_z(x: f64, y: f64, z: f64) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x_and_y_and_z_and_w(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(
        static_method_of = "DomPoint",
        js_class = "DOMPoint",
        js_name = "fromPoint"
    )]
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn from_point() -> DomPoint;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(
        static_method_of = "DomPoint",
        js_class = "DOMPoint",
        js_name = "fromPoint"
    )]
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`*"]
    pub fn from_point_with_other(other: &DomPointInit) -> DomPoint;
}
