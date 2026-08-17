// This file defines all the identifier enums and target-aware logic.

use crate::triple::{Endianness, PointerWidth, Triple};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::str::FromStr;

/// The "architecture" field, which in some cases also specifies a specific
/// subarchitecture.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Architecture {
    Unknown,
    Arm(ArmArchitecture),
    AmdGcn,
    Aarch64(Aarch64Architecture),
    Asmjs,
    Avr,
    Bpfeb,
    Bpfel,
    Hexagon,
    X86_32(X86_32Architecture),
    M68k,
    LoongArch64,
    Mips32(Mips32Architecture),
    Mips64(Mips64Architecture),
    Msp430,
    Nvptx64,
    Pulley32,
    Pulley64,
    Pulley32be,
    Pulley64be,
    Powerpc,
    Powerpc64,
    Powerpc64le,
    Riscv32(Riscv32Architecture),
    Riscv64(Riscv64Architecture),
    S390x,
    Sparc,
    Sparc64,
    Sparcv9,
    Wasm32,
    Wasm64,
    X86_64,
    /// x86_64 target that only supports Haswell-compatible Intel chips.
    X86_64h,
    XTensa,
    Clever(CleverArchitecture),
    /// A software machine that produces zero-knowledge proofs of the execution.
    ///
    /// See https://wiki.polygon.technology/docs/category/zk-assembly/
    #[cfg(feature = "arch_zkasm")]
    ZkAsm,
    #[cfg(feature = "arch_z80")]
    Z80(Z80Architecture),
}

#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ArmArchitecture {
    Arm, // Generic arm
    Armeb,
    Armv4,
    Armv4t,
    Armv5t,
    Armv5te,
    Armv5tej,
    Armv6,
    Armv6j,
    Armv6k,
    Armv6z,
    Armv6kz,
    Armv6t2,
    Armv6m,
    Armv7,
    Armv7a,
    Armv7k,
    Armv7ve,
    Armv7m,
    Armv7r,
    Armv7s,
    Armv8,
    Armv8a,
    Armv8_1a,
    Armv8_2a,
    Armv8_3a,
    Armv8_4a,
    Armv8_5a,
    Armv8mBase,
    Armv8mMain,
    Armv8r,

    Armebv7r,

    Thumbeb,
    Thumbv4t,
    Thumbv5te,
    Thumbv6m,
    Thumbv7a,
    Thumbv7em,
    Thumbv7m,
    Thumbv7neon,
    Thumbv8mBase,
    Thumbv8mMain,
}

#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Aarch64Architecture {
    Aarch64,
    Aarch64be,
}

// #[cfg_attr(feature = "rust_1_40", non_exhaustive)]
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// #[allow(missing_docs)]
// pub enum ArmFpu {
//     Vfp,
//     Vfpv2,
//     Vfpv3,
//     Vfpv3Fp16,
//     Vfpv3Xd,
//     Vfpv3XdFp16,
//     Neon,
//     NeonVfpv3,
//     NeonVfpv4,
//     Vfpv4,
//     Vfpv4D16,
//     Fpv4SpD16,
//     Fpv5SpD16,
//     Fpv5D16,
//     FpArmv8,
//     NeonFpArmv8,
//     CryptoNeonFpArmv8,
// }

impl ArmArchitecture {
    /// Test if this architecture uses the Thumb instruction set.
    #[rustfmt::skip]
    pub fn is_thumb(self) -> bool {
        use ArmArchitecture::*;

        match self {
            Arm
            | Armeb
            | Armv4
            | Armv4t
            | Armv5t
            | Armv5te
            | Armv5tej
            | Armv6
            | Armv6j
            | Armv6k
            | Armv6z
            | Armv6kz
            | Armv6t2
            | Armv6m
            | Armv7
            | Armv7a
            | Armv7k
            | Armv7ve
            | Armv7m
            | Armv7r
            | Armv7s
            | Armv8
            | Armv8a
            | Armv8_1a
            | Armv8_2a
            | Armv8_3a
            | Armv8_4a
            | Armv8_5a
            | Armv8mBase
            | Armv8mMain
            | Armv8r
            | Armebv7r => false,
            Thumbeb
            | Thumbv4t
            | Thumbv5te
            | Thumbv6m
            | Thumbv7a
            | Thumbv7em
            | Thumbv7m
            | Thumbv7neon
            | Thumbv8mBase
            | Thumbv8mMain => true,
        }
    }

    // pub fn has_fpu(self) -> Result<&'static [ArmFpu], ()> {

    // }

    /// Return the pointer bit width of this target's architecture.
    #[rustfmt::skip]
    pub fn pointer_width(self) -> PointerWidth {
        use ArmArchitecture::*;

        match self {
            Arm
            | Armeb
            | Armv4
            | Armv4t
            | Armv5t
            | Armv5te
            | Armv5tej
            | Armv6
            | Armv6j
            | Armv6k
            | Armv6z
            | Armv6kz
            | Armv6t2
            | Armv6m
            | Armv7
            | Armv7a
            | Armv7k
            | Armv7ve
            | Armv7m
            | Armv7r
            | Armv7s
            | Armv8
            | Armv8a
            | Armv8_1a
            | Armv8_2a
            | Armv8_3a
            | Armv8_4a
            | Armv8_5a
            | Armv8mBase
            | Armv8mMain
            | Armv8r
            | Armebv7r
            | Thumbeb
            | Thumbv4t
            | Thumbv5te
            | Thumbv6m
            | Thumbv7a
            | Thumbv7em
            | Thumbv7m
            | Thumbv7neon
            | Thumbv8mBase
            | Thumbv8mMain => PointerWidth::U32,
        }
    }

    /// Return the endianness of this architecture.
    #[rustfmt::skip]
    pub fn endianness(self) -> Endianness {
        use ArmArchitecture::*;

        match self {
            Arm
            | Armv4
            | Armv4t
            | Armv5t
            | Armv5te
            | Armv5tej
            | Armv6
            | Armv6j
            | Armv6k
            | Armv6z
            | Armv6kz
            | Armv6t2
            | Armv6m
            | Armv7
            | Armv7a
            | Armv7k
            | Armv7ve
            | Armv7m
            | Armv7r
            | Armv7s
            | Armv8
            | Armv8a
            | Armv8_1a
            | Armv8_2a
            | Armv8_3a
            | Armv8_4a
            | Armv8_5a
            | Armv8mBase
            | Armv8mMain
            | Armv8r
            | Thumbv4t
            | Thumbv5te
            | Thumbv6m
            | Thumbv7a
            | Thumbv7em
            | Thumbv7m
            | Thumbv7neon
            | Thumbv8mBase
            | Thumbv8mMain => Endianness::Little,
            Armeb | Armebv7r | Thumbeb => Endianness::Big,
        }
    }

    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use ArmArchitecture::*;

        match self {
            Arm => Cow::Borrowed("arm"),
            Armeb => Cow::Borrowed("armeb"),
            Armv4 => Cow::Borrowed("armv4"),
            Armv4t => Cow::Borrowed("armv4t"),
            Armv5t => Cow::Borrowed("armv5t"),
            Armv5te => Cow::Borrowed("armv5te"),
            Armv5tej => Cow::Borrowed("armv5tej"),
            Armv6 => Cow::Borrowed("armv6"),
            Armv6j => Cow::Borrowed("armv6j"),
            Armv6k => Cow::Borrowed("armv6k"),
            Armv6z => Cow::Borrowed("armv6z"),
            Armv6kz => Cow::Borrowed("armv6kz"),
            Armv6t2 => Cow::Borrowed("armv6t2"),
            Armv6m => Cow::Borrowed("armv6m"),
            Armv7 => Cow::Borrowed("armv7"),
            Armv7a => Cow::Borrowed("armv7a"),
            Armv7k => Cow::Borrowed("armv7k"),
            Armv7ve => Cow::Borrowed("armv7ve"),
            Armv7m => Cow::Borrowed("armv7m"),
            Armv7r => Cow::Borrowed("armv7r"),
            Armv7s => Cow::Borrowed("armv7s"),
            Armv8 => Cow::Borrowed("armv8"),
            Armv8a => Cow::Borrowed("armv8a"),
            Armv8_1a => Cow::Borrowed("armv8.1a"),
            Armv8_2a => Cow::Borrowed("armv8.2a"),
            Armv8_3a => Cow::Borrowed("armv8.3a"),
            Armv8_4a => Cow::Borrowed("armv8.4a"),
            Armv8_5a => Cow::Borrowed("armv8.5a"),
            Armv8mBase => Cow::Borrowed("armv8m.base"),
            Armv8mMain => Cow::Borrowed("armv8m.main"),
            Armv8r => Cow::Borrowed("armv8r"),
            Thumbeb => Cow::Borrowed("thumbeb"),
            Thumbv4t => Cow::Borrowed("thumbv4t"),
            Thumbv5te => Cow::Borrowed("thumbv5te"),
            Thumbv6m => Cow::Borrowed("thumbv6m"),
            Thumbv7a => Cow::Borrowed("thumbv7a"),
            Thumbv7em => Cow::Borrowed("thumbv7em"),
            Thumbv7m => Cow::Borrowed("thumbv7m"),
            Thumbv7neon => Cow::Borrowed("thumbv7neon"),
            Thumbv8mBase => Cow::Borrowed("thumbv8m.base"),
            Thumbv8mMain => Cow::Borrowed("thumbv8m.main"),
            Armebv7r => Cow::Borrowed("armebv7r"),
        }
    }
}

impl Aarch64Architecture {
    /// Test if this architecture uses the Thumb instruction set.
    pub fn is_thumb(self) -> bool {
        match self {
            Aarch64Architecture::Aarch64 | Aarch64Architecture::Aarch64be => false,
        }
    }

    // pub fn has_fpu(self) -> Result<&'static [ArmFpu], ()> {

    // }

    /// Return the pointer bit width of this target's architecture.
    ///
    /// This function is only aware of the CPU architecture so it is not aware
    /// of ilp32 ABIs.
    pub fn pointer_width(self) -> PointerWidth {
        match self {
            Aarch64Architecture::Aarch64 | Aarch64Architecture::Aarch64be => PointerWidth::U64,
        }
    }

    /// Return the endianness of this architecture.
    pub fn endianness(self) -> Endianness {
        match self {
            Aarch64Architecture::Aarch64 => Endianness::Little,
            Aarch64Architecture::Aarch64be => Endianness::Big,
        }
    }

    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use Aarch64Architecture::*;

        match self {
            Aarch64 => Cow::Borrowed("aarch64"),
            Aarch64be => Cow::Borrowed("aarch64_be"),
        }
    }
}

#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum CleverArchitecture {
    Clever,
    Clever1_0,
}

impl CleverArchitecture {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use CleverArchitecture::*;

        match self {
            Clever => Cow::Borrowed("clever"),
            Clever1_0 => Cow::Borrowed("clever1.0"),
        }
    }
}

/// An enum for all 32-bit RISC-V architectures.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Riscv32Architecture {
    Riscv32,
    Riscv32gc,
    Riscv32i,
    Riscv32im,
    Riscv32ima,
    Riscv32imac,
    Riscv32imafc,
    Riscv32imc,
}

impl Riscv32Architecture {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use Riscv32Architecture::*;

        match self {
            Riscv32 => Cow::Borrowed("riscv32"),
            Riscv32gc => Cow::Borrowed("riscv32gc"),
            Riscv32i => Cow::Borrowed("riscv32i"),
            Riscv32im => Cow::Borrowed("riscv32im"),
            Riscv32ima => Cow::Borrowed("riscv32ima"),
            Riscv32imac => Cow::Borrowed("riscv32imac"),
            Riscv32imafc => Cow::Borrowed("riscv32imafc"),
            Riscv32imc => Cow::Borrowed("riscv32imc"),
        }
    }
}

/// An enum for all 64-bit RISC-V architectures.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Riscv64Architecture {
    Riscv64,
    Riscv64gc,
    Riscv64imac,
    Riscv64a23,
}

impl Riscv64Architecture {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use Riscv64Architecture::*;

        match self {
            Riscv64 => Cow::Borrowed("riscv64"),
            Riscv64gc => Cow::Borrowed("riscv64gc"),
            Riscv64imac => Cow::Borrowed("riscv64imac"),
            Riscv64a23 => Cow::Borrowed("riscv64a23"),
        }
    }
}

/// An enum for all 32-bit x86 architectures.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum X86_32Architecture {
    I386,
    I586,
    I686,
}

impl X86_32Architecture {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use X86_32Architecture::*;

        match self {
            I386 => Cow::Borrowed("i386"),
            I586 => Cow::Borrowed("i586"),
            I686 => Cow::Borrowed("i686"),
        }
    }
}

/// An enum for all 32-bit MIPS architectures (not just "MIPS32").
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Mips32Architecture {
    Mips,
    Mipsel,
    Mipsisa32r6,
    Mipsisa32r6el,
}

impl Mips32Architecture {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use Mips32Architecture::*;

        match self {
            Mips => Cow::Borrowed("mips"),
            Mipsel => Cow::Borrowed("mipsel"),
            Mipsisa32r6 => Cow::Borrowed("mipsisa32r6"),
            Mipsisa32r6el => Cow::Borrowed("mipsisa32r6el"),
        }
    }
}

/// An enum for all 64-bit MIPS architectures (not just "MIPS64").
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Mips64Architecture {
    Mips64,
    Mips64el,
    Mipsisa64r6,
    Mipsisa64r6el,
}

impl Mips64Architecture {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use Mips64Architecture::*;

        match self {
            Mips64 => Cow::Borrowed("mips64"),
            Mips64el => Cow::Borrowed("mips64el"),
            Mipsisa64r6 => Cow::Borrowed("mipsisa64r6"),
            Mipsisa64r6el => Cow::Borrowed("mipsisa64r6el"),
        }
    }
}

#[cfg(feature = "arch_z80")]
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Z80Architecture {
    Z80,
    Z180,
    Ez80,
    Sm83,
    Rabbit2000,
    Rabbit2000A,
    Rabbit3000,
    Rabbit3000A,
    Tlcs90,
    R800,
}

#[cfg(feature = "arch_z80")]
impl Z80Architecture {
    pub fn into_str(self) -> Cow<'static, str> {
        use Z80Architecture::*;

        match self {
            Z80 => Cow::Borrowed("z80"),
            Z180 => Cow::Borrowed("z180"),
            Ez80 => Cow::Borrowed("ez80"),
            Sm83 => Cow::Borrowed("sm83"),
            Rabbit2000 => Cow::Borrowed("rabbit2000"),
            Rabbit2000A => Cow::Borrowed("rabbit2000a"),
            Rabbit3000 => Cow::Borrowed("rabbit3000"),
            Rabbit3000A => Cow::Borrowed("rabbit3000a"),
            Tlcs90 => Cow::Borrowed("tlcs90"),
            R800 => Cow::Borrowed("r800"),
        }
    }
}

/// A string for a `Vendor::Custom` that can either be used in `const`
/// contexts or hold dynamic strings.
#[derive(Clone, Debug, Eq)]
pub enum CustomVendor {
    /// An owned `String`. This supports the general case.
    Owned(Box<String>),
    /// A static `str`, so that `CustomVendor` can be constructed in `const`
    /// contexts.
    Static(&'static str),
}

impl CustomVendor {
    /// Extracts a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            CustomVendor::Owned(s) => s,
            CustomVendor::Static(s) => s,
        }
    }
}

impl PartialEq for CustomVendor {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Hash for CustomVendor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

/// The "vendor" field, which in practice is little more than an arbitrary
/// modifier.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Vendor {
    Unknown,
    Amd,
    Apple,
    Espressif,
    Experimental,
    Fortanix,
    Ibm,
    Kmc,
    Nintendo,
    Nvidia,
    Pc,
    Rumprun,
    Sun,
    Uwp,
    Wrs,

    /// A custom vendor. "Custom" in this context means that the vendor is
    /// not specifically recognized by upstream Autotools, LLVM, Rust, or other
    /// relevant authorities on triple naming. It's useful for people building
    /// and using locally patched toolchains.
    ///
    /// Outside of such patched environments, users of `target-lexicon` should
    /// treat `Custom` the same as `Unknown` and ignore the string.
    Custom(CustomVendor),
}

impl Vendor {
    /// Extracts a string slice.
    pub fn as_str(&self) -> &str {
        use Vendor::*;

        match self {
            Unknown => "unknown",
            Amd => "amd",
            Apple => "apple",
            Espressif => "espressif",
            Experimental => "experimental",
            Fortanix => "fortanix",
            Ibm => "ibm",
            Kmc => "kmc",
            Nintendo => "nintendo",
            Nvidia => "nvidia",
            Pc => "pc",
            Rumprun => "rumprun",
            Sun => "sun",
            Uwp => "uwp",
            Wrs => "wrs",
            Custom(name) => name.as_str(),
        }
    }
}

/// The minimum OS version that we're compiling for.
///
/// This is formatted as `"major.minor.patch"`.
///
/// The size of the parts here are limited by Mach-O's `LC_BUILD_VERSION`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(missing_docs)]
pub struct DeploymentTarget {
    pub major: u16,
    pub minor: u8,
    pub patch: u8,
}

/// The "operating system" field, which sometimes implies an environment, and
/// sometimes isn't an actual operating system.
///
/// LLVM's Apple triples may optionally include the [deployment target].
///
/// [deployment target]: DeploymentTarget
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum OperatingSystem {
    Unknown,
    Aix,
    AmdHsa,
    Bitrig,
    Cloudabi,
    Cuda,
    Cygwin,
    /// The general [Darwin][darwin-wiki] core OS.
    ///
    /// Generally, `-mmacosx-version-min=...` or similar flags are required by
    /// Clang to determine the actual OS (either macOS, iOS, tvOS, watchOS or
    /// visionOS).
    ///
    /// WARNING: When parsing `rustc` target triples, this matches the macOS
    /// target triples as well.
    ///
    /// [darwin-wiki]: https://en.wikipedia.org/wiki/Darwin_(operating_system)
    Darwin(Option<DeploymentTarget>),
    Dragonfly,
    Emscripten,
    Espidf,
    Freebsd,
    Fuchsia,
    Haiku,
    Hermit,
    Horizon,
    Hurd,
    Illumos,
    IOS(Option<DeploymentTarget>),
    L4re,
    Linux,
    /// macOS.
    ///
    /// WARNING: This does _not_ match the macOS triples when parsing `rustc`
    /// target triples, for that see the [`darwin`](Self::Darwin) OS name.
    MacOSX(Option<DeploymentTarget>),
    Nebulet,
    Netbsd,
    None_,
    Openbsd,
    Psp,
    Redox,
    Solaris,
    SolidAsp3,
    TvOS(Option<DeploymentTarget>),
    Uefi,
    VisionOS(Option<DeploymentTarget>),
    VxWorks,
    Wasi,
    WasiP1,
    WasiP2,
    WatchOS(Option<DeploymentTarget>),
    Windows,
    /// An alternate name for [visionOS][Self::VisionOS].
    XROS(Option<DeploymentTarget>),
}

impl OperatingSystem {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use OperatingSystem::*;

        let darwin_version = |name, deployment_target| {
            if let Some(DeploymentTarget {
                major,
                minor,
                patch,
            }) = deployment_target
            {
                Cow::Owned(format!("{}{}.{}.{}", name, major, minor, patch))
            } else {
                Cow::Borrowed(name)
            }
        };

        match self {
            Unknown => Cow::Borrowed("unknown"),
            Aix => Cow::Borrowed("aix"),
            AmdHsa => Cow::Borrowed("amdhsa"),
            Bitrig => Cow::Borrowed("bitrig"),
            Cloudabi => Cow::Borrowed("cloudabi"),
            Cuda => Cow::Borrowed("cuda"),
            Cygwin => Cow::Borrowed("cygwin"),
            Darwin(deployment_target) => darwin_version("darwin", deployment_target),
            Dragonfly => Cow::Borrowed("dragonfly"),
            Emscripten => Cow::Borrowed("emscripten"),
            Espidf => Cow::Borrowed("espidf"),
            Freebsd => Cow::Borrowed("freebsd"),
            Fuchsia => Cow::Borrowed("fuchsia"),
            Haiku => Cow::Borrowed("haiku"),
            Hermit => Cow::Borrowed("hermit"),
            Horizon => Cow::Borrowed("horizon"),
            Hurd => Cow::Borrowed("hurd"),
            Illumos => Cow::Borrowed("illumos"),
            IOS(deployment_target) => darwin_version("ios", deployment_target),
            L4re => Cow::Borrowed("l4re"),
            Linux => Cow::Borrowed("linux"),
            MacOSX(deployment_target) => darwin_version("macosx", deployment_target),
            Nebulet => Cow::Borrowed("nebulet"),
            Netbsd => Cow::Borrowed("netbsd"),
            None_ => Cow::Borrowed("none"),
            Openbsd => Cow::Borrowed("openbsd"),
            Psp => Cow::Borrowed("psp"),
            Redox => Cow::Borrowed("redox"),
            Solaris => Cow::Borrowed("solaris"),
            SolidAsp3 => Cow::Borrowed("solid_asp3"),
            TvOS(deployment_target) => darwin_version("tvos", deployment_target),
            Uefi => Cow::Borrowed("uefi"),
            VxWorks => Cow::Borrowed("vxworks"),
            VisionOS(deployment_target) => darwin_version("visionos", deployment_target),
            Wasi => Cow::Borrowed("wasi"),
            WasiP1 => Cow::Borrowed("wasip1"),
            WasiP2 => Cow::Borrowed("wasip2"),
            WatchOS(deployment_target) => darwin_version("watchos", deployment_target),
            Windows => Cow::Borrowed("windows"),
            XROS(deployment_target) => darwin_version("xros", deployment_target),
        }
    }

    /// Whether the OS is similar to Darwin.
    ///
    /// This matches on any of:
    /// - [Darwin](Self::Darwin)
    /// - [iOS](Self::IOS)
    /// - [macOS](Self::MacOSX)
    /// - [tvOS](Self::TvOS)
    /// - [visionOS](Self::VisionOS)
    /// - [watchOS](Self::WatchOS)
    /// - [xrOS](Self::XROS)
    pub fn is_like_darwin(&self) -> bool {
        use OperatingSystem::*;

        match self {
            Darwin(_) | IOS(_) | MacOSX(_) | TvOS(_) | VisionOS(_) | WatchOS(_) | XROS(_) => true,
            _ => false,
        }
    }
}

/// The "environment" field, which specifies an ABI environment on top of the
/// operating system. In many configurations, this field is omitted, and the
/// environment is implied by the operating system.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Environment {
    Unknown,
    AmdGiz,
    Android,
    Androideabi,
    Eabi,
    Eabihf,
    Gnu,
    Gnuabi64,
    Gnueabi,
    Gnueabihf,
    Gnuspe,
    Gnux32,
    GnuIlp32,
    GnuLlvm,
    HermitKernel,
    HurdKernel,
    LinuxKernel,
    Macabi,
    Musl,
    Musleabi,
    Musleabihf,
    Muslabi64,
    Msvc,
    Newlib,
    None,
    Kernel,
    Uclibc,
    Uclibceabi,
    Uclibceabihf,
    Sgx,
    Sim,
    Softfloat,
    Spe,
    Threads,
    Ohos,
}

impl Environment {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use Environment::*;

        match self {
            Unknown => Cow::Borrowed("unknown"),
            AmdGiz => Cow::Borrowed("amdgiz"),
            Android => Cow::Borrowed("android"),
            Androideabi => Cow::Borrowed("androideabi"),
            Eabi => Cow::Borrowed("eabi"),
            Eabihf => Cow::Borrowed("eabihf"),
            Gnu => Cow::Borrowed("gnu"),
            Gnuabi64 => Cow::Borrowed("gnuabi64"),
            Gnueabi => Cow::Borrowed("gnueabi"),
            Gnueabihf => Cow::Borrowed("gnueabihf"),
            Gnuspe => Cow::Borrowed("gnuspe"),
            Gnux32 => Cow::Borrowed("gnux32"),
            GnuIlp32 => Cow::Borrowed("gnu_ilp32"),
            GnuLlvm => Cow::Borrowed("gnullvm"),
            HermitKernel => Cow::Borrowed("hermitkernel"),
            HurdKernel => Cow::Borrowed("hurdkernel"),
            LinuxKernel => Cow::Borrowed("linuxkernel"),
            Macabi => Cow::Borrowed("macabi"),
            Musl => Cow::Borrowed("musl"),
            Musleabi => Cow::Borrowed("musleabi"),
            Musleabihf => Cow::Borrowed("musleabihf"),
            Muslabi64 => Cow::Borrowed("muslabi64"),
            Msvc => Cow::Borrowed("msvc"),
            Newlib => Cow::Borrowed("newlib"),
            None => Cow::Borrowed("none"),
            Kernel => Cow::Borrowed("kernel"),
            Uclibc => Cow::Borrowed("uclibc"),
            Uclibceabi => Cow::Borrowed("uclibceabi"),
            Uclibceabihf => Cow::Borrowed("uclibceabihf"),
            Sgx => Cow::Borrowed("sgx"),
            Sim => Cow::Borrowed("sim"),
            Softfloat => Cow::Borrowed("softfloat"),
            Spe => Cow::Borrowed("spe"),
            Threads => Cow::Borrowed("threads"),
            Ohos => Cow::Borrowed("ohos"),
        }
    }
}

/// The "binary format" field, which is usually omitted, and the binary format
/// is implied by the other fields.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum BinaryFormat {
    Unknown,
    Elf,
    Coff,
    Macho,
    Wasm,
    Xcoff,
}

impl BinaryFormat {
    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use BinaryFormat::*;

        match self {
            Unknown => Cow::Borrowed("unknown"),
            Elf => Cow::Borrowed("elf"),
            Coff => Cow::Borrowed("coff"),
            Macho => Cow::Borrowed("macho"),
            Wasm => Cow::Borrowed("wasm"),
            Xcoff => Cow::Borrowed("xcoff"),
        }
    }
}

impl Architecture {
    /// Return the endianness of this architecture.
    #[rustfmt::skip]
    pub fn endianness(self) -> Result<Endianness, ()> {
        use Architecture::*;

        match self {
            Unknown => Err(()),
            Arm(arm) => Ok(arm.endianness()),
            Aarch64(aarch) => Ok(aarch.endianness()),
            AmdGcn
            | Asmjs
            | Avr
            | Bpfel
            | Hexagon
            | X86_32(_)
            | LoongArch64
            | Mips64(Mips64Architecture::Mips64el)
            | Mips32(Mips32Architecture::Mipsel)
            | Mips32(Mips32Architecture::Mipsisa32r6el)
            | Mips64(Mips64Architecture::Mipsisa64r6el)
            | Msp430
            | Nvptx64
            | Pulley32
            | Pulley64
            | Powerpc64le
            | Riscv32(_)
            | Riscv64(_)
            | Wasm32
            | Wasm64
            | X86_64
            | X86_64h
            | XTensa
            | Clever(_) => Ok(Endianness::Little),
            Bpfeb
            | M68k
            | Mips32(Mips32Architecture::Mips)
            | Mips64(Mips64Architecture::Mips64)
            | Mips32(Mips32Architecture::Mipsisa32r6)
            | Mips64(Mips64Architecture::Mipsisa64r6)
            | Powerpc
            | Powerpc64
            | Pulley32be
            | Pulley64be
            | S390x
            | Sparc
            | Sparc64
            | Sparcv9 => Ok(Endianness::Big),
            #[cfg(feature="arch_zkasm")]
            ZkAsm => Ok(Endianness::Big),
            #[cfg(feature = "arch_z80")]
            Z80(_) => Ok(Endianness::Little),
        }
    }

    /// Return the pointer bit width of this target's architecture.
    ///
    /// This function is only aware of the CPU architecture so it is not aware
    /// of ilp32 and x32 ABIs.
    #[rustfmt::skip]
    pub fn pointer_width(self) -> Result<PointerWidth, ()> {
        use Architecture::*;

        match self {
            Unknown => Err(()),
            Avr | Msp430 => Ok(PointerWidth::U16),
            Arm(arm) => Ok(arm.pointer_width()),
            Aarch64(aarch) => Ok(aarch.pointer_width()),
            Asmjs
            | Hexagon
            | X86_32(_)
            | Riscv32(_)
            | Sparc
            | Wasm32
            | M68k
            | Mips32(_)
            | Pulley32
            | Pulley32be
            | Powerpc
            | XTensa => Ok(PointerWidth::U32),
            AmdGcn
            | Bpfeb
            | Bpfel
            | Powerpc64le
            | Riscv64(_)
            | X86_64
            | X86_64h
            | Mips64(_)
            | Nvptx64
            | Pulley64
            | Pulley64be
            | Powerpc64
            | S390x
            | Sparc64
            | Sparcv9
            | LoongArch64
            | Wasm64
            | Clever(_) => Ok(PointerWidth::U64),
            #[cfg(feature="arch_zkasm")]
            ZkAsm => Ok(PointerWidth::U64),
            #[cfg(feature = "arch_z80")]
            Z80(_) => Ok(PointerWidth::U16),
        }
    }

    /// Checks if this Architecture is some variant of Clever-ISA
    pub fn is_clever(&self) -> bool {
        match self {
            Architecture::Clever(_) => true,
            _ => false,
        }
    }

    /// Convert into a string
    pub fn into_str(self) -> Cow<'static, str> {
        use Architecture::*;

        match self {
            Arm(arm) => arm.into_str(),
            Aarch64(aarch) => aarch.into_str(),
            Unknown => Cow::Borrowed("unknown"),
            AmdGcn => Cow::Borrowed("amdgcn"),
            Asmjs => Cow::Borrowed("asmjs"),
            Avr => Cow::Borrowed("avr"),
            Bpfeb => Cow::Borrowed("bpfeb"),
            Bpfel => Cow::Borrowed("bpfel"),
            Hexagon => Cow::Borrowed("hexagon"),
            X86_32(x86_32) => x86_32.into_str(),
            LoongArch64 => Cow::Borrowed("loongarch64"),
            M68k => Cow::Borrowed("m68k"),
            Mips32(mips32) => mips32.into_str(),
            Mips64(mips64) => mips64.into_str(),
            Msp430 => Cow::Borrowed("msp430"),
            Nvptx64 => Cow::Borrowed("nvptx64"),
            Pulley32 => Cow::Borrowed("pulley32"),
            Pulley64 => Cow::Borrowed("pulley64"),
            Pulley32be => Cow::Borrowed("pulley32be"),
            Pulley64be => Cow::Borrowed("pulley64be"),
            Powerpc => Cow::Borrowed("powerpc"),
            Powerpc64 => Cow::Borrowed("powerpc64"),
            Powerpc64le => Cow::Borrowed("powerpc64le"),
            Riscv32(riscv32) => riscv32.into_str(),
            Riscv64(riscv64) => riscv64.into_str(),
            S390x => Cow::Borrowed("s390x"),
            Sparc => Cow::Borrowed("sparc"),
            Sparc64 => Cow::Borrowed("sparc64"),
            Sparcv9 => Cow::Borrowed("sparcv9"),
            Wasm32 => Cow::Borrowed("wasm32"),
            Wasm64 => Cow::Borrowed("wasm64"),
            X86_64 => Cow::Borrowed("x86_64"),
            X86_64h => Cow::Borrowed("x86_64h"),
            XTensa => Cow::Borrowed("xtensa"),
            Clever(ver) => ver.into_str(),
            #[cfg(feature = "arch_zkasm")]
            ZkAsm => Cow::Borrowed("zkasm"),
            #[cfg(feature = "arch_z80")]
            Z80(z80) => z80.into_str(),
        }
    }
}

/// Return the binary format implied by this target triple, ignoring its
/// `binary_format` field.
pub(crate) fn default_binary_format(triple: &Triple) -> BinaryFormat {
    match triple.operating_system {
        OperatingSystem::None_ => match triple.environment {
            Environment::Eabi | Environment::Eabihf => BinaryFormat::Elf,
            _ => BinaryFormat::Unknown,
        },
        OperatingSystem::Aix => BinaryFormat::Xcoff,
        os if os.is_like_darwin() => BinaryFormat::Macho,
        OperatingSystem::Windows => BinaryFormat::Coff,
        OperatingSystem::Nebulet
        | OperatingSystem::Emscripten
        | OperatingSystem::VxWorks
        | OperatingSystem::Wasi
        | OperatingSystem::Unknown => match triple.architecture {
            Architecture::Wasm32 | Architecture::Wasm64 => BinaryFormat::Wasm,
            Architecture::Unknown => BinaryFormat::Unknown,
            // Default to ELF, following `getDefaultFormat` in LLVM.
            _ => BinaryFormat::Elf,
        },
        _ => BinaryFormat::Elf,
    }
}

impl fmt::Display for ArmArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for Aarch64Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for CleverArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for Riscv32Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for Riscv64Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for X86_32Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for Mips32Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for Mips64Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

#[cfg(feature = "arch_z80")]
impl fmt::Display for Z80Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl FromStr for ArmArchitecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use ArmArchitecture::*;

        Ok(match s {
            "arm" => Arm,
            "armeb" => Armeb,
            "armv4" => Armv4,
            "armv4t" => Armv4t,
            "armv5t" => Armv5t,
            "armv5te" => Armv5te,
            "armv5tej" => Armv5tej,
            "armv6" => Armv6,
            "armv6j" => Armv6j,
            "armv6k" => Armv6k,
            "armv6z" => Armv6z,
            "armv6kz" => Armv6kz,
            "armv6t2" => Armv6t2,
            "armv6m" => Armv6m,
            "armv7" => Armv7,
            "armv7a" => Armv7a,
            "armv7k" => Armv7k,
            "armv7ve" => Armv7ve,
            "armv7m" => Armv7m,
            "armv7r" => Armv7r,
            "armv7s" => Armv7s,
            "armv8" => Armv8,
            "armv8a" => Armv8a,
            "armv8.1a" => Armv8_1a,
            "armv8.2a" => Armv8_2a,
            "armv8.3a" => Armv8_3a,
            "armv8.4a" => Armv8_4a,
            "armv8.5a" => Armv8_5a,
            "armv8m.base" => Armv8mBase,
            "armv8m.main" => Armv8mMain,
            "armv8r" => Armv8r,
            "thumbeb" => Thumbeb,
            "thumbv4t" => Thumbv4t,
            "thumbv5te" => Thumbv5te,
            "thumbv6m" => Thumbv6m,
            "thumbv7a" => Thumbv7a,
            "thumbv7em" => Thumbv7em,
            "thumbv7m" => Thumbv7m,
            "thumbv7neon" => Thumbv7neon,
            "thumbv8m.base" => Thumbv8mBase,
            "thumbv8m.main" => Thumbv8mMain,
            "armebv7r" => Armebv7r,
            _ => return Err(()),
        })
    }
}

impl FromStr for Aarch64Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Aarch64Architecture::*;

        Ok(match s {
            "aarch64" => Aarch64,
            "arm64" => Aarch64,
            "aarch64_be" => Aarch64be,
            _ => return Err(()),
        })
    }
}

impl FromStr for CleverArchitecture {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "clever" => Ok(CleverArchitecture::Clever),
            "clever1.0" => Ok(CleverArchitecture::Clever1_0),
            _ => Err(()),
        }
    }
}

impl FromStr for Riscv32Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Riscv32Architecture::*;

        Ok(match s {
            "riscv32" => Riscv32,
            "riscv32gc" => Riscv32gc,
            "riscv32i" => Riscv32i,
            "riscv32im" => Riscv32im,
            "riscv32ima" => Riscv32ima,
            "riscv32imac" => Riscv32imac,
            "riscv32imafc" => Riscv32imafc,
            "riscv32imc" => Riscv32imc,
            _ => return Err(()),
        })
    }
}

impl FromStr for Riscv64Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Riscv64Architecture::*;

        Ok(match s {
            "riscv64" => Riscv64,
            "riscv64gc" => Riscv64gc,
            "riscv64imac" => Riscv64imac,
            "riscv64a23" => Riscv64a23,
            _ => return Err(()),
        })
    }
}

impl FromStr for X86_32Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use X86_32Architecture::*;

        Ok(match s {
            "i386" => I386,
            "i586" => I586,
            "i686" => I686,
            _ => return Err(()),
        })
    }
}

impl FromStr for Mips32Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Mips32Architecture::*;

        Ok(match s {
            "mips" => Mips,
            "mipsel" => Mipsel,
            "mipsisa32r6" => Mipsisa32r6,
            "mipsisa32r6el" => Mipsisa32r6el,
            _ => return Err(()),
        })
    }
}

impl FromStr for Mips64Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Mips64Architecture::*;

        Ok(match s {
            "mips64" => Mips64,
            "mips64el" => Mips64el,
            "mipsisa64r6" => Mipsisa64r6,
            "mipsisa64r6el" => Mipsisa64r6el,
            _ => return Err(()),
        })
    }
}

#[cfg(feature = "arch_z80")]
impl FromStr for Z80Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Z80Architecture::*;

        Ok(match s {
            "z80" => Z80,
            "z180" => Z180,
            "ez80" => Ez80,
            "sm83" => Sm83,
            "rabbit2000" => Rabbit2000,
            "rabbit2000a" => Rabbit2000A,
            "rabbit3000" => Rabbit3000,
            "rabbit3000a" => Rabbit3000A,
            "tlcs90" => Tlcs90,
            "r800" => R800,
            _ => return Err(()),
        })
    }
}

impl FromStr for Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Architecture::*;

        Ok(match s {
            "unknown" => Unknown,
            "amdgcn" => AmdGcn,
            "asmjs" => Asmjs,
            "avr" => Avr,
            "bpfeb" => Bpfeb,
            "bpfel" => Bpfel,
            "hexagon" => Hexagon,
            "loongarch64" => LoongArch64,
            "m68k" => M68k,
            "msp430" => Msp430,
            "nvptx64" => Nvptx64,
            "pulley32" => Pulley32,
            "pulley64" => Pulley64,
            "pulley32be" => Pulley32be,
            "pulley64be" => Pulley64be,
            "powerpc" => Powerpc,
            "powerpc64" => Powerpc64,
            "powerpc64le" => Powerpc64le,
            "s390x" => S390x,
            "sparc" => Sparc,
            "sparc64" => Sparc64,
            "sparcv9" => Sparcv9,
            "wasm32" => Wasm32,
            "wasm64" => Wasm64,
            "x86_64" => X86_64,
            "x86_64h" => X86_64h,
            "xtensa" => XTensa,
            #[cfg(feature = "arch_zkasm")]
            "zkasm" => ZkAsm,
            _ => {
                if let Ok(arm) = ArmArchitecture::from_str(s) {
                    Arm(arm)
                } else if let Ok(aarch64) = Aarch64Architecture::from_str(s) {
                    Aarch64(aarch64)
                } else if let Ok(riscv32) = Riscv32Architecture::from_str(s) {
                    Riscv32(riscv32)
                } else if let Ok(riscv64) = Riscv64Architecture::from_str(s) {
                    Riscv64(riscv64)
                } else if let Ok(x86_32) = X86_32Architecture::from_str(s) {
                    X86_32(x86_32)
                } else if let Ok(mips32) = Mips32Architecture::from_str(s) {
                    Mips32(mips32)
                } else if let Ok(mips64) = Mips64Architecture::from_str(s) {
                    Mips64(mips64)
                } else if let Ok(clever) = CleverArchitecture::from_str(s) {
                    Clever(clever)
                } else {
                    #[cfg(feature = "arch_z80")]
                    {
                        if let Ok(z80) = Z80Architecture::from_str(s) {
                            return Ok(Architecture::Z80(z80));
                        }
                    }
                    return Err(());
                }
            }
        })
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Vendor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Vendor::*;

        Ok(match s {
            "unknown" => Unknown,
            "amd" => Amd,
            "apple" => Apple,
            "espressif" => Espressif,
            "experimental" => Experimental,
            "fortanix" => Fortanix,
            "ibm" => Ibm,
            "kmc" => Kmc,
            "nintendo" => Nintendo,
            "nvidia" => Nvidia,
            "pc" => Pc,
            "rumprun" => Rumprun,
            "sun" => Sun,
            "uwp" => Uwp,
            "wrs" => Wrs,
            custom => {
                #[cfg(not(feature = "std"))]
                use alloc::borrow::ToOwned;

                // A custom vendor. Since triple syntax is so loosely defined,
                // be as conservative as we can to avoid potential ambiguities.
                // We err on the side of being too strict here, as we can
                // always relax it if needed.

                // Don't allow empty string names.
                if custom.is_empty() {
                    return Err(());
                }

                // Don't allow any other recognized name as a custom vendor,
                // since vendors can be omitted in some contexts.
                if Architecture::from_str(custom).is_ok()
                    || OperatingSystem::from_str(custom).is_ok()
                    || Environment::from_str(custom).is_ok()
                    || BinaryFormat::from_str(custom).is_ok()
                {
                    return Err(());
                }

                // Require the first character to be an ascii lowercase.
                if !custom.chars().next().unwrap().is_ascii_lowercase() {
                    return Err(());
                }

                // Restrict the set of characters permitted in a custom vendor.
                let has_restricted = custom.chars().any(|c: char| {
                    !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
                });

                if has_restricted {
                    return Err(());
                }

                Custom(CustomVendor::Owned(Box::new(custom.to_owned())))
            }
        })
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use OperatingSystem::*;

        let mut with_version = |name, deployment_target| {
            if let Some(DeploymentTarget {
                major,
                minor,
                patch,
            }) = deployment_target
            {
                write!(f, "{}{}.{}.{}", name, major, minor, patch)
            } else {
                write!(f, "{}", name)
            }
        };

        match *self {
            Darwin(deployment_target) => with_version("darwin", deployment_target),
            IOS(deployment_target) => with_version("ios", deployment_target),
            MacOSX(deployment_target) => with_version("macosx", deployment_target),
            TvOS(deployment_target) => with_version("tvos", deployment_target),
            VisionOS(deployment_target) => with_version("visionos", deployment_target),
            WatchOS(deployment_target) => with_version("watchos", deployment_target),
            XROS(deployment_target) => with_version("xros", deployment_target),
            os => f.write_str(&os.into_str()),
        }
    }
}

impl FromStr for OperatingSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use OperatingSystem::*;

        let parse_darwin = |name: &str| {
            let s = &s[name.len()..];
            let mut parts = s.split('.');

            if s.is_empty() {
                // Not specifying a version is allowed!
                return Ok(None);
            }

            let major = if let Some(part) = parts.next() {
                part.parse().map_err(|_| ())?
            } else {
                // If the string was just `.`, with no major version, that's
                // clearly an error.
                return Err(());
            };
            let minor = if let Some(part) = parts.next() {
                part.parse().map_err(|_| ())?
            } else {
                // Fall back to 0 if no minor version was set
                0
            };
            let patch = if let Some(part) = parts.next() {
                part.parse().map_err(|_| ())?
            } else {
                // Fall back to 0 if no patch version was set
                0
            };

            if parts.next().is_some() {
                // Too many parts
                return Err(());
            }

            Ok(Some(DeploymentTarget {
                major,
                minor,
                patch,
            }))
        };

        // Parse operating system names that contain a version, like `macosx10.7.0`.
        if s.starts_with("darwin") {
            return Ok(Darwin(parse_darwin("darwin")?));
        }
        if s.starts_with("ios") {
            return Ok(IOS(parse_darwin("ios")?));
        }
        if s.starts_with("macosx") {
            return Ok(MacOSX(parse_darwin("macosx")?));
        }
        if s.starts_with("tvos") {
            return Ok(TvOS(parse_darwin("tvos")?));
        }
        if s.starts_with("visionos") {
            return Ok(VisionOS(parse_darwin("visionos")?));
        }
        if s.starts_with("watchos") {
            return Ok(WatchOS(parse_darwin("watchos")?));
        }
        if s.starts_with("xros") {
            return Ok(XROS(parse_darwin("xros")?));
        }

        Ok(match s {
            "unknown" => Unknown,
            "aix" => Aix,
            "amdhsa" => AmdHsa,
            "bitrig" => Bitrig,
            "cloudabi" => Cloudabi,
            "cuda" => Cuda,
            "cygwin" => Cygwin,
            "dragonfly" => Dragonfly,
            "emscripten" => Emscripten,
            "freebsd" => Freebsd,
            "fuchsia" => Fuchsia,
            "haiku" => Haiku,
            "hermit" => Hermit,
            "horizon" => Horizon,
            "hurd" => Hurd,
            "illumos" => Illumos,
            "l4re" => L4re,
            "linux" => Linux,
            "nebulet" => Nebulet,
            "netbsd" => Netbsd,
            "none" => None_,
            "openbsd" => Openbsd,
            "psp" => Psp,
            "redox" => Redox,
            "solaris" => Solaris,
            "solid_asp3" => SolidAsp3,
            "uefi" => Uefi,
            "vxworks" => VxWorks,
            "wasi" => Wasi,
            "wasip1" => WasiP1,
            "wasip2" => WasiP2,
            "windows" => Windows,
            "espidf" => Espidf,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Environment::*;

        Ok(match s {
            "unknown" => Unknown,
            "amdgiz" => AmdGiz,
            "android" => Android,
            "androideabi" => Androideabi,
            "eabi" => Eabi,
            "eabihf" => Eabihf,
            "gnu" => Gnu,
            "gnuabi64" => Gnuabi64,
            "gnueabi" => Gnueabi,
            "gnueabihf" => Gnueabihf,
            "gnuspe" => Gnuspe,
            "gnux32" => Gnux32,
            "gnu_ilp32" => GnuIlp32,
            "gnullvm" => GnuLlvm,
            "hermitkernel" => HermitKernel,
            "hurdkernel" => HurdKernel,
            "linuxkernel" => LinuxKernel,
            "macabi" => Macabi,
            "musl" => Musl,
            "musleabi" => Musleabi,
            "musleabihf" => Musleabihf,
            "muslabi64" => Muslabi64,
            "msvc" => Msvc,
            "newlib" => Newlib,
            "none" => None,
            "kernel" => Kernel,
            "uclibc" => Uclibc,
            "uclibceabi" => Uclibceabi,
            "uclibceabihf" => Uclibceabihf,
            "sgx" => Sgx,
            "sim" => Sim,
            "softfloat" => Softfloat,
            "spe" => Spe,
            "threads" => Threads,
            "ohos" => Ohos,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for BinaryFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.into_str())
    }
}

impl FromStr for BinaryFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use BinaryFormat::*;

        Ok(match s {
            "unknown" => Unknown,
            "elf" => Elf,
            "coff" => Coff,
            "macho" => Macho,
            "wasm" => Wasm,
            "xcoff" => Xcoff,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn roundtrip_known_triples() {
        // This list is constructed from:
        //  - targets emitted by "rustup target list"
        //  - targets emitted by "rustc +nightly --print target-list"
        //  - targets contributors have added
        let targets = [
            "aarch64-apple-darwin",
            "aarch64-apple-ios",
            "aarch64-apple-ios-macabi",
            "aarch64-apple-ios-sim",
            "aarch64-apple-tvos",
            "aarch64-apple-tvos-sim",
            "aarch64-apple-visionos",
            "aarch64-apple-visionos-sim",
            "aarch64-apple-watchos",
            "aarch64-apple-watchos-sim",
            "aarch64_be-unknown-linux-gnu",
            "aarch64_be-unknown-linux-gnu_ilp32",
            "aarch64_be-unknown-netbsd",
            "aarch64-kmc-solid_asp3",
            "aarch64-linux-android",
            //"aarch64-nintendo-switch-freestanding", // TODO
            "aarch64-pc-windows-gnullvm",
            "aarch64-pc-windows-msvc",
            "aarch64-unknown-cloudabi",
            "aarch64-unknown-freebsd",
            "aarch64-unknown-fuchsia",
            "aarch64-unknown-hermit",
            "aarch64-unknown-illumos",
            "aarch64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu_ilp32",
            "aarch64-unknown-linux-musl",
            "aarch64-unknown-linux-ohos",
            "aarch64-unknown-netbsd",
            "aarch64-unknown-none",
            "aarch64-unknown-none-softfloat",
            //"aarch64-unknown-nto-qnx710", // TODO
            "aarch64-unknown-openbsd",
            "aarch64-unknown-redox",
            //"aarch64-unknown-teeos", // TODO
            "aarch64-unknown-uefi",
            "aarch64-uwp-windows-msvc",
            "aarch64-wrs-vxworks",
            //"arm64_32-apple-watchos", // TODO
            //"arm64e-apple-darwin", // TODO
            "amdgcn-amd-amdhsa",
            "amdgcn-amd-amdhsa-amdgiz",
            //"arm64e-apple-ios", // TODO
            //"arm64ec-pc-windows-msvc", // TODO
            "armeb-unknown-linux-gnueabi",
            "armebv7r-none-eabi",
            "armebv7r-none-eabihf",
            "arm-linux-androideabi",
            "arm-unknown-linux-gnueabi",
            "arm-unknown-linux-gnueabihf",
            "arm-unknown-linux-musleabi",
            "arm-unknown-linux-musleabihf",
            "armv4t-none-eabi",
            "armv4t-unknown-linux-gnueabi",
            "armv5te-none-eabi",
            "armv5te-unknown-linux-gnueabi",
            "armv5te-unknown-linux-musleabi",
            "armv5te-unknown-linux-uclibceabi",
            "armv6k-nintendo-3ds",
            "armv6-unknown-freebsd",
            "armv6-unknown-netbsd-eabihf",
            "armv7a-kmc-solid_asp3-eabi",
            "armv7a-kmc-solid_asp3-eabihf",
            "armv7a-none-eabi",
            "armv7a-none-eabihf",
            "armv7-apple-ios",
            "armv7k-apple-watchos",
            "armv7-linux-androideabi",
            "armv7r-none-eabi",
            "armv7r-none-eabihf",
            "armv7s-apple-ios",
            "armv7-unknown-cloudabi-eabihf",
            //"armv7-sony-vita-newlibeabihf", // TODO
            "armv7-unknown-freebsd",
            "armv7-unknown-linux-gnueabi",
            "armv7-unknown-linux-gnueabihf",
            "armv7-unknown-linux-musleabi",
            "armv7-unknown-linux-musleabihf",
            "armv7-unknown-linux-ohos",
            "armv7-unknown-linux-uclibceabi",
            "armv7-unknown-linux-uclibceabihf",
            "armv7-unknown-netbsd-eabihf",
            "armv7-wrs-vxworks-eabihf",
            "asmjs-unknown-emscripten",
            "armv8r-none-eabihf",
            //"avr-unknown-gnu-atmega328", // TODO
            "avr-unknown-unknown",
            "bpfeb-unknown-none",
            "bpfel-unknown-none",
            //"csky-unknown-linux-gnuabiv2", // TODO
            //"csky-unknown-linux-gnuabiv2hf", // TODO
            "hexagon-unknown-linux-musl",
            "hexagon-unknown-none-elf",
            "i386-apple-ios",
            //"i586-pc-nto-qnx700", // TODO
            "i586-pc-windows-msvc",
            "i586-unknown-linux-gnu",
            "i586-unknown-linux-musl",
            "i586-unknown-netbsd",
            "i686-apple-darwin",
            "i686-linux-android",
            "i686-apple-macosx10.7.0",
            "i686-pc-windows-gnu",
            "i686-pc-windows-gnullvm",
            "i686-pc-windows-msvc",
            "i686-unknown-cloudabi",
            "i686-unknown-dragonfly",
            "i686-unknown-freebsd",
            "i686-unknown-haiku",
            "i686-unknown-hurd-gnu",
            "i686-unknown-linux-gnu",
            "i686-unknown-linux-musl",
            "i686-unknown-netbsd",
            "i686-unknown-openbsd",
            "i686-unknown-redox",
            "i686-unknown-uefi",
            "i686-uwp-windows-gnu",
            "i686-uwp-windows-msvc",
            "i686-win7-windows-msvc",
            "i686-wrs-vxworks",
            "loongarch64-unknown-linux-gnu",
            "loongarch64-unknown-linux-musl",
            "loongarch64-unknown-none",
            "loongarch64-unknown-none-softfloat",
            "m68k-unknown-linux-gnu",
            "mips64el-unknown-linux-gnuabi64",
            "mips64el-unknown-linux-muslabi64",
            "mips64-openwrt-linux-musl",
            "mips64-unknown-linux-gnuabi64",
            "mips64-unknown-linux-muslabi64",
            "mipsel-sony-psp",
            //"mipsel-sony-psx", // TODO
            "mipsel-unknown-linux-gnu",
            "mipsel-unknown-linux-musl",
            "mipsel-unknown-linux-uclibc",
            "mipsel-unknown-netbsd",
            "mipsel-unknown-none",
            "mipsisa32r6el-unknown-linux-gnu",
            "mipsisa32r6-unknown-linux-gnu",
            "mipsisa64r6el-unknown-linux-gnuabi64",
            "mipsisa64r6-unknown-linux-gnuabi64",
            "mips-unknown-linux-gnu",
            "mips-unknown-linux-musl",
            "mips-unknown-linux-uclibc",
            "msp430-none-elf",
            "nvptx64-nvidia-cuda",
            "powerpc64-ibm-aix",
            "powerpc64le-unknown-freebsd",
            "powerpc64le-unknown-linux-gnu",
            "powerpc64le-unknown-linux-musl",
            "powerpc64-unknown-freebsd",
            "powerpc64-unknown-linux-gnu",
            "powerpc64-unknown-linux-musl",
            "powerpc64-unknown-openbsd",
            "powerpc64-wrs-vxworks",
            "powerpc-ibm-aix",
            "powerpc-unknown-freebsd",
            "powerpc-unknown-linux-gnu",
            "powerpc-unknown-linux-gnuspe",
            "powerpc-unknown-linux-musl",
            "powerpc-unknown-netbsd",
            "powerpc-unknown-openbsd",
            "powerpc-wrs-vxworks",
            "powerpc-wrs-vxworks-spe",
            "riscv32gc-unknown-linux-gnu",
            "riscv32gc-unknown-linux-musl",
            "riscv32imac-esp-espidf",
            "riscv32imac-unknown-none-elf",
            //"riscv32imac-unknown-xous-elf", // TODO
            "riscv32imafc-esp-espidf",
            "riscv32imafc-unknown-none-elf",
            "riscv32ima-unknown-none-elf",
            "riscv32imc-esp-espidf",
            "riscv32imc-unknown-none-elf",
            //"riscv32im-risc0-zkvm-elf", // TODO
            "riscv32im-unknown-none-elf",
            "riscv32i-unknown-none-elf",
            "riscv64gc-unknown-freebsd",
            "riscv64gc-unknown-fuchsia",
            "riscv64gc-unknown-hermit",
            "riscv64gc-unknown-linux-gnu",
            "riscv64a23-unknown-linux-gnu",
            "riscv64gc-unknown-linux-musl",
            "riscv64gc-unknown-netbsd",
            "riscv64gc-unknown-none-elf",
            "riscv64gc-unknown-openbsd",
            "riscv64imac-unknown-none-elf",
            "riscv64-linux-android",
            "s390x-unknown-linux-gnu",
            "s390x-unknown-linux-musl",
            "sparc64-unknown-linux-gnu",
            "sparc64-unknown-netbsd",
            "sparc64-unknown-openbsd",
            "sparc-unknown-linux-gnu",
            "sparc-unknown-none-elf",
            "sparcv9-sun-solaris",
            "thumbv4t-none-eabi",
            "thumbv5te-none-eabi",
            "thumbv6m-none-eabi",
            "thumbv7a-pc-windows-msvc",
            "thumbv7a-uwp-windows-msvc",
            "thumbv7em-none-eabi",
            "thumbv7em-none-eabihf",
            "thumbv7m-none-eabi",
            "thumbv7neon-linux-androideabi",
            "thumbv7neon-unknown-linux-gnueabihf",
            "thumbv7neon-unknown-linux-musleabihf",
            "thumbv8m.base-none-eabi",
            "thumbv8m.main-none-eabi",
            "thumbv8m.main-none-eabihf",
            "wasm32-experimental-emscripten",
            "wasm32-unknown-emscripten",
            "wasm32-unknown-unknown",
            "wasm32-wasi",
            "wasm32-wasip1",
            "wasm32-wasip1-threads",
            "wasm32-wasip2",
            "wasm64-unknown-unknown",
            "wasm64-wasi",
            "x86_64-apple-darwin",
            "x86_64-apple-darwin23.6.0",
            "x86_64-apple-ios",
            "x86_64-apple-ios-macabi",
            "x86_64-apple-tvos",
            "x86_64-apple-watchos-sim",
            "x86_64-fortanix-unknown-sgx",
            "x86_64h-apple-darwin",
            "x86_64-linux-android",
            //"x86_64-pc-nto-qnx710", // TODO
            "x86_64-linux-kernel", // Changed to x86_64-unknown-none-linuxkernel in 1.53.0
            "x86_64-apple-macosx",
            "x86_64-apple-macosx10.7.0",
            "x86_64-pc-cygwin",
            "x86_64-pc-solaris",
            "x86_64-pc-windows-gnu",
            "x86_64-pc-windows-gnullvm",
            "x86_64-pc-windows-msvc",
            "x86_64-rumprun-netbsd", // Removed in 1.53.0
            "x86_64-sun-solaris",
            "x86_64-unknown-bitrig",
            "x86_64-unknown-cloudabi",
            "x86_64-unikraft-linux-musl",
            "x86_64-unknown-dragonfly",
            "x86_64-unknown-freebsd",
            "x86_64-unknown-fuchsia",
            "x86_64-unknown-haiku",
            "x86_64-unknown-hermit-kernel", // Changed to x86_64-unknown-none-hermitkernel in 1.53.0
            "x86_64-unknown-hermit",
            "x86_64-unknown-illumos",
            "x86_64-unknown-l4re-uclibc",
            "x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-gnux32",
            "x86_64-unknown-linux-musl",
            "x86_64-unknown-linux-none",
            "x86_64-unknown-linux-ohos",
            "x86_64-unknown-netbsd",
            "x86_64-unknown-none",
            "x86_64-unknown-none-hermitkernel",
            "x86_64-unknown-none-linuxkernel",
            "x86_64-unknown-openbsd",
            "x86_64-unknown-redox",
            "x86_64-unknown-uefi",
            "x86_64-uwp-windows-gnu",
            "x86_64-uwp-windows-msvc",
            "x86_64-win7-windows-msvc",
            "x86_64-wrs-vxworks",
            "xtensa-esp32-espidf",
            "clever-unknown-elf",
            "xtensa-esp32-none-elf",
            "xtensa-esp32s2-espidf",
            "xtensa-esp32s2-none-elf",
            "xtensa-esp32s3-espidf",
            "xtensa-esp32s3-none-elf",
            #[cfg(feature = "arch_zkasm")]
            "zkasm-unknown-unknown",
            #[cfg(feature = "arch_z80")]
            "z80-zilog-none",
            #[cfg(feature = "arch_z80")]
            "sm83-nintendo-none",
            #[cfg(feature = "arch_z80")]
            "tlcs90-toshiba-none",
        ];

        for target in targets.iter() {
            let t = Triple::from_str(target).expect("can't parse target");
            assert_ne!(t.architecture, Architecture::Unknown);
            assert_eq!(t.to_string(), *target, "{:#?}", t);
        }
    }

    #[test]
    fn default_format_to_elf() {
        let t = Triple::from_str("riscv64").expect("can't parse target");
        assert_eq!(
            t.architecture,
            Architecture::Riscv64(Riscv64Architecture::Riscv64),
        );
        assert_eq!(t.vendor, Vendor::Unknown);
        assert_eq!(t.operating_system, OperatingSystem::Unknown);
        assert_eq!(t.environment, Environment::Unknown);
        assert_eq!(t.binary_format, BinaryFormat::Elf);
    }

    #[test]
    fn thumbv7em_none_eabihf() {
        let t = Triple::from_str("thumbv7em-none-eabihf").expect("can't parse target");
        assert_eq!(
            t.architecture,
            Architecture::Arm(ArmArchitecture::Thumbv7em)
        );
        assert_eq!(t.vendor, Vendor::Unknown);
        assert_eq!(t.operating_system, OperatingSystem::None_);
        assert_eq!(t.environment, Environment::Eabihf);
        assert_eq!(t.binary_format, BinaryFormat::Elf);
    }

    #[test]
    fn fuchsia_rename() {
        // Fuchsia targets were renamed to add the `unknown`.
        assert_eq!(
            Triple::from_str("aarch64-fuchsia"),
            Triple::from_str("aarch64-unknown-fuchsia")
        );
        assert_eq!(
            Triple::from_str("x86_64-fuchsia"),
            Triple::from_str("x86_64-unknown-fuchsia")
        );
    }

    #[test]
    fn custom_vendors() {
        // Test various invalid cases.
        assert!(Triple::from_str("x86_64--linux").is_err());
        assert!(Triple::from_str("x86_64-42-linux").is_err());
        assert!(Triple::from_str("x86_64-__customvendor__-linux").is_err());
        assert!(Triple::from_str("x86_64-^-linux").is_err());
        assert!(Triple::from_str("x86_64- -linux").is_err());
        assert!(Triple::from_str("x86_64-CustomVendor-linux").is_err());
        assert!(Triple::from_str("x86_64-linux-linux").is_err());
        assert!(Triple::from_str("x86_64-x86_64-linux").is_err());
        assert!(Triple::from_str("x86_64-elf-linux").is_err());
        assert!(Triple::from_str("x86_64-gnu-linux").is_err());
        assert!(Triple::from_str("x86_64-linux-customvendor").is_err());
        assert!(Triple::from_str("customvendor").is_err());
        assert!(Triple::from_str("customvendor-x86_64").is_err());
        assert!(Triple::from_str("x86_64-").is_err());
        assert!(Triple::from_str("x86_64--").is_err());

        // Test various Unicode things.
        assert!(
            Triple::from_str("x86_64-𝓬𝓾𝓼𝓽𝓸𝓶𝓿𝓮𝓷𝓭𝓸𝓻-linux").is_err(),
            "unicode font hazard"
        );
        assert!(
            Triple::from_str("x86_64-ćúśtőḿvéńdőŕ-linux").is_err(),
            "diacritical mark stripping hazard"
        );
        assert!(
            Triple::from_str("x86_64-customvendοr-linux").is_err(),
            "homoglyph hazard"
        );
        assert!(Triple::from_str("x86_64-customvendor-linux").is_ok());
        assert!(
            Triple::from_str("x86_64-ﬃ-linux").is_err(),
            "normalization hazard"
        );
        assert!(Triple::from_str("x86_64-ffi-linux").is_ok());
        assert!(
            Triple::from_str("x86_64-custom‍vendor-linux").is_err(),
            "zero-width character hazard"
        );
        assert!(
            Triple::from_str("x86_64-﻿customvendor-linux").is_err(),
            "BOM hazard"
        );

        // Test some valid cases.
        let t = Triple::from_str("x86_64-customvendor-linux")
            .expect("can't parse target with custom vendor");
        assert_eq!(t.architecture, Architecture::X86_64);
        assert_eq!(
            t.vendor,
            Vendor::Custom(CustomVendor::Static("customvendor"))
        );
        assert_eq!(t.operating_system, OperatingSystem::Linux);
        assert_eq!(t.environment, Environment::Unknown);
        assert_eq!(t.binary_format, BinaryFormat::Elf);
        assert_eq!(t.to_string(), "x86_64-customvendor-linux");

        let t =
            Triple::from_str("x86_64-customvendor").expect("can't parse target with custom vendor");
        assert_eq!(t.architecture, Architecture::X86_64);
        assert_eq!(
            t.vendor,
            Vendor::Custom(CustomVendor::Static("customvendor"))
        );
        assert_eq!(t.operating_system, OperatingSystem::Unknown);
        assert_eq!(t.environment, Environment::Unknown);
        assert_eq!(t.binary_format, BinaryFormat::Elf);

        assert_eq!(
            Triple::from_str("unknown-foo"),
            Ok(Triple {
                architecture: Architecture::Unknown,
                vendor: Vendor::Custom(CustomVendor::Static("foo")),
                operating_system: OperatingSystem::Unknown,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Unknown,
            })
        );
    }

    #[test]
    fn deployment_version_parsing() {
        assert_eq!(
            Triple::from_str("aarch64-apple-macosx"),
            Ok(Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Apple,
                operating_system: OperatingSystem::MacOSX(None),
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Macho,
            })
        );

        assert_eq!(
            Triple::from_str("aarch64-apple-macosx10.14.6"),
            Ok(Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Apple,
                operating_system: OperatingSystem::MacOSX(Some(DeploymentTarget {
                    major: 10,
                    minor: 14,
                    patch: 6,
                })),
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Macho,
            })
        );

        let expected = Triple {
            architecture: Architecture::X86_64,
            vendor: Vendor::Apple,
            operating_system: OperatingSystem::Darwin(Some(DeploymentTarget {
                major: 23,
                minor: 0,
                patch: 0,
            })),
            environment: Environment::Unknown,
            binary_format: BinaryFormat::Macho,
        };
        assert_eq!(
            Triple::from_str("x86_64-apple-darwin23"),
            Ok(expected.clone())
        );
        assert_eq!(
            Triple::from_str("x86_64-apple-darwin23.0"),
            Ok(expected.clone())
        );
        assert_eq!(Triple::from_str("x86_64-apple-darwin23.0.0"), Ok(expected));

        assert!(Triple::from_str("x86_64-apple-darwin.").is_err());
        assert!(Triple::from_str("x86_64-apple-darwin23.0.0.0").is_err());
    }
}
