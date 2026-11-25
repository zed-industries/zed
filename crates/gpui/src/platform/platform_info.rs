//! Runtime identity of the host operating system and application target.
//!
//! Host conventions are independent of the backend delivering the application:
//! a browser on macOS follows Mac keyboard conventions without native Mac APIs.
//! Backends provide this identity; consumers read it through the application.

/// The host operating system and the kind of application delivering the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlatformInfo {
    /// The operating system whose input conventions the application follows.
    pub os: OperatingSystem,
    /// How the application is delivered, independently of its host OS.
    pub target: PlatformTarget,
}

/// The host operating system, including when the application runs in a browser.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperatingSystem {
    /// Apple's desktop operating system.
    Mac,
    /// Microsoft Windows.
    Windows,
    /// Linux, including browser hosts running ChromeOS.
    Linux,
    /// FreeBSD.
    FreeBsd,
    /// iOS and iPadOS.
    Ios,
    /// Android.
    Android,
    /// The backend could not identify the host operating system.
    Unknown,
}

/// The kind of application delivering the UI, independently of its host OS.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlatformTarget {
    /// A native application, including headless execution.
    Native,
    /// An application running in a browser.
    Web,
}

impl PlatformInfo {
    /// Whether the host operating system is macOS.
    pub fn is_mac(self) -> bool {
        self.os == OperatingSystem::Mac
    }
}
