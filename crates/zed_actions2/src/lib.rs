use gpui::{action, actions};

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
