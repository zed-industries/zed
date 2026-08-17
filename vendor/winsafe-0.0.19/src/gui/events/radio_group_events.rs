use std::ptr::NonNull;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::prelude::*;

/// Exposes button control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
/// for a [`RadioGroup`](crate::gui::RadioGroup).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct RadioGroupEvents {
	parent_ptr: NonNull<Base>,
	ctrl_ids: Vec<u16>,
}

impl RadioGroupEvents {
	pub(in crate::gui) fn new(
		parent_base: &Base,
		ctrl_ids: Vec<u16>,
	) -> Self
	{
		Self {
			parent_ptr: NonNull::from(parent_base),
			ctrl_ids,
		}
	}

	fn parent_user_events(&self) -> &WindowEventsAll {
		unsafe { self.parent_ptr.as_ref().on() }
	}

	/// [`BN_CLICKED`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-clicked)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when the user clicks a button.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// let radios: gui::RadioGroup;
	/// # let radios = gui::RadioGroup::new(&wnd, &[]);
	///
	/// radios.on().bn_clicked({
	///     let radios = radios.clone();
	///     move || -> w::AnyResult<()> {
	///         println!("Selected {}",
	///             radios.checked().unwrap()
	///                 .hwnd().GetWindowText()?,
	///         );
	///         Ok(())
	///     }
	/// });
	/// ```
	pub fn bn_clicked<F>(&self, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::CLICKED, *ctrl_id as _, {
				let shared_func = shared_func.clone();
				move || shared_func()
			});
		}
	}

	/// [`BN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when the user double-clicks a button. This notification code is
	/// sent automatically for [`BS::USERBUTTON`](crate::co::BS::USERBUTTON),
	/// [`BS::RADIOBUTTON`](crate::co::BS::RADIOBUTTON), and
	/// [`BS::OWNERDRAW`](crate::co::BS::OWNERDRAW) buttons. Other button types
	/// send only if they have the [`BS::NOTIFY`](crate::co::BS::NOTIFY) style.
	pub fn bn_dbl_clk<F>(&self, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::DBLCLK, *ctrl_id as _, {
				let shared_func = shared_func.clone();
				move || shared_func()
			});
		}
	}

	/// [`BN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when a button loses the keyboard focus. The button must have the
	/// [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this notification
	/// code.
	pub fn bn_kill_focus<F>(&self, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::KILLFOCUS, *ctrl_id as _, {
				let shared_func = shared_func.clone();
				move || shared_func()
			});
		}
	}

	/// [`BN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when a button receives the keyboard focus. The button must have the
	/// [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this notification
	/// code.
	pub fn bn_set_focus<F>(&self, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::SETFOCUS, *ctrl_id as _, {
				let shared_func = shared_func.clone();
				move || shared_func()
			});
		}
	}
}
