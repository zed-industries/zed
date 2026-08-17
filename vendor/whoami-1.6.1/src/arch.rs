use std::{
    fmt::{self, Display, Formatter},
    io::{Error, ErrorKind},
};

use crate::Result;

/// The address width of a CPU architecture
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum Width {
    /// 32 bits
    Bits32,
    /// 64 bits
    Bits64,
}

impl Display for Width {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Width::Bits32 => "32 bits",
            Width::Bits64 => "64 bits",
        })
    }
}

/// The architecture of a CPU
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Arch {
    /// ARMv5
    ArmV5,
    /// ARMv6 (Sometimes just referred to as ARM)
    ArmV6,
    /// ARMv7 (May or may not support Neon/Thumb)
    ArmV7,
    /// ARM64 (aarch64)
    Arm64,
    /// i386 (x86)
    I386,
    /// i586 (x86)
    I586,
    /// i686 (x86)
    I686,
    /// X86_64 / Amd64
    X64,
    /// MIPS
    Mips,
    /// MIPS (LE)
    MipsEl,
    /// MIPS64
    Mips64,
    /// MIPS64 (LE)
    Mips64El,
    /// PowerPC
    PowerPc,
    /// PowerPC64
    PowerPc64,
    /// PowerPC64LE
    PowerPc64Le,
    /// 32-bit RISC-V
    Riscv32,
    /// 64-bit RISC-V
    Riscv64,
    /// S390x
    S390x,
    /// SPARC
    Sparc,
    /// SPARC64
    Sparc64,
    /// 32-bit Web Assembly
    Wasm32,
    /// 64-bit Web Assembly
    Wasm64,
    /// Unknown Architecture
    Unknown(String),
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::ArmV5 => "armv5",
            Self::ArmV6 => "armv6",
            Self::ArmV7 => "armv7",
            Self::Arm64 => "arm64",
            Self::I386 => "i386",
            Self::I586 => "i586",
            Self::I686 => "i686",
            Self::Mips => "mips",
            Self::MipsEl => "mipsel",
            Self::Mips64 => "mips64",
            Self::Mips64El => "mips64el",
            Self::PowerPc => "powerpc",
            Self::PowerPc64 => "powerpc64",
            Self::PowerPc64Le => "powerpc64le",
            Self::Riscv32 => "riscv32",
            Self::Riscv64 => "riscv64",
            Self::S390x => "s390x",
            Self::Sparc => "sparc",
            Self::Sparc64 => "sparc64",
            Self::Wasm32 => "wasm32",
            Self::Wasm64 => "wasm64",
            Self::X64 => "x86_64",
            Self::Unknown(arch) => arch,
        })
    }
}

impl Arch {
    /// Get the width of this architecture.
    pub fn width(&self) -> Result<Width> {
        match self {
            Arch::ArmV5
            | Arch::ArmV6
            | Arch::ArmV7
            | Arch::I386
            | Arch::I586
            | Arch::I686
            | Arch::Mips
            | Arch::MipsEl
            | Arch::PowerPc
            | Arch::Riscv32
            | Arch::Sparc
            | Arch::Wasm32 => Ok(Width::Bits32),
            Arch::Arm64
            | Arch::Mips64
            | Arch::Mips64El
            | Arch::PowerPc64
            | Arch::PowerPc64Le
            | Arch::Riscv64
            | Arch::S390x
            | Arch::Sparc64
            | Arch::Wasm64
            | Arch::X64 => Ok(Width::Bits64),
            Arch::Unknown(unknown_arch) => Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Tried getting width of unknown arch ({})",
                    unknown_arch,
                ),
            )),
        }
    }
}
