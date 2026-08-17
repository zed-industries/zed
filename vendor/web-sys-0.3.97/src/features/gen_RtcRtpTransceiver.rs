#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCRtpTransceiver",
        typescript_type = "RTCRtpTransceiver"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpTransceiver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub type RtcRtpTransceiver;
    #[wasm_bindgen(method, getter, js_class = "RTCRtpTransceiver", js_name = "mid")]
    #[doc = "Getter for the `mid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/mid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn mid(this: &RtcRtpTransceiver) -> Option<::alloc::string::String>;
    #[cfg(feature = "RtcRtpSender")]
    #[wasm_bindgen(method, getter, js_class = "RTCRtpTransceiver", js_name = "sender")]
    #[doc = "Getter for the `sender` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/sender)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcRtpTransceiver`*"]
    pub fn sender(this: &RtcRtpTransceiver) -> RtcRtpSender;
    #[cfg(feature = "RtcRtpReceiver")]
    #[wasm_bindgen(method, getter, js_class = "RTCRtpTransceiver", js_name = "receiver")]
    #[doc = "Getter for the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/receiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcRtpTransceiver`*"]
    pub fn receiver(this: &RtcRtpTransceiver) -> RtcRtpReceiver;
    #[wasm_bindgen(method, getter, js_class = "RTCRtpTransceiver", js_name = "stopped")]
    #[doc = "Getter for the `stopped` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stopped)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn stopped(this: &RtcRtpTransceiver) -> bool;
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[wasm_bindgen(method, getter, js_class = "RTCRtpTransceiver", js_name = "direction")]
    #[doc = "Getter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
    pub fn direction(this: &RtcRtpTransceiver) -> RtcRtpTransceiverDirection;
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[wasm_bindgen(method, setter, js_class = "RTCRtpTransceiver", js_name = "direction")]
    #[doc = "Setter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
    pub fn set_direction(this: &RtcRtpTransceiver, value: RtcRtpTransceiverDirection);
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCRtpTransceiver",
        js_name = "currentDirection"
    )]
    #[doc = "Getter for the `currentDirection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/currentDirection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
    pub fn current_direction(this: &RtcRtpTransceiver) -> Option<RtcRtpTransceiverDirection>;
    #[wasm_bindgen(method, js_class = "RTCRtpTransceiver", js_name = "getRemoteTrackId")]
    #[doc = "The `getRemoteTrackId()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/getRemoteTrackId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn get_remote_track_id(this: &RtcRtpTransceiver) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        js_class = "RTCRtpTransceiver",
        js_name = "setCodecPreferences"
    )]
    #[doc = "The `setCodecPreferences()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/setCodecPreferences)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn set_codec_preferences(this: &RtcRtpTransceiver, codecs: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, js_class = "RTCRtpTransceiver")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn stop(this: &RtcRtpTransceiver);
}
