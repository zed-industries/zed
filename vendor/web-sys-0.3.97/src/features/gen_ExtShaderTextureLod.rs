#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "EXT_shader_texture_lod" , typescript_type = "EXT_shader_texture_lod")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtShaderTextureLod` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_shader_texture_lod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtShaderTextureLod`*"]
    pub type ExtShaderTextureLod;
}
