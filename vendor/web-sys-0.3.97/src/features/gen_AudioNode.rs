#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "AudioNode",
        typescript_type = "AudioNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub type AudioNode;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(method, getter, js_class = "AudioNode", js_name = "context")]
    #[doc = "Getter for the `context` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/context)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `BaseAudioContext`*"]
    pub fn context(this: &AudioNode) -> BaseAudioContext;
    #[wasm_bindgen(method, getter, js_class = "AudioNode", js_name = "numberOfInputs")]
    #[doc = "Getter for the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfInputs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn number_of_inputs(this: &AudioNode) -> u32;
    #[wasm_bindgen(method, getter, js_class = "AudioNode", js_name = "numberOfOutputs")]
    #[doc = "Getter for the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfOutputs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn number_of_outputs(this: &AudioNode) -> u32;
    #[wasm_bindgen(method, getter, js_class = "AudioNode", js_name = "channelCount")]
    #[doc = "Getter for the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn channel_count(this: &AudioNode) -> u32;
    #[wasm_bindgen(method, setter, js_class = "AudioNode", js_name = "channelCount")]
    #[doc = "Setter for the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn set_channel_count(this: &AudioNode, value: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter, js_class = "AudioNode", js_name = "channelCountMode")]
    #[doc = "Getter for the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelCountMode`*"]
    pub fn channel_count_mode(this: &AudioNode) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter, js_class = "AudioNode", js_name = "channelCountMode")]
    #[doc = "Setter for the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelCountMode`*"]
    pub fn set_channel_count_mode(this: &AudioNode, value: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AudioNode",
        js_name = "channelInterpretation"
    )]
    #[doc = "Getter for the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelInterpretation`*"]
    pub fn channel_interpretation(this: &AudioNode) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "AudioNode",
        js_name = "channelInterpretation"
    )]
    #[doc = "Setter for the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelInterpretation`*"]
    pub fn set_channel_interpretation(this: &AudioNode, value: ChannelInterpretation);
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "connect")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn connect_with_audio_node(
        this: &AudioNode,
        destination: &AudioNode,
    ) -> Result<AudioNode, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "connect")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn connect_with_audio_node_and_output(
        this: &AudioNode,
        destination: &AudioNode,
        output: u32,
    ) -> Result<AudioNode, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "connect")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn connect_with_audio_node_and_output_and_input(
        this: &AudioNode,
        destination: &AudioNode,
        output: u32,
        input: u32,
    ) -> Result<AudioNode, JsValue>;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "connect")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    pub fn connect_with_audio_param(
        this: &AudioNode,
        destination: &AudioParam,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "connect")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    pub fn connect_with_audio_param_and_output(
        this: &AudioNode,
        destination: &AudioParam,
        output: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioNode")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn disconnect(this: &AudioNode) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "disconnect")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn disconnect_with_output(this: &AudioNode, output: u32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "disconnect")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn disconnect_with_audio_node(
        this: &AudioNode,
        destination: &AudioNode,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "disconnect")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn disconnect_with_audio_node_and_output(
        this: &AudioNode,
        destination: &AudioNode,
        output: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "disconnect")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
    pub fn disconnect_with_audio_node_and_output_and_input(
        this: &AudioNode,
        destination: &AudioNode,
        output: u32,
        input: u32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "disconnect")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    pub fn disconnect_with_audio_param(
        this: &AudioNode,
        destination: &AudioParam,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(catch, method, js_class = "AudioNode", js_name = "disconnect")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    pub fn disconnect_with_audio_param_and_output(
        this: &AudioNode,
        destination: &AudioParam,
        output: u32,
    ) -> Result<(), JsValue>;
}
