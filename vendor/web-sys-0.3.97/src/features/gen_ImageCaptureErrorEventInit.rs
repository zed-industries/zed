#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ImageCaptureErrorEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    pub type ImageCaptureErrorEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &ImageCaptureErrorEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &ImageCaptureErrorEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &ImageCaptureErrorEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &ImageCaptureErrorEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &ImageCaptureErrorEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &ImageCaptureErrorEventInit, val: bool);
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Get the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "imageCaptureError")]
    pub fn get_image_capture_error(this: &ImageCaptureErrorEventInit) -> Option<ImageCaptureError>;
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Change the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "imageCaptureError")]
    pub fn set_image_capture_error(
        this: &ImageCaptureErrorEventInit,
        val: Option<&ImageCaptureError>,
    );
}
impl ImageCaptureErrorEventInit {
    #[doc = "Construct a new `ImageCaptureErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "ImageCaptureError")]
    #[deprecated = "Use `set_image_capture_error()` instead."]
    pub fn image_capture_error(&mut self, val: Option<&ImageCaptureError>) -> &mut Self {
        self.set_image_capture_error(val);
        self
    }
}
impl Default for ImageCaptureErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
