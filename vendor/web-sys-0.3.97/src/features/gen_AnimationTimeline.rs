#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AnimationTimeline",
        typescript_type = "AnimationTimeline"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationTimeline` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationTimeline`*"]
    pub type AnimationTimeline;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AnimationTimeline",
        js_name = "currentTime"
    )]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationTimeline`*"]
    pub fn current_time(this: &AnimationTimeline) -> Option<f64>;
}
