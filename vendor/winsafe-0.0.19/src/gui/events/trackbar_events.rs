use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes trackbar control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TrackbarEvents(BaseEventsProxy);

impl TrackbarEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_withparm_noret! { trbn_thumb_pos_changing, co::TRBN::THUMBPOSCHANGING, NMTRBTHUMBPOSCHANGING;
		/// [`TRBN_THUMBPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/trbn-thumbposchanging)
		/// notification.
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-trackbar)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.0.wm_notify(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.raw() as _)));
	}

	pub_fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-trackbar-)
		/// notification.
	}
}
