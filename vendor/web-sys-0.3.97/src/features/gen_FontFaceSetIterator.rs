#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "FontFaceSetIterator" , typescript_type = "FontFaceSetIterator")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetIterator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIterator`*"]
    pub type FontFaceSetIterator;
    #[cfg(feature = "FontFaceSetIteratorResult")]
    #[wasm_bindgen(catch, method, js_class = "FontFaceSetIterator")]
    #[doc = "The `next()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator/next)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIterator`, `FontFaceSetIteratorResult`*"]
    pub fn next(this: &FontFaceSetIterator) -> Result<FontFaceSetIteratorResult, JsValue>;
}
