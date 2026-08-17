#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DelayOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DelayOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub type DelayOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &DelayOptions) -> Option<u32>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &DelayOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DelayOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &DelayOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DelayOptions`*"]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    pub fn set_channel_count_mode(this: &DelayOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DelayOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &DelayOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DelayOptions`*"]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    pub fn set_channel_interpretation(this: &DelayOptions, val: ChannelInterpretation);
    #[doc = "Get the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    #[wasm_bindgen(method, getter = "delayTime")]
    pub fn get_delay_time(this: &DelayOptions) -> Option<f64>;
    #[doc = "Change the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    #[wasm_bindgen(method, setter = "delayTime")]
    pub fn set_delay_time(this: &DelayOptions, val: f64);
    #[doc = "Get the `maxDelayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    #[wasm_bindgen(method, getter = "maxDelayTime")]
    pub fn get_max_delay_time(this: &DelayOptions) -> Option<f64>;
    #[doc = "Change the `maxDelayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    #[wasm_bindgen(method, setter = "maxDelayTime")]
    pub fn set_max_delay_time(this: &DelayOptions, val: f64);
}
impl DelayOptions {
    #[doc = "Construct a new `DelayOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
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
    #[deprecated = "Use `set_delay_time()` instead."]
    pub fn delay_time(&mut self, val: f64) -> &mut Self {
        self.set_delay_time(val);
        self
    }
    #[deprecated = "Use `set_max_delay_time()` instead."]
    pub fn max_delay_time(&mut self, val: f64) -> &mut Self {
        self.set_max_delay_time(val);
        self
    }
}
impl Default for DelayOptions {
    fn default() -> Self {
        Self::new()
    }
}
