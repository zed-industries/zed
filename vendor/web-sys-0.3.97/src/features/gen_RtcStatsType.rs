#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcStatsType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcStatsType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcStatsType {
    InboundRtp = "inbound-rtp",
    OutboundRtp = "outbound-rtp",
    Csrc = "csrc",
    Session = "session",
    Track = "track",
    Transport = "transport",
    CandidatePair = "candidate-pair",
    LocalCandidate = "local-candidate",
    RemoteCandidate = "remote-candidate",
}
