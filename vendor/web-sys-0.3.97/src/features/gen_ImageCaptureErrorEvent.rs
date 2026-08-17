#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "ImageCaptureErrorEvent",
        typescript_type = "ImageCaptureErrorEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
    pub type ImageCaptureErrorEvent;
    #[cfg(feature = "ImageCaptureError")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ImageCaptureErrorEvent",
        js_name = "imageCaptureError"
    )]
    #[doc = "Getter for the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/imageCaptureError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEvent`*"]
    pub fn image_capture_error(this: &ImageCaptureErrorEvent) -> Option<ImageCaptureError>;
    #[wasm_bindgen(catch, constructor, js_class = "ImageCaptureErrorEvent")]
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
    pub fn new(type_: &str) -> Result<ImageCaptureErrorEvent, JsValue>;
    #[cfg(feature = "ImageCaptureErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ImageCaptureErrorEvent")]
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`, `ImageCaptureErrorEventInit`*"]
    pub fn new_with_image_capture_error_init_dict(
        type_: &str,
        image_capture_error_init_dict: &ImageCaptureErrorEventInit,
    ) -> Result<ImageCaptureErrorEvent, JsValue>;
}
