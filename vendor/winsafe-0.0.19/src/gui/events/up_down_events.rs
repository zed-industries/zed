use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes up-down control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct UpDownEvents(BaseEventsProxy);

impl UpDownEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-list-view-)
		/// notification.
	}

	pub_fn_nfy_withparm_i32ret! { udn_delta_pos, co::UDN::DELTAPOS, NMUPDOWN;
		/// [`UDN_DELTAPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/udn-deltapos)
		/// notification.
	}
}
