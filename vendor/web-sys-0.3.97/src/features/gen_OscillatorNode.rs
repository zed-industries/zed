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
        js_name = "OscillatorNode",
        typescript_type = "OscillatorNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OscillatorNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub type OscillatorNode;
    #[cfg(feature = "OscillatorType")]
    #[wasm_bindgen(method, getter, js_class = "OscillatorNode", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
    pub fn type_(this: &OscillatorNode) -> OscillatorType;
    #[cfg(feature = "OscillatorType")]
    #[wasm_bindgen(method, setter, js_class = "OscillatorNode", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
    pub fn set_type(this: &OscillatorNode, value: OscillatorType);
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "OscillatorNode", js_name = "frequency")]
    #[doc = "Getter for the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/frequency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
    pub fn frequency(this: &OscillatorNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "OscillatorNode", js_name = "detune")]
    #[doc = "Getter for the `detune` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/detune)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
    pub fn detune(this: &OscillatorNode) -> AudioParam;
    #[wasm_bindgen(method, getter, js_class = "OscillatorNode", js_name = "onended")]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn onended(this: &OscillatorNode) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "OscillatorNode", js_name = "onended")]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn set_onended(this: &OscillatorNode, value: Option<&::js_sys::Function>);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "OscillatorNode")]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<OscillatorNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "OscillatorOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "OscillatorNode")]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`, `OscillatorOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &OscillatorOptions,
    ) -> Result<OscillatorNode, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(method, js_class = "OscillatorNode", js_name = "setPeriodicWave")]
    #[doc = "The `setPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/setPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`, `PeriodicWave`*"]
    pub fn set_periodic_wave(this: &OscillatorNode, periodic_wave: &PeriodicWave);
    #[wasm_bindgen(catch, method, js_class = "OscillatorNode")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn start(this: &OscillatorNode) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OscillatorNode", js_name = "start")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn start_with_when(this: &OscillatorNode, when: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OscillatorNode")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn stop(this: &OscillatorNode) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OscillatorNode", js_name = "stop")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn stop_with_when(this: &OscillatorNode, when: f64) -> Result<(), JsValue>;
}
