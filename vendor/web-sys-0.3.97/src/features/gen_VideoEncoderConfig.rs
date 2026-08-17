#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "VideoEncoderConfig")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoderConfig` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoEncoderConfig;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &VideoEncoderConfig) -> Option<AlphaOption>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "alpha")]
    pub fn set_alpha(this: &VideoEncoderConfig, val: AlphaOption);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bitrate")]
    pub fn get_bitrate(this: &VideoEncoderConfig) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate(this: &VideoEncoderConfig, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate_f64(this: &VideoEncoderConfig, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderBitrateMode")]
    #[doc = "Get the `bitrateMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderBitrateMode`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bitrateMode")]
    pub fn get_bitrate_mode(this: &VideoEncoderConfig) -> Option<VideoEncoderBitrateMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderBitrateMode")]
    #[doc = "Change the `bitrateMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderBitrateMode`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bitrateMode")]
    pub fn set_bitrate_mode(this: &VideoEncoderConfig, val: VideoEncoderBitrateMode);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "codec")]
    pub fn get_codec(this: &VideoEncoderConfig) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "codec")]
    pub fn set_codec(this: &VideoEncoderConfig, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `contentHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "contentHint")]
    pub fn get_content_hint(this: &VideoEncoderConfig) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `contentHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "contentHint")]
    pub fn set_content_hint(this: &VideoEncoderConfig, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "displayHeight")]
    pub fn get_display_height(this: &VideoEncoderConfig) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "displayHeight")]
    pub fn set_display_height(this: &VideoEncoderConfig, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "displayWidth")]
    pub fn get_display_width(this: &VideoEncoderConfig) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "displayWidth")]
    pub fn set_display_width(this: &VideoEncoderConfig, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "framerate")]
    pub fn get_framerate(this: &VideoEncoderConfig) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "framerate")]
    pub fn set_framerate(this: &VideoEncoderConfig, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    #[doc = "Get the `hardwareAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HardwareAcceleration`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "hardwareAcceleration")]
    pub fn get_hardware_acceleration(this: &VideoEncoderConfig) -> Option<HardwareAcceleration>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    #[doc = "Change the `hardwareAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HardwareAcceleration`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "hardwareAcceleration")]
    pub fn set_hardware_acceleration(this: &VideoEncoderConfig, val: HardwareAcceleration);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &VideoEncoderConfig) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &VideoEncoderConfig, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LatencyMode")]
    #[doc = "Get the `latencyMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LatencyMode`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "latencyMode")]
    pub fn get_latency_mode(this: &VideoEncoderConfig) -> Option<LatencyMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LatencyMode")]
    #[doc = "Change the `latencyMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LatencyMode`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "latencyMode")]
    pub fn set_latency_mode(this: &VideoEncoderConfig, val: LatencyMode);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "scalabilityMode")]
    pub fn get_scalability_mode(this: &VideoEncoderConfig) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "scalabilityMode")]
    pub fn set_scalability_mode(this: &VideoEncoderConfig, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &VideoEncoderConfig) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &VideoEncoderConfig, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderConfig {
    #[doc = "Construct a new `VideoEncoderConfig`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(codec: &str, height: u32, width: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_codec(codec);
        ret.set_height(height);
        ret.set_width(width);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    #[deprecated = "Use `set_alpha()` instead."]
    pub fn alpha(&mut self, val: AlphaOption) -> &mut Self {
        self.set_alpha(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_bitrate()` instead."]
    pub fn bitrate(&mut self, val: u32) -> &mut Self {
        self.set_bitrate(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderBitrateMode")]
    #[deprecated = "Use `set_bitrate_mode()` instead."]
    pub fn bitrate_mode(&mut self, val: VideoEncoderBitrateMode) -> &mut Self {
        self.set_bitrate_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_codec()` instead."]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_content_hint()` instead."]
    pub fn content_hint(&mut self, val: &str) -> &mut Self {
        self.set_content_hint(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_display_height()` instead."]
    pub fn display_height(&mut self, val: u32) -> &mut Self {
        self.set_display_height(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_display_width()` instead."]
    pub fn display_width(&mut self, val: u32) -> &mut Self {
        self.set_display_width(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_framerate()` instead."]
    pub fn framerate(&mut self, val: f64) -> &mut Self {
        self.set_framerate(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    #[deprecated = "Use `set_hardware_acceleration()` instead."]
    pub fn hardware_acceleration(&mut self, val: HardwareAcceleration) -> &mut Self {
        self.set_hardware_acceleration(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: u32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LatencyMode")]
    #[deprecated = "Use `set_latency_mode()` instead."]
    pub fn latency_mode(&mut self, val: LatencyMode) -> &mut Self {
        self.set_latency_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_scalability_mode()` instead."]
    pub fn scalability_mode(&mut self, val: &str) -> &mut Self {
        self.set_scalability_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: u32) -> &mut Self {
        self.set_width(val);
        self
    }
}
