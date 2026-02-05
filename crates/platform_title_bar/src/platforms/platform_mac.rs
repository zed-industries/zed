// Use pixels here instead of a rem-based size because the macOS traffic
// lights are a static size, and don't scale with the rest of the UI.
//
// Magic number: There is one extra pixel of padding on the left side due to
// the 1px border around the window on macOS apps.
#[cfg(macos_sdk_26)]
pub const TRAFFIC_LIGHT_PADDING: f32 = 78.;

#[cfg(not(macos_sdk_26))]
pub const TRAFFIC_LIGHT_PADDING: f32 = 71.;
