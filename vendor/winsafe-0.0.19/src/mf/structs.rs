#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;

/// [`MFCLOCK_PROPERTIES`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ns-mfidl-mfclock_properties)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MFCLOCK_PROPERTIES {
	pub qwCorrelationRate: u64,
	pub guidClockId: GUID,
	pub dwClockFlags: co::MFCLOCK_RELATIONAL_FLAG,
	pub qwClockFrequency: u64,
	pub dwClockTolerance: u32,
	pub dwClockJitter: u32,
}

/// [`MFVideoNormalizedRect`](https://learn.microsoft.com/en-us/windows/win32/api/evr/ns-evr-mfvideonormalizedrect)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct MFVideoNormalizedRect {
	pub left: f32,
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
}

impl std::fmt::Display for MFVideoNormalizedRect {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "left {:.2}, top {:.2}, right {:.2}, bottom {:.2}",
			self.left, self.top, self.right, self.bottom)
	}
}

impl MFVideoNormalizedRect {
	/// Creates a new `MFVideoNormalizedRect`.
	#[must_use]
	pub const fn new(
		left: f32, top: f32, right: f32, bottom: f32) -> MFVideoNormalizedRect
	{
		Self { left, top, right, bottom }
	}
}
