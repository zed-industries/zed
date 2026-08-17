#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "MediaStream",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "CanvasCaptureMediaStream",
        typescript_type = "CanvasCaptureMediaStream"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasCaptureMediaStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*"]
    pub type CanvasCaptureMediaStream;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasCaptureMediaStream",
        js_name = "canvas"
    )]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`, `HtmlCanvasElement`*"]
    pub fn canvas(this: &CanvasCaptureMediaStream) -> HtmlCanvasElement;
    #[wasm_bindgen(
        method,
        js_class = "CanvasCaptureMediaStream",
        js_name = "requestFrame"
    )]
    #[doc = "The `requestFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/requestFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*"]
    pub fn request_frame(this: &CanvasCaptureMediaStream);
}
