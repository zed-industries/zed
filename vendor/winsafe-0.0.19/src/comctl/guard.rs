use std::marker::PhantomData;

use crate::comctl::ffi;
use crate::decl::*;
use crate::prelude::*;

handle_guard! { ImageListDestroyGuard: HIMAGELIST;
	ffi::ImageList_Destroy;
	/// RAII implementation for [`HIMAGELIST`](crate::HIMAGELIST) which
	/// automatically calls
	/// [`ImageList_Destroy`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_destroy)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

/// RAII implementation for image list drag which automatically calls
/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
/// when the object goes out of scope.
pub struct ImageListEndDragGuard<'a> {
	_himagelist: PhantomData<&'a ()>,
}

impl<'a> Drop for ImageListEndDragGuard<'a> {
	fn drop(&mut self) {
		unsafe { ffi::ImageList_EndDrag(); }
	}
}

impl<'a> ImageListEndDragGuard<'a> {
	/// Constructs the guard by taking ownership of the object.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(himagelist: PhantomData<&'a ()>) -> Self {
		Self { _himagelist: himagelist }
	}
}
