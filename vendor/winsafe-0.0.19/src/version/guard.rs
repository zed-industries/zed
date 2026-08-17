use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;

/// RAII implementation for [`HVERSIONINFO`](crate::HVERSIONINFO) which releases
/// the underlying memory block when the object goes out of scope.
pub struct VersionInfoGuard {
	handle: HVERSIONINFO,
}

impl Drop for VersionInfoGuard {
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			let hglobal_ptr = unsafe { HGLOBAL::from_ptr(h.ptr()) };
			let _ = unsafe { GlobalFreeGuard::new(hglobal_ptr) };
		}
	}
}

impl std::ops::Deref for VersionInfoGuard {
	type Target = HVERSIONINFO;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl std::ops::DerefMut for VersionInfoGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.handle
	}
}

impl VersionInfoGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle is a valid [`HGLOBAL`](crate::HGLOBAL) memory block.
	#[must_use]
	pub const unsafe fn new(handle: HVERSIONINFO) -> Self {
		Self { handle }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its
	/// place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a memory
	/// leak.
	#[must_use]
	pub fn leak(&mut self) -> HVERSIONINFO {
		std::mem::replace(&mut self.handle, Handle::INVALID)
	}
}
