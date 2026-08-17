use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::prelude:: *;

struct Obj { // actual fields of RawControl
	raw_base: RawBase,
	opts: WindowControlOpts,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// An ordinary custom control window.
#[derive(Clone)]
pub(in crate::gui) struct RawControl(Pin<Arc<Obj>>);

impl RawControl {
	pub(in crate::gui) fn new(parent: &Base, opts: WindowControlOpts) -> Self {
		let resize_behavior = opts.resize_behavior;
		let opts = WindowControlOpts::define_ctrl_id(opts);
		let new_self = Self(
			Arc::pin(
				Obj {
					raw_base: RawBase::new(Some(parent)),
					opts,
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers(parent, resize_behavior);
		new_self
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.raw_base.as_base()
	}

	pub(in crate::gui) fn hwnd(&self) -> &HWND {
		self.0.raw_base.hwnd()
	}

	pub(in crate::gui) fn ctrl_id(&self) -> u16 {
		self.0.opts.ctrl_id
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.0.raw_base.on()
	}

	pub(in crate::gui) fn privileged_on(&self) -> &WindowEventsAll {
		self.0.raw_base.privileged_on()
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

	fn default_message_handlers(&self,
		parent: &Base,
		resize_behavior: (Horz, Vert),
	) {
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

			let mut wnd_pos = POINT::new(opts.position.0, opts.position.1);
			let mut wnd_sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
			multiply_dpi_or_dtu(parent_base_ref, Some(&mut wnd_pos), Some(&mut wnd_sz))?;

			self2.0.raw_base.create_window(
				Some(parent_base_ref.hwnd()),
				atom,
				None,
				IdMenu::Id(opts.ctrl_id),
				wnd_pos, wnd_sz,
				opts.ex_style, opts.style,
			)?;

			parent_base_ref.add_to_layout_arranger(self2.hwnd(), resize_behavior)?;
			Ok(None) // not meaningful
		});

		let self2 = self.clone();
		self.privileged_on().wm_nc_paint(move |p| {
			paint_control_borders(self2.hwnd(), p)?;
			Ok(())
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`WindowControl`](crate::gui::WindowControl)
/// programmatically with [`WindowControl::new`](crate::gui::WindowControl::new).
pub struct WindowControlOpts {
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
	/// Defaults to `gui::Brush::Color(co::COLOR::WINDOW)`.
	pub class_bg_brush: Brush,

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
	/// Width and height of window to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(100, 80)`.
	pub size: (u32, u32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::TABSTOP | WS::GROUP | WS::VISIBLE | WS::CLIPCHILDREN | WS::CLIPSIBLINGS`.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	///
	/// Suggestion:
	/// * `WS_EX::CLIENTEDGE` to have a border.
	pub ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for WindowControlOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::WINDOW),
			position: (0, 0),
			size: (100, 80),
			style: co::WS::CHILD | co::WS::TABSTOP | co::WS::GROUP | co::WS::VISIBLE | co::WS::CLIPCHILDREN | co::WS::CLIPSIBLINGS,
			ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl WindowControlOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
