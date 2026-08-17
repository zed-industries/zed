#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "EXT_texture_filter_anisotropic" , typescript_type = "EXT_texture_filter_anisotropic")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtTextureFilterAnisotropic` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_texture_filter_anisotropic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureFilterAnisotropic`*"]
    pub type ExtTextureFilterAnisotropic;
}
impl ExtTextureFilterAnisotropic {
    #[doc = "The `EXT_texture_filter_anisotropic.TEXTURE_MAX_ANISOTROPY_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureFilterAnisotropic`*"]
    pub const TEXTURE_MAX_ANISOTROPY_EXT: u32 = 34046u64 as u32;
    #[doc = "The `EXT_texture_filter_anisotropic.MAX_TEXTURE_MAX_ANISOTROPY_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtTextureFilterAnisotropic`*"]
    pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 34047u64 as u32;
}
