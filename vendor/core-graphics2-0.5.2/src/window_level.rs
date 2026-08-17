#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGWindowLevelKey {
    #[doc(alias = "kCGBaseWindowLevelKey")]
    Base              = 0,
    #[doc(alias = "kCGMinimumWindowLevelKey")]
    Minimum           = 1,
    #[doc(alias = "kCGDesktopWindowLevelKey")]
    Desktop           = 2,
    #[doc(alias = "kCGBackstopMenuLevelKey")]
    BackstopMenu      = 3,
    #[doc(alias = "kCGNormalWindowLevelKey")]
    Normal            = 4,
    #[doc(alias = "kCGFloatingWindowLevelKey")]
    Floating          = 5,
    #[doc(alias = "kCGTornOffMenuWindowLevelKey")]
    TornOffMenu       = 6,
    #[doc(alias = "kCGDockWindowLevelKey")]
    Dock              = 7,
    #[doc(alias = "kCGMainMenuWindowLevelKey")]
    MainMenu          = 8,
    #[doc(alias = "kCGStatusWindowLevelKey")]
    Status            = 9,
    #[doc(alias = "kCGModalPanelWindowLevelKey")]
    ModalPanel        = 10,
    #[doc(alias = "kCGPopUpMenuWindowLevelKey")]
    PopUpMenu         = 11,
    #[doc(alias = "kCGDraggingWindowLevelKey")]
    Dragging          = 12,
    #[doc(alias = "kCGScreenSaverWindowLevelKey")]
    ScreenSaver       = 13,
    #[doc(alias = "kCGMaximumWindowLevelKey")]
    Maximum           = 14,
    #[doc(alias = "kCGOverlayWindowLevelKey")]
    Overlay           = 15,
    #[doc(alias = "kCGHelpWindowLevelKey")]
    Help              = 16,
    #[doc(alias = "kCGUtilityWindowLevelKey")]
    Utility           = 17,
    #[doc(alias = "kCGDesktopIconWindowLevelKey")]
    DesktopIcon       = 18,
    #[doc(alias = "kCGCursorWindowLevelKey")]
    Cursor            = 19,
    #[doc(alias = "kCGAssistiveTechHighWindowLevelKey")]
    AssistiveTechHigh = 20,
    #[doc(alias = "kCGNumberOfWindowLevelKeys")]
    NumberOfKeys      = 21,
}

pub type CGWindowLevel = i32;

extern "C" {
    pub fn CGWindowLevelForKey(key: CGWindowLevelKey) -> CGWindowLevel;
}

pub const kCGNumReservedWindowLevels: i32 = 16;
pub const kCGNumReservedBaseWindowLevels: i32 = 5;

pub const kCGBaseWindowLevel: CGWindowLevel = i32::MIN;
pub const kCGMinimumWindowLevel: CGWindowLevel = kCGBaseWindowLevel + kCGNumReservedBaseWindowLevels;
pub const kCGMaximumWindowLevel: CGWindowLevel = i32::MAX - kCGNumReservedWindowLevels;

pub const kCGDesktopWindowLevel: CGWindowLevel = kCGMinimumWindowLevel + 20;
pub const kCGDesktopIconWindowLevel: CGWindowLevel = kCGDesktopWindowLevel + 1;
pub const kCGBackstopMenuLevel: CGWindowLevel = -20;
pub const kCGNormalWindowLevel: CGWindowLevel = 0;
pub const kCGFloatingWindowLevel: CGWindowLevel = 3;
pub const kCGTornOffMenuWindowLevel: CGWindowLevel = 3;
pub const kCGModalPanelWindowLevel: CGWindowLevel = 8;
pub const kCGUtilityWindowLevel: CGWindowLevel = 19;
pub const kCGDockWindowLevel: CGWindowLevel = 20;
pub const kCGMainMenuWindowLevel: CGWindowLevel = 24;
pub const kCGStatusWindowLevel: CGWindowLevel = 25;
pub const kCGPopUpMenuWindowLevel: CGWindowLevel = 101;
pub const kCGOverlayWindowLevel: CGWindowLevel = 102;
pub const kCGHelpWindowLevel: CGWindowLevel = 200;
pub const kCGDraggingWindowLevel: CGWindowLevel = 500;
pub const kCGScreenSaverWindowLevel: CGWindowLevel = 1000;
pub const kCGAssistiveTechHighWindowLevel: CGWindowLevel = 1500;
pub const kCGCursorWindowLevel: CGWindowLevel = kCGMaximumWindowLevel - 1;

pub fn window_level_for_key(key: CGWindowLevelKey) -> CGWindowLevel {
    unsafe { CGWindowLevelForKey(key) }
}
