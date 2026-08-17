use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of DateTimePicker
	base: BaseNativeControl,
	events: DateTimePickerEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [date and time picker](https://learn.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-controls)
/// control.
#[derive(Clone)]
pub struct DateTimePicker(Pin<Arc<Obj>>);

unsafe impl Send for DateTimePicker {}

impl GuiWindow for DateTimePicker {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for DateTimePicker {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for DateTimePicker {}

impl GuiNativeControl for DateTimePicker {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<DateTimePickerEvents> for DateTimePicker {
	fn on(&self) -> &DateTimePickerEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl DateTimePicker {
	/// Instantiates a new `DateTimePicker` object, to be created on the parent
	/// window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `DateTimePicker` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: DateTimePickerOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let opts = DateTimePickerOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: DateTimePickerEvents::new(parent_base_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm(parent_base_ref.wm_create_or_initdialog(), move |_| {
			self2.create(OptsResz::Wnd(&opts))?;
			Ok(None) // not meaningful
		});

		new_self
	}

	/// Instantiates a new `DateTimePicker` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `DateTimePicker` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self
	{
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: DateTimePickerEvents::new(parent_base_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(true) // not meaningful
		});

		new_self
	}

	fn create(&self, opts_resz: OptsResz<&DateTimePickerOpts>) -> SysResult<()> {
		let resize_behavior = match opts_resz {
			OptsResz::Wnd(opts) => opts.resize_behavior,
			OptsResz::Dlg(resize_behavior) => resize_behavior,
		};
		if resize_behavior.1 == Vert::Resize {
			panic!("DateTimePicker cannot be resized with Vert::Resize.");
		}

		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.width as _, 21); // default height
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window(
					"SysDateTimePick32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.date_time_picker_style.into(),
				)?;

				if sz.cx == 0 { // use ideal width?
					let mut sz_ideal = SIZE::default();
					self.hwnd().SendMessage(dtm::GetIdealSize {
						size: &mut sz_ideal,
					});
					sz.cx = sz_ideal.cx; // already adjusted for DPI

					self.hwnd().SetWindowPos(
						HwndPlace::None, POINT::default(), sz,
						co::SWP::NOZORDER | co::SWP::NOMOVE)?;
				}

				self.hwnd().SendMessage(wm::SetFont {
					hfont: unsafe { ui_font().raw_copy() },
					redraw: true,
				});
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), resize_behavior)
	}

	/// Retrieves the currently selected date by sending a
	/// [`dtm::GetSystemTime`](crate::msg::dtm::GetSystemTime) message.
	pub fn date(&self, st: &mut SYSTEMTIME) {
		self.hwnd()
			.SendMessage(dtm::GetSystemTime { system_time: st })
			.unwrap()
	}

	/// Sets the currently selected date by sending a
	/// [`dtm::SetSystemTime`](crate::msg::dtm::SetSystemTime) message.
	pub fn set_date(&self, st: &SYSTEMTIME) {
		self.hwnd()
			.SendMessage(dtm::SetSystemTime { system_time: Some(st) })
			.unwrap()
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`DateTimePicker`](crate::gui::DateTimePicker)
/// programmatically with
/// [`DateTimePicker::new`](crate::gui::DateTimePicker::new).
pub struct DateTimePickerOpts {
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
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to ideal width retrieved with
	/// [`dtm::GetIdealSize`](crate::msg::dtm::GetIdealSize) message, usually
	/// around `250`.
	pub width: u32,
	/// Date and time picker styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `DTS::LONGDATEFORMAT`.
	pub date_time_picker_style: co::DTS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// **Note:** A `DateTimePicker` cannot be resized vertically, so it will
	/// panic if you use `Vert::Resize`.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for DateTimePickerOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			width: 0,
			date_time_picker_style: co::DTS::LONGDATEFORMAT,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl DateTimePickerOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
