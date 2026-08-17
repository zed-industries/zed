#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "OES_vertex_array_object" , typescript_type = "OES_vertex_array_object")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesVertexArrayObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`*"]
    pub type OesVertexArrayObject;
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(
        method,
        js_class = "OES_vertex_array_object",
        js_name = "bindVertexArrayOES"
    )]
    #[doc = "The `bindVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/bindVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn bind_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    );
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(
        method,
        js_class = "OES_vertex_array_object",
        js_name = "createVertexArrayOES"
    )]
    #[doc = "The `createVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/createVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn create_vertex_array_oes(this: &OesVertexArrayObject) -> Option<WebGlVertexArrayObject>;
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(
        method,
        js_class = "OES_vertex_array_object",
        js_name = "deleteVertexArrayOES"
    )]
    #[doc = "The `deleteVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/deleteVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn delete_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    );
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(
        method,
        js_class = "OES_vertex_array_object",
        js_name = "isVertexArrayOES"
    )]
    #[doc = "The `isVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/isVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn is_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    ) -> bool;
}
impl OesVertexArrayObject {
    #[doc = "The `OES_vertex_array_object.VERTEX_ARRAY_BINDING_OES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`*"]
    pub const VERTEX_ARRAY_BINDING_OES: u32 = 34229u64 as u32;
}
