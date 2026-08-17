#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCEncodedAudioFrameOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedAudioFrameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RtcEncodedAudioFrameOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[doc = "Get the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameMetadata`, `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "metadata")]
    pub fn get_metadata(this: &RtcEncodedAudioFrameOptions)
        -> Option<RtcEncodedAudioFrameMetadata>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[doc = "Change the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameMetadata`, `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "metadata")]
    pub fn set_metadata(this: &RtcEncodedAudioFrameOptions, val: &RtcEncodedAudioFrameMetadata);
}
#[cfg(web_sys_unstable_apis)]
impl RtcEncodedAudioFrameOptions {
    #[doc = "Construct a new `RtcEncodedAudioFrameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[deprecated = "Use `set_metadata()` instead."]
    pub fn metadata(&mut self, val: &RtcEncodedAudioFrameMetadata) -> &mut Self {
        self.set_metadata(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for RtcEncodedAudioFrameOptions {
    fn default() -> Self {
        Self::new()
    }
}
