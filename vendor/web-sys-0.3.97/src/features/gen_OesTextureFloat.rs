#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "OES_texture_float" , typescript_type = "OES_texture_float")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesTextureFloat` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_float)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesTextureFloat`*"]
    pub type OesTextureFloat;
}
