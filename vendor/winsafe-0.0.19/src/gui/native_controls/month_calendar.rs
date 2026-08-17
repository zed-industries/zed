use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of MonthCalendar
	base: BaseNativeControl,
	events: MonthCalendarEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [month calendar](https://learn.microsoft.com/en-us/windows/win32/controls/month-calendar-controls)
/// control.
#[derive(Clone)]
pub struct MonthCalendar(Pin<Arc<Obj>>);

unsafe impl Send for MonthCalendar {}

impl GuiWindow for MonthCalendar {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for MonthCalendar {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for MonthCalendar {}

impl GuiNativeControl for MonthCalendar {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<MonthCalendarEvents> for MonthCalendar {
	fn on(&self) -> &MonthCalendarEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl MonthCalendar {
	/// Instantiates a new `MonthCalendar` object, to be created on the parent
	/// window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `MonthCalendar` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: MonthCalendarOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let opts = MonthCalendarOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: MonthCalendarEvents::new(parent_base_ref, ctrl_id),
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

	/// Instantiates a new `MonthCalendar` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `MonthCalendar` in an event closure.
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
					events: MonthCalendarEvents::new(parent_base_ref, ctrl_id),
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

	fn create(&self, opts_resz: OptsResz<&MonthCalendarOpts>) -> SysResult<()> {
		let resize_behavior = match opts_resz {
			OptsResz::Wnd(opts) => opts.resize_behavior,
			OptsResz::Dlg(resize_behavior) => resize_behavior,
		};
		if resize_behavior.0 == Horz::Resize {
			panic!("MonthCalendar cannot be resized with Horz::Resize.");
		} else if resize_behavior.1 == Vert::Resize {
			panic!("MonthCalendar cannot be resized with Vert::Resize.");
		}

		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				multiply_dpi_or_dtu(self.0.base.parent(), Some(&mut pos), None)?;

				self.0.base.create_window(
					"SysMonthCal32", None, pos, SIZE::new(0, 0),
					opts.window_ex_style,
					opts.window_style | opts.month_calendar_style.into(),
				)?;

				let mut bounds_rect = RECT::default();
				self.hwnd().SendMessage(mcm::GetMinReqRect {
					bounds_rect: &mut bounds_rect,
				})?;
				self.hwnd().SetWindowPos(HwndPlace::None, POINT::default(),
					SIZE::new(bounds_rect.right, bounds_rect.bottom),
					co::SWP::NOZORDER | co::SWP::NOMOVE)?;
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), resize_behavior)
	}

	/// Retrieves the currently selected date by sending a
	/// [`mcm::GetCurSel`](crate::msg::mcm::GetCurSel) message.
	pub fn date(&self, st: &mut SYSTEMTIME) {
		self.hwnd().SendMessage(mcm::GetCurSel { info: st }).unwrap();
	}

	/// Sets the currently selected date by sending a
	/// [`mcm::SetCurSel`](crate::msg::mcm::SetCurSel) message.
	pub fn set_date(&self, st: &SYSTEMTIME) {
		self.hwnd().SendMessage(mcm::SetCurSel { info: st }).unwrap();
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`MonthCalendar`](crate::gui::MonthCalendar)
/// programmatically with
/// [`MonthCalendar::new`](crate::gui::MonthCalendar::new).
pub struct MonthCalendarOpts {
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
	/// Month calendar styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `MCS::NoValue`.
	pub month_calendar_style: co::MCS,
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
	/// **Note:** A `MonthCalendar` cannot be resized horizontally or
	/// vertically, so it will panic if you use `Horz::Resize` or
	/// `Vert::Resize`.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for MonthCalendarOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			month_calendar_style: co::MCS::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl MonthCalendarOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
