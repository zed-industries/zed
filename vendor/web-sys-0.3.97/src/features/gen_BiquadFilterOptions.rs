#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "BiquadFilterOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BiquadFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub type BiquadFilterOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &BiquadFilterOptions) -> Option<u32>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &BiquadFilterOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelCountMode`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &BiquadFilterOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelCountMode`*"]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    pub fn set_channel_count_mode(this: &BiquadFilterOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelInterpretation`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &BiquadFilterOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelInterpretation`*"]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    pub fn set_channel_interpretation(this: &BiquadFilterOptions, val: ChannelInterpretation);
    #[doc = "Get the `Q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, getter = "Q")]
    pub fn get_q(this: &BiquadFilterOptions) -> Option<f32>;
    #[doc = "Change the `Q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, setter = "Q")]
    pub fn set_q(this: &BiquadFilterOptions, val: f32);
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, getter = "detune")]
    pub fn get_detune(this: &BiquadFilterOptions) -> Option<f32>;
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, setter = "detune")]
    pub fn set_detune(this: &BiquadFilterOptions, val: f32);
    #[doc = "Get the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, getter = "frequency")]
    pub fn get_frequency(this: &BiquadFilterOptions) -> Option<f32>;
    #[doc = "Change the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, setter = "frequency")]
    pub fn set_frequency(this: &BiquadFilterOptions, val: f32);
    #[doc = "Get the `gain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, getter = "gain")]
    pub fn get_gain(this: &BiquadFilterOptions) -> Option<f32>;
    #[doc = "Change the `gain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    #[wasm_bindgen(method, setter = "gain")]
    pub fn set_gain(this: &BiquadFilterOptions, val: f32);
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `BiquadFilterType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &BiquadFilterOptions) -> Option<BiquadFilterType>;
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `BiquadFilterType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &BiquadFilterOptions, val: BiquadFilterType);
}
impl BiquadFilterOptions {
    #[doc = "Construct a new `BiquadFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
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
    #[deprecated = "Use `set_q()` instead."]
    pub fn q(&mut self, val: f32) -> &mut Self {
        self.set_q(val);
        self
    }
    #[deprecated = "Use `set_detune()` instead."]
    pub fn detune(&mut self, val: f32) -> &mut Self {
        self.set_detune(val);
        self
    }
    #[deprecated = "Use `set_frequency()` instead."]
    pub fn frequency(&mut self, val: f32) -> &mut Self {
        self.set_frequency(val);
        self
    }
    #[deprecated = "Use `set_gain()` instead."]
    pub fn gain(&mut self, val: f32) -> &mut Self {
        self.set_gain(val);
        self
    }
    #[cfg(feature = "BiquadFilterType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: BiquadFilterType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for BiquadFilterOptions {
    fn default() -> Self {
        Self::new()
    }
}
