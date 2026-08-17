#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CaretPosition",
        typescript_type = "CaretPosition"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CaretPosition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`*"]
    pub type CaretPosition;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "CaretPosition", js_name = "offsetNode")]
    #[doc = "Getter for the `offsetNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offsetNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`, `Node`*"]
    pub fn offset_node(this: &CaretPosition) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "CaretPosition", js_name = "offset")]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`*"]
    pub fn offset(this: &CaretPosition) -> u32;
    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, js_class = "CaretPosition", js_name = "getClientRect")]
    #[doc = "The `getClientRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/getClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`, `DomRect`*"]
    pub fn get_client_rect(this: &CaretPosition) -> Option<DomRect>;
}
