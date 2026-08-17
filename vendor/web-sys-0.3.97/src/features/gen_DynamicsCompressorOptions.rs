#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DynamicsCompressorOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DynamicsCompressorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub type DynamicsCompressorOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &DynamicsCompressorOptions) -> Option<u32>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &DynamicsCompressorOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &DynamicsCompressorOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    pub fn set_channel_count_mode(this: &DynamicsCompressorOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(
        this: &DynamicsCompressorOptions,
    ) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    pub fn set_channel_interpretation(this: &DynamicsCompressorOptions, val: ChannelInterpretation);
    #[doc = "Get the `attack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "attack")]
    pub fn get_attack(this: &DynamicsCompressorOptions) -> Option<f32>;
    #[doc = "Change the `attack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "attack")]
    pub fn set_attack(this: &DynamicsCompressorOptions, val: f32);
    #[doc = "Get the `knee` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "knee")]
    pub fn get_knee(this: &DynamicsCompressorOptions) -> Option<f32>;
    #[doc = "Change the `knee` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "knee")]
    pub fn set_knee(this: &DynamicsCompressorOptions, val: f32);
    #[doc = "Get the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "ratio")]
    pub fn get_ratio(this: &DynamicsCompressorOptions) -> Option<f32>;
    #[doc = "Change the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "ratio")]
    pub fn set_ratio(this: &DynamicsCompressorOptions, val: f32);
    #[doc = "Get the `release` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "release")]
    pub fn get_release(this: &DynamicsCompressorOptions) -> Option<f32>;
    #[doc = "Change the `release` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "release")]
    pub fn set_release(this: &DynamicsCompressorOptions, val: f32);
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, getter = "threshold")]
    pub fn get_threshold(this: &DynamicsCompressorOptions) -> Option<f32>;
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    #[wasm_bindgen(method, setter = "threshold")]
    pub fn set_threshold(this: &DynamicsCompressorOptions, val: f32);
}
impl DynamicsCompressorOptions {
    #[doc = "Construct a new `DynamicsCompressorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_channel_count()` instead."]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[deprecated = "Use `set_channel_count_mode()` instead."]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[deprecated = "Use `set_channel_interpretation()` instead."]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation(val);
        self
    }
    #[deprecated = "Use `set_attack()` instead."]
    pub fn attack(&mut self, val: f32) -> &mut Self {
        self.set_attack(val);
        self
    }
    #[deprecated = "Use `set_knee()` instead."]
    pub fn knee(&mut self, val: f32) -> &mut Self {
        self.set_knee(val);
        self
    }
    #[deprecated = "Use `set_ratio()` instead."]
    pub fn ratio(&mut self, val: f32) -> &mut Self {
        self.set_ratio(val);
        self
    }
    #[deprecated = "Use `set_release()` instead."]
    pub fn release(&mut self, val: f32) -> &mut Self {
        self.set_release(val);
        self
    }
    #[deprecated = "Use `set_threshold()` instead."]
    pub fn threshold(&mut self, val: f32) -> &mut Self {
        self.set_threshold(val);
        self
    }
}
impl Default for DynamicsCompressorOptions {
    fn default() -> Self {
        Self::new()
    }
}
