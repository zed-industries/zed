#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "ImageCaptureError" , typescript_type = "ImageCaptureError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub type ImageCaptureError;
    #[wasm_bindgen(method, getter, js_class = "ImageCaptureError", js_name = "code")]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub fn code(this: &ImageCaptureError) -> u16;
    #[wasm_bindgen(method, getter, js_class = "ImageCaptureError", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub fn message(this: &ImageCaptureError) -> ::alloc::string::String;
}
impl ImageCaptureError {
    #[doc = "The `ImageCaptureError.FRAME_GRAB_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const FRAME_GRAB_ERROR: u16 = 1u64 as u16;
    #[doc = "The `ImageCaptureError.SETTINGS_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const SETTINGS_ERROR: u16 = 2u64 as u16;
    #[doc = "The `ImageCaptureError.PHOTO_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const PHOTO_ERROR: u16 = 3u64 as u16;
    #[doc = "The `ImageCaptureError.ERROR_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const ERROR_UNKNOWN: u16 = 4u64 as u16;
}
