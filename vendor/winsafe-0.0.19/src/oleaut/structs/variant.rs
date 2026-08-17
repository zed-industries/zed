#![allow(non_snake_case)]

use std::mem::ManuallyDrop;

use crate::co;
use crate::oleaut::ffi;
use crate::prelude::*;

/// [`VARIANT`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/ns-oaidl-variant)
/// struct.
///
/// Automatically calls
/// [`VariantClear`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-variantclear)
/// when the object goes out of scope.
///
/// The [`Default`](std::default::Default) implementation returns a
/// [`co::VT::EMPTY`](crate::co::VT::EMPTY) value.
#[repr(C)]
pub struct VARIANT {
	vt: co::VT,
	wReserved1: u16,
	wReserved2: u16,
	wReserved3: u16,
	data: [u8; 16],
}

impl Drop for VARIANT {
	fn drop(&mut self) {
		if self.vt() != co::VT::EMPTY {
			unsafe { ffi::VariantClear(self as *mut _ as _); } // ignore errors
		}
	}
}

impl Default for VARIANT {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		unsafe { ffi::VariantInit(&mut obj as *mut _ as _); }
		obj
	}
}

impl oleaut_Variant for VARIANT {
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

impl VARIANT {
	/// Creates a new object holding an [`IDispatch`](crate::IDispatch) COM
	/// value.
	///
	/// Note that `val` will be cloned into the `VARIANT` – that is,
	/// [`IUnknown::AddRef`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// will be called –, so `val` will remain valid to be used thereafter.
	#[must_use]
	pub fn new_idispatch(val: &impl oleaut_IDispatch) -> Self {
		let mut cloned = val.clone();
		let ptr = cloned.leak() as usize;
		unsafe { Self::from_raw(co::VT::DISPATCH, &ptr.to_ne_bytes()) }
	}

	/// If the object holds an [`IDispatch`](crate::IDispatch) COM value,
	/// returns it, otherwise `None`.
	///
	/// Note that the returned object will be a clone – that is,
	/// [`IUnknown::AddRef`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// will be called.
	#[must_use]
	pub fn idispatch<T>(&self) -> Option<T>
		where T: oleaut_IDispatch,
	{
		if self.vt() == co::VT::DISPATCH {
			let ptr = usize::from_ne_bytes(self.raw()[..8].try_into().unwrap());
			let obj = ManuallyDrop::new(unsafe { T::from_ptr(ptr as *mut _) }); // won't release the stored pointer
			let cloned = T::clone(&obj); // call AddRef
			Some(cloned)
		} else {
			None
		}
	}

	/// Creates a new object holding an [`IUnknown`](crate::IUnknown) COM value.
	///
	/// Note that `val` will be cloned into the `VARIANT` – that is,
	/// [`IUnknown::AddRef`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// will be called –, so `val` will remain valid to be used thereafter.
	#[must_use]
	pub fn new_iunknown<T>(val: &impl ole_IUnknown) -> Self {
		let mut cloned = val.clone();
		let ptr = cloned.leak() as usize;
		unsafe { Self::from_raw(co::VT::UNKNOWN, &ptr.to_ne_bytes()) }
	}

	/// If the object holds an [`IUnknown`](crate::IUnknown) COM value, returns
	/// it, otherwise `None`.
	///
	/// Note that the returned object will be a clone – that is,
	/// [`IUnknown::AddRef`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// will be called.
	#[must_use]
	pub fn iunknown<T>(&self) -> Option<T>
		where T: ole_IUnknown,
	{
		if self.vt() == co::VT::UNKNOWN {
			let ptr = usize::from_ne_bytes(self.raw()[..8].try_into().unwrap());
			let obj = ManuallyDrop::new(unsafe { T::from_ptr(ptr as *mut _) }); // won't release the stored pointer
			let cloned = T::clone(&obj); // call AddRef
			Some(cloned)
		} else {
			None
		}
	}
}
