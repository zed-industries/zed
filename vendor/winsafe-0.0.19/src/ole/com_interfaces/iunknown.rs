#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;

/// [`IUnknown`](crate::IUnknown) virtual table, base to all COM virtual tables.
#[repr(C)]
pub struct IUnknownVT {
	pub QueryInterface: fn(COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub AddRef: fn(COMPTR) -> u32,
	pub Release: fn(COMPTR) -> u32,
}

com_interface! { IUnknown: "00000000-0000-0000-c000-000000000046";
	/// [`IUnknown`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
	/// COM interface over [`IUnknownVT`](crate::vt::IUnknownVT). It's the base to
	/// all COM interfaces.
	///
	/// The `clone` method calls
	/// [`AddRef`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// internally.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IUnknown`](crate::IUnknown). It is the base trait for all COM traits.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
///
/// Note that the [`IUnknownVT`](crate::vt::IUnknownVT) virtual table has two
/// other methods: `AddRef` and `Release`. While these methods are relevant in
/// C++, here they are abstracted away as it follows:
///
/// * `AddRef` – called along the `clone` method from the
/// [`Clone`](std::clone::Clone) trait;
///
/// * `Release` – called automatically by the [`Drop`](std::ops::Drop) trait, so
/// you don't need to worry about it.
pub trait ole_IUnknown: Clone {
	/// The COM interface ID.
	const IID: co::IID;

	/// Creates an object from a COM virtual table pointer.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// Be sure the pointer points to a properly allocated COM virtual table.
	#[must_use]
	unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self;

	/// Returns the pointer to the underlying COM virtual table.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	#[must_use]
	fn ptr(&self) -> *mut std::ffi::c_void;

	/// Returns a mutable reference do the underlying COM virtual table pointer.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// Be sure the pointer being set points to a properly allocated COM virtual
	/// table.
	#[must_use]
	unsafe fn as_mut(&mut self) -> &mut *mut std::ffi::c_void;

	/// Creates an object from a null COM virtual table pointer.
	///
	/// # Safety
	///
	/// The pointer must be initialized before any call. Do not call methods on
	/// a null COM pointer.
	#[must_use]
	unsafe fn null() -> Self {
		Self::from_ptr(std::ptr::null_mut())
	}

	/// Returns the pointer to the underlying COM virtual table and sets the
	/// internal pointer to null, so that
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// won't be called.
	///
	/// Be sure to release the pointer, otherwise, as the name of this method
	/// implies, you will cause a resource leak.
	#[must_use]
	fn leak(&mut self) -> *mut std::ffi::c_void {
		let p = self.ptr();
		unsafe { *self.as_mut() = std::ptr::null_mut(); }
		p
	}

	/// [`IUnknown::QueryInterface`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
	/// method.
	#[must_use]
	fn QueryInterface<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IUnknownVT>(self).QueryInterface)(
					self.ptr(),
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}
}
