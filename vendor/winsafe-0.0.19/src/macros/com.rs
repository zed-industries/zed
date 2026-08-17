#![allow(unused_macros)]

/// Declares an ordinary COM interface, and implements ole_IUnknown trait.
macro_rules! com_interface {
	(
		$name:ident : $guid:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		pub struct $name(crate::kernel::ffi_types::COMPTR);

		impl Drop for $name {
			fn drop(&mut self) {
				if !self.0.is_null() {
					unsafe {
						use crate::{prelude::ole_IUnknown, vt::IUnknownVT};
						(crate::ole::privs::vt::<IUnknownVT>(self).Release)(self.ptr());
					}
				}
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				use crate::{prelude::ole_IUnknown, vt::IUnknownVT};
				unsafe { (crate::ole::privs::vt::<IUnknownVT>(self).AddRef)(self.ptr()); }
				Self(self.ptr())
			}
		}

		impl crate::prelude::ole_IUnknown for $name {
			const IID: crate::co::IID = unsafe { crate::co::IID::from_raw($guid) };

			unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self {
				Self(p)
			}

			unsafe fn as_mut(&mut self) -> &mut *mut std::ffi::c_void {
				&mut self.0
			}

			fn ptr(&self) -> *mut std::ffi::c_void {
				self.0
			}
		}
	};
}

/// Creates multiple `GUID`-derived pub const values.
macro_rules! const_guid_values {
	(
		$name:ident;
		$( $pubname:ident $guid:expr )*
	) => {
		impl $name {
			$(
				pub const $pubname: $name = unsafe { $name::from_raw($guid) };
			)*
		}
	};
}

/// Declares the type of a `GUID`-derived constant, along with public values.
macro_rules! const_guid {
	(
		$name:ident;
		$( #[$doc:meta] )*
		=>
		$( $pubname:ident $guid:expr )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
		pub struct $name(crate::kernel::decl::GUID);

		impl From<crate::kernel::decl::GUID> for $name {
			fn from(guid: crate::kernel::decl::GUID) -> Self {
				Self(guid)
			}
		}

		impl AsRef<crate::kernel::decl::GUID> for $name {
			fn as_ref(&self) -> &crate::kernel::decl::GUID {
				&self.0
			}
		}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				std::fmt::Debug::fmt(&self.0, f)
			}
		}
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				std::fmt::Display::fmt(&self.0, f)
			}
		}

		impl $name {
			/// Creates a new `GUID` from a representative hex string, which can
			/// be copied straight from standard `GUID` declarations.
			///
			/// # Safety
			///
			/// Be sure the given value is meaningful for the actual type.
			#[must_use]
			pub const unsafe fn from_raw(guid_str: &str) -> Self {
				Self(crate::kernel::decl::GUID::new(guid_str))
			}
		}

		const_guid_values! {
			$name;
			$( $pubname $guid )*
		}
	};
}

/// Implements a trait function with no parameters.
macro_rules! fn_com_noparm {
	(
		$method:ident : $vt:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $method(&self) -> HrResult<()> {
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(self.ptr())
				},
			)
		}
	};
}

/// Implements a trait function with no parameters and no return.
macro_rules! fn_com_noparm_noret {
	(
		$method:ident : $vt:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $method(&self) {
			unsafe { (crate::ole::privs::vt::<$vt>(self).$method)(self.ptr()); }
		}
	};
}

/// Implements a trait function for a COM interface getter, no parameters.
macro_rules! fn_com_interface_get {
	(
		$method:ident : $vt:ty, $iface:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[must_use]
		fn $method(&self) -> HrResult<$iface> {
			use crate::prelude::ole_IUnknown;
			let mut queried = unsafe { <$iface>::null() };
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(self.ptr(), queried.as_mut())
				},
			).map(|_| queried)
		}
	};
}

/// Implements a trait function for a BSTR getter, no parameters.
macro_rules! fn_com_bstr_get {
	(
		$method:ident : $vt:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[must_use]
		fn $method(&self) -> HrResult<String> {
			let mut pstr = std::ptr::null_mut::<u16>();
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(self.ptr(), &mut pstr)
				},
			).map(|_| {
				let bstr = unsafe { crate::oleaut::decl::BSTR::from_ptr(pstr) };
				bstr.to_string()
			})
		}
	};
}

/// Implements a trait function for a BSTR setter, single parameter.
macro_rules! fn_com_bstr_set {
	(
		$method:ident : $vt:ty, $arg:ident;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $method(&self, $arg: &str) -> HrResult<()> {
			crate::ole::privs::ok_to_hrresult(
				unsafe {
					(crate::ole::privs::vt::<$vt>(self).$method)(
						self.ptr(),
						crate::oleaut::decl::BSTR::SysAllocString($arg)?.as_ptr(),
					)
				},
			)
		}
	};
}
