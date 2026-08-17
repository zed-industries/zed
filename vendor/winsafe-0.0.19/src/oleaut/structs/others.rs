use crate::decl::*;
use crate::oleaut::privs::*;

/// [`PROPERTYKEY`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ns-wtypes-propertykey)
/// struct.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PROPERTYKEY {
	pub fmtid: GUID,
	pub pid: u32,
}

impl_default!(PROPERTYKEY);

impl PROPERTYKEY {
	/// Creates a new `PROPERTYKEY` by setting `pid` to `PID_FIRST_USABLE`
	/// (`0x02`).
	pub const fn new(fmtid: GUID) -> Self {
		Self { fmtid, pid: PID_FIRST_USABLE }
	}
}
