#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IStream`](crate::IStream) virtual table.
#[repr(C)]
pub struct IStreamVT {
	pub ISequentialStreamVT: ISequentialStreamVT,
	pub Seek: fn(COMPTR, i64, u32, *mut u64) -> HRES,
	pub SetSize: fn(COMPTR, u64) -> HRES,
	pub CopyTo: fn(COMPTR, COMPTR, u64, *mut u64, *mut u64) -> HRES,
	pub Commit: fn(COMPTR, u32)-> HRES,
	pub Revert: fn(COMPTR) -> HRES,
	pub LockRegion: fn(COMPTR, u64, u64, u32) -> HRES,
	pub UnlockRegion: fn(COMPTR, u64, u64, u32) -> HRES,
	pub Stat: fn(COMPTR, PVOID, u32) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IStream: "0000000c-0000-0000-c000-000000000046";
	/// [`IStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-istream)
	/// COM interface over [`IStreamVT`](crate::vt::IStreamVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// Loading from a `Vec`:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let raw_data: Vec<u8>; // initialized somewhere
	/// # let raw_data = Vec::<u8>::default();
	///
	/// let stream = w::SHCreateMemStream(&raw_data)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl ole_ISequentialStream for IStream {}
impl ole_IStream for IStream {}

/// [`IStream`](crate::IStream) methods from `ole` feature.
pub trait ole_IStream: ole_ISequentialStream {
	/// [`IStream::Commit`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-commit)
	/// method.
	fn Commit(&self, flags: co::STGC) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IStreamVT>(self).Commit)(self.ptr(), flags.raw()) },
		)
	}

	/// [`IStream::CopyTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-copyto)
	/// method.
	///
	/// Returns the number of bytes read and written.
	fn CopyTo(&self,
		dest: &impl ole_IStream,
		num_bytes: u64,
	) -> HrResult<(u64, u64)>
	{
		let (mut read, mut written) = (u64::default(), u64::default());
		ok_to_hrresult(
			unsafe {
				(vt::<IStreamVT>(self).CopyTo)(
					self.ptr(),
					dest.ptr(),
					num_bytes,
					&mut read,
					&mut written,
				)
			},
		).map(|_| (read, written))
	}

	/// [`IStream::LockRegion`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-lockregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`IStream::UnlockRegion`](crate::prelude::ole_IStream::UnlockRegion)
	/// call.
	fn LockRegion(&self,
		offset: u64,
		length: u64,
		lock_type: co::LOCKTYPE,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IStreamVT>(self).LockRegion)(
					self.ptr(),
					offset,
					length,
					lock_type.raw(),
				)
			},
		)
	}

	fn_com_noparm! { Revert: IStreamVT;
		/// [`IStream::Revert`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-revert)
		/// method.
	}

	/// [`IStream::Seek`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-seek)
	/// method.
	///
	/// Returns the new absolute offset.
	fn Seek(&self,
		displacement: i64,
		origin: co::STREAM_SEEK,
	) -> HrResult<u64>
	{
		let mut new_off = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IStreamVT>(self).Seek)(
					self.ptr(),
					displacement,
					origin.raw(),
					&mut new_off,
				)
			},
		).map(|_| new_off)
	}

	/// [`IStream::SetSize`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-setsize)
	/// method.
	fn SetSize(&self, new_size: u64) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IStreamVT>(self).SetSize)(self.ptr(), new_size) },
		)
	}

	/// [`IStream::UnlockRegion`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-unlockregion)
	/// method.
	fn UnlockRegion(&self,
		offset: u64,
		length: u64,
		lock_type: co::LOCKTYPE,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IStreamVT>(self).UnlockRegion)(
					self.ptr(),
					offset,
					length,
					lock_type.raw(),
				)
			},
		)
	}
}
