#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "DomRectReadOnly",
        extends = "::js_sys::Object",
        js_name = "DOMRect",
        typescript_type = "DOMRect"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub type DomRect;
    #[wasm_bindgen(method, getter, js_class = "DOMRect", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn x(this: &DomRect) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMRect", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_x(this: &DomRect, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMRect", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn y(this: &DomRect) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMRect", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_y(this: &DomRect, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMRect", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn width(this: &DomRect) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMRect", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_width(this: &DomRect, value: f64);
    #[wasm_bindgen(method, getter, js_class = "DOMRect", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn height(this: &DomRect) -> f64;
    #[wasm_bindgen(method, setter, js_class = "DOMRect", js_name = "height")]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_height(this: &DomRect, value: f64);
    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new() -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x(x: f64) -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x_and_y_and_width(x: f64, y: f64, width: f64) -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x_and_y_and_width_and_height(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRect, JsValue>;
}
