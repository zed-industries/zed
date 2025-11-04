pub const LINUX_SMALL: Runner = Runner("namespace-profile-2x4-ubuntu-2404");
pub const LINUX_DEFAULT: Runner = LINUX_XL;
pub const LINUX_XL: Runner = Runner("namespace-profile-16x32-ubuntu-2204");
pub const LINUX_LARGE: Runner = Runner("namespace-profile-8x16-ubuntu-2204");
pub const LINUX_MEDIUM: Runner = Runner("namespace-profile-4x8-ubuntu-2204");

// Using Ubuntu 20.04 for minimal glibc version
pub const LINUX_X86_BUNDLER: Runner = Runner("namespace-profile-32x64-ubuntu-2004");
pub const LINUX_ARM_BUNDLER: Runner = Runner("namespace-profile-8x32-ubuntu-2004-arm-m4");

pub const MAC_DEFAULT: Runner = Runner("self-mini-macos");
pub const WINDOWS_DEFAULT: Runner = Runner("self-32vcpu-windows-2022");

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
