#![allow(non_snake_case)]

use crate::uxtheme::ffi;

/// [`IsThemeActive`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemeactive)
/// function.
#[must_use]
pub fn IsThemeActive() -> bool {
	unsafe { ffi::IsThemeActive() != 0 }
}

/// [`IsAppThemed`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isappthemed)
/// function.
#[must_use]
pub fn IsAppThemed() -> bool {
	unsafe { ffi::IsAppThemed() != 0 }
}

/// [`IsCompositionActive`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-iscompositionactive)
/// function.
#[must_use]
pub fn IsCompositionActive() -> bool {
	unsafe { ffi::IsCompositionActive() != 0 }
}

/// [`IsThemeDialogTextureEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemedialogtextureenabled)
/// function.
///
/// **Note:** This function doesn't exist in x32.
#[cfg(target_pointer_width = "64")]
#[must_use]
pub fn IsThemeDialogTextureEnabled() -> bool {
	unsafe { ffi::IsThemeDialogTextureEnabled() != 0 }
}
