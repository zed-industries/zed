#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dshow::iterators::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IEnumMediaTypes`](crate::IEnumMediaTypes) virtual table.
#[repr(C)]
pub struct IEnumMediaTypesVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(COMPTR, u32, *mut PVOID, *mut u32) -> HRES,
	pub Skip: fn(COMPTR, u32) -> HRES,
	pub Reset: fn(COMPTR) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IEnumMediaTypes: "89c31040-846b-11ce-97d3-00aa0055595a";
	/// [`IEnumMediaTypes`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienummediatypes)
	/// COM interface over [`IEnumMediaTypesVT`](crate::vt::IEnumMediaTypesVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IEnumMediaTypes for IEnumMediaTypes {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IEnumMediaTypes`](crate::IEnumMediaTypes).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IEnumMediaTypes: ole_IUnknown {
	/// Returns an iterator over the [`AM_MEDIA_TYPE`](crate::AM_MEDIA_TYPE)
	/// elements which calls
	/// [`IEnumMediaTypes::next`](crate::prelude::dshow_IEnumMediaTypes::Next)
	/// internally.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let types: w::IEnumMediaTypes; // initialized somewhere
	/// # let types = unsafe { w::IEnumMediaTypes::null() };
	///
	/// for amt in types.iter() {
	///     let amt = amt?;
	///     println!("{} {}",
	///         amt.majortype.to_string(), amt.lSampleSize);
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self,
	) -> Box<dyn Iterator<Item = HrResult<&'_ AM_MEDIA_TYPE<'_>>> + '_>
	{
		Box::new(IenummediatypesIter::new(self))
	}

	/// [`IEnumMediaTypes::Next`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienummediatypes-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumMediaTypes::iter`](crate::prelude::dshow_IEnumMediaTypes::iter),
	/// which is simpler.
	#[must_use]
	fn Next(&self, mt: &mut AM_MEDIA_TYPE) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe {
				(vt::<IEnumMediaTypesVT>(self).Next)(
					self.ptr(),
					1, // retrieve only 1
					mt as *mut _ as _,
					std::ptr::null_mut(),
				)
			},
		)
	}

	fn_com_noparm! { Reset: IEnumMediaTypesVT;
		/// [`IEnumMediaTypes::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienummediatypes-reset)
		/// method.
	}

	/// [`IEnumMediaTypes::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienummediatypes-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe { (vt::<IEnumMediaTypesVT>(self).Skip)(self.ptr(), count) },
		)
	}
}
