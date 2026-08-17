#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtpEncodingParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpEncodingParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub type RtcRtpEncodingParameters;
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &RtcRtpEncodingParameters) -> Option<bool>;
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &RtcRtpEncodingParameters, val: bool);
    #[cfg(feature = "RtcDegradationPreference")]
    #[doc = "Get the `degradationPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "degradationPreference")]
    pub fn get_degradation_preference(
        this: &RtcRtpEncodingParameters,
    ) -> Option<RtcDegradationPreference>;
    #[cfg(feature = "RtcDegradationPreference")]
    #[doc = "Change the `degradationPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "degradationPreference")]
    pub fn set_degradation_preference(
        this: &RtcRtpEncodingParameters,
        val: RtcDegradationPreference,
    );
    #[cfg(feature = "RtcFecParameters")]
    #[doc = "Get the `fec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "fec")]
    pub fn get_fec(this: &RtcRtpEncodingParameters) -> Option<RtcFecParameters>;
    #[cfg(feature = "RtcFecParameters")]
    #[doc = "Change the `fec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "fec")]
    pub fn set_fec(this: &RtcRtpEncodingParameters, val: &RtcFecParameters);
    #[doc = "Get the `maxBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "maxBitrate")]
    pub fn get_max_bitrate(this: &RtcRtpEncodingParameters) -> Option<u32>;
    #[doc = "Change the `maxBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "maxBitrate")]
    pub fn set_max_bitrate(this: &RtcRtpEncodingParameters, val: u32);
    #[cfg(feature = "RtcPriorityType")]
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "priority")]
    pub fn get_priority(this: &RtcRtpEncodingParameters) -> Option<RtcPriorityType>;
    #[cfg(feature = "RtcPriorityType")]
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority(this: &RtcRtpEncodingParameters, val: RtcPriorityType);
    #[doc = "Get the `rid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "rid")]
    pub fn get_rid(this: &RtcRtpEncodingParameters) -> Option<::alloc::string::String>;
    #[doc = "Change the `rid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "rid")]
    pub fn set_rid(this: &RtcRtpEncodingParameters, val: &str);
    #[cfg(feature = "RtcRtxParameters")]
    #[doc = "Get the `rtx` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*"]
    #[wasm_bindgen(method, getter = "rtx")]
    pub fn get_rtx(this: &RtcRtpEncodingParameters) -> Option<RtcRtxParameters>;
    #[cfg(feature = "RtcRtxParameters")]
    #[doc = "Change the `rtx` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*"]
    #[wasm_bindgen(method, setter = "rtx")]
    pub fn set_rtx(this: &RtcRtpEncodingParameters, val: &RtcRtxParameters);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "scalabilityMode")]
    pub fn get_scalability_mode(this: &RtcRtpEncodingParameters)
        -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "scalabilityMode")]
    pub fn set_scalability_mode(this: &RtcRtpEncodingParameters, val: &str);
    #[doc = "Get the `scaleResolutionDownBy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "scaleResolutionDownBy")]
    pub fn get_scale_resolution_down_by(this: &RtcRtpEncodingParameters) -> Option<f32>;
    #[doc = "Change the `scaleResolutionDownBy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "scaleResolutionDownBy")]
    pub fn set_scale_resolution_down_by(this: &RtcRtpEncodingParameters, val: f32);
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, getter = "ssrc")]
    pub fn get_ssrc(this: &RtcRtpEncodingParameters) -> Option<u32>;
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[wasm_bindgen(method, setter = "ssrc")]
    pub fn set_ssrc(this: &RtcRtpEncodingParameters, val: u32);
}
impl RtcRtpEncodingParameters {
    #[doc = "Construct a new `RtcRtpEncodingParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[cfg(feature = "RtcDegradationPreference")]
    #[deprecated = "Use `set_degradation_preference()` instead."]
    pub fn degradation_preference(&mut self, val: RtcDegradationPreference) -> &mut Self {
        self.set_degradation_preference(val);
        self
    }
    #[cfg(feature = "RtcFecParameters")]
    #[deprecated = "Use `set_fec()` instead."]
    pub fn fec(&mut self, val: &RtcFecParameters) -> &mut Self {
        self.set_fec(val);
        self
    }
    #[deprecated = "Use `set_max_bitrate()` instead."]
    pub fn max_bitrate(&mut self, val: u32) -> &mut Self {
        self.set_max_bitrate(val);
        self
    }
    #[cfg(feature = "RtcPriorityType")]
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: RtcPriorityType) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_rid()` instead."]
    pub fn rid(&mut self, val: &str) -> &mut Self {
        self.set_rid(val);
        self
    }
    #[cfg(feature = "RtcRtxParameters")]
    #[deprecated = "Use `set_rtx()` instead."]
    pub fn rtx(&mut self, val: &RtcRtxParameters) -> &mut Self {
        self.set_rtx(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_scalability_mode()` instead."]
    pub fn scalability_mode(&mut self, val: &str) -> &mut Self {
        self.set_scalability_mode(val);
        self
    }
    #[deprecated = "Use `set_scale_resolution_down_by()` instead."]
    pub fn scale_resolution_down_by(&mut self, val: f32) -> &mut Self {
        self.set_scale_resolution_down_by(val);
        self
    }
    #[deprecated = "Use `set_ssrc()` instead."]
    pub fn ssrc(&mut self, val: u32) -> &mut Self {
        self.set_ssrc(val);
        self
    }
}
impl Default for RtcRtpEncodingParameters {
    fn default() -> Self {
        Self::new()
    }
}
