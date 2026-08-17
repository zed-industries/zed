use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// The class background brush to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
pub enum Brush {
	/// A solid [system color](co::COLOR).
	Color(co::COLOR),
	/// A brush handle, previously created by you.
	Handle(HBRUSH),
	/// No brush.
	None,
}

impl Brush {
	/// Converts the contents of `Brush` to `HBRUSH`.
	pub fn as_hbrush(&self) -> HBRUSH {
		match self {
			Brush::Color(c) => HBRUSH::from_sys_color(*c),
			Brush::Handle(h) => unsafe { h.raw_copy() },
			Brush::None => HBRUSH::NULL,
		}
	}
}

/// The class cursor to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
pub enum Cursor {
	/// A cursor handle, previously loaded by you.
	Handle(HCURSOR),
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// No cursor.
	None,
	/// A resource string identifier.
	Str(WString),
}

impl Cursor {
	/// Converts the contents of `Cursor` to `HCURSOR`.
	pub fn as_hcursor(&self, hinst: &HINSTANCE) -> SysResult<HCURSOR> {
		unsafe {
			Ok(match self {
				Cursor::Handle(h) => h.raw_copy(),
				Cursor::Id(id) => hinst.LoadCursor(IdIdcStr::Id(*id))?.leak(),
				Cursor::Idc(idc) => HINSTANCE::NULL.LoadCursor(IdIdcStr::Idc(*idc))?.leak(),
				Cursor::None => HCURSOR::NULL,
				Cursor::Str(s) => hinst.LoadCursor(IdIdcStr::Str(s.clone()))?.leak(),
			})
		}
	}
}

/// The class icon to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
pub enum Icon {
	/// An icon handle, previously loaded by you.
	Handle(HICON),
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system icon.
	Idi(co::IDI),
	/// No icon.
	None,
	/// A resource string identifier.
	Str(WString),
}

impl Icon {
	/// Converts the contents of `Icon` to `HICON`.
	pub fn as_hicon(&self, hinst: &HINSTANCE) -> SysResult<HICON> {
		unsafe {
			Ok(match self {
				Icon::Handle(h) => h.raw_copy(),
				Icon::Id(id) => hinst.LoadIcon(IdIdiStr::Id(*id))?.leak(),
				Icon::Idi(idi) => HINSTANCE::NULL.LoadIcon(IdIdiStr::Idi(*idi))?.leak(),
				Icon::None => HICON::NULL,
				Icon::Str(s) => hinst.LoadIcon(IdIdiStr::Str(s.clone()))?.leak(),
			})
		}
	}
}

//------------------------------------------------------------------------------

/// Base to all ordinary windows.
///
/// Owns the window procedure for all ordinary windows.
pub(in crate::gui) struct RawBase {
	base: Base,
}

impl Drop for RawBase {
	fn drop(&mut self) {
		if *self.base.hwnd() != HWND::NULL {
			self.base.hwnd().SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl RawBase {
	pub(in crate::gui) fn new(parent: Option<&Base>) -> Self {
		Self { base: Base::new(false, parent) }
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		// At this moment, the parent struct is already created and pinned.
		&self.base as *const _ as _
	}

	pub(in crate::gui) const fn hwnd(&self) -> &HWND {
		self.base.hwnd()
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.base.on()
	}

	pub(in crate::gui) fn privileged_on(&self) -> &WindowEventsAll {
		self.base.privileged_on()
	}

	pub(in crate::gui) fn parent(&self) -> Option<&Base> {
		self.base.parent()
	}

	pub(in crate::gui) fn parent_hinstance(&self) -> SysResult<HINSTANCE> {
		self.base.parent_hinstance()
	}

	pub(in crate::gui) fn delegate_focus_to_first_child(&self) {
		if let Some(hwnd_cur_focus) = HWND::GetFocus() {
			if *self.hwnd() == hwnd_cur_focus {
				// https://stackoverflow.com/a/2835220/6923555
				if let Ok(hchild_first) = self.hwnd().GetWindow(co::GW::CHILD) {
					hchild_first.SetFocus(); // if window receives focus, delegate to first child
				}
			}
		}
	}

	/// Fills `WNDCLASSEX` with the given values, and generates a class name as
	/// a hash of all fields.
	pub(in crate::gui) fn fill_wndclassex<'a>(
		hinst: &'a HINSTANCE,
		class_style: co::CS,
		class_icon: &'a Icon,
		class_icon_sm: &'a Icon,
		class_bg_brush: &'a Brush,
		class_cursor: &'a Cursor,
		wcx: &mut WNDCLASSEX<'a>,
		class_name_buf: &'a mut WString,
	) -> SysResult<()>
	{
		wcx.lpfnWndProc = Some(Self::window_proc);
		wcx.hInstance = unsafe { hinst.raw_copy() };
		wcx.style = class_style;
		wcx.hIcon = class_icon.as_hicon(hinst)?;
		wcx.hIconSm = class_icon_sm.as_hicon(hinst)?;
		wcx.hbrBackground = class_bg_brush.as_hbrush();
		wcx.hCursor = class_cursor.as_hcursor(hinst)?;

		if wcx.lpszClassName().is_none() { // an actual class name was not provided?
			*class_name_buf = WString::from_str(
				&format!(
					"WNDCLASS.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}",
					wcx.style,
					wcx.lpfnWndProc.map_or(0, |p| p as usize),
					wcx.cbClsExtra, wcx.cbWndExtra,
					wcx.hInstance, wcx.hIcon, wcx.hCursor, wcx.hbrBackground,
					wcx.lpszMenuName(),
					wcx.hIconSm,
				),
			);
			wcx.set_lpszClassName(Some(class_name_buf));
		}

		Ok(())
	}

	pub(in crate::gui) fn register_class(&self,
		wcx: &mut WNDCLASSEX,
	) -> SysResult<ATOM>
	{
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { RegisterClassEx(&wcx) } {
			Ok(atom) => Ok(atom),
			Err(err) => match err {
				co::ERROR::CLASS_ALREADY_EXISTS => {
					// https://devblogs.microsoft.com/oldnewthing/20150429-00/?p=44984
					// https://devblogs.microsoft.com/oldnewthing/20041011-00/?p=37603
					// Retrieve ATOM of existing window class.
					let hinst = unsafe { wcx.hInstance.raw_copy() };
					Ok(hinst.GetClassInfoEx(&wcx.lpszClassName().unwrap(), wcx)?)
				},
				err => Err(err),
			},
		}
	}

	pub(in crate::gui) fn create_window(
		&self,
		hparent: Option<&HWND>, // passed because message-only window is a special case
		class_name: ATOM,
		title: Option<&str>,
		hmenu: IdMenu,
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS,
	) -> SysResult<()>
	{
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot create window twice.");
		}

		// Our hwnd member is set during WM_NCCREATE processing; already set when
		// CreateWindowEx returns.
		unsafe {
			HWND::CreateWindowEx(
				ex_styles,
				AtomStr::Atom(class_name),
				title, styles,
				pos, sz,
				hparent,
				hmenu,
				&self.base.parent_hinstance()?,
				// Pass pointer to Self.
				// At this moment, the parent struct is already created and pinned.
				Some(self as *const _ as _),
			)?;
		}

		Ok(())
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		self.base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static
	{
		self.base.run_ui_thread(func);
	}

	extern "system" fn window_proc(
		hwnd: HWND,
		msg: co::WM,
		wparam: usize,
		lparam: isize,
	) -> isize
	{
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::window_proc_proc(hwnd, wm_any)
			.unwrap_or_else(|err| { post_quit_error(wm_any, err); 0 })
	}

	fn window_proc_proc(hwnd: HWND, wm_any: WndMsg) -> AnyResult<isize> {
		let ptr_self = match wm_any.msg_id {
			co::WM::NCCREATE => { // first message being handled
				let wm_ncc = wm::NcCreate::from_generic_wm(wm_any);
				let ptr_self = wm_ncc.createstruct.lpCreateParams as *mut Self;
				hwnd.SetWindowLongPtr(co::GWLP::USERDATA, ptr_self as _); // store
				let ref_self = unsafe { &mut *ptr_self };
				ref_self.base.set_hwnd(unsafe { hwnd.raw_copy() }); // store HWND in struct field
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::USERDATA) as *mut Self, // retrieve
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_NCCREATE and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return Ok(hwnd.DefWindowProc(wm_any));
		}

		// Execute privileged closures, keep track if at least one was executed.
		let ref_self = unsafe { &mut *ptr_self };
		if wm_any.msg_id == co::WM::CREATE {
			ref_self.base.init_layout_arranger().unwrap(); // shoule be privileged, but we need HWND
		}
		let at_least_one_privileged = ref_self.base.process_privileged_messages(wm_any)?;

		// Execute user closure, if any.
		let process_result = ref_self.base.process_user_message(wm_any)?;

		if wm_any.msg_id == co::WM::NCDESTROY { // always check
			hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
			ref_self.base.set_hwnd(HWND::NULL); // clear stored HWND
			ref_self.base.clear_events(); // prevents circular references
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => if at_least_one_privileged {
				0
			} else {
				hwnd.DefWindowProc(wm_any).into()
			},
		})
	}
}
