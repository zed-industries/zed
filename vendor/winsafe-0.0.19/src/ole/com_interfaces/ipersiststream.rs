#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IPersistStream`](crate::IPersistStream) virtual table.
#[repr(C)]
pub struct IPersistStreamVT {
	pub IPersistVT: IPersistVT,
	pub IsDirty: fn(COMPTR) -> HRES,
	pub Load: fn(COMPTR, COMPTR) -> HRES,
	pub Save: fn(COMPTR, COMPTR, BOOL) -> HRES,
	pub GetSizeMax: fn(COMPTR, *mut u64) -> HRES,
}

com_interface! { IPersistStream: "00000109-0000-0000-c000-000000000046";
	/// [`IPersistStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersiststream)
	/// COM interface over [`IPersistStreamVT`](crate::vt::IPersistStreamVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IPersistStream {}
impl ole_IPersistStream for IPersistStream {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPersistStream`](crate::IPersistStream).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPersistStream: ole_IPersist {
	/// [`IPersistStream::GetSizeMax`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-getsizemax)
	/// method.
	#[must_use]
	fn GetSizeMax(&self) -> HrResult<u64> {
		let mut max = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IPersistStreamVT>(self).GetSizeMax)(self.ptr(), &mut max)
			},
		).map(|_| max)
	}

	/// [`IPersistStream::IsDirty`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-isdirty)
	/// method.
	#[must_use]
	fn IsDirty(&self) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe { (vt::<IPersistStreamVT>(self).IsDirty)(self.ptr()) },
		)
	}

	/// [`IPersistStream::Load`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-load)
	/// method.
	fn Load(&self, stream: &impl ole_IStream) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IPersistStreamVT>(self).Load)(self.ptr(), stream.ptr())
			},
		)
	}

	/// [`IPersistStream::Save`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-save)
	/// method.
	fn Save(&self,
		stream: &impl ole_IStream,
		clear_dirty: bool,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IPersistStreamVT>(self).Save)(
					self.ptr(),
					stream.ptr(),
					clear_dirty as _,
				)
			},
		)
	}
}
