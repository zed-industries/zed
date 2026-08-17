use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes tab control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TabEvents(BaseEventsProxy);

impl TabEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_noparm_noret! { nm_click, co::NM::CLICK;
		/// [`NM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-click-tab)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_dbl_clk, co::NM::DBLCLK;
		/// [`NM_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-dblclk-tab)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_r_click, co::NM::RCLICK;
		/// [`NM_RCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rclick-tab)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_r_dbl_clk, co::NM::RDBLCLK;
		/// [`NM_RDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-tab)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-tab-)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { tcn_focus_change, co::TCN::FOCUSCHANGE;
		/// [`TCN_FOCUSCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-focuschange)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tcn_get_object, co::TCN::GETOBJECT, NMOBJECTNOTIFY;
		/// [`TCN_GETOBJECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-getobject)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tcn_key_down, co::TCN::KEYDOWN, NMTCKEYDOWN;
		/// [`TCN_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-keydown)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { tcn_sel_change, co::TCN::SELCHANGE;
		/// [`TCN_SELCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-selchange)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let tab: gui::Tab;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let tab = gui::Tab::new(&wnd, gui::TabOpts::default());
		///
		/// tab.on().tcn_sel_change(
		///     move || -> w::AnyResult<()> {
		///         println!("Changed tab.");
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_nfy_noparm_boolret! { tcn_sel_changing, co::TCN::SELCHANGING;
		/// [`TCN_SELCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-selchanging)
		/// notification.
	}
}
