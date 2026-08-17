#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioData",
        typescript_type = "AudioData"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioData;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    #[wasm_bindgen(method, getter, js_class = "AudioData", js_name = "format")]
    #[doc = "Getter for the `format` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/format)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioSampleFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(this: &AudioData) -> Option<AudioSampleFormat>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "AudioData", js_name = "sampleRate")]
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_rate(this: &AudioData) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "AudioData", js_name = "numberOfFrames")]
    #[doc = "Getter for the `numberOfFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/numberOfFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn number_of_frames(this: &AudioData) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "AudioData", js_name = "numberOfChannels")]
    #[doc = "Getter for the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/numberOfChannels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn number_of_channels(this: &AudioData) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "AudioData", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(this: &AudioData) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "AudioData", js_name = "timestamp")]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(this: &AudioData) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataInit")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioData")]
    #[doc = "The `new AudioData(..)` constructor, creating a new instance of `AudioData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/AudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &AudioDataInit) -> Result<AudioData, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[wasm_bindgen(catch, method, js_class = "AudioData", js_name = "allocationSize")]
    #[doc = "The `allocationSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/allocationSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn allocation_size(
        this: &AudioData,
        options: &AudioDataCopyToOptions,
    ) -> Result<u32, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "AudioData")]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn clone(this: &AudioData) -> Result<AudioData, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "AudioData")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &AudioData);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[wasm_bindgen(catch, method, js_class = "AudioData", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_buffer_source(
        this: &AudioData,
        destination: &::js_sys::Object,
        options: &AudioDataCopyToOptions,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[wasm_bindgen(catch, method, js_class = "AudioData", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_u8_slice(
        this: &AudioData,
        destination: &mut [u8],
        options: &AudioDataCopyToOptions,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[wasm_bindgen(catch, method, js_class = "AudioData", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_u8_array(
        this: &AudioData,
        destination: &::js_sys::Uint8Array,
        options: &AudioDataCopyToOptions,
    ) -> Result<(), JsValue>;
}
