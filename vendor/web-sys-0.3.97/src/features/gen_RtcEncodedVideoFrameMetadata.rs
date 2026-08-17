#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCEncodedVideoFrameMetadata")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedVideoFrameMetadata` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RtcEncodedVideoFrameMetadata;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `contributingSources` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "contributingSources")]
    pub fn get_contributing_sources(
        this: &RtcEncodedVideoFrameMetadata,
    ) -> Option<::js_sys::Array<::js_sys::Number>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `contributingSources` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "contributingSources")]
    pub fn set_contributing_sources(this: &RtcEncodedVideoFrameMetadata, val: &[::js_sys::Number]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dependencies` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "dependencies")]
    pub fn get_dependencies(
        this: &RtcEncodedVideoFrameMetadata,
    ) -> Option<::js_sys::Array<::js_sys::Number>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dependencies` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dependencies")]
    pub fn set_dependencies(this: &RtcEncodedVideoFrameMetadata, val: &[::js_sys::Number]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dependencies` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dependencies")]
    pub fn set_dependencies_f64_sequence(
        this: &RtcEncodedVideoFrameMetadata,
        val: &[::js_sys::Number],
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `frameId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &RtcEncodedVideoFrameMetadata) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `frameId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &RtcEncodedVideoFrameMetadata, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `frameId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id_f64(this: &RtcEncodedVideoFrameMetadata, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &RtcEncodedVideoFrameMetadata) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &RtcEncodedVideoFrameMetadata, val: u16);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &RtcEncodedVideoFrameMetadata) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &RtcEncodedVideoFrameMetadata, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "payloadType")]
    pub fn get_payload_type(this: &RtcEncodedVideoFrameMetadata) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "payloadType")]
    pub fn set_payload_type(this: &RtcEncodedVideoFrameMetadata, val: u8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `rtpTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "rtpTimestamp")]
    pub fn get_rtp_timestamp(this: &RtcEncodedVideoFrameMetadata) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `rtpTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "rtpTimestamp")]
    pub fn set_rtp_timestamp(this: &RtcEncodedVideoFrameMetadata, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `spatialIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "spatialIndex")]
    pub fn get_spatial_index(this: &RtcEncodedVideoFrameMetadata) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `spatialIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "spatialIndex")]
    pub fn set_spatial_index(this: &RtcEncodedVideoFrameMetadata, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `synchronizationSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "synchronizationSource")]
    pub fn get_synchronization_source(this: &RtcEncodedVideoFrameMetadata) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `synchronizationSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "synchronizationSource")]
    pub fn set_synchronization_source(this: &RtcEncodedVideoFrameMetadata, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `temporalIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "temporalIndex")]
    pub fn get_temporal_index(this: &RtcEncodedVideoFrameMetadata) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `temporalIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "temporalIndex")]
    pub fn set_temporal_index(this: &RtcEncodedVideoFrameMetadata, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcEncodedVideoFrameMetadata) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &RtcEncodedVideoFrameMetadata, val: i32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp_f64(this: &RtcEncodedVideoFrameMetadata, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &RtcEncodedVideoFrameMetadata) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &RtcEncodedVideoFrameMetadata, val: u16);
}
#[cfg(web_sys_unstable_apis)]
impl RtcEncodedVideoFrameMetadata {
    #[doc = "Construct a new `RtcEncodedVideoFrameMetadata`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_contributing_sources()` instead."]
    pub fn contributing_sources(&mut self, val: &[::js_sys::Number]) -> &mut Self {
        self.set_contributing_sources(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_dependencies()` instead."]
    pub fn dependencies(&mut self, val: &[::js_sys::Number]) -> &mut Self {
        self.set_dependencies(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: u32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: u16) -> &mut Self {
        self.set_height(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_payload_type()` instead."]
    pub fn payload_type(&mut self, val: u8) -> &mut Self {
        self.set_payload_type(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_rtp_timestamp()` instead."]
    pub fn rtp_timestamp(&mut self, val: u32) -> &mut Self {
        self.set_rtp_timestamp(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_spatial_index()` instead."]
    pub fn spatial_index(&mut self, val: u32) -> &mut Self {
        self.set_spatial_index(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_synchronization_source()` instead."]
    pub fn synchronization_source(&mut self, val: u32) -> &mut Self {
        self.set_synchronization_source(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_temporal_index()` instead."]
    pub fn temporal_index(&mut self, val: u32) -> &mut Self {
        self.set_temporal_index(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: i32) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: u16) -> &mut Self {
        self.set_width(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for RtcEncodedVideoFrameMetadata {
    fn default() -> Self {
        Self::new()
    }
}
