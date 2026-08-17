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
        js_name = "BiquadFilterNode",
        typescript_type = "BiquadFilterNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BiquadFilterNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub type BiquadFilterNode;
    #[cfg(feature = "BiquadFilterType")]
    #[wasm_bindgen(method, getter, js_class = "BiquadFilterNode", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
    pub fn type_(this: &BiquadFilterNode) -> BiquadFilterType;
    #[cfg(feature = "BiquadFilterType")]
    #[wasm_bindgen(method, setter, js_class = "BiquadFilterNode", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
    pub fn set_type(this: &BiquadFilterNode, value: BiquadFilterType);
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "BiquadFilterNode", js_name = "frequency")]
    #[doc = "Getter for the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/frequency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn frequency(this: &BiquadFilterNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "BiquadFilterNode", js_name = "detune")]
    #[doc = "Getter for the `detune` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/detune)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn detune(this: &BiquadFilterNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "BiquadFilterNode", js_name = "Q")]
    #[doc = "Getter for the `Q` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/Q)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn q(this: &BiquadFilterNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "BiquadFilterNode", js_name = "gain")]
    #[doc = "Getter for the `gain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/gain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn gain(this: &BiquadFilterNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "BiquadFilterNode")]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<BiquadFilterNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "BiquadFilterOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "BiquadFilterNode")]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`, `BiquadFilterOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &BiquadFilterOptions,
    ) -> Result<BiquadFilterNode, JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response(
        this: &BiquadFilterNode,
        frequency_hz: &mut [f32],
        mag_response: &mut [f32],
        phase_response: &mut [f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response_with_f32_array_and_f32_slice_and_f32_slice(
        this: &BiquadFilterNode,
        frequency_hz: &::js_sys::Float32Array,
        mag_response: &mut [f32],
        phase_response: &mut [f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response_with_f32_slice_and_f32_array_and_f32_slice(
        this: &BiquadFilterNode,
        frequency_hz: &mut [f32],
        mag_response: &::js_sys::Float32Array,
        phase_response: &mut [f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response_with_f32_array_and_f32_array_and_f32_slice(
        this: &BiquadFilterNode,
        frequency_hz: &::js_sys::Float32Array,
        mag_response: &::js_sys::Float32Array,
        phase_response: &mut [f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response_with_f32_slice_and_f32_slice_and_f32_array(
        this: &BiquadFilterNode,
        frequency_hz: &mut [f32],
        mag_response: &mut [f32],
        phase_response: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response_with_f32_array_and_f32_slice_and_f32_array(
        this: &BiquadFilterNode,
        frequency_hz: &::js_sys::Float32Array,
        mag_response: &mut [f32],
        phase_response: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response_with_f32_slice_and_f32_array_and_f32_array(
        this: &BiquadFilterNode,
        frequency_hz: &mut [f32],
        mag_response: &::js_sys::Float32Array,
        phase_response: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "BiquadFilterNode",
        js_name = "getFrequencyResponse"
    )]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response_with_f32_array_and_f32_array_and_f32_array(
        this: &BiquadFilterNode,
        frequency_hz: &::js_sys::Float32Array,
        mag_response: &::js_sys::Float32Array,
        phase_response: &::js_sys::Float32Array,
    );
}
