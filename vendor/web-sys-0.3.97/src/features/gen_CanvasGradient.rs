#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CanvasGradient",
        typescript_type = "CanvasGradient"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasGradient` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`*"]
    pub type CanvasGradient;
    #[wasm_bindgen(catch, method, js_class = "CanvasGradient", js_name = "addColorStop")]
    #[doc = "The `addColorStop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient/addColorStop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`*"]
    pub fn add_color_stop(this: &CanvasGradient, offset: f32, color: &str) -> Result<(), JsValue>;
}
