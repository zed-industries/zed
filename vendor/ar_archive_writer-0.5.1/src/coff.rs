// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(u16)]
#[allow(clippy::upper_case_acronyms)]
pub enum MachineTypes {
    AMD64 = object::pe::IMAGE_FILE_MACHINE_AMD64,
    ARMNT = object::pe::IMAGE_FILE_MACHINE_ARMNT,
    ARM64 = object::pe::IMAGE_FILE_MACHINE_ARM64,
    ARM64EC = object::pe::IMAGE_FILE_MACHINE_ARM64EC,
    ARM64X = 0xA64E,
    I386 = object::pe::IMAGE_FILE_MACHINE_I386,
}

impl From<MachineTypes> for u16 {
    fn from(val: MachineTypes) -> Self {
        val as u16
    }
}

impl TryInto<MachineTypes> for u16 {
    type Error = ();

    fn try_into(self) -> Result<MachineTypes, Self::Error> {
        match self {
            object::pe::IMAGE_FILE_MACHINE_AMD64 => Ok(MachineTypes::AMD64),
            object::pe::IMAGE_FILE_MACHINE_ARMNT => Ok(MachineTypes::ARMNT),
            object::pe::IMAGE_FILE_MACHINE_ARM64 => Ok(MachineTypes::ARM64),
            object::pe::IMAGE_FILE_MACHINE_ARM64EC => Ok(MachineTypes::ARM64EC),
            0xA64E => Ok(MachineTypes::ARM64X),
            object::pe::IMAGE_FILE_MACHINE_I386 => Ok(MachineTypes::I386),
            _ => Err(()),
        }
    }
}

pub fn is_arm64ec(machine: MachineTypes) -> bool {
    machine == MachineTypes::ARM64EC || machine == MachineTypes::ARM64X
}

pub fn is_any_arm64(machine: MachineTypes) -> bool {
    machine == MachineTypes::ARM64 || is_arm64ec(machine)
}

pub fn is_64_bit(machine: MachineTypes) -> bool {
    machine == MachineTypes::AMD64 || is_any_arm64(machine)
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u16)]
pub enum ImportType {
    /// An executable code symbol.
    Code = 0,
    /// A data symbol.
    Data = 1,
    /// A constant value.
    Const = 2,
}

impl From<ImportType> for u16 {
    fn from(val: ImportType) -> Self {
        val as u16
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u16)]
pub enum ImportNameType {
    /// Import is by ordinal. This indicates that the value in the Ordinal/Hint
    /// field of the import header is the import's ordinal. If this constant is
    /// not specified, then the Ordinal/Hint field should always be interpreted
    /// as the import's hint.
    Ordinal = 0,
    /// The import name is identical to the public symbol name
    Name = 1,
    /// The import name is the public symbol name, but skipping the leading ?,
    /// @, or optionally _.
    NameNoprefix = 2,
    /// The import name is the public symbol name, but skipping the leading ?,
    /// @, or optionally _, and truncating at the first @.
    NameUndecorate = 3,
    /// The import name is specified as a separate string in the import library
    /// object file.
    NameExportas = 4,
}

impl From<ImportNameType> for u16 {
    fn from(val: ImportNameType) -> Self {
        val as u16
    }
}
