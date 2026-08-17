#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `SpeechSynthesisErrorCode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechSynthesisErrorCode {
    Canceled = "canceled",
    Interrupted = "interrupted",
    AudioBusy = "audio-busy",
    AudioHardware = "audio-hardware",
    Network = "network",
    SynthesisUnavailable = "synthesis-unavailable",
    SynthesisFailed = "synthesis-failed",
    LanguageUnavailable = "language-unavailable",
    VoiceUnavailable = "voice-unavailable",
    TextTooLong = "text-too-long",
    InvalidArgument = "invalid-argument",
}
