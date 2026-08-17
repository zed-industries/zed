#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AudioBufferSourceOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub type AudioBufferSourceOptions;
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, getter = "buffer")]
    pub fn get_buffer(this: &AudioBufferSourceOptions) -> Option<AudioBuffer>;
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, setter = "buffer")]
    pub fn set_buffer(this: &AudioBufferSourceOptions, val: Option<&AudioBuffer>);
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, getter = "detune")]
    pub fn get_detune(this: &AudioBufferSourceOptions) -> Option<f32>;
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, setter = "detune")]
    pub fn set_detune(this: &AudioBufferSourceOptions, val: f32);
    #[doc = "Get the `loop` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, getter = "loop")]
    pub fn get_loop(this: &AudioBufferSourceOptions) -> Option<bool>;
    #[doc = "Change the `loop` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, setter = "loop")]
    pub fn set_loop(this: &AudioBufferSourceOptions, val: bool);
    #[doc = "Get the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, getter = "loopEnd")]
    pub fn get_loop_end(this: &AudioBufferSourceOptions) -> Option<f64>;
    #[doc = "Change the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, setter = "loopEnd")]
    pub fn set_loop_end(this: &AudioBufferSourceOptions, val: f64);
    #[doc = "Get the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, getter = "loopStart")]
    pub fn get_loop_start(this: &AudioBufferSourceOptions) -> Option<f64>;
    #[doc = "Change the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, setter = "loopStart")]
    pub fn set_loop_start(this: &AudioBufferSourceOptions, val: f64);
    #[doc = "Get the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, getter = "playbackRate")]
    pub fn get_playback_rate(this: &AudioBufferSourceOptions) -> Option<f32>;
    #[doc = "Change the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    #[wasm_bindgen(method, setter = "playbackRate")]
    pub fn set_playback_rate(this: &AudioBufferSourceOptions, val: f32);
}
impl AudioBufferSourceOptions {
    #[doc = "Construct a new `AudioBufferSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "AudioBuffer")]
    #[deprecated = "Use `set_buffer()` instead."]
    pub fn buffer(&mut self, val: Option<&AudioBuffer>) -> &mut Self {
        self.set_buffer(val);
        self
    }
    #[deprecated = "Use `set_detune()` instead."]
    pub fn detune(&mut self, val: f32) -> &mut Self {
        self.set_detune(val);
        self
    }
    #[deprecated = "Use `set_loop()` instead."]
    pub fn loop_(&mut self, val: bool) -> &mut Self {
        self.set_loop(val);
        self
    }
    #[deprecated = "Use `set_loop_end()` instead."]
    pub fn loop_end(&mut self, val: f64) -> &mut Self {
        self.set_loop_end(val);
        self
    }
    #[deprecated = "Use `set_loop_start()` instead."]
    pub fn loop_start(&mut self, val: f64) -> &mut Self {
        self.set_loop_start(val);
        self
    }
    #[deprecated = "Use `set_playback_rate()` instead."]
    pub fn playback_rate(&mut self, val: f32) -> &mut Self {
        self.set_playback_rate(val);
        self
    }
}
impl Default for AudioBufferSourceOptions {
    fn default() -> Self {
        Self::new()
    }
}
