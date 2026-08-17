#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WaveShaperOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WaveShaperOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    pub type WaveShaperOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &WaveShaperOptions) -> Option<u32>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &WaveShaperOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `WaveShaperOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &WaveShaperOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `WaveShaperOptions`*"]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    pub fn set_channel_count_mode(this: &WaveShaperOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `WaveShaperOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &WaveShaperOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `WaveShaperOptions`*"]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    pub fn set_channel_interpretation(this: &WaveShaperOptions, val: ChannelInterpretation);
    #[doc = "Get the `curve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    #[wasm_bindgen(method, getter = "curve")]
    pub fn get_curve(this: &WaveShaperOptions) -> Option<::js_sys::Array>;
    #[doc = "Change the `curve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    #[wasm_bindgen(method, setter = "curve")]
    pub fn set_curve(this: &WaveShaperOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "OverSampleType")]
    #[doc = "Get the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperOptions`*"]
    #[wasm_bindgen(method, getter = "oversample")]
    pub fn get_oversample(this: &WaveShaperOptions) -> Option<OverSampleType>;
    #[cfg(feature = "OverSampleType")]
    #[doc = "Change the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperOptions`*"]
    #[wasm_bindgen(method, setter = "oversample")]
    pub fn set_oversample(this: &WaveShaperOptions, val: OverSampleType);
}
impl WaveShaperOptions {
    #[doc = "Construct a new `WaveShaperOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
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
    #[deprecated = "Use `set_curve()` instead."]
    pub fn curve(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_curve(val);
        self
    }
    #[cfg(feature = "OverSampleType")]
    #[deprecated = "Use `set_oversample()` instead."]
    pub fn oversample(&mut self, val: OverSampleType) -> &mut Self {
        self.set_oversample(val);
        self
    }
}
impl Default for WaveShaperOptions {
    fn default() -> Self {
        Self::new()
    }
}
