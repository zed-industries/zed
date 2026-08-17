#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMoniker`](crate::IMoniker) virtual table.
#[repr(C)]
pub struct IMonikerVT {
	pub IPersistStreamVT: IPersistStreamVT,
	pub BindToObject: fn(COMPTR, COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub BindToStorage: fn(COMPTR, COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub Reduce: fn(COMPTR, COMPTR, u32, *mut COMPTR, *mut COMPTR) -> HRES,
	pub ComposeWith: fn(COMPTR, COMPTR, BOOL, *mut COMPTR) -> HRES,
	pub Enum: fn(COMPTR, BOOL, *mut COMPTR) -> HRES,
	pub IsEqual: fn(COMPTR, COMPTR) -> HRES,
	pub Hash: fn(COMPTR, *mut u32) -> HRES,
	pub IsRunning: fn(COMPTR, COMPTR, COMPTR, COMPTR) -> HRES,
	pub GetTimeOfLastChange: fn(COMPTR, COMPTR, COMPTR, PVOID) -> HRES,
	pub Inverse: fn(COMPTR, *mut COMPTR) -> HRES,
	pub CommonPrefixWith: fn(COMPTR, COMPTR, *mut COMPTR) -> HRES,
	pub RelativePathTo: fn(COMPTR, COMPTR, *mut COMPTR) -> HRES,
	pub GetDisplayName: fn(COMPTR, COMPTR, COMPTR, *mut PSTR) -> HRES,
	pub ParseDisplayName: fn(COMPTR, COMPTR, COMPTR, PCSTR, *mut u32, *mut COMPTR) -> HRES,
	pub IsSystemMoniker: fn(COMPTR, *mut u32) -> HRES,
}

com_interface! { IMoniker: "0000000f-0000-0000-c000-000000000046";
	/// [`IMoniker`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-imoniker)
	/// COM interface over [`IMonikerVT`](crate::vt::IMonikerVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IMoniker {}
impl ole_IPersistStream for IMoniker {}
impl ole_IMoniker for IMoniker {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IMoniker`](crate::IMoniker).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IMoniker: ole_IPersistStream {
	/// [`IMoniker::BindToObject`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-bindtoobject)
	/// method.
	#[must_use]
	fn BindToObject<T>(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).BindToObject)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMoniker::BindToStorage`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-bindtostorage)
	/// method.
	#[must_use]
	fn BindToStorage<T>(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).BindToStorage)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMoniker::CommonPrefixWith`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-commonprefixwith)
	/// method.
	#[must_use]
	fn CommonPrefixWith(&self, other: &impl ole_IMoniker) -> HrResult<IMoniker> {
		let mut queried = unsafe { IMoniker::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).CommonPrefixWith)(
					self.ptr(),
					other.ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMoniker::ComposeWith`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-composewith)
	/// method.
	#[must_use]
	fn ComposeWith(&self,
		moniker_to_right: &impl ole_IMoniker,
		only_if_not_generic: bool,
	) -> HrResult<IMoniker>
	{
		let mut queried = unsafe { IMoniker::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).ComposeWith)(
					self.ptr(),
					moniker_to_right.ptr(),
					only_if_not_generic as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMoniker::Enum`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-enum)
	/// method.
	#[must_use]
	fn Enum(&self, forward: bool) -> HrResult<IMoniker> {
		let mut queried = unsafe { IMoniker::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).Enum)(
					self.ptr(),
					forward as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMoniker::GetDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-getdisplayname)
	/// method.
	#[must_use]
	fn GetDisplayName(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<String>
	{
		let mut pstr = std::ptr::null_mut::<u16>();
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).GetDisplayName)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
					&mut pstr,
				)
			},
		).map(|_| {
			let name = unsafe { WString::from_wchars_nullt(pstr) };
			let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) }; // https://stackoverflow.com/q/3079508/6923555
			name.to_string()
		})
	}

	/// [`IMoniker::GetTimeOfLastChange`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-gettimeoflastchange)
	/// method.
	#[must_use]
	fn GetTimeOfLastChange(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
	) -> HrResult<FILETIME>
	{
		let mut ft = FILETIME::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).GetTimeOfLastChange)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
					&mut ft as *mut _ as _,
				)
			},
		).map(|_| ft)
	}

	/// [`IMoniker::Hash`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-hash)
	/// method.
	#[must_use]
	fn Hash(&self) -> HrResult<u32> {
		let mut hash = u32::default();
		ok_to_hrresult(
			unsafe { (vt::<IMonikerVT>(self).Hash)(self.ptr(), &mut hash) },
		).map(|_| hash)
	}

	fn_com_interface_get! { Inverse: IMonikerVT, IMoniker;
		/// [`IMoniker::Inverse`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-inverse)
		/// method.
	}

	/// [`IMoniker::IsEqual`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-isequal)
	/// method.
	#[must_use]
	fn IsEqual(&self, other_moniker: &impl ole_IMoniker) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).IsEqual)(self.ptr(), other_moniker.ptr())
			},
		)
	}

	/// [`IMoniker::IsRunning`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-isrunning)
	/// method.
	#[must_use]
	fn IsRunning(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: Option<&impl ole_IMoniker>,
		moniker_newly_running: Option<&impl ole_IMoniker>,
	) -> HrResult<bool>
	{
		okfalse_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).IsRunning)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.map_or(std::ptr::null_mut(), |m| m.ptr()),
					moniker_newly_running.map_or(std::ptr::null_mut(), |m| m.ptr()),
				)
			},
		)
	}

	/// [`IMoniker::IsSystemMoniker](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-issystemmoniker)
	/// method.
	#[must_use]
	fn IsSystemMoniker(&self) -> HrResult<(bool, co::MKSYS)> {
		let mut mksys = co::MKSYS::default();
		okfalse_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).IsSystemMoniker)(
					self.ptr(),
					mksys.as_mut(),
				)
			},
		).map(|b| (b, mksys))
	}

	/// [`IMoniker::ParseDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-parsedisplayname)
	/// method.
	#[must_use]
	fn ParseDisplayName(&self,
		bind_ctx: &impl ole_IBindCtx,
		moniker_to_left: &impl ole_IMoniker,
		display_name: &str,
	) -> HrResult<(u32, IMoniker)>
	{
		let mut ch_eaten = u32::default();
		let mut queried = unsafe { IMoniker::null() };

		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).ParseDisplayName)(
					self.ptr(),
					bind_ctx.ptr(),
					moniker_to_left.ptr(),
					WString::from_str(display_name).as_ptr(),
					&mut ch_eaten,
					queried.as_mut(),
				)
			},
		).map(|_| (ch_eaten, queried))
	}

	/// [`IMoniker::Reduce`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-reduce)
	/// method.
	///
	/// Returns the moniker to the left, and the reduced moniker, respectively.
	#[must_use]
	fn Reduce(&self,
		bind_ctx: &impl ole_IBindCtx,
		reduce_how_far: co::MKRREDUCE,
	) -> HrResult<(IMoniker, IMoniker)>
	{
		let (mut queried, mut queried2) = unsafe {(
			IMoniker::null(),
			IMoniker::null(),
		)};

		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).Reduce)(
					self.ptr(),
					bind_ctx.ptr(),
					reduce_how_far.raw(),
					queried.as_mut(),
					queried2.as_mut(),
				)
			},
		).map(|_| (queried, queried2))
	}

	/// [`IMoniker::RelativePathTo`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-imoniker-relativepathto)
	/// method.
	#[must_use]
	fn RelativePathTo(&self,
		other_moniker: &impl ole_IMoniker,
	) -> HrResult<IMoniker>
	{
		let mut queried = unsafe { IMoniker::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMonikerVT>(self).RelativePathTo)(
					self.ptr(),
					other_moniker.ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}
}
