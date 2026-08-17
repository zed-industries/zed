use std::fmt::{self, Display, Formatter};

// FIXME: V2: Move `Unknown` variants to the top of the enum.

/// The desktop environment of a system
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum DesktopEnv {
    /// Popular GTK-based desktop environment on Linux
    Gnome,
    /// One of the desktop environments for a specific version of Windows
    Windows,
    /// Linux desktop environment optimized for low resource requirements
    Lxde,
    /// Stacking window manager for X Windows on Linux
    Openbox,
    /// Desktop environment for Linux, BSD and illumos
    Mate,
    /// Lightweight desktop enivornment for unix-like operating systems
    Xfce,
    /// KDE Plasma desktop enviroment
    // FIXME: Rename to 'Plasma' in whoami 2.0.0
    Kde,
    /// Default desktop environment on Linux Mint
    Cinnamon,
    /// Tiling window manager for Linux
    I3,
    /// Desktop environment for MacOS
    Aqua,
    /// Desktop environment for iOS
    Ios,
    /// Desktop environment for Android
    Android,
    /// Running as Web Assembly on a web page
    WebBrowser,
    /// A desktop environment for a video game console
    Console,
    /// Ubuntu-branded GNOME
    Ubuntu,
    /// Default shell for Fuchsia
    Ermine,
    /// Default desktop environment for Redox
    Orbital,
    /// Unknown desktop environment
    Unknown(String),
}

impl Display for DesktopEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::Gnome => "Gnome",
            Self::Windows => "Windows",
            Self::Lxde => "LXDE",
            Self::Openbox => "Openbox",
            Self::Mate => "Mate",
            Self::Xfce => "XFCE",
            Self::Kde => "KDE",
            Self::Cinnamon => "Cinnamon",
            Self::I3 => "I3",
            Self::Aqua => "Aqua",
            Self::Ios => "IOS",
            Self::Android => "Android",
            Self::WebBrowser => "Web Browser",
            Self::Console => "Console",
            Self::Ubuntu => "Ubuntu",
            Self::Ermine => "Ermine",
            Self::Orbital => "Orbital",
            Self::Unknown(a) => a,
        })
    }
}

impl DesktopEnv {
    /// Returns true if the desktop environment is based on GTK.
    pub fn is_gtk(&self) -> bool {
        *self == Self::Gnome
            || *self == Self::Ubuntu
            || *self == Self::Cinnamon
            || *self == Self::Lxde
            || *self == Self::Mate
            || *self == Self::Xfce
    }

    /// Returns true if the desktop environment is based on KDE.
    pub fn is_kde(&self) -> bool {
        *self == Self::Kde
    }
}
