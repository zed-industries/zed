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
        js_name = "AnalyserNode",
        typescript_type = "AnalyserNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnalyserNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub type AnalyserNode;
    #[wasm_bindgen(method, getter, js_class = "AnalyserNode", js_name = "fftSize")]
    #[doc = "Getter for the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn fft_size(this: &AnalyserNode) -> u32;
    #[wasm_bindgen(method, setter, js_class = "AnalyserNode", js_name = "fftSize")]
    #[doc = "Setter for the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn set_fft_size(this: &AnalyserNode, value: u32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AnalyserNode",
        js_name = "frequencyBinCount"
    )]
    #[doc = "Getter for the `frequencyBinCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/frequencyBinCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn frequency_bin_count(this: &AnalyserNode) -> u32;
    #[wasm_bindgen(method, getter, js_class = "AnalyserNode", js_name = "minDecibels")]
    #[doc = "Getter for the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn min_decibels(this: &AnalyserNode) -> f64;
    #[wasm_bindgen(method, setter, js_class = "AnalyserNode", js_name = "minDecibels")]
    #[doc = "Setter for the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn set_min_decibels(this: &AnalyserNode, value: f64);
    #[wasm_bindgen(method, getter, js_class = "AnalyserNode", js_name = "maxDecibels")]
    #[doc = "Getter for the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn max_decibels(this: &AnalyserNode) -> f64;
    #[wasm_bindgen(method, setter, js_class = "AnalyserNode", js_name = "maxDecibels")]
    #[doc = "Setter for the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn set_max_decibels(this: &AnalyserNode, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AnalyserNode",
        js_name = "smoothingTimeConstant"
    )]
    #[doc = "Getter for the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn smoothing_time_constant(this: &AnalyserNode) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "AnalyserNode",
        js_name = "smoothingTimeConstant"
    )]
    #[doc = "Setter for the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn set_smoothing_time_constant(this: &AnalyserNode, value: f64);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "AnalyserNode")]
    #[doc = "The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `BaseAudioContext`*"]
    pub fn new(context: &BaseAudioContext) -> Result<AnalyserNode, JsValue>;
    #[cfg(all(feature = "AnalyserOptions", feature = "BaseAudioContext",))]
    #[wasm_bindgen(catch, constructor, js_class = "AnalyserNode")]
    #[doc = "The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `AnalyserOptions`, `BaseAudioContext`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &AnalyserOptions,
    ) -> Result<AnalyserNode, JsValue>;
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getByteFrequencyData")]
    #[doc = "The `getByteFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_byte_frequency_data(this: &AnalyserNode, array: &mut [u8]);
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getByteFrequencyData")]
    #[doc = "The `getByteFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_byte_frequency_data_with_u8_array(this: &AnalyserNode, array: &::js_sys::Uint8Array);
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getByteTimeDomainData")]
    #[doc = "The `getByteTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_byte_time_domain_data(this: &AnalyserNode, array: &mut [u8]);
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getByteTimeDomainData")]
    #[doc = "The `getByteTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_byte_time_domain_data_with_u8_array(
        this: &AnalyserNode,
        array: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getFloatFrequencyData")]
    #[doc = "The `getFloatFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_float_frequency_data(this: &AnalyserNode, array: &mut [f32]);
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getFloatFrequencyData")]
    #[doc = "The `getFloatFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_float_frequency_data_with_f32_array(
        this: &AnalyserNode,
        array: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getFloatTimeDomainData")]
    #[doc = "The `getFloatTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_float_time_domain_data(this: &AnalyserNode, array: &mut [f32]);
    #[wasm_bindgen(method, js_class = "AnalyserNode", js_name = "getFloatTimeDomainData")]
    #[doc = "The `getFloatTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
    pub fn get_float_time_domain_data_with_f32_array(
        this: &AnalyserNode,
        array: &::js_sys::Float32Array,
    );
}
