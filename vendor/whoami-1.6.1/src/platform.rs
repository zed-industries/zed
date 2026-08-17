use std::fmt::{self, Display, Formatter};

/// The underlying platform for a system
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum Platform {
    Linux,
    Bsd,
    Windows,
    // FIXME: Non-standard casing; Rename to 'Mac' rather than 'MacOs' in
    // whoami 2.0.0
    MacOS,
    Illumos,
    Ios,
    Android,
    // FIXME: Separate for different Nintendo consoles in whoami 2.0.0,
    // currently only used for 3DS
    Nintendo,
    // FIXME: Currently unused, remove in whoami 2.0.0
    Xbox,
    PlayStation,
    Fuchsia,
    Redox,
    Hurd,
    Unknown(String),
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::Linux => "Linux",
            Self::Bsd => "BSD",
            Self::Windows => "Windows",
            Self::MacOS => "Mac OS",
            Self::Illumos => "illumos",
            Self::Ios => "iOS",
            Self::Android => "Android",
            Self::Nintendo => "Nintendo",
            Self::Xbox => "XBox",
            Self::PlayStation => "PlayStation",
            Self::Fuchsia => "Fuchsia",
            Self::Redox => "Redox",
            Self::Hurd => "GNU Hurd",
            Self::Unknown(a) => a,
        })
    }
}
