#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "RTCPeerConnectionIceErrorEvent",
        typescript_type = "RTCPeerConnectionIceErrorEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnectionIceErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
    pub type RtcPeerConnectionIceErrorEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnectionIceErrorEvent",
        js_name = "address"
    )]
    #[doc = "Getter for the `address` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/address)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
    pub fn address(this: &RtcPeerConnectionIceErrorEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnectionIceErrorEvent",
        js_name = "port"
    )]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
    pub fn port(this: &RtcPeerConnectionIceErrorEvent) -> Option<u16>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnectionIceErrorEvent",
        js_name = "url"
    )]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
    pub fn url(this: &RtcPeerConnectionIceErrorEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnectionIceErrorEvent",
        js_name = "errorCode"
    )]
    #[doc = "Getter for the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/errorCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
    pub fn error_code(this: &RtcPeerConnectionIceErrorEvent) -> u16;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCPeerConnectionIceErrorEvent",
        js_name = "errorText"
    )]
    #[doc = "Getter for the `errorText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/errorText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
    pub fn error_text(this: &RtcPeerConnectionIceErrorEvent) -> ::alloc::string::String;
}
