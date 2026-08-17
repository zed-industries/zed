#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ImageBitmapRenderingContext",
        typescript_type = "ImageBitmapRenderingContext"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmapRenderingContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapRenderingContext`*"]
    pub type ImageBitmapRenderingContext;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        method,
        js_class = "ImageBitmapRenderingContext",
        js_name = "transferFromImageBitmap"
    )]
    #[doc = "The `transferFromImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferFromImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
    pub fn transfer_from_image_bitmap(this: &ImageBitmapRenderingContext, bitmap: &ImageBitmap);
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        method,
        js_class = "ImageBitmapRenderingContext",
        js_name = "transferImageBitmap"
    )]
    #[doc = "The `transferImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
    pub fn transfer_image_bitmap(this: &ImageBitmapRenderingContext, bitmap: &ImageBitmap);
}
