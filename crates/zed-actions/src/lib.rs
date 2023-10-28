use std::sync::Arc;

use gpui::{actions, impl_actions};
use serde::Deserialize;

actions!(
    zed,
    [
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
