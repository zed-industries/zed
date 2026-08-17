// This file defines the `Triple` type and support code shared by all targets.

use crate::data_model::CDataModel;
use crate::parse_error::ParseError;
use crate::targets::{
    default_binary_format, Architecture, ArmArchitecture, BinaryFormat, Environment,
    OperatingSystem, Riscv32Architecture, Vendor,
};
#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
use core::fmt;
use core::str::FromStr;

/// The target memory endianness.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Endianness {
    Little,
    Big,
}

/// The width of a pointer (in the default address space).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum PointerWidth {
    U16,
    U32,
    U64,
}

impl PointerWidth {
    /// Return the number of bits in a pointer.
    pub fn bits(self) -> u8 {
        match self {
            PointerWidth::U16 => 16,
            PointerWidth::U32 => 32,
            PointerWidth::U64 => 64,
        }
    }

    /// Return the number of bytes in a pointer.
    ///
    /// For these purposes, there are 8 bits in a byte.
    pub fn bytes(self) -> u8 {
        match self {
            PointerWidth::U16 => 2,
            PointerWidth::U32 => 4,
            PointerWidth::U64 => 8,
        }
    }
}

/// The calling convention, which specifies things like which registers are
/// used for passing arguments, which registers are callee-saved, and so on.
#[cfg_attr(feature = "rust_1_40", non_exhaustive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallingConvention {
    /// "System V", which is used on most Unix-like platfoms. Note that the
    /// specific conventions vary between hardware architectures; for example,
    /// x86-32's "System V" is entirely different from x86-64's "System V".
    SystemV,

    /// The WebAssembly C ABI.
    /// https://github.com/WebAssembly/tool-conventions/blob/master/BasicCABI.md
    WasmBasicCAbi,

    /// "Windows Fastcall", which is used on Windows. Note that like "System V",
    /// this varies between hardware architectures. On x86-32 it describes what
    /// Windows documentation calls "fastcall", and on x86-64 it describes what
    /// Windows documentation often just calls the Windows x64 calling convention
    /// (though the compiler still recognizes "fastcall" as an alias for it).
    WindowsFastcall,

    /// Apple Aarch64 platforms use their own variant of the common Aarch64
    /// calling convention.
    ///
    /// <https://developer.apple.com/documentation/xcode/writing_arm64_code_for_apple_platforms>
    AppleAarch64,
}

/// An LLVM target "triple". Historically such things had three fields, though
/// they've added additional fields over time.
///
/// Note that `Triple` doesn't implement `Default` itself. If you want a type
/// which defaults to the host triple, or defaults to unknown-unknown-unknown,
/// use `DefaultToHost` or `DefaultToUnknown`, respectively.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Triple {
    /// The "architecture" (and sometimes the subarchitecture).
    pub architecture: Architecture,
    /// The "vendor" (whatever that means).
    pub vendor: Vendor,
    /// The "operating system" (sometimes also the environment).
    pub operating_system: OperatingSystem,
    /// The "environment" on top of the operating system (often omitted for
    /// operating systems with a single predominant environment).
    pub environment: Environment,
    /// The "binary format" (rarely used).
    pub binary_format: BinaryFormat,
}

impl Triple {
    /// Return the endianness of this target's architecture.
    pub fn endianness(&self) -> Result<Endianness, ()> {
        self.architecture.endianness()
    }

    /// Return the pointer width of this target's architecture.
    ///
    /// This function is aware of x32 and ilp32 ABIs on 64-bit architectures.
    pub fn pointer_width(&self) -> Result<PointerWidth, ()> {
        // Some ABIs have a different pointer width than the CPU architecture.
        match self.environment {
            Environment::Gnux32 | Environment::GnuIlp32 => return Ok(PointerWidth::U32),
            _ => {}
        }

        self.architecture.pointer_width()
    }

    /// Return the default calling convention for the given target triple.
    pub fn default_calling_convention(&self) -> Result<CallingConvention, ()> {
        Ok(match self.operating_system {
            os if os.is_like_darwin() => match self.architecture {
                Architecture::Aarch64(_) => CallingConvention::AppleAarch64,
                _ => CallingConvention::SystemV,
            },
            OperatingSystem::Aix
            | OperatingSystem::Bitrig
            | OperatingSystem::Cloudabi
            | OperatingSystem::Dragonfly
            | OperatingSystem::Freebsd
            | OperatingSystem::Fuchsia
            | OperatingSystem::Haiku
            | OperatingSystem::Hermit
            | OperatingSystem::Hurd
            | OperatingSystem::L4re
            | OperatingSystem::Linux
            | OperatingSystem::Netbsd
            | OperatingSystem::Openbsd
            | OperatingSystem::Redox
            | OperatingSystem::Solaris => CallingConvention::SystemV,
            OperatingSystem::Windows => CallingConvention::WindowsFastcall,
            OperatingSystem::Nebulet
            | OperatingSystem::Emscripten
            | OperatingSystem::Wasi
            | OperatingSystem::Unknown => match self.architecture {
                Architecture::Wasm32 => CallingConvention::WasmBasicCAbi,
                _ => return Err(()),
            },
            _ => return Err(()),
        })
    }

    /// The C data model for a given target. If the model is not known, returns `Err(())`.
    pub fn data_model(&self) -> Result<CDataModel, ()> {
        match self.pointer_width()? {
            PointerWidth::U64 => {
                if self.operating_system == OperatingSystem::Windows {
                    Ok(CDataModel::LLP64)
                } else if self.default_calling_convention() == Ok(CallingConvention::SystemV)
                    || self.architecture == Architecture::Wasm64
                    || self.default_calling_convention() == Ok(CallingConvention::AppleAarch64)
                {
                    Ok(CDataModel::LP64)
                } else {
                    Err(())
                }
            }
            PointerWidth::U32 => {
                if self.operating_system == OperatingSystem::Windows
                    || self.default_calling_convention() == Ok(CallingConvention::SystemV)
                    || self.architecture == Architecture::Wasm32
                {
                    Ok(CDataModel::ILP32)
                } else {
                    Err(())
                }
            }
            // TODO: on 16-bit machines there is usually a distinction
            // between near-pointers and far-pointers.
            // Additionally, code pointers sometimes have a different size than data pointers.
            // We don't handle this case.
            PointerWidth::U16 => Err(()),
        }
    }

    /// Return a `Triple` with all unknown fields.
    pub fn unknown() -> Self {
        Self {
            architecture: Architecture::Unknown,
            vendor: Vendor::Unknown,
            operating_system: OperatingSystem::Unknown,
            environment: Environment::Unknown,
            binary_format: BinaryFormat::Unknown,
        }
    }
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(res) = self.special_case_display(f) {
            return res;
        }

        let implied_binary_format = default_binary_format(&self);

        write!(f, "{}", self.architecture)?;
        if self.vendor == Vendor::Unknown
            && (self.environment != Environment::HermitKernel
                && self.environment != Environment::LinuxKernel)
            && ((self.operating_system == OperatingSystem::Linux
                && (self.environment == Environment::Android
                    || self.environment == Environment::Androideabi
                    || self.environment == Environment::Kernel))
                || self.operating_system == OperatingSystem::Wasi
                || self.operating_system == OperatingSystem::WasiP1
                || self.operating_system == OperatingSystem::WasiP2
                || (self.operating_system == OperatingSystem::None_
                    && (self.architecture == Architecture::Arm(ArmArchitecture::Armv4t)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Armv5te)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Armebv7r)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Armv7a)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Armv7r)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Armv8r)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Thumbv4t)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Thumbv5te)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Thumbv6m)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Thumbv7em)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Thumbv7m)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Thumbv8mBase)
                        || self.architecture == Architecture::Arm(ArmArchitecture::Thumbv8mMain)
                        || self.architecture == Architecture::Msp430)))
        {
            // As a special case, omit the vendor for Android, Wasi, and sometimes
            // None_, depending on the hardware architecture. This logic is entirely
            // ad-hoc, and is just sufficient to handle the current set of recognized
            // triples.
            write!(f, "-{}", self.operating_system)?;
        } else if self.architecture.is_clever() && self.operating_system == OperatingSystem::Unknown
        {
            write!(f, "-{}", self.vendor)?;
        } else {
            write!(f, "-{}-{}", self.vendor, self.operating_system)?;
        }

        match (&self.vendor, self.operating_system, self.environment) {
            (Vendor::Nintendo, OperatingSystem::Horizon, Environment::Newlib)
            | (Vendor::Espressif, OperatingSystem::Espidf, Environment::Newlib) => {
                // The triple representations of these platforms don't have an environment field.
            }
            (_, _, Environment::Unknown) => {
                // Don't print out the environment if it is unknown.
            }
            _ => {
                write!(f, "-{}", self.environment)?;
            }
        }

        if self.binary_format != implied_binary_format || show_binary_format_with_no_os(self) {
            // As a special case, omit a non-default binary format for some
            // targets which happen to exclude it.
            write!(f, "-{}", self.binary_format)?;
        }
        Ok(())
    }
}

fn show_binary_format_with_no_os(triple: &Triple) -> bool {
    if triple.binary_format == BinaryFormat::Unknown {
        return false;
    }

    #[cfg(feature = "arch_zkasm")]
    {
        if triple.architecture == Architecture::ZkAsm {
            return false;
        }
    }

    triple.environment != Environment::Eabi
        && triple.environment != Environment::Eabihf
        && triple.environment != Environment::Sgx
        && triple.architecture != Architecture::Avr
        && triple.architecture != Architecture::Wasm32
        && triple.architecture != Architecture::Wasm64
        && (triple.operating_system == OperatingSystem::None_
            || triple.operating_system == OperatingSystem::Unknown)
}

impl FromStr for Triple {
    type Err = ParseError;

    /// Parse a triple from an LLVM target triple.
    ///
    /// This may also be able to parse `rustc` target triples, though support
    /// for that is secondary.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(triple) = Triple::special_case_from_str(s) {
            return Ok(triple);
        }

        let mut parts = s.split('-');
        let mut result = Self::unknown();
        let mut current_part;

        current_part = parts.next();
        if let Some(s) = current_part {
            if let Ok(architecture) = Architecture::from_str(s) {
                result.architecture = architecture;
                current_part = parts.next();
            } else {
                // Insist that the triple start with a valid architecture.
                return Err(ParseError::UnrecognizedArchitecture(s.to_owned()));
            }
        }

        let mut has_vendor = false;
        let mut has_operating_system = false;
        if let Some(s) = current_part {
            if let Ok(vendor) = Vendor::from_str(s) {
                has_vendor = true;
                result.vendor = vendor;
                current_part = parts.next();
            }
        }

        if !has_operating_system {
            if let Some(s) = current_part {
                if let Ok(operating_system) = OperatingSystem::from_str(s) {
                    has_operating_system = true;
                    result.operating_system = operating_system;
                    current_part = parts.next();
                }
            }
        }

        let mut has_environment = false;

        if !has_environment {
            if let Some(s) = current_part {
                if let Ok(environment) = Environment::from_str(s) {
                    has_environment = true;
                    result.environment = environment;
                    current_part = parts.next();
                }
            }
        }

        let mut has_binary_format = false;
        if let Some(s) = current_part {
            if let Ok(binary_format) = BinaryFormat::from_str(s) {
                has_binary_format = true;
                result.binary_format = binary_format;
                current_part = parts.next();
            }
        }

        // The binary format is frequently omitted; if that's the case here,
        // infer it from the other fields.
        if !has_binary_format {
            result.binary_format = default_binary_format(&result);
        }

        if let Some(s) = current_part {
            Err(
                if !has_vendor && !has_operating_system && !has_environment && !has_binary_format {
                    ParseError::UnrecognizedVendor(s.to_owned())
                } else if !has_operating_system && !has_environment && !has_binary_format {
                    ParseError::UnrecognizedOperatingSystem(s.to_owned())
                } else if !has_environment && !has_binary_format {
                    ParseError::UnrecognizedEnvironment(s.to_owned())
                } else if !has_binary_format {
                    ParseError::UnrecognizedBinaryFormat(s.to_owned())
                } else {
                    ParseError::UnrecognizedField(s.to_owned())
                },
            )
        } else {
            Ok(result)
        }
    }
}

impl Triple {
    /// Handle special cases in the `Display` implementation.
    fn special_case_display(&self, f: &mut fmt::Formatter) -> Option<fmt::Result> {
        let res = match self {
            Triple {
                architecture: Architecture::Arm(ArmArchitecture::Armv6k),
                vendor: Vendor::Nintendo,
                operating_system: OperatingSystem::Horizon,
                ..
            } => write!(f, "{}-{}-3ds", self.architecture, self.vendor),
            Triple {
                architecture: Architecture::Riscv32(Riscv32Architecture::Riscv32imc),
                vendor: Vendor::Espressif,
                operating_system: OperatingSystem::Espidf,
                ..
            } => write!(f, "{}-esp-{}", self.architecture, self.operating_system),
            _ => return None,
        };
        Some(res)
    }

    /// Handle special cases in the `FromStr` implementation.
    fn special_case_from_str(s: &str) -> Option<Self> {
        let mut triple = Triple::unknown();

        match s {
            "armv6k-nintendo-3ds" => {
                triple.architecture = Architecture::Arm(ArmArchitecture::Armv6k);
                triple.vendor = Vendor::Nintendo;
                triple.operating_system = OperatingSystem::Horizon;
                triple.environment = Environment::Newlib;
                triple.binary_format = default_binary_format(&triple);
            }
            "riscv32imc-esp-espidf" => {
                triple.architecture = Architecture::Riscv32(Riscv32Architecture::Riscv32imc);
                triple.vendor = Vendor::Espressif;
                triple.operating_system = OperatingSystem::Espidf;
                triple.environment = Environment::Newlib;
                triple.binary_format = default_binary_format(&triple);
            }
            _ => return None,
        }

        Some(triple)
    }
}

/// A convenient syntax for triple literals.
///
/// This currently expands to code that just calls `Triple::from_str` and does
/// an `expect`, though in the future it would be cool to use procedural macros
/// or so to report errors at compile time instead.
#[macro_export]
macro_rules! triple {
    ($str:tt) => {
        <$crate::Triple as core::str::FromStr>::from_str($str).expect("invalid triple literal")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_errors() {
        assert_eq!(
            Triple::from_str(""),
            Err(ParseError::UnrecognizedArchitecture("".to_owned()))
        );
        assert_eq!(
            Triple::from_str("foo"),
            Err(ParseError::UnrecognizedArchitecture("foo".to_owned()))
        );
        assert_eq!(
            Triple::from_str("unknown-unknown-foo"),
            Err(ParseError::UnrecognizedOperatingSystem("foo".to_owned()))
        );
        assert_eq!(
            Triple::from_str("unknown-unknown-unknown-foo"),
            Err(ParseError::UnrecognizedEnvironment("foo".to_owned()))
        );
        assert_eq!(
            Triple::from_str("unknown-unknown-unknown-unknown-foo"),
            Err(ParseError::UnrecognizedBinaryFormat("foo".to_owned()))
        );
        assert_eq!(
            Triple::from_str("unknown-unknown-unknown-unknown-unknown-foo"),
            Err(ParseError::UnrecognizedField("foo".to_owned()))
        );
    }

    #[test]
    fn defaults() {
        assert_eq!(
            Triple::from_str("unknown-unknown-unknown"),
            Ok(Triple::unknown())
        );
        assert_eq!(
            Triple::from_str("unknown-unknown-unknown-unknown"),
            Ok(Triple::unknown())
        );
        assert_eq!(
            Triple::from_str("unknown-unknown-unknown-unknown-unknown"),
            Ok(Triple::unknown())
        );
    }

    #[test]
    fn unknown_properties() {
        assert_eq!(Triple::unknown().endianness(), Err(()));
        assert_eq!(Triple::unknown().pointer_width(), Err(()));
        assert_eq!(Triple::unknown().default_calling_convention(), Err(()));
    }

    #[test]
    fn apple_calling_convention() {
        for triple in &[
            "aarch64-apple-darwin",
            "aarch64-apple-ios",
            "aarch64-apple-ios-macabi",
            "aarch64-apple-tvos",
            "aarch64-apple-watchos",
        ] {
            let triple = Triple::from_str(triple).unwrap();
            assert_eq!(
                triple.default_calling_convention().unwrap(),
                CallingConvention::AppleAarch64
            );
            assert_eq!(triple.data_model().unwrap(), CDataModel::LP64);
        }

        for triple in &["aarch64-linux-android", "x86_64-apple-ios"] {
            assert_eq!(
                Triple::from_str(triple)
                    .unwrap()
                    .default_calling_convention()
                    .unwrap(),
                CallingConvention::SystemV
            );
        }
    }

    #[test]
    fn p32_abi() {
        // Test that special 32-bit pointer ABIs on 64-bit architectures are
        // reported as having 32-bit pointers.
        for triple in &[
            "x86_64-unknown-linux-gnux32",
            "aarch64_be-unknown-linux-gnu_ilp32",
            "aarch64-unknown-linux-gnu_ilp32",
        ] {
            assert_eq!(
                Triple::from_str(triple).unwrap().pointer_width().unwrap(),
                PointerWidth::U32
            );
        }

        // Test that the corresponding ABIs have 64-bit pointers.
        for triple in &[
            "x86_64-unknown-linux-gnu",
            "aarch64_be-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ] {
            assert_eq!(
                Triple::from_str(triple).unwrap().pointer_width().unwrap(),
                PointerWidth::U64
            );
        }
    }
}
