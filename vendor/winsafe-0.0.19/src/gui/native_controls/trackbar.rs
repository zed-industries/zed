use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of Trackbar
	base: BaseNativeControl,
	events: TrackbarEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [trackbar](https://learn.microsoft.com/en-us/windows/win32/controls/trackbar-controls)
/// control.
#[derive(Clone)]
pub struct Trackbar(Pin<Arc<Obj>>);

unsafe impl Send for Trackbar {}

impl GuiWindow for Trackbar {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for Trackbar {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for Trackbar {}

impl GuiNativeControl for Trackbar {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<TrackbarEvents> for Trackbar {
	fn on(&self) -> &TrackbarEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl Trackbar {
	/// Instantiates a new `Trackbar` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Trackbar` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: TrackbarOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let opts = TrackbarOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: TrackbarEvents::new(parent_base_ref, ctrl_id),
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

	/// Instantiates a new `Trackbar` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Trackbar` in an event closure.
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
					events: TrackbarEvents::new(parent_base_ref, ctrl_id),
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

	fn create(&self, opts_resz: OptsResz<&TrackbarOpts>) -> SysResult<()> {
		let resize_behavior = match opts_resz {
			OptsResz::Wnd(opts) => opts.resize_behavior,
			OptsResz::Dlg(resize_behavior) => resize_behavior,
		};

		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window( // may panic
					"msctls_trackbar32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.trackbar_style.into(),
				)?;

				if opts.range != (0, 100) {
					self.set_range(opts.range.0, opts.range.1);
				}
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), resize_behavior)
	}

	/// Retrieves the current position by sending a
	/// [`trbm::GetPos`](crate::msg::trbm::GetPos) message.
	#[must_use]
	pub fn pos(&self) -> u32 {
		self.hwnd().SendMessage(trbm::GetPos {})
	}

	/// Retrieves the minimum and maximum position values by sending
	/// [`trbm::GetRangeMin`](crate::msg::trbm::GetRangeMin) and
	/// [`trbm::GetRangeMax`](crate::msg::trbm::GetRangeMax) messages.
	#[must_use]
	pub fn range(&self) -> (u32, u32) {
		(
			self.hwnd().SendMessage(trbm::GetRangeMin {}),
			self.hwnd().SendMessage(trbm::GetRangeMax {}),
		)
	}

	/// Sets the current position by sending a
	/// [`trbm::SetPos`](crate::msg::trbm::SetPos) message.
	pub fn set_pos(&self, pos: u32) {
		self.hwnd().SendMessage(trbm::SetPos { redraw: true, pos });
	}

	/// Sets the minimum and maximum position values by sending
	/// [`trbm::SetRangeMin`](crate::msg::trbm::SetRangeMin) and
	/// [`trbm::SetRangeMax`](crate::msg::trbm::SetRangeMax) messages.
	pub fn set_range(&self, min: u32, max: u32) {
		self.hwnd().SendMessage(trbm::SetRangeMin { redraw: false, min });
		self.hwnd().SendMessage(trbm::SetRangeMax { redraw: true, max });
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Trackbar`](crate::gui::Trackbar) programmatically with
/// [`Trackbar::new`](crate::gui::Trackbar::new).
pub struct TrackbarOpts {
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
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(120, 23)`.
	pub size: (u32, u32),
	/// Trackbar styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TBS::HORZ | TBS::AUTOTICKS`.
	pub trackbar_style: co::TBS,
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
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),

	/// The minimum and maximum position values.
	///
	/// Defaults to `(0, 100)`.
	pub range: (u32, u32),
}

impl Default for TrackbarOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (120, 23),
			trackbar_style: co::TBS::HORZ | co::TBS::AUTOTICKS,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			range: (0, 100),
		}
	}
}

impl TrackbarOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
