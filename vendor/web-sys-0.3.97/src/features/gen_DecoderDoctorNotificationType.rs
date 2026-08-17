#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `DecoderDoctorNotificationType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotificationType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderDoctorNotificationType {
    CannotPlay = "cannot-play",
    PlatformDecoderNotFound = "platform-decoder-not-found",
    CanPlayButSomeMissingDecoders = "can-play-but-some-missing-decoders",
    CannotInitializePulseaudio = "cannot-initialize-pulseaudio",
    UnsupportedLibavcodec = "unsupported-libavcodec",
    DecodeError = "decode-error",
    DecodeWarning = "decode-warning",
}
