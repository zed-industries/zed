#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebGLShaderPrecisionFormat",
        typescript_type = "WebGLShaderPrecisionFormat"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlShaderPrecisionFormat` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub type WebGlShaderPrecisionFormat;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebGLShaderPrecisionFormat",
        js_name = "rangeMin"
    )]
    #[doc = "Getter for the `rangeMin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub fn range_min(this: &WebGlShaderPrecisionFormat) -> i32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebGLShaderPrecisionFormat",
        js_name = "rangeMax"
    )]
    #[doc = "Getter for the `rangeMax` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMax)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub fn range_max(this: &WebGlShaderPrecisionFormat) -> i32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebGLShaderPrecisionFormat",
        js_name = "precision"
    )]
    #[doc = "Getter for the `precision` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/precision)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub fn precision(this: &WebGlShaderPrecisionFormat) -> i32;
}
