use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of UpDown
	base: BaseNativeControl,
	events: UpDownEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [up-down](https://learn.microsoft.com/en-us/windows/win32/controls/up-down-controls)
/// control.
///
/// Note that if the `UpDown` is created with
/// [`UDS::AUTOBUDDY`](crate::co::UDS::AUTOBUDDY) style, it takes the control
/// created immediately before the `UpDown` as the buddy one, attaching the
/// `UpDown` to it. This control should be an [`Edit`](crate::gui::Edit) with
/// [`ES::NUMBER`](crate::co::ES::NUMBER) style.
#[derive(Clone)]
pub struct UpDown(Pin<Arc<Obj>>);

unsafe impl Send for UpDown {}

impl GuiWindow for UpDown {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for UpDown {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiNativeControl for UpDown {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<UpDownEvents> for UpDown {
	fn on(&self) -> &UpDownEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl UpDown {
	/// Instantiates a new `UpDown` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create an `UpDown` in an event closure.
	///
	/// # Examples
	///
	/// In the example below, the `UpDown` has
	/// [`UDS::AUTOBUDDY`](crate::co::UDS::AUTOBUDDY) style by default, so it
	/// will take the [`Edit`](crate::gui::Edit), which was created immediately
	/// prior, as its buddy control. The `UpDown` will automatically attach
	/// itself to the `Edit`.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let txt = gui::Edit::new(
	///     &wnd,
	///     gui::EditOpts {
	///         edit_style: co::ES::AUTOHSCROLL | co::ES::NOHIDESEL | co::ES::NUMBER,
	///         ..Default::default()
	///     },
	/// );
	///
	/// let updn = gui::UpDown::new(
	///     &wnd,
	///     gui::UpDownOpts {
	///         range: (0, 50),
	///         ..Default::default()
	///     },
	/// );
	/// ```
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: UpDownOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let opts = UpDownOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: UpDownEvents::new(parent_base_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm(parent_base_ref.wm_create_or_initdialog(), move |_| {
			self2.create(Some(&opts))?;
			Ok(None) // not meaningful
		});

		new_self
	}

	/// Instantiates a new `UpDown` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create an `UpDown` in an event closure.
	#[must_use]
	pub fn new_dlg(parent: &impl GuiParent, ctrl_id: u16) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: UpDownEvents::new(parent_base_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(None)?;
			Ok(true) // not meaningful
		});

		new_self
	}

	fn create(&self, opts: Option<&UpDownOpts>) -> SysResult<()> {
		match opts {
			Some(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(0, opts.height as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window( // may panic
					"msctls_updown32", None, pos, SIZE::new(0, opts.height as _),
					opts.window_ex_style,
					opts.window_style | opts.up_down_style.into(),
				)?;

				if opts.range != (0, 100) {
					self.set_range(opts.range.0, opts.range.1);
					if opts.up_down_style.has(co::UDS::AUTOBUDDY) {
						let prev_ctrl = self.hwnd().GetWindow(co::GW::HWNDPREV)?;
						prev_ctrl.SetWindowText(&opts.range.0.to_string())?;
					}
				}
			},
			None => self.0.base.create_dlg()?,
		}

		Ok(())
	}

	/// Retrieves the current position by sending an
	/// [`udm::GetPos32`](crate::msg::udm::GetPos32) message.
	#[must_use]
	pub fn pos(&self) -> i32 {
		self.hwnd().SendMessage(udm::GetPos32 { success_flag: None })
	}

	/// Retrieves the minimum and maximum position values by sending an
	/// [`udm::GetRange32`](crate::msg::udm::GetRange32) message.
	#[must_use]
	pub fn range(&self) -> (i32, i32) {
		let (mut min, mut max) = (i32::default(), i32::default());
		self.hwnd().SendMessage(udm::GetRange32 {
			min: &mut min,
			max: &mut max,
		});
		(min, max)
	}

	/// Sets the current position by sending an
	/// [`udm::SetPos32`](crate::msg::udm::SetPos32) message.
	pub fn set_pos(&self, pos: i32) {
		self.hwnd().SendMessage(udm::SetPos32 { pos });
	}

	/// Set the control range by sending an
	/// [`udm::SetRange32`](crate::msg::udm::SetRange32) message.
	pub fn set_range(&self, min: i32, max: i32) {
		self.hwnd().SendMessage(udm::SetRange32 { min, max });
	}
}

//------------------------------------------------------------------------------

/// Options to create an [`UpDown`](crate::gui::UpDown) programmatically with
/// [`UpDown::new`](crate::gui::UpDown::new).
pub struct UpDownOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Note that the `UDS::AUTOBUDDY` style automatically positions the
	/// `UpDown`; thus, with this style, `position` is meaningless.
	///
	/// Defaults to `(0, 0)`.
	pub position: (i32, i32),
	/// Control height to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the value is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// Note that the `UDS::AUTOBUDDY` style automatically resizes the `UpDown`;
	/// thus, with this style, `height` is meaningless.
	///
	/// Defaults to `40`.
	pub height: u32,
	/// Up-down styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Note that the `UDS::AUTOBUDDY` style will take the control created
	/// immediately before the `UpDown` as the buddy one, attaching the `UpDown`
	/// to it. This control should be an [`Edit`](crate::gui::Edit) with
	/// [`ES::NUMBER`](crate::co::ES::NUMBER) style.
	///
	/// Defaults to `UDS::AUTOBUDDY | UDS::SETBUDDYINT | UDS::ALIGNRIGHT | UDS::ARROWKEYS | UDS::HOTTRACK`.
	pub up_down_style: co::UDS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILDWINDOW | WS::VISIBLE`.
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

	/// The minimum and maximum position values.
	///
	/// Defaults to `(0, 100)`.
	pub range: (i32, i32),
}

impl Default for UpDownOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			height: 40,
			up_down_style: co::UDS::AUTOBUDDY | co::UDS::SETBUDDYINT |
				co::UDS::ALIGNRIGHT | co::UDS::ARROWKEYS | co::UDS::HOTTRACK,
			window_style: co::WS::CHILDWINDOW | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			range: (0, 100),
		}
	}
}

impl UpDownOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
