pub const LINUX_SMALL: Runner = Runner("ubuntu-latest");
pub const LINUX_DEFAULT: Runner = LINUX_XL;
pub const LINUX_XL: Runner = Runner("ubuntu-latest");
pub const LINUX_LARGE: Runner = Runner("ubuntu-latest");
pub const LINUX_MEDIUM: Runner = Runner("ubuntu-latest");

// Using Ubuntu 20.04 for minimal glibc version
pub const LINUX_X86_BUNDLER: Runner = Runner("ubuntu-latest");
pub const LINUX_ARM_BUNDLER: Runner = Runner("ubuntu-latest");

// Larger Ubuntu runner with glibc 2.39 for extension bundling
pub const LINUX_LARGE_RAM: Runner = Runner("ubuntu-latest");

pub const MAC_DEFAULT: Runner = Runner("macos-latest");
pub const WINDOWS_DEFAULT: Runner = Runner("windows-latest");

pub struct Runner(&'static str);

impl Into<gh_workflow::RunsOn> for Runner {
    fn into(self) -> gh_workflow::RunsOn {
        self.0.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    X86_64,
    AARCH64,
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::X86_64 => write!(f, "x86_64"),
            Arch::AARCH64 => write!(f, "aarch64"),
        }
    }
}

impl Arch {
    pub fn linux_bundler(&self) -> Runner {
        match self {
            Arch::X86_64 => LINUX_X86_BUNDLER,
            Arch::AARCH64 => LINUX_ARM_BUNDLER,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    Linux,
    Mac,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Windows => write!(f, "windows"),
            Platform::Linux => write!(f, "linux"),
            Platform::Mac => write!(f, "mac"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReleaseChannel {
    Nightly,
}
