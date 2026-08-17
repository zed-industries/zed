#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "RTCPeerConnectionIceEvent",
        typescript_type = "RTCPeerConnectionIceEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnectionIceEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`*"]
    pub type RtcPeerConnectionIceEvent;
    #[cfg(feature = "RtcIceCandidate")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnectionIceEvent",
        js_name = "candidate"
    )]
    #[doc = "Getter for the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/candidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEvent`*"]
    pub fn candidate(this: &RtcPeerConnectionIceEvent) -> Option<RtcIceCandidate>;
    #[wasm_bindgen(catch, constructor, js_class = "RTCPeerConnectionIceEvent")]
    #[doc = "The `new RtcPeerConnectionIceEvent(..)` constructor, creating a new instance of `RtcPeerConnectionIceEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/RTCPeerConnectionIceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`*"]
    pub fn new(type_: &str) -> Result<RtcPeerConnectionIceEvent, JsValue>;
    #[cfg(feature = "RtcPeerConnectionIceEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCPeerConnectionIceEvent")]
    #[doc = "The `new RtcPeerConnectionIceEvent(..)` constructor, creating a new instance of `RtcPeerConnectionIceEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/RTCPeerConnectionIceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`, `RtcPeerConnectionIceEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &RtcPeerConnectionIceEventInit,
    ) -> Result<RtcPeerConnectionIceEvent, JsValue>;
}
