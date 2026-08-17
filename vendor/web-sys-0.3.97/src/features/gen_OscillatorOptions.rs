#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "OscillatorOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OscillatorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub type OscillatorOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &OscillatorOptions) -> Option<u32>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &OscillatorOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &OscillatorOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*"]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    pub fn set_channel_count_mode(this: &OscillatorOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &OscillatorOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*"]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    pub fn set_channel_interpretation(this: &OscillatorOptions, val: ChannelInterpretation);
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    #[wasm_bindgen(method, getter = "detune")]
    pub fn get_detune(this: &OscillatorOptions) -> Option<f32>;
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    #[wasm_bindgen(method, setter = "detune")]
    pub fn set_detune(this: &OscillatorOptions, val: f32);
    #[doc = "Get the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    #[wasm_bindgen(method, getter = "frequency")]
    pub fn get_frequency(this: &OscillatorOptions) -> Option<f32>;
    #[doc = "Change the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    #[wasm_bindgen(method, setter = "frequency")]
    pub fn set_frequency(this: &OscillatorOptions, val: f32);
    #[cfg(feature = "PeriodicWave")]
    #[doc = "Get the `periodicWave` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*"]
    #[wasm_bindgen(method, getter = "periodicWave")]
    pub fn get_periodic_wave(this: &OscillatorOptions) -> Option<PeriodicWave>;
    #[cfg(feature = "PeriodicWave")]
    #[doc = "Change the `periodicWave` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*"]
    #[wasm_bindgen(method, setter = "periodicWave")]
    pub fn set_periodic_wave(this: &OscillatorOptions, val: &PeriodicWave);
    #[cfg(feature = "OscillatorType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OscillatorOptions) -> Option<OscillatorType>;
    #[cfg(feature = "OscillatorType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OscillatorOptions, val: OscillatorType);
}
impl OscillatorOptions {
    #[doc = "Construct a new `OscillatorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
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
    #[cfg(feature = "PeriodicWave")]
    #[deprecated = "Use `set_periodic_wave()` instead."]
    pub fn periodic_wave(&mut self, val: &PeriodicWave) -> &mut Self {
        self.set_periodic_wave(val);
        self
    }
    #[cfg(feature = "OscillatorType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: OscillatorType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for OscillatorOptions {
    fn default() -> Self {
        Self::new()
    }
}
