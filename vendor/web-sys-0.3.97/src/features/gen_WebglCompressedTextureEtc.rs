#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "WEBGL_compressed_texture_etc" , typescript_type = "WEBGL_compressed_texture_etc")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglCompressedTextureEtc` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_etc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub type WebglCompressedTextureEtc;
}
impl WebglCompressedTextureEtc {
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_R11_EAC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_R11_EAC: u32 = 37488u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_SIGNED_R11_EAC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_SIGNED_R11_EAC: u32 = 37489u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_RG11_EAC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_RG11_EAC: u32 = 37490u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_SIGNED_RG11_EAC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_SIGNED_RG11_EAC: u32 = 37491u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_RGB8_ETC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_RGB8_ETC2: u32 = 37492u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_SRGB8_ETC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_SRGB8_ETC2: u32 = 37493u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 37494u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 37495u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_RGBA8_ETC2_EAC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_RGBA8_ETC2_EAC: u32 = 37496u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_etc.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureEtc`*"]
    pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: u32 = 37497u64 as u32;
}
