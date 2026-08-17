#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFSourceResolver`](crate::IMFSourceResolver).
#[repr(C)]
pub struct IMFSourceResolverVT {
	pub IUnknownVT: IUnknownVT,
	pub CreateObjectFromURL: fn(COMPTR, PCSTR, u32, COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub CreateObjectFromByteStream: fn(COMPTR, COMPTR, PCSTR, u32, COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub BeginCreateObjectFromURL: fn(COMPTR, PCSTR, u32, COMPTR, *mut COMPTR, COMPTR, COMPTR) -> HRES,
	pub EndCreateObjectFromURL: fn(COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub BeginCreateObjectFromByteStream: fn(COMPTR, COMPTR, PCSTR, u32, COMPTR, *mut COMPTR, COMPTR, COMPTR) -> HRES,
	pub EndCreateObjectFromByteStream: fn(COMPTR, COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub CancelObjectCreation: fn(COMPTR, COMPTR) -> HRES,
}

com_interface! { IMFSourceResolver: "fbe5a32d-a497-4b61-bb85-97b1a848a6e3";
	/// [`IMFSourceResolver`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfsourceresolver)
	/// COM interface over
	/// [`IMFSourceResolverVT`](crate::vt::IMFSourceResolverVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`MFCreateSourceResolver`](crate::MFCreateSourceResolver) function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let source_resolver = w::MFCreateSourceResolver()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl mf_IMFSourceResolver for IMFSourceResolver {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFSourceResolver`](crate::IMFSourceResolver).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFSourceResolver: ole_IUnknown {

}
