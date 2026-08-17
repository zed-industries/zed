#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioWorkletProcessor",
        typescript_type = "AudioWorkletProcessor"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletProcessor` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletProcessor`*"]
    pub type AudioWorkletProcessor;
    #[cfg(feature = "MessagePort")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "AudioWorkletProcessor",
        js_name = "port"
    )]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletProcessor`, `MessagePort`*"]
    pub fn port(this: &AudioWorkletProcessor) -> Result<MessagePort, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "AudioWorkletProcessor")]
    #[doc = "The `new AudioWorkletProcessor(..)` constructor, creating a new instance of `AudioWorkletProcessor`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/AudioWorkletProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletProcessor`*"]
    pub fn new() -> Result<AudioWorkletProcessor, JsValue>;
    #[cfg(feature = "AudioWorkletNodeOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioWorkletProcessor")]
    #[doc = "The `new AudioWorkletProcessor(..)` constructor, creating a new instance of `AudioWorkletProcessor`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/AudioWorkletProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `AudioWorkletProcessor`*"]
    pub fn new_with_options(
        options: &AudioWorkletNodeOptions,
    ) -> Result<AudioWorkletProcessor, JsValue>;
}
