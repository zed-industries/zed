#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCIceCandidate",
        typescript_type = "RTCIceCandidate"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidate` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub type RtcIceCandidate;
    #[wasm_bindgen(method, getter, js_class = "RTCIceCandidate", js_name = "candidate")]
    #[doc = "Getter for the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn candidate(this: &RtcIceCandidate) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "RTCIceCandidate", js_name = "candidate")]
    #[doc = "Setter for the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn set_candidate(this: &RtcIceCandidate, value: &str);
    #[wasm_bindgen(method, getter, js_class = "RTCIceCandidate", js_name = "sdpMid")]
    #[doc = "Getter for the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn sdp_mid(this: &RtcIceCandidate) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, setter, js_class = "RTCIceCandidate", js_name = "sdpMid")]
    #[doc = "Setter for the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn set_sdp_mid(this: &RtcIceCandidate, value: Option<&str>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCIceCandidate",
        js_name = "sdpMLineIndex"
    )]
    #[doc = "Getter for the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn sdp_m_line_index(this: &RtcIceCandidate) -> Option<u16>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCIceCandidate",
        js_name = "sdpMLineIndex"
    )]
    #[doc = "Setter for the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn set_sdp_m_line_index(this: &RtcIceCandidate, value: Option<u16>);
    #[cfg(feature = "RtcIceCandidateInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCIceCandidate")]
    #[doc = "The `new RtcIceCandidate(..)` constructor, creating a new instance of `RtcIceCandidate`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/RTCIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcIceCandidateInit`*"]
    pub fn new(candidate_init_dict: &RtcIceCandidateInit) -> Result<RtcIceCandidate, JsValue>;
    #[wasm_bindgen(method, js_class = "RTCIceCandidate", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn to_json(this: &RtcIceCandidate) -> ::js_sys::Object;
}
