#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaError",
        typescript_type = "MediaError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub type MediaError;
    #[wasm_bindgen(method, getter, js_class = "MediaError", js_name = "code")]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub fn code(this: &MediaError) -> u16;
    #[wasm_bindgen(method, getter, js_class = "MediaError", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub fn message(this: &MediaError) -> ::alloc::string::String;
}
impl MediaError {
    #[doc = "The `MediaError.MEDIA_ERR_ABORTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_ABORTED: u16 = 1u64 as u16;
    #[doc = "The `MediaError.MEDIA_ERR_NETWORK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_NETWORK: u16 = 2u64 as u16;
    #[doc = "The `MediaError.MEDIA_ERR_DECODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_DECODE: u16 = 3u64 as u16;
    #[doc = "The `MediaError.MEDIA_ERR_SRC_NOT_SUPPORTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_SRC_NOT_SUPPORTED: u16 = 4u64 as u16;
}
