#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "DomMatrix",
        extends = "DomMatrixReadOnly",
        extends = "::js_sys::Object",
        js_name = "WebKitCSSMatrix",
        typescript_type = "WebKitCSSMatrix"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebKitCssMatrix` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub type WebKitCssMatrix;
    #[wasm_bindgen(catch, constructor, js_class = "WebKitCSSMatrix")]
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn new() -> Result<WebKitCssMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebKitCSSMatrix")]
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn new_with_transform_list(transform_list: &str) -> Result<WebKitCssMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebKitCSSMatrix")]
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn new_with_other(other: &WebKitCssMatrix) -> Result<WebKitCssMatrix, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebKitCSSMatrix")]
    #[doc = "The `inverse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/inverse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn inverse(this: &WebKitCssMatrix) -> Result<WebKitCssMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix")]
    #[doc = "The `multiply()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/multiply)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn multiply(this: &WebKitCssMatrix, other: &WebKitCssMatrix) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotate")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_with_rot_x(this: &WebKitCssMatrix, rot_x: f64) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotate")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_with_rot_x_and_rot_y(
        this: &WebKitCssMatrix,
        rot_x: f64,
        rot_y: f64,
    ) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotate")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_with_rot_x_and_rot_y_and_rot_z(
        this: &WebKitCssMatrix,
        rot_x: f64,
        rot_y: f64,
        rot_z: f64,
    ) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotateAxisAngle")]
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotateAxisAngle")]
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x(this: &WebKitCssMatrix, x: f64) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotateAxisAngle")]
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x_and_y(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
    ) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotateAxisAngle")]
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x_and_y_and_z(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
        z: f64,
    ) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "rotateAxisAngle")]
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x_and_y_and_z_and_angle(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "scale")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale_with_scale_x(this: &WebKitCssMatrix, scale_x: f64) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "scale")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale_with_scale_x_and_scale_y(
        this: &WebKitCssMatrix,
        scale_x: f64,
        scale_y: f64,
    ) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "scale")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale_with_scale_x_and_scale_y_and_scale_z(
        this: &WebKitCssMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> WebKitCssMatrix;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebKitCSSMatrix",
        js_name = "setMatrixValue"
    )]
    #[doc = "The `setMatrixValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/setMatrixValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn set_matrix_value(
        this: &WebKitCssMatrix,
        transform_list: &str,
    ) -> Result<WebKitCssMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "skewX")]
    #[doc = "The `skewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_x(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "skewX")]
    #[doc = "The `skewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_x_with_sx(this: &WebKitCssMatrix, sx: f64) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "skewY")]
    #[doc = "The `skewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_y(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "skewY")]
    #[doc = "The `skewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_y_with_sy(this: &WebKitCssMatrix, sy: f64) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "translate")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate_with_tx(this: &WebKitCssMatrix, tx: f64) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "translate")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate_with_tx_and_ty(this: &WebKitCssMatrix, tx: f64, ty: f64) -> WebKitCssMatrix;
    #[wasm_bindgen(method, js_class = "WebKitCSSMatrix", js_name = "translate")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate_with_tx_and_ty_and_tz(
        this: &WebKitCssMatrix,
        tx: f64,
        ty: f64,
        tz: f64,
    ) -> WebKitCssMatrix;
}
