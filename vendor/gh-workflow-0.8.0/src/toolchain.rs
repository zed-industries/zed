//! The typed version of the `setup-rust-toolchain` action in a GitHub workflow.
//! Docs: <https://github.com/actions-rust-lang/setup-rust-toolchain>

use std::fmt::{Display, Formatter};

use derive_setters::Setters;

use crate::{private, Input, RustFlags, Step, Use};

#[derive(Clone)]
pub enum Version {
    Stable,
    Nightly,
    Custom((u64, u64, u64)),
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Version::Stable => write!(f, "stable"),
            Version::Nightly => write!(f, "nightly"),
            Version::Custom(s) => write!(f, "{}.{}.{}", s.0, s.1, s.2),
        }
    }
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version::Custom((major, minor, patch))
    }
}

#[derive(Clone, Debug)]
pub enum Component {
    Clippy,
    Rustfmt,
    RustDoc,
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Component::Clippy => "clippy",
            Component::Rustfmt => "rustfmt",
            Component::RustDoc => "rust-doc",
        };
        write!(f, "{val}")
    }
}

#[derive(Clone)]
pub enum Arch {
    X86_64,
    Aarch64,
    Arm,
    Wasm32,
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
            Arch::Arm => "arm",
            Arch::Wasm32 => "wasm32",
        };
        write!(f, "{val}")
    }
}

#[derive(Clone)]
pub enum Vendor {
    Unknown,
    Apple,
    PC,
}

impl Display for Vendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Vendor::Unknown => "unknown",
            Vendor::Apple => "apple",
            Vendor::PC => "pc",
        };
        write!(f, "{val}")
    }
}

#[derive(Clone)]
pub enum System {
    Unknown,
    Windows,
    Linux,
    Darwin,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            System::Unknown => "unknown",
            System::Windows => "windows",
            System::Linux => "linux",
            System::Darwin => "darwin",
        };
        write!(f, "{val}")
    }
}

#[derive(Clone)]
pub enum Abi {
    Unknown,
    Gnu,
    Msvc,
    Musl,
}

impl Display for Abi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Abi::Unknown => "unknown",
            Abi::Gnu => "gnu",
            Abi::Msvc => "msvc",
            Abi::Musl => "musl",
        };
        write!(f, "{val}")
    }
}

#[derive(Clone, Setters)]
pub struct Target {
    pub arch: Arch,
    pub vendor: Vendor,
    pub system: System,
    pub abi: Option<Abi>,
}

///
/// A Rust representation for the inputs of the setup-rust action.
/// More information can be found [here](https://github.com/actions-rust-lang/setup-rust-toolchain/blob/main/action.yml).
/// NOTE: The public API should be close to the original action as much as
/// possible.
#[derive(Default, Clone, Setters)]
#[setters(
    strip_option,
    into,
    generate_delegates(ty = "Step<Toolchain>", field = "marker")
)]
pub struct Toolchain {
    pub version: Vec<Version>,
    #[setters(skip)]
    pub target: Option<Target>,
    pub components: Vec<Component>,
    pub cache: Option<bool>,
    pub cache_directories: Vec<String>,
    pub cache_workspaces: Vec<String>,
    pub cache_on_failure: Option<bool>,
    pub cache_key: Option<String>,
    pub matcher: Option<bool>,
    pub rust_flags: Option<RustFlags>,
    pub override_default: Option<bool>,
}

impl Toolchain {
    pub fn add_version(mut self, version: Version) -> Self {
        self.version.push(version);
        self
    }

    pub fn add_component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    pub fn add_stable(mut self) -> Self {
        self.version.push(Version::Stable);
        self
    }

    pub fn add_nightly(mut self) -> Self {
        self.version.push(Version::Nightly);
        self
    }

    pub fn add_clippy(mut self) -> Self {
        self.components.push(Component::Clippy);
        self
    }

    pub fn add_fmt(mut self) -> Self {
        self.components.push(Component::Rustfmt);
        self
    }

    pub fn target(mut self, arch: Arch, vendor: Vendor, system: System, abi: Option<Abi>) -> Self {
        self.target = Some(Target { arch, vendor, system, abi });
        self
    }
}

impl From<Toolchain> for Step<Use> {
    fn from(value: Toolchain) -> Self {
        let mut step = Step::new("Setup Rust Toolchain").uses(
            "actions-rust-lang",
            "setup-rust-toolchain",
            "v1",
        );

        let toolchain = value
            .version
            .iter()
            .map(|t| match t {
                Version::Stable => "stable".to_string(),
                Version::Nightly => "nightly".to_string(),
                Version::Custom((major, minor, patch)) => {
                    format!("{major}.{minor}.{patch}")
                }
            })
            .reduce(|acc, a| format!("{acc}, {a}"));

        let mut input = Input::default();

        if let Some(toolchain) = toolchain {
            input = input.add("toolchain", toolchain);
        }

        if let Some(target) = value.target {
            let target = format!(
                "{}-{}-{}{}",
                target.arch,
                target.vendor,
                target.system,
                target.abi.map(|v| v.to_string()).unwrap_or_default(),
            );

            input = input.add("target", target);
        }

        if !value.components.is_empty() {
            let components = value
                .components
                .iter()
                .map(|c| c.to_string())
                .reduce(|acc, a| format!("{acc}, {a}"))
                .unwrap_or_default();

            input = input.add("components", components);
        }

        if let Some(cache) = value.cache {
            input = input.add("cache", cache);
        }

        if !value.cache_directories.is_empty() {
            let cache_directories = value.cache_directories.join("\n");
            input = input.add("cache-directories", cache_directories);
        }

        if !value.cache_workspaces.is_empty() {
            let cache_workspaces = value
                .cache_workspaces
                .iter()
                .fold("".to_string(), |acc, a| format!("{acc}\n{a}"));

            input = input.add("cache-workspaces", cache_workspaces);
        }

        if let Some(cache_on_failure) = value.cache_on_failure {
            input = input.add("cache-on-failure", cache_on_failure);
        }

        if let Some(cache_key) = value.cache_key {
            input = input.add("cache-key", cache_key);
        }

        if let Some(matcher) = value.matcher {
            input = input.add("matcher", matcher);
        }

        if let Some(rust_flags) = value.rust_flags {
            input = input.add("rust-flags", rust_flags.to_string());
        }

        if let Some(override_default) = value.override_default {
            input = input.add("override", override_default);
        }

        if !input.is_empty() {
            step = step.with(input);
        }
        step
    }
}

impl private::Sealed for Toolchain {}
