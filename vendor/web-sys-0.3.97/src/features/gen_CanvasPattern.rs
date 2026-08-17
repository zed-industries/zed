#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CanvasPattern",
        typescript_type = "CanvasPattern"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasPattern` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`*"]
    pub type CanvasPattern;
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(method, js_class = "CanvasPattern", js_name = "setTransform")]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `SvgMatrix`*"]
    pub fn set_transform(this: &CanvasPattern, matrix: &SvgMatrix);
}
