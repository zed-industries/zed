#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCEncodedVideoFrameOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedVideoFrameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RtcEncodedVideoFrameOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameMetadata")]
    #[doc = "Get the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`, `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "metadata")]
    pub fn get_metadata(this: &RtcEncodedVideoFrameOptions)
        -> Option<RtcEncodedVideoFrameMetadata>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameMetadata")]
    #[doc = "Change the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`, `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "metadata")]
    pub fn set_metadata(this: &RtcEncodedVideoFrameOptions, val: &RtcEncodedVideoFrameMetadata);
}
#[cfg(web_sys_unstable_apis)]
impl RtcEncodedVideoFrameOptions {
    #[doc = "Construct a new `RtcEncodedVideoFrameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameMetadata")]
    #[deprecated = "Use `set_metadata()` instead."]
    pub fn metadata(&mut self, val: &RtcEncodedVideoFrameMetadata) -> &mut Self {
        self.set_metadata(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for RtcEncodedVideoFrameOptions {
    fn default() -> Self {
        Self::new()
    }
}
