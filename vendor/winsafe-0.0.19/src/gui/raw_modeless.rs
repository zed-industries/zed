use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::prelude::*;

struct Obj { // actual fields of RawModeless
	raw_base: RawBase,
	opts: WindowModelessOpts,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// An ordinary modeless window.
#[derive(Clone)]
pub(in crate::gui) struct RawModeless(Pin<Arc<Obj>>);

impl RawModeless {
	pub(in crate::gui) fn new(parent: &Base, opts: WindowModelessOpts) -> Self {
		let new_self = Self(
			Arc::pin(
				Obj {
					raw_base: RawBase::new(Some(parent)),
					opts,
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers(parent);
		new_self
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.raw_base.as_base()
	}

	pub(in crate::gui) fn hwnd(&self) -> &HWND {
		self.0.raw_base.hwnd()
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.0.raw_base.on()
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		self.0.raw_base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static
	{
		self.0.raw_base.run_ui_thread(func);
	}

	fn default_message_handlers(&self, parent: &Base) {
		let self2 = self.clone();
		parent.privileged_on().wm(parent.wm_create_or_initdialog(), move |_| {
			let parent_base_ref = self2.0.raw_base.parent().unwrap();
			let opts = &self2.0.opts;

			let parent_hinst = self2.0.raw_base.parent_hinstance()?;
			let mut wcx = WNDCLASSEX::default();
			let mut class_name_buf = WString::default();
			RawBase::fill_wndclassex(
				&parent_hinst,
				opts.class_style, &opts.class_icon, &opts.class_icon,
				&opts.class_bg_brush, &opts.class_cursor, &mut wcx,
				&mut class_name_buf)?;
			let atom = self2.0.raw_base.register_class(&mut wcx)?;

			let wnd_pos = adjust_modeless_pos(
				parent_base_ref, POINT::new(opts.position.0, opts.position.1))?;

			let mut wnd_sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
			multiply_dpi_or_dtu(parent_base_ref, None, Some(&mut wnd_sz))?;

			self2.0.raw_base.create_window(
				Some(parent_base_ref.hwnd()),
				atom,
				Some(&opts.title),
				IdMenu::Menu(&HMENU::NULL),
				wnd_pos, wnd_sz,
				opts.ex_style, opts.style,
			)?;
			Ok(None) // not meaningful
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`WindowModeless`](crate::gui::WindowModeless)
/// programmatically with
/// [`WindowModeless::new`](crate::gui::WindowModeless::new).
pub struct WindowModelessOpts {
	/// Window class name to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Icon::None`.
	pub class_icon: Icon,
	/// Window cursor to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Brush::Color(co::COLOR::BTNFACE)`.
	pub class_bg_brush: Brush,

	/// Window title to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: String,
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of window client area, in pixels, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to `(220, 150)`.
	pub size: (u32, u32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::TOOLWINDOW`.
	pub ex_style: co::WS_EX,
}

impl Default for WindowModelessOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			position: (0, 0),
			size: (220, 150),
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT | co::WS_EX::TOOLWINDOW,
		}
	}
}
