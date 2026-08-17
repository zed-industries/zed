#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "AudioWorkletNode",
        typescript_type = "AudioWorkletNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
    pub type AudioWorkletNode;
    #[cfg(feature = "AudioParamMap")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "AudioWorkletNode",
        js_name = "parameters"
    )]
    #[doc = "Getter for the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/parameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`, `AudioWorkletNode`*"]
    pub fn parameters(this: &AudioWorkletNode) -> Result<AudioParamMap, JsValue>;
    #[cfg(feature = "MessagePort")]
    #[wasm_bindgen(catch, method, getter, js_class = "AudioWorkletNode", js_name = "port")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`, `MessagePort`*"]
    pub fn port(this: &AudioWorkletNode) -> Result<MessagePort, JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioWorkletNode",
        js_name = "onprocessorerror"
    )]
    #[doc = "Getter for the `onprocessorerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
    pub fn onprocessorerror(this: &AudioWorkletNode) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "AudioWorkletNode",
        js_name = "onprocessorerror"
    )]
    #[doc = "Setter for the `onprocessorerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
    pub fn set_onprocessorerror(this: &AudioWorkletNode, value: Option<&::js_sys::Function>);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioWorkletNode")]
    #[doc = "The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`, `BaseAudioContext`*"]
    pub fn new(context: &BaseAudioContext, name: &str) -> Result<AudioWorkletNode, JsValue>;
    #[cfg(all(feature = "AudioWorkletNodeOptions", feature = "BaseAudioContext",))]
    #[wasm_bindgen(catch, constructor, js_class = "AudioWorkletNode")]
    #[doc = "The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`, `AudioWorkletNodeOptions`, `BaseAudioContext`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        name: &str,
        options: &AudioWorkletNodeOptions,
    ) -> Result<AudioWorkletNode, JsValue>;
}
