#![allow(non_camel_case_types, non_snake_case)]

use std::any::Any;

use crate::prelude::*;

/// This trait is enabled with the `gdi` feature, and implements methods for any
/// [`HGDIOBJ`](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj)
/// handle, which is the base handle for
/// [GDI objects](https://learn.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GdiObject: Handle + Any {}

/// This trait is enabled with the `gdi` feature, and implements methods for any
/// [`HGDIOBJ`](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj)
/// handle which can be passed to
/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GdiObjectSelect: GdiObject {}
