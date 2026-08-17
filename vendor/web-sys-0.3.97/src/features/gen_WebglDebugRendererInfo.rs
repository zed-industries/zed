#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "WEBGL_debug_renderer_info" , typescript_type = "WEBGL_debug_renderer_info")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglDebugRendererInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_renderer_info)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglDebugRendererInfo`*"]
    pub type WebglDebugRendererInfo;
}
impl WebglDebugRendererInfo {
    #[doc = "The `WEBGL_debug_renderer_info.UNMASKED_VENDOR_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglDebugRendererInfo`*"]
    pub const UNMASKED_VENDOR_WEBGL: u32 = 37445u64 as u32;
    #[doc = "The `WEBGL_debug_renderer_info.UNMASKED_RENDERER_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglDebugRendererInfo`*"]
    pub const UNMASKED_RENDERER_WEBGL: u32 = 37446u64 as u32;
}
