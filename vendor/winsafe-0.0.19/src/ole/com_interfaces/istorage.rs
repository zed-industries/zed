#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IStorage`](crate::IStorage) virtual table.
#[repr(C)]
pub struct IStorageVT {
	pub IUnknownVT: IUnknownVT,
	pub CreateStream: fn(COMPTR, PCSTR, u32, u32, u32, *mut COMPTR) -> HRES,
	pub OpenStream: fn(COMPTR, PCSTR, PVOID, u32, u32, *mut COMPTR) -> HRES,
	pub CreateStorage: fn(COMPTR, PCSTR, u32, u32, u32, *mut COMPTR) -> HRES,
	pub OpenStorage: fn(COMPTR, PCSTR, COMPTR, u32, *mut PSTR, u32, *mut COMPTR) -> HRES,
	pub CopyTo: fn(COMPTR, u32, PCVOID, *mut PSTR, COMPTR) -> HRES,
	pub MoveElementTo: fn(COMPTR, PCSTR, COMPTR, PCSTR, u32) -> HRES,
	pub Commit: fn(COMPTR, u32) -> HRES,
	pub Revert: fn(COMPTR) -> HRES,
	pub EnumElements: fn(COMPTR, u32, PVOID, u32, *mut COMPTR) -> HRES,
	pub DestroyElement: fn(COMPTR, PCSTR) -> HRES,
	pub RenameElement: fn(COMPTR, PCSTR, PCSTR) -> HRES,
	pub SetElementTimes: fn(COMPTR, PCSTR, PCVOID, PCVOID, PCVOID) -> HRES,
	pub SetClass: fn(COMPTR, PCVOID) -> HRES,
	pub SetStateBits: fn(COMPTR, u32, u32) -> HRES,
	pub Stat: fn(COMPTR, PVOID, u32) -> HRES,
}

com_interface! { IStorage: "0000000b-0000-0000-c000-000000000046";
	/// [`IStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-istorage)
	/// COM interface over [`IStorageVT`](crate::vt::IStorageVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IStorage for IStorage {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IStorage`](crate::IStorage).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IStorage: ole_IUnknown {
	/// [`IStorage::Commit`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-commit)
	/// method.
	fn Commit(&self, commit_flags: co::STGC) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).Commit)(
					self.ptr(),
					commit_flags.raw(),
				)
			},
		)
	}

	/// [`IStorage::CopyTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-copyto)
	/// method.
	fn CopyTo(&self,
		iid_exclude: Option<&[co::IID]>,
		snb_exclude: Option<&[impl AsRef<str>]>,
		stg_dest: &impl ole_IStorage,
	) -> HrResult<()>
	{
		let snb = snb_exclude.map_or(Ok(SNB::default()), |strs| SNB::new(strs))?;
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).CopyTo)(
					self.ptr(),
					iid_exclude.map_or(0, |iids| iids.len()) as _,
					iid_exclude.map_or(std::ptr::null(), |iids| iids.as_ptr()) as _,
					snb.as_ptr(),
					stg_dest.ptr(),
				)
			},
		)
	}

	/// [`IStorage::CreateStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-createstorage)
	/// method.
	#[must_use]
	fn CreateStorage(&self,
		name: &str,
		grf_mode: co::STGM,
	) -> HrResult<IStorage>
	{
		let mut queried = unsafe { IStorage::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).CreateStorage)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					grf_mode.raw(),
					0,
					0,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IStorage::CreateStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-createstream)
	/// method.
	#[must_use]
	fn CreateStream(&self,
		name: &str,
		grf_mode: co::STGM,
	) -> HrResult<IStream>
	{
		let mut queried = unsafe { IStream::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).CreateStream)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					grf_mode.raw(),
					0,
					0,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IStorage::DestroyElement`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-destroyelement)
	/// method.
	fn DestroyElement(&self, name: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).DestroyElement)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
				)
			},
		)
	}

	/// [`IStorage::MoveElementTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-moveelementto)
	/// method.
	fn MoveElementTo(&self,
		name: &str,
		stg_dest: &impl ole_IStorage,
		new_name: &str,
		grf_flags: co::STGMOVE,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).MoveElementTo)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					stg_dest.ptr(),
					WString::from_str(new_name).as_ptr(),
					grf_flags.raw()
				)
			},
		)
	}

	/// [`IStorage::OpenStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-openstorage)
	/// method.
	#[must_use]
	fn OpenStorage(&self,
		name: &str,
		grf_mode: co::STGM,
	) -> HrResult<IStorage>
	{
		let mut queried = unsafe { IStorage::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).OpenStorage)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					std::ptr::null_mut(),
					grf_mode.raw(),
					std::ptr::null_mut(),
					0,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IStorage::OpenStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-openstream)
	/// method.
	#[must_use]
	fn OpenStream(&self, name: &str, grf_mode: co::STGM) -> HrResult<IStream> {
		let mut queried = unsafe { IStream::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).OpenStream)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					std::ptr::null_mut(),
					grf_mode.raw(),
					0,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IStorage::RenameElement`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-renameelement)
	/// method.
	fn RenameElement(&self, old_name: &str, new_name: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).RenameElement)(
					self.ptr(),
					WString::from_str(old_name).as_ptr(),
					WString::from_str(new_name).as_ptr(),
				)
			},
		)
	}

	fn_com_noparm! { Revert: IStorageVT;
		/// [`IStorage::Revert`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-revert)
		/// method.
	}

	/// [`IStorage::SetClass`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-setclass)
	/// method.
	fn SetClass(&self, clsid: &co::CLSID) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).SetClass)(
					self.ptr(),
					clsid as *const _ as  _,
				)
			},
		)
	}

	/// [`IStorage::SetElementTimes`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-setelementtimes)
	/// method.
	fn SetElementTimes(&self,
		name: Option<&str>,
		creation: Option<&FILETIME>,
		access: Option<&FILETIME>,
		modification: Option<&FILETIME>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).SetElementTimes)(
					self.ptr(),
					WString::from_opt_str(name).as_ptr(),
					creation.map_or(std::ptr::null(), |ft| ft as *const _ as _),
					access.map_or(std::ptr::null(), |ft| ft as *const _ as _),
					modification.map_or(std::ptr::null(), |ft| ft as *const _ as _),
				)
			},
		)
	}

	/// [`IStorage::SetStateBits`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istorage-setstatebits)
	/// method.
	fn SetStateBits(&self, state_bits: u32, mask: u32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IStorageVT>(self).SetStateBits)(
					self.ptr(),
					state_bits,
					mask,
				)
			},
		)
	}
}
