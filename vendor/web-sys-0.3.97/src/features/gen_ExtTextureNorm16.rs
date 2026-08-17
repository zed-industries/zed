#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "EXT_texture_norm16" , typescript_type = "EXT_texture_norm16")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtTextureNorm16` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_texture_norm16)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub type ExtTextureNorm16;
}
impl ExtTextureNorm16 {
    #[doc = "The `EXT_texture_norm16.R16_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const R16_EXT: u32 = 33322u64 as u32;
    #[doc = "The `EXT_texture_norm16.RG16_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const RG16_EXT: u32 = 33324u64 as u32;
    #[doc = "The `EXT_texture_norm16.RGB16_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const RGB16_EXT: u32 = 32852u64 as u32;
    #[doc = "The `EXT_texture_norm16.RGBA16_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const RGBA16_EXT: u32 = 32859u64 as u32;
    #[doc = "The `EXT_texture_norm16.R16_SNORM_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const R16_SNORM_EXT: u32 = 36760u64 as u32;
    #[doc = "The `EXT_texture_norm16.RG16_SNORM_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const RG16_SNORM_EXT: u32 = 36761u64 as u32;
    #[doc = "The `EXT_texture_norm16.RGB16_SNORM_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const RGB16_SNORM_EXT: u32 = 36762u64 as u32;
    #[doc = "The `EXT_texture_norm16.RGBA16_SNORM_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureNorm16`*"]
    pub const RGBA16_SNORM_EXT: u32 = 36763u64 as u32;
}
