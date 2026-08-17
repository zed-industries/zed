use std::any::Any;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Native
/// [radio button](https://learn.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control.
///
/// You cannot directly instantiate this object, you must use
/// [`RadioGroup`](crate::gui::RadioGroup).
pub struct RadioButton {
	base: BaseNativeControl,
	events: ButtonEvents,
}

impl GuiWindow for RadioButton {
	fn hwnd(&self) -> &HWND {
		self.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindowText for RadioButton {}

impl GuiChild for RadioButton {
	fn ctrl_id(&self) -> u16 {
		self.base.ctrl_id()
	}
}

impl GuiChildFocus for RadioButton {}

impl GuiNativeControl for RadioButton {
	fn on_subclass(&self) -> &WindowEvents {
		self.base.on_subclass()
	}
}

impl GuiNativeControlEvents<ButtonEvents> for RadioButton {
	fn on(&self) -> &ButtonEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.events
	}
}

impl RadioButton {
	pub(in crate::gui) fn new(
		parent: &impl GuiParent,
		opts: RadioButtonOpts,
	) -> Self
	{
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let opts = RadioButtonOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		Self {
			base: BaseNativeControl::new(parent_base_ref, ctrl_id),
			events: ButtonEvents::new(parent_base_ref, ctrl_id),
		}
	}

	pub(in crate::gui) fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
	) -> Self
	{
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };

		Self {
			base: BaseNativeControl::new(parent_base_ref, ctrl_id),
			events: ButtonEvents::new(parent_base_ref, ctrl_id),
		}
	}

	pub(in crate::gui) fn create(&self,
		opts_resz: &OptsResz<RadioButtonOpts>,
	) -> SysResult<()>
	{
		let resize_behavior = match opts_resz {
			OptsResz::Wnd(opts) => opts.resize_behavior,
			OptsResz::Dlg(resize_behavior) => *resize_behavior,
		};

		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				multiply_dpi_or_dtu(
					self.base.parent(), Some(&mut pos), None)?;

				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
					if sz.cx == -1 && sz.cy == -1 {
						sz = calc_text_bound_box_check(&opts.text)?; // resize to fit text
					} else {
						multiply_dpi_or_dtu(
							self.base.parent(), None, Some(&mut sz))?; // user-defined size
					}

				self.base.create_window( // may panic
					"BUTTON", Some(&opts.text), pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.button_style.into(),
				)?;

				self.hwnd().SendMessage(wm::SetFont {
					hfont: unsafe { ui_font().raw_copy() },
					redraw: true,
				});
				if opts.selected { self.select(true); }
			},
			OptsResz::Dlg(_) => self.base.create_dlg()?,
		}

		self.base.parent().add_to_layout_arranger(self.hwnd(), resize_behavior)?;
		self.hwnd().SendMessage(bm::SetDontClick { dont_click: true });
		Ok(())
	}

	/// Emulates the click event for the radio button by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn emulate_click(&self) {
		self.hwnd().SendMessage(bm::Click {});
	}

	/// Tells if this radio button is the currently selected one by sending a
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) message.
	#[must_use]
	pub fn is_selected(&self) -> bool {
		self.hwnd().SendMessage(bm::GetCheck {}) == co::BST::CHECKED
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn select(&self, selected: bool) {
		self.hwnd().SendMessage(bm::SetCheck {
			state: if selected { co::BST::CHECKED } else { co::BST::UNCHECKED },
		});
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn select_and_trigger(&self, selected: bool) -> SysResult<()> {
		self.select(selected);
		self.hwnd().GetParent()?.SendMessage(wm::Command {
			event: AccelMenuCtrl::Ctrl(
				AccelMenuCtrlData {
					notif_code: co::BN::CLICKED.into(),
					ctrl_id: self.ctrl_id(),
					ctrl_hwnd: unsafe { self.hwnd().raw_copy() },
				},
			),
		});
		Ok(())
	}

	/// Calls [`set_text`](crate::prelude::GuiWindowText::set_text) and resizes
	/// the control to exactly fit the new text.
	pub fn set_text_and_resize(&self, text: &str) {
		self.set_text(text);
		let bound_box = calc_text_bound_box_check(text).unwrap();
		self.hwnd().SetWindowPos(
			HwndPlace::None, POINT::default(),
			bound_box, co::SWP::NOZORDER | co::SWP::NOMOVE).unwrap();
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`RadioButton`](crate::gui::RadioButton)
/// programmatically with [`RadioGroup::new`](crate::gui::RadioGroup::new).
#[derive(Clone)]
pub struct RadioButtonOpts {
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
	/// Radio button styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTORADIOBUTTON`.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
	///
	/// The first RadioButton of a group should also have `WS::TABSTOP | WS::GROUP`.
	/// If this object being passed to a [`RadioGroup`](crate::gui::RadioGroup),
	/// this will be automatically set.
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

	/// Initial selection state.
	///
	/// Defaults to `false`.
	pub selected: bool,
}

impl Default for RadioButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: (0, 0),
			size: (-1i32 as _, -1i32 as _), // will resize to fit the text
			button_style: co::BS::AUTORADIOBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			selected: false,
		}
	}
}

impl RadioButtonOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}

	pub(in crate::gui) fn manual_clone(&self) -> RadioButtonOpts { // avoids a public clone method
		Self {
			text: self.text.clone(),
			position: self.position,
			size: self.size,
			button_style: self.button_style,
			window_style: self.window_style,
			window_ex_style: self.window_ex_style,
			ctrl_id: self.ctrl_id,
			resize_behavior: self.resize_behavior,
			selected: self.selected,
		}
	}
}
