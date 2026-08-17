use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Possible states of a [`CheckBox`](crate::gui::CheckBox) control.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckState {
	/// CheckBox is checked.
	Checked,
	/// CheckBox is grayed, indicating an indeterminate state. Applicable only
	/// if the CheckBox was created with [`BS::R3STATE`](crate::co::BS::R3STATE)
	/// or [`BS::AUTO3STATE`](crate::co::BS::AUTO3STATE) styles.
	Indeterminate,
	/// CheckBox is cleared.
	Unchecked,
}

struct Obj { // actual fields of CheckBox
	base: BaseNativeControl,
	events: ButtonEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [check box](https://learn.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#check-boxes)
/// control, actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
#[derive(Clone)]
pub struct CheckBox(Pin<Arc<Obj>>);

unsafe impl Send for CheckBox {}

impl GuiWindow for CheckBox {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindowText for CheckBox {}

impl GuiChild for CheckBox {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for CheckBox {}

impl GuiNativeControl for CheckBox {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<ButtonEvents> for CheckBox {
	fn on(&self) -> &ButtonEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl CheckBox {
	/// Instantiates a new `CheckBox` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `CheckBox` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: CheckBoxOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let opts = CheckBoxOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: ButtonEvents::new(parent_base_ref, ctrl_id),
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

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `CheckBox` in an event closure.
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
					events: ButtonEvents::new(parent_base_ref, ctrl_id),
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

	fn create(&self, opts_resz: OptsResz<&CheckBoxOpts>) -> SysResult<()> {
		let resize_behavior = match opts_resz {
			OptsResz::Wnd(opts) => opts.resize_behavior,
			OptsResz::Dlg(resize_behavior) => resize_behavior,
		};

		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), None)?;

				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				if sz.cx == -1 && sz.cy == -1 {
					sz = calc_text_bound_box_check(&opts.text)?; // resize to fit text
				} else {
					multiply_dpi_or_dtu(
						self.0.base.parent(), None, Some(&mut sz))?; // user-defined size
				}

				self.0.base.create_window(
					"BUTTON", Some(&opts.text), pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.button_style.into(),
				)?;

				self.hwnd().SendMessage(wm::SetFont {
					hfont: unsafe { ui_font().raw_copy() },
					redraw: true,
				});
				if opts.check_state != CheckState::Unchecked {
					self.set_check_state(opts.check_state);
				}
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), resize_behavior)
	}

	/// Retrieves the current check state by sending a
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) message.
	#[must_use]
	pub fn check_state(&self) -> CheckState {
		match self.hwnd().SendMessage(bm::GetCheck {}) {
			co::BST::CHECKED => CheckState::Checked,
			co::BST::INDETERMINATE => CheckState::Indeterminate,
			_ => CheckState::Unchecked,
		}
	}

	/// Emulates the click event for the check box by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn emulate_click(&self) {
		self.hwnd().SendMessage(bm::Click {});
	}

	/// Calls [`check_state`](crate::gui::CheckBox::check_state) and compares
	/// the result with
	/// [`CheckState::Checked`](crate::gui::CheckState::Checked).
	#[must_use]
	pub fn is_checked(&self) -> bool {
		self.check_state() == CheckState::Checked
	}

	/// Sets the current check state by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn set_check_state(&self, state: CheckState) {
		self.hwnd().SendMessage(bm::SetCheck {
			state: match state {
				CheckState::Checked => co::BST::CHECKED,
				CheckState::Indeterminate => co::BST::INDETERMINATE,
				CheckState::Unchecked => co::BST::UNCHECKED,
			},
		});
	}

	/// Sets the current check state by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn set_check_state_and_trigger(&self, state: CheckState) {
		self.set_check_state(state);
		self.hwnd().GetParent().unwrap().SendMessage(
			wm::Command {
				event: AccelMenuCtrl::Ctrl(
					AccelMenuCtrlData {
						notif_code: co::BN::CLICKED.into(),
						ctrl_id: self.ctrl_id(),
						ctrl_hwnd: unsafe { self.hwnd().raw_copy() },
					},
				),
			},
		);
	}

	/// Calls [`set_text`](crate::prelude::GuiWindowText::set_text) and resizes
	/// the control to exactly fit the new text.
	pub fn set_text_and_resize(&self, text: &str) {
		self.set_text(text);
		let bound_box = calc_text_bound_box_check(text).unwrap();
		self.hwnd().SetWindowPos(
			HwndPlace::None, POINT::default(), bound_box,
			co::SWP::NOZORDER | co::SWP::NOMOVE).unwrap();
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`CheckBox`](crate::gui::CheckBox) programmatically with
/// [`CheckBox::new`](crate::gui::CheckBox::new).
pub struct CheckBoxOpts {
	/// Text of the control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
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
	/// Defaults to the size needed to fit the text.
	pub size: (u32, u32),
	/// Check box styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTOCHECKBOX`.
	///
	/// Suggestions:
	/// * replace with `BS::AUTO3STATE` for a 3-state check box.
	pub button_style: co::BS,
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

	/// Initial check state.
	///
	/// Defaults to `gui::CheckState::Unchecked`.
	pub check_state: CheckState,
}

impl Default for CheckBoxOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: (0, 0),
			size: (-1i32 as _, -1i32 as _), // will resize to fit the text
			button_style: co::BS::AUTOCHECKBOX,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			check_state: CheckState::Unchecked,
		}
	}
}

impl CheckBoxOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
