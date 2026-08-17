#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ImageData",
        typescript_type = "ImageData"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub type ImageData;
    #[wasm_bindgen(method, getter, js_class = "ImageData", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn width(this: &ImageData) -> u32;
    #[wasm_bindgen(method, getter, js_class = "ImageData", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn height(this: &ImageData) -> u32;
    #[wasm_bindgen(method, getter, js_class = "ImageData", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn data(this: &ImageData) -> ::wasm_bindgen::Clamped<::alloc::vec::Vec<u8>>;
    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_sw(sw: u32, sh: u32) -> Result<ImageData, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_u8_clamped_array(
        data: ::wasm_bindgen::Clamped<&[u8]>,
        sw: u32,
    ) -> Result<ImageData, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_js_u8_clamped_array(
        data: &::js_sys::Uint8ClampedArray,
        sw: u32,
    ) -> Result<ImageData, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_u8_clamped_array_and_sh(
        data: ::wasm_bindgen::Clamped<&[u8]>,
        sw: u32,
        sh: u32,
    ) -> Result<ImageData, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "ImageData")]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
    pub fn new_with_js_u8_clamped_array_and_sh(
        data: &::js_sys::Uint8ClampedArray,
        sw: u32,
        sh: u32,
    ) -> Result<ImageData, JsValue>;
}
