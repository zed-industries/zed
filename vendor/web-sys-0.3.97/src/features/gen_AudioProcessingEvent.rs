#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "AudioProcessingEvent",
        typescript_type = "AudioProcessingEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioProcessingEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
    pub type AudioProcessingEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioProcessingEvent",
        js_name = "playbackTime"
    )]
    #[doc = "Getter for the `playbackTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/playbackTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
    pub fn playback_time(this: &AudioProcessingEvent) -> f64;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "AudioProcessingEvent",
        js_name = "inputBuffer"
    )]
    #[doc = "Getter for the `inputBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/inputBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
    pub fn input_buffer(this: &AudioProcessingEvent) -> Result<AudioBuffer, JsValue>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "AudioProcessingEvent",
        js_name = "outputBuffer"
    )]
    #[doc = "Getter for the `outputBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/outputBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
    pub fn output_buffer(this: &AudioProcessingEvent) -> Result<AudioBuffer, JsValue>;
}
