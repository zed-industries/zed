#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::ops::Deref;
use core::ptr::{null, null_mut};

mod bindings;
use bindings::*;

mod open_options;
pub use open_options::OpenOptions;

mod key;
pub use key::Key;

mod transaction;
pub use transaction::Transaction;

mod value;
pub use value::Value;

mod data;
use data::Data;

mod pcwstr;
use pcwstr::*;

mod key_iterator;
pub use key_iterator::KeyIterator;

mod value_iterator;
pub use value_iterator::ValueIterator;

mod r#type;
pub use r#type::Type;

pub use windows_result::Result;
use windows_result::*;

pub use windows_strings::HSTRING;
use windows_strings::{PCWSTR, *};

/// The predefined `HKEY_CLASSES_ROOT` registry key.
pub const CLASSES_ROOT: &Key = &Key(HKEY_CLASSES_ROOT);

/// The predefined `HKEY_CURRENT_CONFIG` registry key.
pub const CURRENT_CONFIG: &Key = &Key(HKEY_CURRENT_CONFIG);

/// The predefined `HKEY_CURRENT_USER` registry key.
pub const CURRENT_USER: &Key = &Key(HKEY_CURRENT_USER);

/// The predefined `HKEY_LOCAL_MACHINE` registry key.
pub const LOCAL_MACHINE: &Key = &Key(HKEY_LOCAL_MACHINE);

/// The predefined `HKEY_USERS` registry key.
pub const USERS: &Key = &Key(HKEY_USERS);

fn win32_error(result: u32) -> Result<()> {
    if result == 0 {
        Ok(())
    } else {
        Err(Error::from_hresult(HRESULT::from_win32(result)))
    }
}

fn invalid_data() -> Error {
    Error::from_hresult(HRESULT::from_win32(ERROR_INVALID_DATA))
}

fn from_le_bytes(ty: Type, from: &[u8]) -> Result<u64> {
    match ty {
        Type::U32 if from.len() == 4 => Ok(u32::from_le_bytes(from.try_into().unwrap()).into()),
        Type::U64 if from.len() == 8 => Ok(u64::from_le_bytes(from.try_into().unwrap())),
        _ => Err(invalid_data()),
    }
}

// Get the string as 8-bit bytes including the two terminating null bytes.
fn as_bytes(value: &HSTRING) -> &[u8] {
    unsafe { core::slice::from_raw_parts(value.as_ptr() as *const _, (value.len() + 1) * 2) }
}
