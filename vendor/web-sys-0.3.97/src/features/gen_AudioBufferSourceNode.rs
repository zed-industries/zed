#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioScheduledSourceNode",
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "AudioBufferSourceNode",
        typescript_type = "AudioBufferSourceNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub type AudioBufferSourceNode;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, getter, js_class = "AudioBufferSourceNode", js_name = "buffer")]
    #[doc = "Getter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
    pub fn buffer(this: &AudioBufferSourceNode) -> Option<AudioBuffer>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, setter, js_class = "AudioBufferSourceNode", js_name = "buffer")]
    #[doc = "Setter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
    pub fn set_buffer(this: &AudioBufferSourceNode, value: Option<&AudioBuffer>);
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioBufferSourceNode",
        js_name = "playbackRate"
    )]
    #[doc = "Getter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
    pub fn playback_rate(this: &AudioBufferSourceNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "AudioBufferSourceNode", js_name = "detune")]
    #[doc = "Getter for the `detune` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/detune)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
    pub fn detune(this: &AudioBufferSourceNode) -> AudioParam;
    #[wasm_bindgen(method, getter, js_class = "AudioBufferSourceNode", js_name = "loop")]
    #[doc = "Getter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn loop_(this: &AudioBufferSourceNode) -> bool;
    #[wasm_bindgen(method, setter, js_class = "AudioBufferSourceNode", js_name = "loop")]
    #[doc = "Setter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn set_loop(this: &AudioBufferSourceNode, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioBufferSourceNode",
        js_name = "loopStart"
    )]
    #[doc = "Getter for the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn loop_start(this: &AudioBufferSourceNode) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "AudioBufferSourceNode",
        js_name = "loopStart"
    )]
    #[doc = "Setter for the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn set_loop_start(this: &AudioBufferSourceNode, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioBufferSourceNode",
        js_name = "loopEnd"
    )]
    #[doc = "Getter for the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn loop_end(this: &AudioBufferSourceNode) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "AudioBufferSourceNode",
        js_name = "loopEnd"
    )]
    #[doc = "Setter for the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn set_loop_end(this: &AudioBufferSourceNode, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioBufferSourceNode",
        js_name = "onended"
    )]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
    pub fn onended(this: &AudioBufferSourceNode) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "AudioBufferSourceNode",
        js_name = "onended"
    )]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
    pub fn set_onended(this: &AudioBufferSourceNode, value: Option<&::js_sys::Function>);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioBufferSourceNode")]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `BaseAudioContext`*"]
    pub fn new(context: &BaseAudioContext) -> Result<AudioBufferSourceNode, JsValue>;
    #[cfg(all(feature = "AudioBufferSourceOptions", feature = "BaseAudioContext",))]
    #[wasm_bindgen(catch, constructor, js_class = "AudioBufferSourceNode")]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioBufferSourceOptions`, `BaseAudioContext`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &AudioBufferSourceOptions,
    ) -> Result<AudioBufferSourceNode, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBufferSourceNode")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start(this: &AudioBufferSourceNode) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBufferSourceNode", js_name = "start")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start_with_when(this: &AudioBufferSourceNode, when: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBufferSourceNode", js_name = "start")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start_with_when_and_grain_offset(
        this: &AudioBufferSourceNode,
        when: f64,
        offset: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBufferSourceNode", js_name = "start")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start_with_when_and_grain_offset_and_grain_duration(
        this: &AudioBufferSourceNode,
        when: f64,
        offset: f64,
        duration: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBufferSourceNode")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
    pub fn stop(this: &AudioBufferSourceNode) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBufferSourceNode", js_name = "stop")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
    pub fn stop_with_when(this: &AudioBufferSourceNode, when: f64) -> Result<(), JsValue>;
}
