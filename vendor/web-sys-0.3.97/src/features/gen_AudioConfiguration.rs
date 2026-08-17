#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AudioConfiguration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub type AudioConfiguration;
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, getter = "bitrate")]
    pub fn get_bitrate(this: &AudioConfiguration) -> Option<f64>;
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate(this: &AudioConfiguration, val: f64);
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate_u32(this: &AudioConfiguration, val: u32);
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate_f64(this: &AudioConfiguration, val: f64);
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, getter = "channels")]
    pub fn get_channels(this: &AudioConfiguration) -> Option<::alloc::string::String>;
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, setter = "channels")]
    pub fn set_channels(this: &AudioConfiguration, val: &str);
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, getter = "contentType")]
    pub fn get_content_type(this: &AudioConfiguration) -> Option<::alloc::string::String>;
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, setter = "contentType")]
    pub fn set_content_type(this: &AudioConfiguration, val: &str);
    #[doc = "Get the `samplerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, getter = "samplerate")]
    pub fn get_samplerate(this: &AudioConfiguration) -> Option<u32>;
    #[doc = "Change the `samplerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    #[wasm_bindgen(method, setter = "samplerate")]
    pub fn set_samplerate(this: &AudioConfiguration, val: u32);
}
impl AudioConfiguration {
    #[doc = "Construct a new `AudioConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bitrate()` instead."]
    pub fn bitrate(&mut self, val: f64) -> &mut Self {
        self.set_bitrate(val);
        self
    }
    #[deprecated = "Use `set_channels()` instead."]
    pub fn channels(&mut self, val: &str) -> &mut Self {
        self.set_channels(val);
        self
    }
    #[deprecated = "Use `set_content_type()` instead."]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        self.set_content_type(val);
        self
    }
    #[deprecated = "Use `set_samplerate()` instead."]
    pub fn samplerate(&mut self, val: u32) -> &mut Self {
        self.set_samplerate(val);
        self
    }
}
impl Default for AudioConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
