#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "TextTrackCue",
        typescript_type = "TextTrackCue"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackCue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub type TextTrackCue;
    #[cfg(feature = "TextTrack")]
    #[wasm_bindgen(method, getter, js_class = "TextTrackCue", js_name = "track")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCue`*"]
    pub fn track(this: &TextTrackCue) -> Option<TextTrack>;
    #[wasm_bindgen(method, getter, js_class = "TextTrackCue", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn id(this: &TextTrackCue) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "TextTrackCue", js_name = "id")]
    #[doc = "Setter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_id(this: &TextTrackCue, value: &str);
    #[wasm_bindgen(method, getter, js_class = "TextTrackCue", js_name = "startTime")]
    #[doc = "Getter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn start_time(this: &TextTrackCue) -> f64;
    #[wasm_bindgen(method, setter, js_class = "TextTrackCue", js_name = "startTime")]
    #[doc = "Setter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_start_time(this: &TextTrackCue, value: f64);
    #[wasm_bindgen(method, getter, js_class = "TextTrackCue", js_name = "endTime")]
    #[doc = "Getter for the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn end_time(this: &TextTrackCue) -> f64;
    #[wasm_bindgen(method, setter, js_class = "TextTrackCue", js_name = "endTime")]
    #[doc = "Setter for the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_end_time(this: &TextTrackCue, value: f64);
    #[wasm_bindgen(method, getter, js_class = "TextTrackCue", js_name = "pauseOnExit")]
    #[doc = "Getter for the `pauseOnExit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn pause_on_exit(this: &TextTrackCue) -> bool;
    #[wasm_bindgen(method, setter, js_class = "TextTrackCue", js_name = "pauseOnExit")]
    #[doc = "Setter for the `pauseOnExit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_pause_on_exit(this: &TextTrackCue, value: bool);
    #[wasm_bindgen(method, getter, js_class = "TextTrackCue", js_name = "onenter")]
    #[doc = "Getter for the `onenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn onenter(this: &TextTrackCue) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TextTrackCue", js_name = "onenter")]
    #[doc = "Setter for the `onenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_onenter(this: &TextTrackCue, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "TextTrackCue", js_name = "onexit")]
    #[doc = "Getter for the `onexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn onexit(this: &TextTrackCue) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TextTrackCue", js_name = "onexit")]
    #[doc = "Setter for the `onexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
    pub fn set_onexit(this: &TextTrackCue, value: Option<&::js_sys::Function>);
}
