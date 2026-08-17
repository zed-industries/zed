#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PannerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PannerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub type PannerOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &PannerOptions) -> Option<u32>;
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "channelCount")]
    pub fn set_channel_count(this: &PannerOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &PannerOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    pub fn set_channel_count_mode(this: &PannerOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &PannerOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    pub fn set_channel_interpretation(this: &PannerOptions, val: ChannelInterpretation);
    #[doc = "Get the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "coneInnerAngle")]
    pub fn get_cone_inner_angle(this: &PannerOptions) -> Option<f64>;
    #[doc = "Change the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "coneInnerAngle")]
    pub fn set_cone_inner_angle(this: &PannerOptions, val: f64);
    #[doc = "Get the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "coneOuterAngle")]
    pub fn get_cone_outer_angle(this: &PannerOptions) -> Option<f64>;
    #[doc = "Change the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "coneOuterAngle")]
    pub fn set_cone_outer_angle(this: &PannerOptions, val: f64);
    #[doc = "Get the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "coneOuterGain")]
    pub fn get_cone_outer_gain(this: &PannerOptions) -> Option<f64>;
    #[doc = "Change the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "coneOuterGain")]
    pub fn set_cone_outer_gain(this: &PannerOptions, val: f64);
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Get the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "distanceModel")]
    pub fn get_distance_model(this: &PannerOptions) -> Option<DistanceModelType>;
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Change the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "distanceModel")]
    pub fn set_distance_model(this: &PannerOptions, val: DistanceModelType);
    #[doc = "Get the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "maxDistance")]
    pub fn get_max_distance(this: &PannerOptions) -> Option<f64>;
    #[doc = "Change the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "maxDistance")]
    pub fn set_max_distance(this: &PannerOptions, val: f64);
    #[doc = "Get the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "orientationX")]
    pub fn get_orientation_x(this: &PannerOptions) -> Option<f32>;
    #[doc = "Change the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "orientationX")]
    pub fn set_orientation_x(this: &PannerOptions, val: f32);
    #[doc = "Get the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "orientationY")]
    pub fn get_orientation_y(this: &PannerOptions) -> Option<f32>;
    #[doc = "Change the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "orientationY")]
    pub fn set_orientation_y(this: &PannerOptions, val: f32);
    #[doc = "Get the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "orientationZ")]
    pub fn get_orientation_z(this: &PannerOptions) -> Option<f32>;
    #[doc = "Change the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "orientationZ")]
    pub fn set_orientation_z(this: &PannerOptions, val: f32);
    #[cfg(feature = "PanningModelType")]
    #[doc = "Get the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
    #[wasm_bindgen(method, getter = "panningModel")]
    pub fn get_panning_model(this: &PannerOptions) -> Option<PanningModelType>;
    #[cfg(feature = "PanningModelType")]
    #[doc = "Change the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
    #[wasm_bindgen(method, setter = "panningModel")]
    pub fn set_panning_model(this: &PannerOptions, val: PanningModelType);
    #[doc = "Get the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "positionX")]
    pub fn get_position_x(this: &PannerOptions) -> Option<f32>;
    #[doc = "Change the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "positionX")]
    pub fn set_position_x(this: &PannerOptions, val: f32);
    #[doc = "Get the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "positionY")]
    pub fn get_position_y(this: &PannerOptions) -> Option<f32>;
    #[doc = "Change the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "positionY")]
    pub fn set_position_y(this: &PannerOptions, val: f32);
    #[doc = "Get the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "positionZ")]
    pub fn get_position_z(this: &PannerOptions) -> Option<f32>;
    #[doc = "Change the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "positionZ")]
    pub fn set_position_z(this: &PannerOptions, val: f32);
    #[doc = "Get the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "refDistance")]
    pub fn get_ref_distance(this: &PannerOptions) -> Option<f64>;
    #[doc = "Change the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "refDistance")]
    pub fn set_ref_distance(this: &PannerOptions, val: f64);
    #[doc = "Get the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, getter = "rolloffFactor")]
    pub fn get_rolloff_factor(this: &PannerOptions) -> Option<f64>;
    #[doc = "Change the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    #[wasm_bindgen(method, setter = "rolloffFactor")]
    pub fn set_rolloff_factor(this: &PannerOptions, val: f64);
}
impl PannerOptions {
    #[doc = "Construct a new `PannerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
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
    #[deprecated = "Use `set_cone_inner_angle()` instead."]
    pub fn cone_inner_angle(&mut self, val: f64) -> &mut Self {
        self.set_cone_inner_angle(val);
        self
    }
    #[deprecated = "Use `set_cone_outer_angle()` instead."]
    pub fn cone_outer_angle(&mut self, val: f64) -> &mut Self {
        self.set_cone_outer_angle(val);
        self
    }
    #[deprecated = "Use `set_cone_outer_gain()` instead."]
    pub fn cone_outer_gain(&mut self, val: f64) -> &mut Self {
        self.set_cone_outer_gain(val);
        self
    }
    #[cfg(feature = "DistanceModelType")]
    #[deprecated = "Use `set_distance_model()` instead."]
    pub fn distance_model(&mut self, val: DistanceModelType) -> &mut Self {
        self.set_distance_model(val);
        self
    }
    #[deprecated = "Use `set_max_distance()` instead."]
    pub fn max_distance(&mut self, val: f64) -> &mut Self {
        self.set_max_distance(val);
        self
    }
    #[deprecated = "Use `set_orientation_x()` instead."]
    pub fn orientation_x(&mut self, val: f32) -> &mut Self {
        self.set_orientation_x(val);
        self
    }
    #[deprecated = "Use `set_orientation_y()` instead."]
    pub fn orientation_y(&mut self, val: f32) -> &mut Self {
        self.set_orientation_y(val);
        self
    }
    #[deprecated = "Use `set_orientation_z()` instead."]
    pub fn orientation_z(&mut self, val: f32) -> &mut Self {
        self.set_orientation_z(val);
        self
    }
    #[cfg(feature = "PanningModelType")]
    #[deprecated = "Use `set_panning_model()` instead."]
    pub fn panning_model(&mut self, val: PanningModelType) -> &mut Self {
        self.set_panning_model(val);
        self
    }
    #[deprecated = "Use `set_position_x()` instead."]
    pub fn position_x(&mut self, val: f32) -> &mut Self {
        self.set_position_x(val);
        self
    }
    #[deprecated = "Use `set_position_y()` instead."]
    pub fn position_y(&mut self, val: f32) -> &mut Self {
        self.set_position_y(val);
        self
    }
    #[deprecated = "Use `set_position_z()` instead."]
    pub fn position_z(&mut self, val: f32) -> &mut Self {
        self.set_position_z(val);
        self
    }
    #[deprecated = "Use `set_ref_distance()` instead."]
    pub fn ref_distance(&mut self, val: f64) -> &mut Self {
        self.set_ref_distance(val);
        self
    }
    #[deprecated = "Use `set_rolloff_factor()` instead."]
    pub fn rolloff_factor(&mut self, val: f64) -> &mut Self {
        self.set_rolloff_factor(val);
        self
    }
}
impl Default for PannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
