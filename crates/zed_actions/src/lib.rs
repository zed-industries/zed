use gpui::{actions, impl_actions};
use serde::Deserialize;

// If the zed binary doesn't use anything in this crate, it will be optimized away
// and the actions won't initialize. So we just provide an empty initialization function
// to be called from main.
//
// These may provide relevant context:
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

#[derive(Clone, PartialEq, Deserialize)]
pub struct OpenBrowser {
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct OpenZedUrl {
    pub url: String,
}

impl_actions!(zed, [OpenBrowser, OpenZedUrl]);

actions!(
    zed,
    [
        OpenSettings,
        Quit,
        OpenKeymap,
        About,
        OpenLicenses,
        OpenTelemetryLog,
        DecreaseBufferFontSize,
        IncreaseBufferFontSize,
        ResetBufferFontSize,
        DecreaseUiFontSize,
        IncreaseUiFontSize,
        ResetUiFontSize
    ]
);
