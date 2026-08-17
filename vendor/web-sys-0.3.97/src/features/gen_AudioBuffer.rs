#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioBuffer",
        typescript_type = "AudioBuffer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBuffer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub type AudioBuffer;
    #[wasm_bindgen(method, getter, js_class = "AudioBuffer", js_name = "sampleRate")]
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn sample_rate(this: &AudioBuffer) -> f32;
    #[wasm_bindgen(method, getter, js_class = "AudioBuffer", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn length(this: &AudioBuffer) -> u32;
    #[wasm_bindgen(method, getter, js_class = "AudioBuffer", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn duration(this: &AudioBuffer) -> f64;
    #[wasm_bindgen(method, getter, js_class = "AudioBuffer", js_name = "numberOfChannels")]
    #[doc = "Getter for the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/numberOfChannels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn number_of_channels(this: &AudioBuffer) -> u32;
    #[cfg(feature = "AudioBufferOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioBuffer")]
    #[doc = "The `new AudioBuffer(..)` constructor, creating a new instance of `AudioBuffer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/AudioBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferOptions`*"]
    pub fn new(options: &AudioBufferOptions) -> Result<AudioBuffer, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyFromChannel")]
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_from_channel(
        this: &AudioBuffer,
        destination: &mut [f32],
        channel_number: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyFromChannel")]
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_from_channel_with_f32_array(
        this: &AudioBuffer,
        destination: &::js_sys::Float32Array,
        channel_number: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyFromChannel")]
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_from_channel_with_start_in_channel(
        this: &AudioBuffer,
        destination: &mut [f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyFromChannel")]
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_from_channel_with_f32_array_and_start_in_channel(
        this: &AudioBuffer,
        destination: &::js_sys::Float32Array,
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyToChannel")]
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_to_channel(
        this: &AudioBuffer,
        source: &[f32],
        channel_number: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyToChannel")]
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_to_channel_with_f32_array(
        this: &AudioBuffer,
        source: &::js_sys::Float32Array,
        channel_number: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyToChannel")]
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_to_channel_with_start_in_channel(
        this: &AudioBuffer,
        source: &[f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "copyToChannel")]
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_to_channel_with_f32_array_and_start_in_channel(
        this: &AudioBuffer,
        source: &::js_sys::Float32Array,
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioBuffer", js_name = "getChannelData")]
    #[doc = "The `getChannelData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/getChannelData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn get_channel_data(
        this: &AudioBuffer,
        channel: u32,
    ) -> Result<::alloc::vec::Vec<f32>, JsValue>;
}
