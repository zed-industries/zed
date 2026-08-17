#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IShellItem`](crate::IShellItem) virtual table.
#[repr(C)]
pub struct IShellItemVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(COMPTR, PVOID, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub GetParent: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetDisplayName: fn(COMPTR, u32, *mut PSTR) -> HRES,
	pub GetAttributes: fn(COMPTR, u32, *mut u32) -> HRES,
	pub Compare: fn(COMPTR, PVOID, u32, *mut i32) -> HRES,
}

com_interface! { IShellItem: "43826d1e-e718-42ee-bc55-a1e261c37bfe";
	/// [`IShellItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
	/// COM interface over [`IShellItemVT`](crate::vt::IShellItemVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`SHCreateItemFromParsingName`](crate::SHCreateItemFromParsingName)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl shell_IShellItem for IShellItem {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItem`](crate::IShellItem).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItem: ole_IUnknown {
	/// [`IShellItem::BindToHandler`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-bindtohandler)
	/// method.
	///
	/// # Examples
	///
	/// Retrieving the items inside a directory:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let sh_folder: w::IShellItem; // initialized somewhere
	/// # let sh_folder = unsafe { w::IShellItem::null() };
	///
	/// let sh_items = sh_folder.BindToHandler::<w::IEnumShellItems>(
	///     None::<&w::IBindCtx>,
	///     &co::BHID::EnumItems,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn BindToHandler<T>(&self,
		bind_ctx: Option<&impl ole_IBindCtx>,
		bhid: &co::BHID,
	) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItemVT>(self).BindToHandler)(
					self.ptr(),
					bind_ctx.map_or(std::ptr::null_mut(), |i| i.ptr() as _),
					bhid as *const _ as _,
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IShellItem::Compare`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-compare)
	/// method.
	#[must_use]
	fn Compare(&self,
		other: &impl shell_IShellItem,
		hint: co::SICHINTF,
	) -> HrResult<i32>
	{
		let mut order = i32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItemVT>(self).Compare)(
					self.ptr(),
					other.ptr(),
					hint.raw(),
					&mut order,
				)
			},
		).map(|_| order)
	}

	/// [`IShellItem::GetAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
	/// method.
	#[must_use]
	fn GetAttributes(&self, sfgao_mask: co::SFGAO) -> HrResult<co::SFGAO> {
		let mut attrs = u32::default();
		match unsafe {
			co::HRESULT::from_raw(
				(vt::<IShellItemVT>(self).GetAttributes)(
					self.ptr(),
					sfgao_mask.raw(),
					&mut attrs,
				),
			)
		} {
			co::HRESULT::S_OK
			| co::HRESULT::S_FALSE => Ok(unsafe { co::SFGAO::from_raw(attrs) }),
			hr => Err(hr),
		}
	}

	/// [`IShellItem::GetDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getdisplayname)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	///
	/// let full_path = shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///
	/// println!("{}", full_path);
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn GetDisplayName(&self, sigdn_name: co::SIGDN) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItemVT>(self).GetDisplayName)(
					self.ptr(),
					sigdn_name.raw(),
					&mut pstr,
				)
			},
		).map(|_| {
			let name = unsafe { WString::from_wchars_nullt(pstr) };
			let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) };
			name.to_string()
		})
	}

	fn_com_interface_get! { GetParent: IShellItemVT, IShellItem;
		/// [`IShellItem::GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getparent)
		/// method.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, co};
		///
		/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem>(
		///     "C:\\Temp\\foo.txt",
		///     None::<&w::IBindCtx>,
		/// )?;
		///
		/// let parent_shi = shi.GetParent()?;
		/// let full_path = parent_shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
		///
		/// println!("{}", full_path);
		/// # Ok::<_, co::HRESULT>(())
		/// ```
	}
}
