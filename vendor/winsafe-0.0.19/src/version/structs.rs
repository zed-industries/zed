#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;

/// [`VS_FIXEDFILEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo)
/// struct.
#[repr(C)]
pub struct VS_FIXEDFILEINFO {
	dwSignature: u32,
	pub dwStrucVersion: u32,
	dwFileVersionMS: u32,
	dwFileVersionLS: u32,
	dwProductVersionMS: u32,
	dwProductVersionLS: u32,
	pub dwFileFlagsMask: co::VS_FF,
	pub dwFileFlags: co::VS_FF,
	pub dwFileOS: co::VOS,
	pub dwFileType: co::VFT,
	pub dwFileSubtype: co::VFT2,
	dwFileDateMS: u32,
	dwFileDateLS: u32,
}

impl Default for VS_FIXEDFILEINFO {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.dwSignature = 0xfeef_04bd;
		obj
	}
}

impl VS_FIXEDFILEINFO {
	/// Returns the `dwFileVersionMS` and `dwFileVersionLS` fields.
	#[must_use]
	pub const fn dwFileVersion(&self) -> [u16; 4] {
		[HIWORD(self.dwFileVersionMS), LOWORD(self.dwFileVersionMS),
			HIWORD(self.dwFileVersionLS), LOWORD(self.dwFileVersionLS)]
	}

	/// Sets the `dwFileVersionMS` and `dwFileVersionLS` fields.
	pub fn set_dwFileVersion(&mut self, val: [u16; 4]) {
		self.dwFileVersionMS = MAKEDWORD(val[1], val[0]);
		self.dwFileVersionLS = MAKEDWORD(val[3], val[2]);
	}

	/// Returns the `dwProductVersionMS` and `dwProductVersionLS` fields.
	#[must_use]
	pub const fn dwProductVersion(&self) -> [u16; 4] {
		[HIWORD(self.dwProductVersionMS), LOWORD(self.dwProductVersionMS),
			HIWORD(self.dwProductVersionLS), LOWORD(self.dwProductVersionLS)]
	}

	/// Sets the `dwProductVersionMS` and `dwProductVersionLS` fields.
	pub fn set_dwProductVersion(&mut self, val: [u16; 4]) {
		self.dwProductVersionMS = MAKEDWORD(val[1], val[0]);
		self.dwProductVersionLS = MAKEDWORD(val[3], val[2]);
	}

	/// Returns the `dwFileDateMS` and `dwFileDateLS` fields.
	#[must_use]
	pub const fn dwFileDate(&self) -> u64 {
		MAKEQWORD(self.dwFileDateLS, self.dwFileDateMS)
	}

	/// Sets the `dwFileDateMS` and `dwFileDateLS` fields.
	pub fn set_dwFileDate(&mut self, val: u64) {
		self.dwFileDateLS = LODWORD(val);
		self.dwFileDateMS = HIDWORD(val);
	}
}
