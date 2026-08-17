#![allow(non_camel_case_types)]

use std::mem::ManuallyDrop;

use crate::co;
use crate::decl::*;

/// This trait is enabled with the `oleaut` feature, and provides common methods
/// for [`VARIANT`](crate::VARIANT) and [`PROPVARIANT`](crate::PROPVARIANT).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_Variant: Default {
	/// Returns a reference to the raw data being held.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	#[must_use]
	fn raw(&self) -> &[u8; 16];

	/// Creates an object straight from raw data. Up to 16 `u8` elements will be
	/// actually copied.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// Be sure the binary data is correct.
	#[must_use]
	unsafe fn from_raw(vt: co::VT, data: &[u8]) -> Self;

	/// Returns the [`co::VT`](crate::co::VT) variant type currently being held.
	#[must_use]
	fn vt(&self) -> co::VT;

	/// Tells whether no value is being held, that is, the variant type is
	/// [`co::VT::EMPTY`](crate::co::VT::EMPTY).
	#[must_use]
	fn is_empty(&self) -> bool {
		self.vt() == co::VT::EMPTY
	}

	/// Tells whether the object holds an SQL style null, that is, the variant
	/// type is [`co::VT::NULL`](crate::co::VT::NULL).
	#[must_use]
	fn is_null(&self) -> bool {
		self.vt() == co::VT::NULL
	}

	/// Crates a new object holding a `bool` value.
	#[must_use]
	fn new_bool(val: bool) -> Self
		where Self: Sized,
	{
		let val16: i16 = if val { -1 } else { 0 };
		unsafe { Self::from_raw(co::VT::BOOL, &val16.to_ne_bytes()) }
	}

	/// If the object holds a `bool` value, returns it, otherwise `None`.
	#[must_use]
	fn bool(&self) -> Option<bool> {
		if self.vt() == co::VT::BOOL {
			let val16 = i16::from_ne_bytes(self.raw()[..2].try_into().unwrap());
			Some(val16 != 0)
		} else {
			None
		}
	}

	/// Creates a new object holding a [`BSTR`](crate::BSTR) value.
	#[must_use]
	fn new_bstr(val: &str) -> HrResult<Self>
		where Self: Sized,
	{
		let mut bstr = BSTR::SysAllocString(val)?;
		let ptr = bstr.leak() as usize;
		Ok(unsafe { Self::from_raw(co::VT::BSTR, &ptr.to_ne_bytes()) })
	}

	/// If the object holds a [`BSTR`](crate::BSTR) value, returns it, otherwise
	/// `None`.
	#[must_use]
	fn bstr(&self) -> Option<String> {
		if self.vt() == co::VT::BSTR {
			let ptr = usize::from_ne_bytes(self.raw()[..8].try_into().unwrap());
			let bstr = ManuallyDrop::new(unsafe { BSTR::from_ptr(ptr as _) }); // won't release the stored pointer
			Some(bstr.to_string())
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `f32` value.
	#[must_use]
	fn new_f32(val: f32) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::R4, &val.to_ne_bytes()) }
	}

	/// If the `VARIANT` holds an `f32` value, returns it, otherwise `None`.
	#[must_use]
	fn f32(&self) -> Option<f32> {
		if self.vt() == co::VT::R4 {
			Some(f32::from_ne_bytes(self.raw()[..4].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding an `f64` value.
	#[must_use]
	fn new_f64(val: f64) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::R8, &val.to_ne_bytes()) }
	}

	/// If the object holds an `f64` value, returns it, otherwise `None`.
	#[must_use]
	fn f64(&self) -> Option<f64> {
		if self.vt() == co::VT::R8 {
			Some(f64::from_ne_bytes(self.raw()[..8].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding an `i8` value.
	#[must_use]
	fn new_i8(val: i8) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::I1, &val.to_ne_bytes()) }
	}

	/// If the object holds an `i8` value, returns it, otherwise `None`.
	#[must_use]
	fn i8(&self) -> Option<i8> {
		if self.vt() == co::VT::I1 {
			Some(i8::from_ne_bytes(self.raw()[..1].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding an `i16` value.
	#[must_use]
	fn new_i16(val: i16) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::I2, &val.to_ne_bytes()) }
	}

	/// If the object holds an `i16` value, returns it, otherwise `None`.
	#[must_use]
	fn i16(&self) -> Option<i16> {
		if self.vt() == co::VT::I2 {
			Some(i16::from_ne_bytes(self.raw()[..2].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding an `i32` value.
	#[must_use]
	fn new_i32(val: i32) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::I4, &val.to_ne_bytes()) }
	}

	/// If the object holds an `i32` value, returns it, otherwise `None`.
	#[must_use]
	fn i32(&self) -> Option<i32> {
		if self.vt() == co::VT::I4 {
			Some(i32::from_ne_bytes(self.raw()[..4].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding a date/time value.
	#[must_use]
	fn new_time(val: &SYSTEMTIME) -> SysResult<Self>
		where Self: Sized,
	{
		let double = SystemTimeToVariantTime(val)?;
		Ok(unsafe { Self::from_raw(co::VT::DATE, &double.to_ne_bytes()) })
	}

	/// If the object holds a date/time value, returns it, otherwise `None`.
	#[must_use]
	fn time(&self) -> Option<SYSTEMTIME> {
		if self.vt() == co::VT::DATE {
			let double = f64::from_ne_bytes(self.raw()[..8].try_into().unwrap());
			let mut st = SYSTEMTIME::default();
			VariantTimeToSystemTime(double, &mut st).unwrap();
			Some(st)
		} else {
			None
		}
	}

	/// Creates a new object holding an `u8` value.
	#[must_use]
	fn new_u8(val: u8) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::UI1, &val.to_ne_bytes()) }
	}

	/// If the object holds an `u8` value, returns it, otherwise `None`.
	#[must_use]
	fn u8(&self) -> Option<u8> {
		if self.vt() == co::VT::UI1 {
			Some(u8::from_ne_bytes(self.raw()[..1].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding an `u16` value.
	#[must_use]
	fn new_u16(val: u16) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::UI2, &val.to_ne_bytes()) }
	}

	/// If the object holds an `u16` value, returns it, otherwise `None`.
	#[must_use]
	fn u16(&self) -> Option<u16> {
		if self.vt() == co::VT::UI2 {
			Some(u16::from_ne_bytes(self.raw()[..2].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new object holding an `u32` value.
	#[must_use]
	fn new_u32(val: u32) -> Self
		where Self: Sized,
	{
		unsafe { Self::from_raw(co::VT::UI4, &val.to_ne_bytes()) }
	}

	/// If the object holds an `u32` value, returns it, otherwise `None`.
	#[must_use]
	fn u32(&self) -> Option<u32> {
		if self.vt() == co::VT::UI4 {
			Some(u32::from_ne_bytes(self.raw()[..4].try_into().unwrap()))
		} else {
			None
		}
	}
}
