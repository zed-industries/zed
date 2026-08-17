#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCEncodedAudioFrame",
        typescript_type = "RTCEncodedAudioFrame"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedAudioFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RtcEncodedAudioFrame;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "RTCEncodedAudioFrame", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data(this: &RtcEncodedAudioFrame) -> ::js_sys::ArrayBuffer;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "RTCEncodedAudioFrame", js_name = "data")]
    #[doc = "Setter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_data(this: &RtcEncodedAudioFrame, value: &::js_sys::ArrayBuffer);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "RTCEncodedAudioFrame")]
    #[doc = "The `new RtcEncodedAudioFrame(..)` constructor, creating a new instance of `RtcEncodedAudioFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/RTCEncodedAudioFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(original_frame: &RtcEncodedAudioFrame) -> Result<RtcEncodedAudioFrame, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCEncodedAudioFrame")]
    #[doc = "The `new RtcEncodedAudioFrame(..)` constructor, creating a new instance of `RtcEncodedAudioFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/RTCEncodedAudioFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`, `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_options(
        original_frame: &RtcEncodedAudioFrame,
        options: &RtcEncodedAudioFrameOptions,
    ) -> Result<RtcEncodedAudioFrame, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[wasm_bindgen(method, js_class = "RTCEncodedAudioFrame", js_name = "getMetadata")]
    #[doc = "The `getMetadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/getMetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`, `RtcEncodedAudioFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_metadata(this: &RtcEncodedAudioFrame) -> RtcEncodedAudioFrameMetadata;
}
