use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes button control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ButtonEvents(BaseEventsProxy);

impl ButtonEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_withparm_noret! { bcn_drop_down, co::BCN::DROPDOWN, NMBCDROPDOWN;
		/// [`BCN_DROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/bcn-dropdown)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { bcn_hot_item_change, co::BCN::HOTITEMCHANGE, NMBCHOTITEM;
		/// [`BCN_HOTITEMCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcn-hotitemchange)
		/// notification.
	}

	pub_fn_cmd_noparm_noret! { bn_clicked, co::BN::CLICKED;
		/// [`BN_CLICKED`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-clicked)
		/// command notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let btn: gui::Button;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let btn = gui::Button::new(&wnd, gui::ButtonOpts::default());
		///
		/// btn.on().bn_clicked(
		///     move || -> w::AnyResult<()> {
		///         println!("Clicked.");
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_cmd_noparm_noret! { bn_dbl_clk, co::BN::DBLCLK;
		/// [`BN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { bn_kill_focus, co::BN::KILLFOCUS;
		/// [`BN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { bn_set_focus, co::BN::SETFOCUS;
		/// [`BN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
		/// command notification.
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-button)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.0.wm_notify(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.raw() as _)));
	}
}
