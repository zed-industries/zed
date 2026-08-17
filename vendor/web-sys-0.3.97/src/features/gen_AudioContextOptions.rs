#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AudioContextOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioContextOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    pub type AudioContextOptions;
    #[doc = "Get the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[wasm_bindgen(method, getter = "latencyHint")]
    pub fn get_latency_hint(this: &AudioContextOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[wasm_bindgen(method, setter = "latencyHint")]
    pub fn set_latency_hint(this: &AudioContextOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioContextLatencyCategory")]
    #[doc = "Change the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextLatencyCategory`, `AudioContextOptions`*"]
    #[wasm_bindgen(method, setter = "latencyHint")]
    pub fn set_latency_hint_audio_context_latency_category(
        this: &AudioContextOptions,
        val: AudioContextLatencyCategory,
    );
    #[doc = "Change the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[wasm_bindgen(method, setter = "latencyHint")]
    pub fn set_latency_hint_f64(this: &AudioContextOptions, val: f64);
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[wasm_bindgen(method, getter = "sampleRate")]
    pub fn get_sample_rate(this: &AudioContextOptions) -> Option<f32>;
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[wasm_bindgen(method, setter = "sampleRate")]
    pub fn set_sample_rate(this: &AudioContextOptions, val: f32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "sinkId")]
    pub fn get_sink_id(this: &AudioContextOptions) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "sinkId")]
    pub fn set_sink_id(this: &AudioContextOptions, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkOptions")]
    #[doc = "Change the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`, `AudioSinkOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "sinkId")]
    pub fn set_sink_id_audio_sink_options(this: &AudioContextOptions, val: &AudioSinkOptions);
}
impl AudioContextOptions {
    #[doc = "Construct a new `AudioContextOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_latency_hint()` instead."]
    pub fn latency_hint(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_latency_hint(val);
        self
    }
    #[deprecated = "Use `set_sample_rate()` instead."]
    pub fn sample_rate(&mut self, val: f32) -> &mut Self {
        self.set_sample_rate(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_sink_id()` instead."]
    pub fn sink_id(&mut self, val: &str) -> &mut Self {
        self.set_sink_id(val);
        self
    }
}
impl Default for AudioContextOptions {
    fn default() -> Self {
        Self::new()
    }
}
