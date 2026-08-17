#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "WorkletGlobalScope",
        extends = "::js_sys::Object",
        js_name = "AudioWorkletGlobalScope",
        typescript_type = "AudioWorkletGlobalScope"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub type AudioWorkletGlobalScope;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioWorkletGlobalScope",
        js_name = "currentFrame"
    )]
    #[doc = "Getter for the `currentFrame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn current_frame(this: &AudioWorkletGlobalScope) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioWorkletGlobalScope",
        js_name = "currentTime"
    )]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn current_time(this: &AudioWorkletGlobalScope) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioWorkletGlobalScope",
        js_name = "sampleRate"
    )]
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn sample_rate(this: &AudioWorkletGlobalScope) -> f32;
    #[wasm_bindgen(
        method,
        js_class = "AudioWorkletGlobalScope",
        js_name = "registerProcessor"
    )]
    #[doc = "The `registerProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/registerProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn register_processor(
        this: &AudioWorkletGlobalScope,
        name: &str,
        processor_ctor: &::js_sys::Function,
    );
}
