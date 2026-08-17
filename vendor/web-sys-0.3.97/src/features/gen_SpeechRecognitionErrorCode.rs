#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `SpeechRecognitionErrorCode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechRecognitionErrorCode {
    NoSpeech = "no-speech",
    Aborted = "aborted",
    AudioCapture = "audio-capture",
    Network = "network",
    NotAllowed = "not-allowed",
    ServiceNotAllowed = "service-not-allowed",
    BadGrammar = "bad-grammar",
    LanguageNotSupported = "language-not-supported",
}
