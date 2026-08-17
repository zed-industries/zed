#![allow(non_snake_case)]

use crate::co;
use crate::oleaut::ffi;
use crate::prelude::*;

/// [`PROPVARIANT`](https://learn.microsoft.com/en-us/windows/win32/api/propidlbase/ns-propidlbase-propvariant)
/// struct.
///
/// Automatically calls
/// [`PropVariantClear`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-propvariantclear)
/// when the object goes out of scope.
///
/// The [`Default`](std::default::Default) implementation returns a
/// [`co::VT::EMPTY`](crate::co::VT::EMPTY) value.
#[repr(C)]
pub struct PROPVARIANT {
	vt: co::VT,
	wReserved1: u16,
	wReserved2: u16,
	wReserved3: u16,
	data: [u8; 16],
}

impl Drop for PROPVARIANT {
	fn drop(&mut self) {
		if self.vt() != co::VT::EMPTY {
			unsafe { ffi::PropVariantClear(self as *mut _ as _); } // ignore errors
		}
	}
}

impl Default for PROPVARIANT {
	fn default() -> Self {
		unsafe { std::mem::zeroed::<Self>() } // PropVariantInit() is just a macro
	}
}

impl oleaut_Variant for PROPVARIANT {
	fn raw(&self) -> &[u8; 16] {
		&self.data
	}

	unsafe fn from_raw(vt: co::VT, data: &[u8]) -> Self {
		let mut obj = Self::default();
		obj.vt = vt;
		data.iter()
			.zip(&mut obj.data)
			.for_each(|(src, dest)| *dest = *src);
		obj
	}

	fn vt(&self) -> co::VT {
		self.vt
	}
}

impl PROPVARIANT {
	/// Creates a new object holding an `i64` value.
	#[must_use]
	pub fn new_i64(val: i64) -> Self {
		unsafe { Self::from_raw(co::VT::I8, &val.to_ne_bytes()) }
	}

	/// If the object holds an `i64` value, returns it, otherwise `None`.
	#[must_use]
	pub fn i64(&self) -> Option<i64> {
		if self.vt() == co::VT::I8 {
			Some(i64::from_ne_bytes(self.raw()[..8].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding an `u64` value.
	#[must_use]
	pub fn new_u64(val: u64) -> Self {
		unsafe { Self::from_raw(co::VT::UI8, &val.to_ne_bytes()) }
	}

	/// If the object holds an `u64` value, returns it, otherwise `None`.
	#[must_use]
	pub fn u64(&self) -> Option<u64> {
		if self.vt() == co::VT::UI8 {
			Some(u64::from_ne_bytes(self.raw()[..8].try_into().unwrap()))
		} else {
			None
		}
	}
}
