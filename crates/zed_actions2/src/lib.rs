use gpui::{action, actions};

// If the zed binary doesn't use anything in this crate, it will be optimized away
// and the actions won't initialize. So we just provide an empty initialization function
// to be called from main.
//
// These may provide relevant context:
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

actions!(
    About,
    DebugElements,
    DecreaseBufferFontSize,
    Hide,
    HideOthers,
    IncreaseBufferFontSize,
    Minimize,
    OpenDefaultKeymap,
    OpenDefaultSettings,
    OpenKeymap,
    OpenLicenses,
    OpenLocalSettings,
    OpenLog,
    OpenSettings,
    OpenTelemetryLog,
    Quit,
    ResetBufferFontSize,
    ResetDatabase,
    ShowAll,
    ToggleFullScreen,
    Zoom,
);

#[action]
pub struct OpenBrowser {
    pub url: String,
}
#[action]
pub struct OpenZedURL {
    pub url: String,
}
