use crate::decl::*;
use crate::prelude::*;
use crate::uxtheme::ffi;

handle_guard! { CloseThemeDataGuard: HTHEME;
	ffi::CloseThemeData;
	/// RAII implementation for [`HTHEME`](crate::HTHEME) which automatically calls
	/// [`CloseThemeData`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// when the object goes out of scope.
}
