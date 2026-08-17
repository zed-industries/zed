use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes label control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
/// who is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by
/// the control.
pub struct LabelEvents(BaseEventsProxy);

impl LabelEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_noparm_noret! { stn_clicked, co::STN::CLICKED;
		/// [`STN_CLICKED`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-clicked)
		/// notification.
	}

	pub_fn_cmd_noparm_noret! { stn_dbl_clk, co::STN::DBLCLK;
		/// [`STN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-dblclk)
		/// notification.
	}

	pub_fn_cmd_noparm_noret! { stn_disable, co::STN::DISABLE;
		/// [`STN_DISABLE`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-disable)
		/// notification.
	}

	pub_fn_cmd_noparm_noret! { stn_enable, co::STN::ENABLE;
		/// [`STN_ENABLE`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-enable)
		/// notification.
	}
}
