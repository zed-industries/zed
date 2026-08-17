#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;
use crate::uxtheme::ffi;

impl uxtheme_Hwnd for HWND {}

/// This trait is enabled with the `uxtheme` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait uxtheme_Hwnd: ole_Hwnd {
	/// [`OpenThemeData`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-openthemedata)
	/// function.
	#[must_use]
	fn OpenThemeData(&self, class_list: &str) -> Option<CloseThemeDataGuard> {
		unsafe {
			ffi::OpenThemeData(
				self.ptr(),
				WString::from_str(class_list).as_ptr(),
			).as_mut()
				.map(|ptr| CloseThemeDataGuard::new(HTHEME::from_ptr(ptr)))
		}
	}
}
