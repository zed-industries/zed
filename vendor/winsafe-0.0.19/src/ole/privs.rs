use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;

/// Returns a reference to the virtual table of the COM object.
pub(crate) unsafe fn vt<T>(obj: &impl ole_IUnknown) -> &T {
	let ppvt = obj.ptr() as *mut *mut T;
	&**ppvt
}

/// If value is `S_OK` yields `Ok()`, othersize `Err(hresult)`.
pub(crate) const fn ok_to_hrresult(hr: HRES) -> HrResult<()> {
	match unsafe { co::HRESULT::from_raw(hr) } {
		co::HRESULT::S_OK => Ok(()),
		hr => Err(hr),
	}
}

/// If value is `S_OK` yields `Ok(true)`, if `S_FALSE` yields `Ok(false)`
/// othersize `Err(hresult)`.
pub(crate) const fn okfalse_to_hrresult(hr: HRES) -> HrResult<bool> {
	match unsafe { co::HRESULT::from_raw(hr) } {
		co::HRESULT::S_OK => Ok(true),
		co::HRESULT::S_FALSE => Ok(false),
		hr => Err(hr),
	}
}
