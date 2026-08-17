#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "RTCTrackEvent",
        typescript_type = "RTCTrackEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTrackEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
    pub type RtcTrackEvent;
    #[cfg(feature = "RtcRtpReceiver")]
    #[wasm_bindgen(method, getter, js_class = "RTCTrackEvent", js_name = "receiver")]
    #[doc = "Getter for the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/receiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEvent`*"]
    pub fn receiver(this: &RtcTrackEvent) -> RtcRtpReceiver;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, getter, js_class = "RTCTrackEvent", js_name = "track")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEvent`*"]
    pub fn track(this: &RtcTrackEvent) -> MediaStreamTrack;
    #[wasm_bindgen(method, getter, js_class = "RTCTrackEvent", js_name = "streams")]
    #[doc = "Getter for the `streams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/streams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
    pub fn streams(this: &RtcTrackEvent) -> ::js_sys::Array;
    #[cfg(feature = "RtcRtpTransceiver")]
    #[wasm_bindgen(method, getter, js_class = "RTCTrackEvent", js_name = "transceiver")]
    #[doc = "Getter for the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/transceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEvent`*"]
    pub fn transceiver(this: &RtcTrackEvent) -> RtcRtpTransceiver;
    #[cfg(feature = "RtcTrackEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCTrackEvent")]
    #[doc = "The `new RtcTrackEvent(..)` constructor, creating a new instance of `RtcTrackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/RTCTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEvent`, `RtcTrackEventInit`*"]
    pub fn new(type_: &str, event_init_dict: &RtcTrackEventInit) -> Result<RtcTrackEvent, JsValue>;
}
