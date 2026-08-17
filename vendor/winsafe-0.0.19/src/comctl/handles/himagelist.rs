#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::comctl::ffi;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl_handle! { HIMAGELIST;
	/// Handle to an
	/// [image list](https://learn.microsoft.com/en-us/windows/win32/controls/image-lists).
}

impl comctl_Himagelist for HIMAGELIST {}

/// This trait is enabled with the `comctl` feature, and provides methods for
/// [`HIMAGELIST`](crate::HIMAGELIST).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait comctl_Himagelist: Handle {
	/// [`ImageList_Add`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_add)
	/// function.
	///
	/// A copy of the bitmap is made and stored in the image list, so you're
	/// free to release the original bitmap.
	fn Add(&self,
		hbmp_image: &HBITMAP,
		hbmp_mask: Option<&HBITMAP>,
	) -> SysResult<u32>
	{
		match unsafe {
			ffi::ImageList_Add(
				self.ptr(),
				hbmp_image.ptr(),
				hbmp_mask.map_or(std::ptr::null_mut(), |h| h.ptr()),
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_AddIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addicon)
	/// function.
	///
	/// A copy of the icon is made and stored in the image list, so you're free
	/// to release the original icon.
	fn AddIcon(&self, hicon: &HICON) -> SysResult<u32> {
		self.ReplaceIcon(None, hicon)
	}

	/// [`ImageList_AddMasked`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// function.
	///
	/// A copy of the bitmap is made and stored in the image list, so you're
	/// free to release the original bitmap.
	fn AddMasked(&self,
		hbmp_image: &HBITMAP,
		color_mask: COLORREF,
	) -> SysResult<u32>
	{
		match unsafe {
			ffi::ImageList_AddMasked(
				self.ptr(), hbmp_image.ptr(), color_mask.into(),
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_BeginDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_begindrag)
	/// function.
	///
	/// In the original C implementation, you must call
	/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
	/// as a cleanup operation.
	///
	/// Here, the cleanup is performed automatically, because `BeginDrag`
	/// returns an
	/// [`ImageListEndDragGuard`](crate::guard::ImageListEndDragGuard), which
	/// automatically calls `ImageList_EndDrag` when the guard goes out of
	/// scope. You must, however, keep the guard alive, otherwise the cleanup
	/// will be performed right away.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let himgl: w::HIMAGELIST; // initialized somewhere
	/// # let himgl = w::HIMAGELIST::NULL;
	///
	/// let _drag = himgl.BeginDrag(0, w::POINT::new(0, 0))?; // keep guard alive
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn BeginDrag(&self,
		itrack: u32,
		hotspot: POINT,
	) -> SysResult<ImageListEndDragGuard<'_>>
	{
		unsafe {
			bool_to_sysresult(
				ffi::ImageList_BeginDrag(
					self.ptr(),
					itrack as _,
					hotspot.x, hotspot.y,
				),
			).map(|_| ImageListEndDragGuard::new(PhantomData))
		}
	}

	/// [`ImageList_Create`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_create)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let himgl = w::HIMAGELIST::Create(
	///     w::SIZE::new(16, 16),
	///     co::ILC::COLOR32,
	///     1,
	///     1,
	/// )?;
	///
	/// // ImageList_Destroy() automatically called
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn Create(
		image_sz: SIZE,
		flags: co::ILC,
		initial_size: i32,
		grow_size: i32,
	) -> SysResult<ImageListDestroyGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::ImageList_Create(
					image_sz.cx, image_sz.cy,
					flags.raw(),
					initial_size,
					grow_size,
				)
			).map(|h| ImageListDestroyGuard::new(h))
		}
	}

	/// [`ImageList_DragMove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// function.
	fn DragMove(&self, x: i32, y: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ImageList_DragMove(self.ptr(), x, y) })
	}

	/// [`ImageList_DragShowNolock`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// function.
	fn DragShowNolock(show: bool) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ImageList_DragShowNolock(show as _) })
	}

	/// [`ImageList_GetIconSize`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticonsize)
	/// function.
	#[must_use]
	fn GetIconSize(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				ffi::ImageList_GetIconSize(self.ptr(), &mut sz.cx, &mut sz.cy)
			}
		).map(|_| sz)
	}

	/// [`ImageList_GetImageCount`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// function.
	#[must_use]
	fn GetImageCount(&self) -> u32 {
		unsafe { ffi::ImageList_GetImageCount(self.ptr()) as _ }
	}

	/// [`ImageList_Remove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// function.
	fn Remove(&self, index: Option<u32>) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::ImageList_Remove(self.ptr(), index.map_or(-1, |i| i as _))
			},
		)
	}

	/// [`ImageList_ReplaceIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_replaceicon)
	/// function.
	///
	/// Note that a copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	fn ReplaceIcon(&self,
		index: Option<u32>,
		hicon_new: &HICON,
	) -> SysResult<u32>
	{
		match unsafe {
			ffi::ImageList_ReplaceIcon(
				self.ptr(),
				index.map_or(-1, |i| i as _),
				hicon_new.ptr(),
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_SetImageCount`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_setimagecount)
	/// function.
	fn SetImageCount(&self, new_count: u32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ImageList_SetImageCount(self.ptr(), new_count) },
		)
	}
}
