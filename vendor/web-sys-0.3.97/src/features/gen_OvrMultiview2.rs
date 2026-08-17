#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "OVR_multiview2",
        typescript_type = "OVR_multiview2"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OvrMultiview2` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OVR_multiview2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OvrMultiview2`*"]
    pub type OvrMultiview2;
    #[cfg(feature = "WebGlTexture")]
    #[wasm_bindgen(
        method,
        js_class = "OVR_multiview2",
        js_name = "framebufferTextureMultiviewOVR"
    )]
    #[doc = "The `framebufferTextureMultiviewOVR()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OVR_multiview2/framebufferTextureMultiviewOVR)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OvrMultiview2`, `WebGlTexture`*"]
    pub fn framebuffer_texture_multiview_ovr(
        this: &OvrMultiview2,
        target: u32,
        attachment: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
        base_view_index: i32,
        num_views: i32,
    );
}
impl OvrMultiview2 {
    #[doc = "The `OVR_multiview2.FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OvrMultiview2`*"]
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: u32 = 38448u64 as u32;
    #[doc = "The `OVR_multiview2.FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OvrMultiview2`*"]
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: u32 = 38450u64 as u32;
    #[doc = "The `OVR_multiview2.MAX_VIEWS_OVR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OvrMultiview2`*"]
    pub const MAX_VIEWS_OVR: u32 = 38449u64 as u32;
    #[doc = "The `OVR_multiview2.FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OvrMultiview2`*"]
    pub const FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR: u32 = 38451u64 as u32;
}
