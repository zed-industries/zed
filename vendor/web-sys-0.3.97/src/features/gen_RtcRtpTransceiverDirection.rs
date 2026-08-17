#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcRtpTransceiverDirection` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcRtpTransceiverDirection {
    Sendrecv = "sendrecv",
    Sendonly = "sendonly",
    Recvonly = "recvonly",
    Inactive = "inactive",
    Stopped = "stopped",
}
