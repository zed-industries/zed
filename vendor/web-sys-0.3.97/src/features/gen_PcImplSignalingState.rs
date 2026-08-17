#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PcImplSignalingState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PcImplSignalingState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcImplSignalingState {
    SignalingInvalid = "SignalingInvalid",
    SignalingStable = "SignalingStable",
    SignalingHaveLocalOffer = "SignalingHaveLocalOffer",
    SignalingHaveRemoteOffer = "SignalingHaveRemoteOffer",
    SignalingHaveLocalPranswer = "SignalingHaveLocalPranswer",
    SignalingHaveRemotePranswer = "SignalingHaveRemotePranswer",
    SignalingClosed = "SignalingClosed",
}
