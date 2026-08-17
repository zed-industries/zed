#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDropTarget`](crate::IDropTarget) virtual table.
#[repr(C)]
pub struct IDropTargetVT {
	pub IUnknownVT: IUnknownVT,
	pub DragEnter: fn(COMPTR, COMPTR, u32, u64, *mut u32) -> HRES,
	pub DragOver: fn(COMPTR, u32, u64, *mut u32) -> HRES,
	pub DragLeave: fn(COMPTR) -> HRES,
	pub Drop: fn(COMPTR, COMPTR, u32, u64, *mut u32) -> HRES,
}

com_interface! { IDropTarget: "00000122-0000-0000-c000-000000000046";
	/// [`IDropTarget`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nn-oleidl-idroptarget)
	/// COM interface over [`IDropTargetVT`](crate::vt::IDropTargetVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IDropTarget for IDropTarget {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IDropTarget`](crate::IDropTarget).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IDropTarget: ole_IUnknown {
	/// [`IDropTarget::DragEnter`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragenter)
	/// method.
	fn DragEnter(&self,
		data_obj: &impl ole_IDataObject,
		key_state: co::MK,
		pt: POINT,
		effect: co::DROPEFFECT,
	) -> HrResult<co::DROPEFFECT>
	{
		let mut effect_buf = effect;
		ok_to_hrresult(
			unsafe {
				(vt::<IDropTargetVT>(self).DragEnter)(
					self.ptr(),
					data_obj.ptr(),
					key_state.raw() as _,
					MAKEQWORD(pt.x as _, pt.y as _),
					&mut effect_buf as *mut _ as _,
				)
			},
		).map(|_| effect_buf)
	}

	fn_com_noparm! { DragLeave: IDropTargetVT;
		/// [`IDropTarget::DragLeave`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragleave)
		/// method.
	}

	/// [`IDropTarget::DragOver`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-dragover)
	/// method.
	fn DragOver(&self,
		key_state: co::MK,
		pt: POINT,
		effect: co::DROPEFFECT,
	) -> HrResult<co::DROPEFFECT>
	{
		let mut effect_buf = effect;
		ok_to_hrresult(
			unsafe {
				(vt::<IDropTargetVT>(self).DragOver)(
					self.ptr(),
					key_state.raw() as _,
					MAKEQWORD(pt.x as _, pt.y as _),
					&mut effect_buf as *mut _ as _,
				)
			},
		).map(|_| effect_buf)
	}

	/// [`IDropTarget::Drop`](https://learn.microsoft.com/en-us/windows/win32/api/oleidl/nf-oleidl-idroptarget-drop)
	/// method.
	fn Drop(&self,
		data_obj: &impl ole_IDataObject,
		key_state: co::MK,
		pt: POINT,
		effect: co::DROPEFFECT,
	) -> HrResult<co::DROPEFFECT>
	{
		let mut effect_buf = effect;
		ok_to_hrresult(
			unsafe {
				(vt::<IDropTargetVT>(self).Drop)(
					self.ptr(),
					data_obj.ptr(),
					key_state.raw() as _,
					MAKEQWORD(pt.x as _, pt.y as _),
					&mut effect_buf as *mut _ as _,
				)
			},
		).map(|_| effect_buf)
	}
}
