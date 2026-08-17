//! Shell Dynamic Command Protocol
//!
//! Defined in UEFI Shell Specification, Section 2.4

use super::{shell, shell_parameters};

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x3c7200e9,
    0x005f,
    0x4ea4,
    0x87,
    0xde,
    &[0xa3, 0xdf, 0xac, 0x8a, 0x27, 0xc3],
);

pub type CommandHandler = eficall! {fn(
    *mut Protocol,
    *mut crate::system::SystemTable,
    *mut shell_parameters::Protocol,
    *mut shell::Protocol,
) -> crate::base::Status};

pub type CommandGetHelp = eficall! {fn(
    *mut Protocol,
    *mut crate::base::Char8,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub command_name: *mut crate::base::Char16,
    pub handler: CommandHandler,
    pub get_help: CommandGetHelp,
}
