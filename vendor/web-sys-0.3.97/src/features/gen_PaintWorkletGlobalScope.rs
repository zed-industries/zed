#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "WorkletGlobalScope",
        extends = "::js_sys::Object",
        js_name = "PaintWorkletGlobalScope",
        typescript_type = "PaintWorkletGlobalScope"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaintWorkletGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*"]
    pub type PaintWorkletGlobalScope;
    #[wasm_bindgen(
        method,
        js_class = "PaintWorkletGlobalScope",
        js_name = "registerPaint"
    )]
    #[doc = "The `registerPaint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope/registerPaint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*"]
    pub fn register_paint(
        this: &PaintWorkletGlobalScope,
        name: &str,
        paint_ctor: &::js_sys::Function,
    );
}
