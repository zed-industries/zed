#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcStatsIceCandidateType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcStatsIceCandidateType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcStatsIceCandidateType {
    Host = "host",
    Serverreflexive = "serverreflexive",
    Peerreflexive = "peerreflexive",
    Relayed = "relayed",
}
