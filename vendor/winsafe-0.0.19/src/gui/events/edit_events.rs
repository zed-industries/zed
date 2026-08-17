use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes edit control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct EditEvents(BaseEventsProxy);

impl EditEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_noparm_noret! { en_align_ltr_ec, co::EN::ALIGN_LTR_EC;
		/// [`EN_ALIGN_LTR_EC`](https://learn.microsoft.com/en-us/windows/win32/controls/en-align-ltr-ec)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_align_rtl_ec, co::EN::ALIGN_RTL_EC;
		/// [`EN_ALIGN_LTR_EC`](https://learn.microsoft.com/en-us/windows/win32/controls/en-align-rtl-ec)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_change, co::EN::CHANGE;
		/// [`EN_CHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/en-change)
		/// command notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let txt: gui::Edit;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let txt = gui::Edit::new(&wnd, gui::EditOpts::default());
		///
		/// let txt2 = txt.clone(); // to pass into the closure
		///
		/// txt.on().en_change(
		///     move || -> w::AnyResult<()> {
		///         println!("Text: {}", txt2.text());
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_cmd_noparm_noret! { en_err_space, co::EN::ERRSPACE;
		/// [`EN_ERRSPACE`](https://learn.microsoft.com/en-us/windows/win32/controls/en-errspace)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_h_scroll, co::EN::HSCROLL;
		/// [`EN_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/en-hscroll)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_kill_focus, co::EN::KILLFOCUS;
		/// [`EN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/en-killfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_max_text, co::EN::MAXTEXT;
		/// [`EN_MAXTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/en-maxtext)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_set_focus, co::EN::SETFOCUS;
		/// [`EN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/en-setfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_update, co::EN::UPDATE;
		/// [`EN_UPDATE`](https://learn.microsoft.com/en-us/windows/win32/controls/en-update)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_v_scroll, co::EN::VSCROLL;
		/// [`EN_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/en-vscroll)
		/// command notification.
	}
}
