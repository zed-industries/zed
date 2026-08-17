#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::{ffi, privs::*, proc};

impl_handle! { HWND;
	/// Handle to a
	/// [window](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
}

impl user_Hwnd for HWND {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hwnd: Handle {
	/// Represents all top-level windows in
	/// [`HWND::PostMessage`](crate::prelude::user_Hwnd::PostMessage) and
	/// [`HWND::SendMessage`](crate::prelude::user_Hwnd::SendMessage).
	const BROADCAST: HWND = HWND(0xffff as _);

	/// Represents the desktop window in
	/// [`HWND::GetDC`](crate::prelude::user_Hwnd::GetDC).
	const DESKTOP: HWND = HWND(std::ptr::null_mut());

	/// [`GetWindowLongPtr`](crate::prelude::user_Hwnd::GetWindowLongPtr)
	/// wrapper to retrieve the window [`HINSTANCE`](crate::HINSTANCE).
	#[must_use]
	fn hinstance(&self) -> HINSTANCE {
		unsafe {
			HINSTANCE::from_ptr(self.GetWindowLongPtr(co::GWLP::HINSTANCE) as _)
		}
	}

	/// [`ArrangeIconicWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-arrangeiconicwindows)
	/// function.
	fn ArrangeIconicWindows(&self) -> SysResult<u32> {
		match unsafe { ffi::ArrangeIconicWindows(self.ptr()) } {
			0 => Err(GetLastError()),
			height => Ok(height),
		}
	}

	/// [`BeginPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	/// function.
	///
	/// In the original C implementation, `BeginPaint` returns a handle which
	/// must be passed to
	/// [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint),
	/// as a cleanup operation. Also, you must allocate and pass a
	/// [`PAINTSTRUCT`](crate::PAINTSTRUCT) object.
	///
	/// Here, the cleanup is performed automatically, because `BeginPaint`
	/// returns an [`EndPaintGuard`](crate::guard::EndPaintGuard), which stores
	/// the `PAINTSTRUCT` and automatically calls `EndPaint` when the guard goes
	/// out of scope. You must, however, keep the guard alive, otherwise the
	/// cleanup will be performed right away.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let hdc = hwnd.BeginPaint()?;
	///
	/// // do your hdc painting...
	///
	/// // EndPaint() called automatically
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// If you don't use the returned device context handle, you must still keep
	/// the guard alive:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let _hdc = hwnd.BeginPaint()?; // keep guard alive
	///
	/// // do your hdc painting...
	///
	/// // EndPaint() called automatically
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn BeginPaint(&self) -> SysResult<EndPaintGuard<'_, Self>> {
		let mut ps = PAINTSTRUCT::default();
		unsafe {
			ptr_to_sysresult_handle(
				ffi::BeginPaint(self.ptr(), &mut ps as *mut _ as _),
			).map(|h| EndPaintGuard::new(self, h, ps))
		}
	}

	/// [`BringWindowToTop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop)
	/// function.
	fn BringWindowToTop(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::BringWindowToTop(self.ptr()) })
	}

	/// [`ChildWindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-childwindowfrompoint)
	/// function.
	#[must_use]
	fn ChildWindowFromPoint(&self, pt: POINT) -> Option<HWND> {
		ptr_to_option_handle(
			unsafe { ffi::ChildWindowFromPoint(self.ptr(), pt.x, pt.y) },
		)
	}

	/// [`ClientToScreen`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen)
	/// function.
	///
	/// If you need to convert a [`RECT`](crate::RECT), see the
	/// [`HWND::ClientToScreenRc`](crate::prelude::user_Hwnd::ClientToScreenRc)
	/// function.
	fn ClientToScreen(&self, pt: &mut POINT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ClientToScreen(self.ptr(), pt as *mut _ as _) },
		)
	}

	/// [`ClientToScreen`](crate::prelude::user_Hwnd::ClientToScreen) method for
	/// a [`RECT`](crate::RECT).
	fn ClientToScreenRc(&self, rc: &mut RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::ClientToScreen(self.ptr(), &mut rc.left as *mut _ as _)
			},
		)?;
		bool_to_sysresult(
			unsafe {
				ffi::ClientToScreen(self.ptr(), &mut rc.right as *mut _ as _)
			},
		)
	}

	/// [`CloseWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
	/// function.
	///
	/// Note that this method will actually minimize the window, not destroy it.
	fn CloseWindow(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::CloseWindow(self.ptr()) })
	}

	/// [`CreateWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// function.
	///
	/// # Safety
	///
	/// This method will create raw dynamic windows and controls outside the
	/// library safety – it's up to you to handle all the messages. You must use
	/// a properly registered class name and, if creating a custom window,
	/// provide its own window procedure.
	///
	/// The usable ID range for dynamic child controls goes from 1 to 19,999.
	/// IDs starting from 20,000 are used internally by the library, do not use
	/// them.
	unsafe fn CreateWindowEx(
		ex_style: co::WS_EX,
		class_name: AtomStr,
		title: Option<&str>,
		style: co::WS,
		pos: POINT,
		size: SIZE,
		hwnd_parent: Option<&HWND>,
		hmenu: IdMenu,
		hinstance: &HINSTANCE,
		lparam: Option<isize>,
	) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			ffi::CreateWindowExW(
				ex_style.raw(),
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
				style.raw(),
				pos.x, pos.y,
				size.cx, size.cy,
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
				hmenu.as_ptr(),
				hinstance.ptr(),
				lparam.unwrap_or_default() as _,
			),
		)
	}

	/// [`DefWindowProc`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// function.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	fn DefWindowProc<M>(&self, msg: M) -> M::RetType
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				ffi::DefWindowProcW(
					self.ptr(), wm_any.msg_id.raw(), wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`DestroyWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// function.
	///
	/// Usually you don't need to call this method directly, since it's
	/// automatically called inside the internal message loop. The ordinary way
	/// to close a window is sending a [`wm::Close`](crate::msg::wm::Close)
	/// message.
	fn DestroyWindow(&self) -> SysResult<()> {
		bool_to_sysresult( unsafe { ffi::DestroyWindow(self.ptr()) })
	}

	/// [`DragDetect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dragdetect)
	/// function.
	#[must_use]
	fn DragDetect(&self, pt: POINT) -> bool {
		unsafe { ffi::DragDetect(self.ptr(), pt.x, pt.y) != 0 }
	}

	/// [`DrawCaption`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawcaption)
	/// function.
	fn DrawCaption(&self,
		hdc: &HDC,
		rect: &RECT,
		flags: Option<co::DC>,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::DrawCaption(
					self.ptr(),
					hdc.ptr(),
					rect as *const _ as _,
					flags.unwrap_or_default().raw(),
				)
			},
		)
	}

	/// [`DrawMenuBar`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawmenubar)
	/// function.
	fn DrawMenuBar(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::DrawMenuBar(self.ptr()) })
	}

	/// [`EnableScrollBar`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablescrollbar)
	/// function.
	fn EnableScrollBar(&self,
		sb_flags: co::SBB,
		arrows: co::ESB,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::EnableScrollBar(
					self.ptr(),
					sb_flags.raw() as _,
					arrows.raw(),
				)
			},
		)
	}

	/// [`EnableWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// function.
	fn EnableWindow(&self, enable: bool) -> bool {
		unsafe { ffi::EnableWindow(self.ptr(), enable as _) != 0 }
	}

	/// [`EndDialog`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// function.
	fn EndDialog(&self, result: isize) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::EndDialog(self.ptr(), result) })
	}

	/// [`EnumChildWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.EnumChildWindows(|hchild: w::HWND| -> bool {
	///     println!("Child HWND: {}", hchild);
	///     true
	/// });
	/// ```
	fn EnumChildWindows<F>(&self, func: F)
		where F: FnMut(HWND) -> bool,
	{
		unsafe {
			ffi::EnumChildWindows(
				self.ptr(),
				proc::hwnd_enum_child_windows::<F> as _, // https://redd.it/npehj9
				&func as *const _ as _,
			);
		}
	}

	/// [`FindWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// function.
	#[must_use]
	fn FindWindow(
		class_name: Option<AtomStr>,
		title: Option<&str>,
	) -> SysResult<Option<HWND>>
	{
		let ptr = unsafe {
			ffi::FindWindowW(
				class_name.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
				WString::from_opt_str(title).as_ptr(),
			)
		};

		if ptr.is_null() {
			match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no window found
				err => Err(err), // actual error
			}
		} else {
			Ok(Some(unsafe { HWND::from_ptr(ptr) }))
		}
	}

	/// [`FindWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw)
	/// function.
	#[must_use]
	fn FindWindowEx(&self,
		hwnd_child_after: Option<&HWND>,
		class_name: AtomStr,
		title: Option<&str>,
	) -> SysResult<Option<HWND>>
	{
		let ptr = unsafe {
			ffi::FindWindowExW(
				self.ptr(),
				hwnd_child_after.map_or(std::ptr::null_mut(), |h| h.ptr()),
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
			)
		};

		if ptr.is_null() {
			match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no window found
				err => Err(err), // actual error
			}
		} else {
			Ok(Some(unsafe { HWND::from_ptr(ptr) }))
		}
	}

	/// [`GetActiveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getactivewindow)
	/// function.
	#[must_use]
	fn GetActiveWindow() -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::GetActiveWindow() })
	}

	/// [`GetAltTabInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getalttabinfow)
	/// function.
	///
	/// If `item` is `None`, the item text is not retrieved.
	///
	/// The `sz_item_text` is the maximum number of expected chars for the item
	/// text. If `None`, defaults to 100.
	fn GetAltTabInfo(&self,
		item: Option<u32>,
		ati: &mut ALTTABINFO,
		sz_item_text: Option<u32>,
	) -> SysResult<String>
	{
		let buf_sz = sz_item_text.unwrap_or(100) + 1;
		let mut buf = match item {
			None => WString::default(),
			Some(_) => WString::new_alloc_buf(buf_sz as _), // room for terminating null
		};

		bool_to_sysresult(
			unsafe {
				ffi::GetAltTabInfoW(
					self.ptr(),
					item.map_or(-1, |item| item as i32),
					ati as *mut _ as _,
					item.map_or(std::ptr::null_mut(), |_| buf.as_mut_ptr()),
					buf_sz,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`GetAncestor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// function.
	#[must_use]
	fn GetAncestor(&self, flags: co::GA) -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::GetAncestor(self.ptr(), flags.raw()) })
	}

	/// [`GetCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcapture)
	/// function.
	#[must_use]
	fn GetCapture() -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::GetCapture() })
	}

	/// [`GetClassLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclasslongptrw)
	/// function.
	#[must_use]
	fn GetClassLongPtr(&self, index: co::GCLP) -> usize {
		unsafe { ffi::GetClassLongPtrW(self.ptr(), index.raw()) }
	}

	/// [`GetClassName`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassnamew)
	/// function.
	#[must_use]
	fn GetClassName(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(256 + 1); // according to WNDCLASSEX docs
		bool_to_sysresult(
			unsafe {
				ffi::GetClassNameW(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`GetClientRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
	/// function.
	#[must_use]
	fn GetClientRect(&self) -> SysResult<RECT> {
		let mut rc = RECT::default();
		bool_to_sysresult(
			unsafe { ffi::GetClientRect(self.ptr(), &mut rc as *mut _ as _) },
		).map(|_| rc)
	}

	/// [`GetDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// function.
	///
	/// # Examples
	///
	/// Retrieving the device context of the desktop window:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hdc_desktop = w::HWND::DESKTOP.GetDC()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn GetDC(&self) -> SysResult<ReleaseDCGuard<'_, Self>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::GetDC(self.ptr()))
				.map(|h| ReleaseDCGuard::new(self, h))
		}
	}

	/// [`GetDesktopWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)
	/// function.
	#[must_use]
	fn GetDesktopWindow() -> HWND {
		unsafe { HWND::from_ptr(ffi::GetDesktopWindow()) }
	}

	/// [`GetDlgCtrlID`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgctrlid)
	/// function.
	#[must_use]
	fn GetDlgCtrlID(&self) -> SysResult<u16> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { ffi::GetDlgCtrlID(self.ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual ID is zero
				err => Err(err),
			},
			id => Ok(id as _),
		}
	}

	/// [`GetDlgItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgitem)
	/// function.
	#[must_use]
	fn GetDlgItem(&self, ctrl_id: u16) -> SysResult<HWND> {
		ptr_to_sysresult_handle(
			unsafe { ffi::GetDlgItem(self.ptr(), ctrl_id as _) },
		)
	}

	/// [`GetDpiForWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdpiforwindow)
	/// function.
	#[must_use]
	fn GetDpiForWindow(&self) -> u32 {
		unsafe { ffi::GetDpiForWindow(self.ptr()) }
	}

	/// [`GetFocus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// function.
	#[must_use]
	fn GetFocus() -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::GetFocus() })
	}

	/// [`GetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// function.
	#[must_use]
	fn GetForegroundWindow() -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::GetForegroundWindow() })
	}

	/// [`GetLastActivePopup`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getlastactivepopup)
	/// function.
	#[must_use]
	fn GetLastActivePopup(&self) -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::GetLastActivePopup(self.ptr()) })
	}

	/// [`GetMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenu)
	/// function.
	#[must_use]
	fn GetMenu(&self) -> Option<HMENU> {
		ptr_to_option_handle(unsafe { ffi::GetMenu(self.ptr()) })
	}

	/// [`GetMenuBarInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenubarinfo)
	/// function.
	fn GetMenuBarInfo(&self,
		obj_id: co::OBJID,
		item_id: u32,
		mbi: &mut MENUBARINFO,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::GetMenuBarInfo(
					self.ptr(),
					obj_id.raw() as _,
					item_id as _,
					mbi as *mut _ as _,
				)
			},
		)
	}

	/// [`GetMenuItemRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemrect)
	/// function.
	fn GetMenuItemRect(&self,
		hmenu: &HMENU,
		item_pos: u32,
		rc_item: &mut RECT,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::GetMenuItemRect(
					self.ptr(),
					hmenu.ptr(),
					item_pos,
					rc_item as *mut _ as _,
				)
			},
		)
	}

	/// [`GetNextDlgGroupItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// function.
	#[must_use]
	fn GetNextDlgGroupItem(&self,
		hwnd_ctrl: &HWND,
		previous: bool,
	) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			unsafe {
				ffi::GetNextDlgGroupItem(self.ptr(), hwnd_ctrl.ptr(), previous as _)
			},
		)
	}

	/// [`GetNextDlgTabItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// function.
	#[must_use]
	fn GetNextDlgTabItem(&self,
		hwnd_ctrl: &HWND,
		previous: bool,
	) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			unsafe {
				ffi::GetNextDlgTabItem(
					self.ptr(),
					hwnd_ctrl.ptr(),
					previous as _,
				)
			},
		)
	}

	/// [`GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// function.
	#[must_use]
	fn GetParent(&self) -> SysResult<HWND> {
		ptr_to_sysresult_handle(unsafe { ffi::GetParent(self.ptr()) })
	}

	/// [`GetScrollInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollinfo)
	/// function.
	fn GetScrollInfo(&self,
		bar: co::SBB,
		si: &mut SCROLLINFO,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::GetScrollInfo(self.ptr(), bar.raw(), si as *mut _ as _)
			},
		)
	}

	/// [`GetScrollPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollpos)
	/// function.
	#[must_use]
	fn GetScrollPos(&self, bar: co::SBB) -> SysResult<i32> {
		match unsafe { ffi::GetScrollPos(self.ptr(), bar.raw()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`GetShellWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getshellwindow)
	/// function.
	#[must_use]
	fn GetShellWindow() -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::GetShellWindow() })
	}

	/// [`GetSystemMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmenu)
	/// function.
	#[must_use]
	fn GetSystemMenu(&self, revert: bool) -> Option<HMENU> {
		ptr_to_option_handle(
			unsafe { ffi::GetSystemMenu(self.ptr(), revert as _) },
		)
	}

	/// [`GetTopWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-gettopwindow)
	/// function.
	#[must_use]
	fn GetTopWindow(&self) -> SysResult<Option<HWND>> {
		match ptr_to_option_handle(unsafe { ffi::GetTopWindow(self.ptr()) }) {
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no child window
				err => Err(err),
			},
			Some(h) => Ok(Some(h)),
		}
	}

	/// [`GetUpdateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdaterect)
	/// function.
	#[must_use]
	fn GetUpdateRect(&self, erase: bool) -> Option<RECT> {
		let mut rc = RECT::default();
		zero_as_none(
			unsafe {
				ffi::GetUpdateRect(
					self.ptr(),
					&mut rc as *mut _ as _,
					erase as _,
				)
			} as _,
		).map(|_| rc)
	}

	/// [`GetUpdateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// function.
	#[must_use]
	fn GetUpdateRgn(&self, hrgn: &HRGN, erase: bool) -> SysResult<co::REGION> {
		match unsafe { ffi::GetUpdateRgn(self.ptr(), hrgn.ptr(), erase as _) } {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`GetWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// function.
	#[must_use]
	fn GetWindow(&self, cmd: co::GW) -> SysResult<HWND> {
		ptr_to_sysresult_handle( unsafe { ffi::GetWindow(self.ptr(), cmd.raw()) })
	}

	/// [`GetWindowDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// function.
	#[must_use]
	fn GetWindowDC(&self) -> SysResult<ReleaseDCGuard<'_, Self>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::GetWindowDC(self.ptr()))
				.map(|h| ReleaseDCGuard::new(self, h))
		}
	}

	/// [`GetWindowDisplayAffinity`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity)
	/// function.
	#[must_use]
	fn GetWindowDisplayAffinity(&self) -> SysResult<co::WDA> {
		let mut affinity = co::WDA::default();
		bool_to_sysresult(
			unsafe {
				ffi::GetWindowDisplayAffinity(
					self.ptr(),
					&mut affinity as *mut _ as _,
				)
			},
		).map(|_| affinity)
	}

	/// [`GetWindowInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// function.
	fn GetWindowInfo(&self, wi: &mut WINDOWINFO) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::GetWindowInfo(self.ptr(), wi as *mut _ as _) },
		)
	}

	/// [`GetWindowLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// function.
	#[must_use]
	fn GetWindowLongPtr(&self, index: co::GWLP) -> isize {
		#[cfg(target_pointer_width = "32")]
		unsafe { ffi::GetWindowLongW(self.ptr(), index.raw()) }

		#[cfg(target_pointer_width = "64")]
		unsafe { ffi::GetWindowLongPtrW(self.ptr(), index.raw()) }
	}

	/// [`GetWindowModuleFileName`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowmodulefilenamew)
	/// function.
	#[must_use]
	fn GetWindowModuleFileName(&self) -> String {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		unsafe {
			ffi::GetWindowModuleFileNameW(
				self.ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as u32 - 1,
			);
		}
		buf.to_string()
	}

	/// [`GetWindowPlacement`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// function.
	fn GetWindowPlacement(&self, wp: &mut WINDOWPLACEMENT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::GetWindowPlacement(self.ptr(), wp as *mut _ as _) },
		)
	}

	/// [`GetWindowRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// function.
	#[must_use]
	fn GetWindowRect(&self) -> SysResult<RECT> {
		let mut rc = RECT::default();
		bool_to_sysresult(
			unsafe { ffi::GetWindowRect(self.ptr(), &mut rc as *mut _ as _) },
		).map(|_| rc)
	}

	/// [`GetWindowRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// function.
	#[must_use]
	fn GetWindowRgn(&self, hrgn: &HRGN) -> SysResult<co::REGION> {
		match unsafe { ffi::GetWindowRgn(self.ptr(), hrgn.ptr()) } {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`GetWindowRgnBox`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// function.
	#[must_use]
	fn GetWindowRgnBox(&self, rc: &mut RECT) -> SysResult<co::REGION> {
		match unsafe { ffi::GetWindowRgnBox(self.ptr(), rc as *mut _ as _) } {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`GetWindowText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)
	/// function.
	///
	/// Calls
	/// [`GetWindowTextLength`](crate::prelude::user_Hwnd::GetWindowTextLength)
	/// and performs all necessary allocations, returning an ordinary
	/// [`String`](std::string::String).
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let text = hwnd.GetWindowText()?;
	/// println!("Text: {}", text);
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn GetWindowText(&self) -> SysResult<String> {
		let len = self.GetWindowTextLength()?;
		if len == 0 {
			return Ok(String::default()); // window has no text
		}

		let mut buf = WString::new_alloc_buf(len as usize + 1); // plus terminating null
		match unsafe {
			ffi::GetWindowTextW(self.ptr(), buf.as_mut_ptr(), len + 1)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(String::default()), // no chars copied for some reason
				err => Err(err),
			},
			_ => Ok(buf.to_string()),
		}
	}

	/// [`GetWindowTextLength`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthw)
	/// function.
	///
	/// Does not count the terminating null.
	///
	/// You usually don't need to call this method directly, since
	/// [`GetWindowText`](crate::prelude::user_Hwnd::GetWindowText) returns a
	/// [`String`](std::string::String), performing all necessary allocations.
	#[must_use]
	fn GetWindowTextLength(&self) -> SysResult<i32> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { ffi::GetWindowTextLengthW(self.ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero length
				err => Err(err),
			},
			len => Ok(len),
		}
	}

	/// [`GetWindowThreadProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)
	/// function.
	///
	/// Returns thread ID and process ID, respectively.
	#[must_use]
	fn GetWindowThreadProcessId(&self) -> (u32, u32) {
		let mut proc_id = u32::default();
		let thread_id = unsafe {
			ffi::GetWindowThreadProcessId(self.ptr(), &mut proc_id)
		};
		(thread_id, proc_id)
	}

	/// [`HiliteMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hilitemenuitem)
	/// function.
	fn HiliteMenuItem(&self,
		hmenu: &HMENU,
		id_or_pos: IdPos,
		hilite: bool,
	) -> bool
	{
		unsafe {
			ffi::HiliteMenuItem(
				self.ptr(),
				hmenu.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.mf_flag().raw()
					| if hilite { co::MF::HILITE } else { co::MF::UNHILITE }.raw(),
			) != 0
		}
	}

	/// [`InvalidateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect)
	/// function.
	///
	/// # Examples
	///
	/// Most of the time you'll just want update the entire client area:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.InvalidateRect(None, true)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn InvalidateRect(&self, rc: Option<&RECT>, erase: bool) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::InvalidateRect(
					self.ptr(),
					rc.map_or(std::ptr::null(), |lp| lp as *const _ as _),
					erase as _,
				)
			},
		)
	}

	/// [`InvalidateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// function.
	fn InvalidateRgn(&self, hrgn: &HRGN, erase: bool) {
		unsafe { ffi::InvalidateRgn(self.ptr(), hrgn.ptr(), erase as _); }
	}

	/// [`IsChild`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// function.
	#[must_use]
	fn IsChild(&self, hwnd_possible_child: &HWND) -> bool {
		unsafe { ffi::IsChild(self.ptr(), hwnd_possible_child.ptr()) != 0 }
	}

	/// [`IsDialogMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// function.
	#[must_use]
	fn IsDialogMessage(&self, msg: &mut MSG) -> bool {
		unsafe { ffi::IsDialogMessageW(self.ptr(), msg as *mut _ as _) != 0 }
	}

	/// [`IsIconic`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic)
	/// function.
	#[must_use]
	fn IsIconic(&self) -> bool {
		unsafe { ffi::IsIconic(self.ptr()) != 0 }
	}

	/// [`IsWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)
	/// function.
	#[must_use]
	fn IsWindow(&self) -> bool {
		unsafe { ffi::IsWindow(self.ptr()) != 0 }
	}

	/// [`IsWindowEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowenabled)
	/// function.
	#[must_use]
	fn IsWindowEnabled(&self) -> bool {
		unsafe { ffi::IsWindowEnabled(self.ptr()) != 0 }
	}

	/// [`IsWindowUnicode`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowunicode)
	/// function.
	#[must_use]
	fn IsWindowUnicode(&self) -> bool {
		unsafe { ffi::IsWindowUnicode(self.ptr()) != 0 }
	}

	/// [`IsWindowVisible`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible)
	/// function.
	#[must_use]
	fn IsWindowVisible(&self) -> bool {
		unsafe { ffi::IsWindowVisible(self.ptr()) != 0 }
	}

	/// [`IsZoomed`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed)
	/// function.
	#[must_use]
	fn IsZoomed(&self) -> bool {
		unsafe { ffi::IsZoomed(self.ptr()) != 0 }
	}

	/// [`KillTimer`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer)
	/// function.
	///
	/// This function ends the timer calls for the given timer ID. If you don't
	/// call this function, the timer calls will continue until the window is
	/// destroyed – at this point, any remaining timers will be automatically
	/// cleared.
	fn KillTimer(&self, event_id: usize) -> SysResult<()> {
		match unsafe { ffi::KillTimer(self.ptr(), event_id) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()),
				e => Err(e),
			}
			_ => Ok(()),
		}
	}

	/// [`LockWindowUpdate`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-lockwindowupdate)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// // Lock the window – only one window can be locked at a time.
	/// hwnd.LockWindowUpdate()?;
	///
	/// // After all operations, unlock the currently locked window.
	/// w::HWND::NULL.LockWindowUpdate()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn LockWindowUpdate(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::LockWindowUpdate(self.ptr()) })
	}

	/// [`LogicalToPhysicalPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-logicaltophysicalpoint)
	/// function.
	fn LogicalToPhysicalPoint(&self, pt: *mut POINT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::LogicalToPhysicalPoint(self.ptr(), pt as _) },
		)
	}

	/// [`MapDialogRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// function.
	fn MapDialogRect(&self, rc: &mut RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::MapDialogRect(self.ptr(), rc as *mut _ as _) },
		)
	}

	/// [`MapWindowPoints`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints)
	/// function.
	///
	/// This method can convert either a series of [`POINT`](crate::POINT)
	/// structs or a single [`RECT`](crate::RECT).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	/// let hwnd_dest: w::HWND;
	/// # let hwnd_dest = w::HWND::NULL;
	///
	/// let mut points = vec![w::POINT::default(), w::POINT::default()];
	///
	/// hwnd.MapWindowPoints(
	///     &hwnd_dest,
	///     w::PtsRc::Pts(&mut points),
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn MapWindowPoints(&self,
		hdest: &HWND,
		points: PtsRc,
	) -> SysResult<(i16, i16)>
	{
		let forced_pts = match points {
			PtsRc::Pts(pts) => pts,
			PtsRc::Rc(rc) => unsafe {
				std::slice::from_raw_parts_mut(rc as *mut _ as _, 2)
			},
		};

		SetLastError(co::ERROR::SUCCESS);
		match unsafe {
			ffi::MapWindowPoints(
				self.ptr(),
				hdest.ptr(),
				forced_pts.as_mut_ptr() as _,
				forced_pts.len() as _,
			)
		} {
			0 => Err(GetLastError()),
			n => Ok((LOWORD(n as _) as _, HIWORD(n as _) as _)),
		}
	}

	/// [`MessageBox`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// function.
	///
	/// Consider using the more modern
	/// [`HWND::TaskDialog`](crate::prelude::comctl_Hwnd::TaskDialog) method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.MessageBox("Hello, world", "title",
	///     co::MB::OKCANCEL | co::MB::ICONINFORMATION)?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	///
	/// Usually the message box has a valid parent window, however, if for some
	/// reason you don't have a window to serve as parent, you still can show a
	/// non-modal, parent-less message box by retrieving the desktop handle:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// w::HWND::GetDesktopWindow()
	///     .MessageBox("Hello, world", "Title", co::MB::ICONEXCLAMATION)?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn MessageBox(&self,
		text: &str,
		caption: &str,
		flags: co::MB,
	) -> SysResult<co::DLGID>
	{
		match unsafe {
			ffi::MessageBoxW(
				self.ptr(),
				WString::from_str(text).as_ptr(),
				WString::from_str(caption).as_ptr(),
				flags.raw(),
			)
		} {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::DLGID::from_raw(ret as _) }),
		}
	}

	/// [`MonitorFromWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromwindow)
	/// function.
	#[must_use]
	fn MonitorFromWindow(&self, flags: co::MONITOR) -> HMONITOR {
		unsafe {
			HMONITOR::from_ptr(ffi::MonitorFromWindow(self.ptr(), flags.raw()))
		}
	}

	/// [`MoveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow)
	/// function.
	fn MoveWindow(&self,
		pos: POINT,
		size: SIZE,
		repaint: bool,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::MoveWindow(
					self.ptr(),
					pos.x, pos.y,
					size.cx, size.cy,
					repaint as _,
				)
			},
		)
	}

	/// [`OpenClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard)
	/// function.
	///
	/// In the original C implementation, you must call
	/// [`CloseClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
	/// as a cleanup operation.
	///
	/// Here, the cleanup is performed automatically, because `OpenClipboard`
	/// returns a [`CloseClipboardGuard`](crate::guard::CloseClipboardGuard),
	/// which automatically calls `CloseClipboard` when the guard goes out of
	/// scope. You must, however, keep the guard alive, otherwise the cleanup
	/// will be performed right away.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let _hclip = hwnd.OpenClipboard()?; // keep guard alive
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// You can also open the clipboard without an `HWND` owner:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let _hclip = w::HWND::NULL.OpenClipboard()?; // keep guard alive
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn OpenClipboard(&self) -> SysResult<CloseClipboardGuard<'_>> {
		unsafe {
			bool_to_sysresult(ffi::OpenClipboard(self.ptr()))
				.map(|_| CloseClipboardGuard::new(PhantomData))
		}
	}

	/// [`PostMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)
	/// function.
	///
	/// Note that this method is asychronous.
	fn PostMessage<M>(&self, msg: M) -> SysResult<()>
		where M: MsgSend + Send + Copy + 'static,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		bool_to_sysresult(
			unsafe {
				ffi::PostMessageW(
					self.ptr(), wm_any.msg_id.raw(), wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`RealChildWindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realchildwindowfrompoint)
	/// function.
	#[must_use]
	fn RealChildWindowFromPoint(&self,
		pt_parent_client_coords: POINT,
	) -> Option<HWND>
	{
		ptr_to_option_handle(
			unsafe {
				ffi::RealChildWindowFromPoint(
					self.ptr(),
					pt_parent_client_coords.x,
					pt_parent_client_coords.y,
				)
			},
		)
	}

	/// [`RealGetWindowClass`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realgetwindowclassw)
	/// function.
	#[must_use]
	fn RealGetWindowClass(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(256 + 1); // according to WNDCLASSEX docs
		bool_to_sysresult(
			unsafe {
				ffi::RealGetWindowClassW(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				)
			} as _,
		).map(|_| buf.to_string())
	}

	/// [`RedrawWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow)
	/// function.
	fn RedrawWindow(&self,
		rc_update: &RECT,
		hrgn_update: &HRGN,
		flags: co::RDW,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::RedrawWindow(
					self.ptr(),
					rc_update as *const _ as _,
					hrgn_update.ptr(),
					flags.raw(),
				)
			},
		)
	}

	/// [`ScreenToClient`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
	/// function.
	///
	/// If you need to convert a [`RECT`](crate::RECT), see the
	/// [`HWND::ScreenToClientRc`](crate::prelude::user_Hwnd::ScreenToClientRc)
	/// function.
	fn ScreenToClient(&self, pt: &mut POINT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ScreenToClient(self.ptr(), pt as *mut _ as _) },
		)
	}

	/// [`ScreenToClient`](crate::prelude::user_Hwnd::ScreenToClient) method for
	/// a [`RECT`](crate::RECT).
	fn ScreenToClientRc(&self, rc: &mut RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::ScreenToClient(self.ptr(), &mut rc.left as *mut _ as _)
			},
		)?;
		bool_to_sysresult(
			unsafe {
				ffi::ScreenToClient(self.ptr(), &mut rc.right as *mut _ as _)
			},
		)
	}

	/// [`ScrollWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindowex)
	/// function.
	fn ScrollWindowEx(&self,
		dx: i32,
		dy: i32,
		client_area_portion: Option<&RECT>,
		clipping_rect: Option<&RECT>,
		hrgn_update: Option<&HRGN>,
		updated_boundaries: Option<&mut RECT>,
		flags: co::SCROLLW,
	) -> SysResult<co::REGION>
	{
		match unsafe {
			ffi::ScrollWindowEx(
				self.ptr(),
				dx, dy,
				client_area_portion.map_or(std::ptr::null(), |rc| rc as *const _ as _),
				clipping_rect.map_or(std::ptr::null(), |rc| rc as *const _ as _),
				hrgn_update.map_or(std::ptr::null_mut(), |h| h.ptr()),
				updated_boundaries.map_or(std::ptr::null_mut(), |rc| rc as *mut _ as _),
				flags.raw(),
			)
		} {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::REGION::from_raw(v) }),
		}
	}

	/// [`SendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// function.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	///
	/// # Examples
	///
	/// Sending a [`bm::GetImage`](crate::msg::bm::GetImage) button message,
	/// which demands an image type parameter. Note that this specific message
	/// can also return an error, which is handled with
	/// [`?`](https://doc.rust-lang.org/std/result/index.html#the-question-mark-operator-):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, msg};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let bmp = hwnd.SendMessage(
	///     msg::bm::GetImage {
	///         img_type: co::IMAGE_TYPE::BITMAP,
	///     },
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	///
	/// Sending an [`em::CharFromPos`](crate::msg::em::CharFromPos) edit message,
	/// which receives point coordinates and returns two values:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, msg};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let (char_pos, line_pos) = hwnd.SendMessage(
	///     msg::em::CharFromPos {
	///         coords: w::POINT::new(12, 20),
	///     },
	/// );
	/// ```
	fn SendMessage<M>(&self, msg: M) -> M::RetType
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				ffi::SendMessageW(
					self.ptr(), wm_any.msg_id.raw(), wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`SendMessageTimeout`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)
	/// function.
	fn SendMessageTimeout<M>(&self,
		msg: M,
		flags: co::SMTO,
		timeout_ms: u32,
	) -> SysResult<M::RetType>
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		let mut result = isize::default();

		bool_to_sysresult(
			unsafe {
				ffi::SendMessageTimeoutW(
					self.ptr(),
					wm_any.msg_id.raw(),
					wm_any.wparam,
					wm_any.lparam,
					flags.raw(),
					timeout_ms,
					&mut result,
				)
			} as _,
		).map(|_| msg.convert_ret(result))
	}

	/// [`SetActiveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setactivewindow)
	/// function.
	fn SetActiveWindow(&self) -> SysResult<HWND> {
		ptr_to_sysresult_handle(unsafe { ffi::SetActiveWindow(self.ptr()) })
	}

	/// [`SetCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcapture)
	/// function.
	fn SetCapture(&self) -> ReleaseCaptureGuard<'_, Self> {
		unsafe {
			ReleaseCaptureGuard::new(
				self,
				ffi::SetCapture(self.ptr())
					.as_mut()
					.map(|ptr| HWND::from_ptr(ptr)),
			)
		}
	}

	/// [`SetFocus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setfocus)
	/// function.
	fn SetFocus(&self) -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::SetFocus(self.ptr()) })
	}

	/// [`SetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow)
	/// function.
	fn SetForegroundWindow(&self) -> bool {
		unsafe { ffi::SetForegroundWindow(self.ptr()) != 0 }
	}

	/// [`SetLayeredWindowAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setlayeredwindowattributes)
	/// function.
	fn SetLayeredWindowAttributes(&self,
		transparency_color_key: COLORREF,
		alpha: u8,
		flags: co::LWA,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetLayeredWindowAttributes(
					self.ptr(),
					transparency_color_key.raw(),
					alpha,
					flags.raw(),
				)
			},
		)
	}

	/// [`SetMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
	/// function.
	fn SetMenu(&self, hmenu: &HMENU) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetMenu(self.ptr(), hmenu.ptr()) })
	}

	/// [`SetParent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// function.
	fn SetParent(&self, hwnd_new_parent: &HWND) -> SysResult<Option<HWND>> {
		match ptr_to_option_handle(
			unsafe { ffi::SetParent(self.ptr(), hwnd_new_parent.ptr()) },
		) {
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no previous parent
				err => Err(err),
			},
			Some(h) => Ok(Some(h)),
		}
	}

	/// [`SetScrollInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollinfo)
	/// function.
	fn SetScrollInfo(&self, bar: co::SBB, si: &SCROLLINFO, redraw: bool) -> i32 {
		unsafe {
			ffi::SetScrollInfo(
				self.ptr(),
				bar.raw(),
				si as *const _ as _,
				redraw as _,
			)
		}
	}

	/// [`SetScrollPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollpos)
	/// function.
	fn SetScrollPos(&self,
		b: co::SBB,
		pos: i32,
		redraw: bool,
	) -> SysResult<i32>
	{
		match unsafe {
			ffi::SetScrollPos(self.ptr(), b.raw(), pos, redraw as _)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`SetScrollRange`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollrange)
	/// function.
	fn SetScrollRange(&self,
		bar: co::SBB,
		min_pos: i32,
		max_pos: i32,
		redraw: bool,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetScrollRange(
					self.ptr(),
					bar.raw(),
					min_pos,
					max_pos,
					redraw as _,
				)
			},
		)
	}

	/// This method returns the timer ID, to be passed to
	/// [`HWND::KillTimer`](crate::prelude::user_Hwnd::KillTimer).
	///
	/// The timer calls – either
	/// [`wm_timer`](crate::prelude::GuiEventsAll::wm_timer) message or callback
	/// function – will continuously be executed until you call `KillTimer`. If
	/// you don't call `KillTimer`, the timer calls will continue until the
	/// window is destroyed – at this point, any remaining timers will be
	/// automatically cleared.
	///
	/// # Why not closures?
	///
	/// A common C++ technique to use closures with `SetTimer` is allocating a
	/// closure on the heap and use its pointer as the timer ID. When the
	/// callback function is called, the pointer is dereferenced and the closure
	/// is then executed.
	///
	/// The problem with this approach is that the closure must be freed after
	/// `KillTimer`, which can be called from anywhere, including from the
	/// closure itself – that means you must keep the pointer outside the
	/// closure and free it somehow after the closure finishes.
	///
	/// Such approach is, obviously, incredibly unsafe, and only possible within
	/// Rust's rigid ownership rules if we use some sort of garbage-collection,
	/// which will free the allocated closure some time after `KillTimer` is
	/// called and the closure itself finishes. Since that would incur in a
	/// performance penalty, the current implementation of `SetTimer` will only
	/// accept ordinary function pointers, not closures.
	///
	/// Handling the `wm_timer` message is simply more practical and efficient,
	/// so the use of a callback is discouraged here.
	fn SetTimer(&self,
		event_id: usize,
		elapse_ms: u32,
		timer_func: Option<TIMERPROC>,
	) -> SysResult<usize>
	{
		match unsafe {
			ffi::SetTimer(
				self.ptr(),
				event_id,
				elapse_ms,
				timer_func.map_or(std::ptr::null(), |lp| lp as _),
			)
		} {
			0 => Err(GetLastError()),
			tid => Ok(tid),
		}
	}

	/// [`SetWindowDisplayAffinity`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)
	/// function.
	fn SetWindowDisplayAffinity(&self, affinity: co::WDA) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::SetWindowDisplayAffinity(self.ptr(), affinity.raw()) },
		)
	}

	/// [`SetWindowLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// function.
	fn SetWindowLongPtr(&self, index: co::GWLP, new_long: isize) -> isize {
		#[cfg(target_pointer_width = "32")]
		unsafe { ffi::SetWindowLongW(self.ptr(), index.raw(), new_long) }

		#[cfg(target_pointer_width = "64")]
		unsafe { ffi::SetWindowLongPtrW(self.ptr(), index.raw(), new_long) }
	}

	/// [`SetWindowPlacement`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// function.
	fn SetWindowPlacement(&self, wp: &WINDOWPLACEMENT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::SetWindowPlacement(self.ptr(), wp as *const _ as _) },
		)
	}

	/// [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.SetWindowPos(
	///     w::HwndPlace::None,
	///     w::POINT::new(10, 10),
	///     w::SIZE::default(),
	///     co::SWP::NOZORDER | co::SWP::NOSIZE,
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn SetWindowPos(&self,
		hwnd_insert_after: HwndPlace,
		pos: POINT,
		size: SIZE,
		flags: co::SWP,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetWindowPos(
					self.ptr(),
					hwnd_insert_after.as_ptr(),
					pos.x, pos.y,
					size.cx, size.cy,
					flags.raw(),
				)
			},
		)
	}

	/// [`SetWindowRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowrgn)
	/// function.
	fn SetWindowRgn(&self, hrgn: &HRGN, redraw: bool) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::SetWindowRgn(self.ptr(), hrgn.ptr(), redraw as _) },
		)
	}

	/// [`SetWindowText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// function.
	fn SetWindowText(&self, text: &str) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::SetWindowTextW(
					self.ptr(),
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}

	/// [`ShowCaret`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcaret)
	/// function.
	fn ShowCaret(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ShowCaret(self.ptr()) })
	}

	/// [`ShowOwnedPopups`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showownedpopups)
	/// function.
	fn ShowOwnedPopups(&self, show: bool) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ShowOwnedPopups(self.ptr(), show as _) })
	}

	/// [`ShowWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// function.
	fn ShowWindow(&self, show_cmd: co::SW) -> bool {
		unsafe { ffi::ShowWindow(self.ptr(), show_cmd.raw()) != 0 }
	}

	/// [`ShowWindowAsync`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindowasync)
	/// function.
	fn ShowWindowAsync(&self, show_cmd: co::SW) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ShowWindowAsync(self.ptr(), show_cmd.raw()) },
		)
	}

	/// [`TileWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-tilewindows)
	/// function.
	fn TileWindows(&self,
		how: co::MDITILE,
		rect: Option<RECT>,
		kids: Option<&[&HWND]>,
	) -> SysResult<u16>
	{
		match unsafe {
			ffi::TileWindows(
				self.ptr(),
				how.raw(),
				rect.map_or(std::ptr::null(), |rc| &rc as *const _ as _),
				kids.map_or(0, |s| s.len() as _),
				kids.map_or(std::ptr::null(), |hs| vec_ptr(hs) as *const _ as _),
			)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0),
				err => Err(err),
			},
			c => Ok(c),
		}
	}

	/// [`TranslateAccelerator`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// function.
	fn TranslateAccelerator(&self,
		haccel_table: &HACCEL,
		msg: &mut MSG,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::TranslateAcceleratorW(
					self.ptr(),
					haccel_table.ptr(),
					msg as *mut _ as _,
				)
			},
		)
	}

	/// [`UpdateWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// function.
	fn UpdateWindow(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::UpdateWindow(self.ptr()) })
	}

	/// [`ValidateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect)
	/// function.
	fn ValidateRect(&self, rc: &RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ValidateRect(self.ptr(), rc as *const _ as _) },
		)
	}

	/// [`ValidateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// function.
	fn ValidateRgn(&self, hrgn: &HRGN) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ValidateRgn(self.ptr(), hrgn.ptr()) })
	}

	/// [`WindowFromPhysicalPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint)
	/// function.
	#[must_use]
	fn WindowFromPhysicalPoint(pt: POINT) -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::WindowFromPhysicalPoint(pt.x, pt.y) })
	}

	/// [`WindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint)
	/// function.
	#[must_use]
	fn WindowFromPoint(pt: POINT) -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::WindowFromPoint(pt.x, pt.y) })
	}

	/// [`WinHelp`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-winhelpw)
	/// function.
	fn WinHelp(&self,
		help_file: &str,
		cmd: co::HELPW,
		data: usize,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::WinHelpW(
					self.ptr(),
					WString::from_str(help_file).as_ptr(),
					cmd.raw(),
					data,
				)
			},
		)
	}
}
