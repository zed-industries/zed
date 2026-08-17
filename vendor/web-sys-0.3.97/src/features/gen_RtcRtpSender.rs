#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCRtpSender",
        typescript_type = "RTCRtpSender"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSender` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    pub type RtcRtpSender;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, getter, js_class = "RTCRtpSender", js_name = "track")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
    pub fn track(this: &RtcRtpSender) -> Option<MediaStreamTrack>;
    #[cfg(feature = "RtcdtmfSender")]
    #[wasm_bindgen(method, getter, js_class = "RTCRtpSender", js_name = "dtmf")]
    #[doc = "Getter for the `dtmf` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/dtmf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcdtmfSender`*"]
    pub fn dtmf(this: &RtcRtpSender) -> Option<RtcdtmfSender>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "RTCRtpSender", js_name = "transform")]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transform(this: &RtcRtpSender) -> Option<::js_sys::Object>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransform")]
    #[wasm_bindgen(method, setter, js_class = "RTCRtpSender", js_name = "transform")]
    #[doc = "Setter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_transform_opt_s_frame_transform(
        this: &RtcRtpSender,
        value: Option<&SFrameTransform>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcRtpScriptTransform")]
    #[wasm_bindgen(method, setter, js_class = "RTCRtpSender", js_name = "transform")]
    #[doc = "Setter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_transform_opt_rtc_rtp_script_transform(
        this: &RtcRtpSender,
        value: Option<&RtcRtpScriptTransform>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "RTCRtpSender", js_name = "generateKeyFrame")]
    #[doc = "The `generateKeyFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn generate_key_frame(this: &RtcRtpSender) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "RTCRtpSender", js_name = "generateKeyFrame")]
    #[doc = "The `generateKeyFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn generate_key_frame_with_rids(
        this: &RtcRtpSender,
        rids: &[::js_sys::JsString],
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(feature = "RtcRtpCapabilities")]
    #[wasm_bindgen(
        static_method_of = "RtcRtpSender",
        js_class = "RTCRtpSender",
        js_name = "getCapabilities"
    )]
    #[doc = "The `getCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getCapabilities_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`, `RtcRtpSender`*"]
    pub fn get_capabilities(kind: &str) -> Option<RtcRtpCapabilities>;
    #[cfg(feature = "RtcRtpParameters")]
    #[wasm_bindgen(method, js_class = "RTCRtpSender", js_name = "getParameters")]
    #[doc = "The `getParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
    pub fn get_parameters(this: &RtcRtpSender) -> RtcRtpParameters;
    #[wasm_bindgen(method, js_class = "RTCRtpSender", js_name = "getStats")]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    pub fn get_stats(this: &RtcRtpSender) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, js_class = "RTCRtpSender", js_name = "replaceTrack")]
    #[doc = "The `replaceTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/replaceTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
    pub fn replace_track(
        this: &RtcRtpSender,
        with_track: Option<&MediaStreamTrack>,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "RTCRtpSender", js_name = "setParameters")]
    #[doc = "The `setParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    pub fn set_parameters(this: &RtcRtpSender) -> ::js_sys::Promise;
    #[cfg(feature = "RtcRtpParameters")]
    #[wasm_bindgen(method, js_class = "RTCRtpSender", js_name = "setParameters")]
    #[doc = "The `setParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
    pub fn set_parameters_with_parameters(
        this: &RtcRtpSender,
        parameters: &RtcRtpParameters,
    ) -> ::js_sys::Promise;
}
