#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMRectReadOnly",
        typescript_type = "DOMRectReadOnly"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRectReadOnly` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub type DomRectReadOnly;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn x(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn y(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn width(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn height(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "top")]
    #[doc = "Getter for the `top` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/top)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn top(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "right")]
    #[doc = "Getter for the `right` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/right)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn right(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "bottom")]
    #[doc = "Getter for the `bottom` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/bottom)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn bottom(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DOMRectReadOnly", js_name = "left")]
    #[doc = "Getter for the `left` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/left)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn left(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new() -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x(x: f64) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x_and_y_and_width(
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x_and_y_and_width_and_height(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(method, js_class = "DOMRectReadOnly", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn to_json(this: &DomRectReadOnly) -> ::js_sys::Object;
}
