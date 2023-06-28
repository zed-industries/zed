use gpui::actions;

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
