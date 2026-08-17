#![allow(dead_code)]

use crate::co;
use crate::decl::*;

pub(crate) const ASFW_ANY: u32 = -1i32 as _;
pub(crate) const CB_ERR: i32 = -1;
pub(crate) const CB_ERRSPACE: i32 = -2;
pub(crate) const CCHDEVICENAME: usize = 32;
pub(crate) const CCHFORMNAME: usize = 32;
pub(crate) const CCHILDREN_TITLEBAR: usize = 5;
pub(crate) const DM_SPECVERSION: u16 = 0x0401;
pub(crate) const FAPPCOMMAND_MASK: u16 = 0xf000;
pub(crate) const HWND_MESSAGE: isize = -3;
pub(crate) const LB_ERR: i32 = -1;
pub(crate) const LB_ERRSPACE: i32 = -2;
pub(crate) const WC_DIALOG: u16 = 0x8002;

/// Takes an `isize` and returns `Err` if `-1`.
pub(crate) const fn minus1_as_badargs(v: isize) -> SysResult<isize> {
	match v {
		-1 => Err(co::ERROR::BAD_ARGUMENTS), // all message errors will return this code
		v => Ok(v),
	}
}

/// Takes an `isize` and returns `None` if `-1`.
pub(crate) const fn minus1_as_none(v: isize) -> Option<isize> {
	match v {
		-1 => None,
		v => Some(v),
	}
}

/// Takes an `isize` and returns `Err` if zero.
pub(crate) const fn zero_as_badargs(v: isize) -> SysResult<isize> {
	match v {
		0 => Err(co::ERROR::BAD_ARGUMENTS), // all message errors will return this code
		v => Ok(v),
	}
}

/// Takes an `isize` and returns `None` if zero.
pub(crate) const fn zero_as_none(v: isize) -> Option<isize> {
	match v {
		0 => None,
		v => Some(v),
	}
}
