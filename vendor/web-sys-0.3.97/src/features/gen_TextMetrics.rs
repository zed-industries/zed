#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TextMetrics",
        typescript_type = "TextMetrics"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextMetrics` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub type TextMetrics;
    #[wasm_bindgen(method, getter, js_class = "TextMetrics", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub fn width(this: &TextMetrics) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TextMetrics",
        js_name = "actualBoundingBoxLeft"
    )]
    #[doc = "Getter for the `actualBoundingBoxLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub fn actual_bounding_box_left(this: &TextMetrics) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TextMetrics",
        js_name = "actualBoundingBoxRight"
    )]
    #[doc = "Getter for the `actualBoundingBoxRight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxRight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub fn actual_bounding_box_right(this: &TextMetrics) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TextMetrics",
        js_name = "fontBoundingBoxAscent"
    )]
    #[doc = "Getter for the `fontBoundingBoxAscent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/fontBoundingBoxAscent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub fn font_bounding_box_ascent(this: &TextMetrics) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TextMetrics",
        js_name = "fontBoundingBoxDescent"
    )]
    #[doc = "Getter for the `fontBoundingBoxDescent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/fontBoundingBoxDescent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub fn font_bounding_box_descent(this: &TextMetrics) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TextMetrics",
        js_name = "actualBoundingBoxAscent"
    )]
    #[doc = "Getter for the `actualBoundingBoxAscent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxAscent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub fn actual_bounding_box_ascent(this: &TextMetrics) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TextMetrics",
        js_name = "actualBoundingBoxDescent"
    )]
    #[doc = "Getter for the `actualBoundingBoxDescent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxDescent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
    pub fn actual_bounding_box_descent(this: &TextMetrics) -> f64;
}
