#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ChannelMergerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelMergerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    pub type ChannelMergerOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &ChannelMergerOptions) -> Option<u32>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &ChannelMergerOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &ChannelMergerOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    pub fn set_channel_count_mode(this: &ChannelMergerOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &ChannelMergerOptions)
        -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    pub fn set_channel_interpretation(this: &ChannelMergerOptions, val: ChannelInterpretation);
    #[doc = "Get the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, getter = "numberOfInputs")]
    pub fn get_number_of_inputs(this: &ChannelMergerOptions) -> Option<u32>;
    #[doc = "Change the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    #[wasm_bindgen(method, setter = "numberOfInputs")]
    pub fn set_number_of_inputs(this: &ChannelMergerOptions, val: u32);
}
impl ChannelMergerOptions {
    #[doc = "Construct a new `ChannelMergerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
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
    #[deprecated = "Use `set_number_of_inputs()` instead."]
    pub fn number_of_inputs(&mut self, val: u32) -> &mut Self {
        self.set_number_of_inputs(val);
        self
    }
}
impl Default for ChannelMergerOptions {
    fn default() -> Self {
        Self::new()
    }
}
