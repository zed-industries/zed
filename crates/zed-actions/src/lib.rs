use std::sync::Arc;

use gpui::{actions, impl_actions};
use serde::Deserialize;

actions!(
    zed,
    [
        About,
        Hide,
        HideOthers,
        ShowAll,
        Minimize,
        Zoom,
        ToggleFullScreen,
        Quit,
        DebugElements,
        OpenLog,
        OpenLicenses,
        OpenTelemetryLog,
        OpenKeymap,
        OpenSettings,
        OpenLocalSettings,
        OpenDefaultSettings,
        OpenDefaultKeymap,
        IncreaseBufferFontSize,
        DecreaseBufferFontSize,
        ResetBufferFontSize,
        ResetDatabase,
    ]
);

#[derive(Deserialize, Clone, PartialEq)]
pub struct OpenBrowser {
    pub url: Arc<str>,
}
#[derive(Deserialize, Clone, PartialEq)]
pub struct OpenZedURL {
    pub url: String,
}
impl_actions!(zed, [OpenBrowser, OpenZedURL]);
