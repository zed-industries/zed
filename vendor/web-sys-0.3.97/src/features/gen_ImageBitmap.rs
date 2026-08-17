#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ImageBitmap",
        typescript_type = "ImageBitmap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
    pub type ImageBitmap;
    #[wasm_bindgen(method, getter, js_class = "ImageBitmap", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
    pub fn width(this: &ImageBitmap) -> u32;
    #[wasm_bindgen(method, getter, js_class = "ImageBitmap", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
    pub fn height(this: &ImageBitmap) -> u32;
    #[wasm_bindgen(method, js_class = "ImageBitmap")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
    pub fn close(this: &ImageBitmap);
}
