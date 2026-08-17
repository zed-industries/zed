#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VideoPlaybackQuality",
        typescript_type = "VideoPlaybackQuality"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoPlaybackQuality` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub type VideoPlaybackQuality;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VideoPlaybackQuality",
        js_name = "creationTime"
    )]
    #[doc = "Getter for the `creationTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/creationTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn creation_time(this: &VideoPlaybackQuality) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VideoPlaybackQuality",
        js_name = "totalVideoFrames"
    )]
    #[doc = "Getter for the `totalVideoFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/totalVideoFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn total_video_frames(this: &VideoPlaybackQuality) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VideoPlaybackQuality",
        js_name = "droppedVideoFrames"
    )]
    #[doc = "Getter for the `droppedVideoFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/droppedVideoFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn dropped_video_frames(this: &VideoPlaybackQuality) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VideoPlaybackQuality",
        js_name = "corruptedVideoFrames"
    )]
    #[doc = "Getter for the `corruptedVideoFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/corruptedVideoFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    pub fn corrupted_video_frames(this: &VideoPlaybackQuality) -> u32;
}
