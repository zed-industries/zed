use std::ops::{Deref, DerefMut};

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::prelude::*;

handle_guard! { DeleteDCGuard: HDC;
	ffi::DeleteDC;
	/// RAII implementation for [`HDC`](crate::HDC) which automatically calls
	/// [`DeleteDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

/// RAII implementation for a [`GdiObject`](crate::prelude::GdiObject) which
/// automatically calls
/// [`DeleteObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
/// when the object goes out of scope.
pub struct DeleteObjectGuard<T>
	where T: GdiObject,
{
	handle: T,
}

impl<T> Drop for DeleteObjectGuard<T>
	where T: GdiObject,
{
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			unsafe { ffi::DeleteObject(h.ptr()); } // ignore errors
		}
	}
}

impl<T> Deref for DeleteObjectGuard<T>
	where T: GdiObject,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl<T> DerefMut for DeleteObjectGuard<T>
	where T: GdiObject,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.handle
	}
}

impl<T> DeleteObjectGuard<T>
	where T: GdiObject,
{
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`DeleteObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(handle: T) -> Self {
		Self { handle }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> T {
		std::mem::replace(&mut self.handle, T::INVALID)
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for [`LOGPALETTE`](crate::LOGPALETTE) which manages the
/// allocated memory.
pub struct LogpaletteGuard {
	ptr: GlobalFreeGuard,
}

impl Deref for LogpaletteGuard {
	type Target = LOGPALETTE;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.ptr.ptr() as *const _) }
	}
}

impl DerefMut for LogpaletteGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(self.ptr.ptr() as *mut _) }
	}
}

impl LogpaletteGuard {
	pub(in crate::gdi) fn new(
		pal_version: u16,
		entries: &[PALETTEENTRY],
	) -> SysResult<Self>
	{
		let sz = std::mem::size_of::<LOGPALETTE>() // size in bytes of the allocated struct
			- std::mem::size_of::<PALETTEENTRY>()
			+ (entries.len() * std::mem::size_of::<PALETTEENTRY>());
		let mut new_self = Self {
			ptr: HGLOBAL::GlobalAlloc(
				Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
				sz,
			)?,
		};
		new_self.palVersion = pal_version;
		new_self.palNumEntries = entries.len() as _;
		entries.iter()
			.zip(new_self.palPalEntry_mut())
			.for_each(|(src, dest)| *dest = *src); // copy all PALETTEENTRY into struct room
		Ok(new_self)
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for
/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject) calls, which
/// automatically selects the previous GDI object at the end of the scope.
pub struct SelectObjectGuard<'a, H, G>
	where H: gdi_Hdc,
		G: GdiObject,
{
	hdc: &'a H,
	prev_hgdi: G,
	region: Option<co::REGION>,
}

impl<'a, H, G> Drop for SelectObjectGuard<'a, H, G>
	where H: gdi_Hdc,
		G: GdiObject,
{
	fn drop(&mut self) {
		if let Some(h) = self.hdc.as_opt() {
			if let Some(g) = self.prev_hgdi.as_opt() {
				unsafe { ffi::SelectObject(h.ptr(), g.ptr()); } // ignore errors
			}
		}
	}
}

impl<'a, H, G> SelectObjectGuard<'a, H, G>
	where H: gdi_Hdc,
		G: GdiObject,
{
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be again passed to
	/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(
		hdc: &'a H,
		prev_hgdi: G,
		region: Option<co::REGION>,
	) -> Self
	{
		Self { hdc, prev_hgdi, region }
	}

	/// Returns a handle to the object that has been replaced.
	#[must_use]
	pub const fn prev_object(&self) -> &G {
		&self.prev_hgdi
	}

	/// Returns the region information returned by the source
	/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject) call, if
	/// the [`GdiObject`](crate::prelude::GdiObject) was an
	/// [`HRGN`](crate::HRGN); otherwise returns `None`.
	#[must_use]
	pub const fn region(&self) -> Option<co::REGION> {
		self.region
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> G {
		std::mem::replace(&mut self.prev_hgdi, G::INVALID)
	}
}
