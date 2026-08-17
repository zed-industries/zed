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
        js_name = "VideoStreamTrack",
        typescript_type = "VideoStreamTrack"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoStreamTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoStreamTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoStreamTrack`*"]
    pub type VideoStreamTrack;
}
