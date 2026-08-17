#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "MediaStreamTrack",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "CanvasCaptureMediaStreamTrack",
        typescript_type = "CanvasCaptureMediaStreamTrack"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasCaptureMediaStreamTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStreamTrack`*"]
    pub type CanvasCaptureMediaStreamTrack;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasCaptureMediaStreamTrack",
        js_name = "canvas"
    )]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStreamTrack`, `HtmlCanvasElement`*"]
    pub fn canvas(this: &CanvasCaptureMediaStreamTrack) -> HtmlCanvasElement;
    #[wasm_bindgen(
        method,
        js_class = "CanvasCaptureMediaStreamTrack",
        js_name = "requestFrame"
    )]
    #[doc = "The `requestFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack/requestFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStreamTrack`*"]
    pub fn request_frame(this: &CanvasCaptureMediaStreamTrack);
}
