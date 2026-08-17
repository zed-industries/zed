#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "OES_element_index_uint" , typescript_type = "OES_element_index_uint")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesElementIndexUint` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_element_index_uint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesElementIndexUint`*"]
    pub type OesElementIndexUint;
}
