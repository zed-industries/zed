#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ResizeObserverSize",
        typescript_type = "ResizeObserverSize"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserverSize` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverSize`*"]
    pub type ResizeObserverSize;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ResizeObserverSize",
        js_name = "inlineSize"
    )]
    #[doc = "Getter for the `inlineSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize/inlineSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverSize`*"]
    pub fn inline_size(this: &ResizeObserverSize) -> f64;
    #[wasm_bindgen(method, getter, js_class = "ResizeObserverSize", js_name = "blockSize")]
    #[doc = "Getter for the `blockSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize/blockSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverSize`*"]
    pub fn block_size(this: &ResizeObserverSize) -> f64;
}
