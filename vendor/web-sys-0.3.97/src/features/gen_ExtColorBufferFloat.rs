#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "EXT_color_buffer_float" , typescript_type = "EXT_color_buffer_float")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtColorBufferFloat` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_color_buffer_float)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtColorBufferFloat`*"]
    pub type ExtColorBufferFloat;
}
