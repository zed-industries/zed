use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes combo box control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ComboBoxEvents(BaseEventsProxy);

impl ComboBoxEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_noparm_noret! { cbn_close_up, co::CBN::CLOSEUP;
		/// [`CBN_CLOSEUP`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-closeup)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_dbl_clk, co::CBN::DBLCLK;
		/// [`CBN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-dblclk)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_drop_down, co::CBN::DROPDOWN;
		/// [`CBN_DROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-dropdown)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_edit_change, co::CBN::EDITCHANGE;
		/// [`CBN_EDITCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-editchange)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_edit_update, co::CBN::EDITUPDATE;
		/// [`CBN_EDITUPDATE`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-editupdate)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_err_space, co::CBN::ERRSPACE;
		/// [`CBN_ERRSPACE`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-errspace)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_kill_focus, co::CBN::KILLFOCUS;
		/// [`CBN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-killfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_sel_change, co::CBN::SELCHANGE;
		/// [`CBN_SELCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-selchange)
		/// command notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let cmb: gui::ComboBox;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let cmb = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
		///
		/// let cmb2 = cmb.clone(); // to pass into the closure
		///
		/// cmb.on().cbn_sel_change(
		///     move || -> w::AnyResult<()> {
		///         if let Some(sel_text) = cmb2.items().selected_text() {
		///             println!("New selected text: {}", sel_text);
		///         }
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_cmd_noparm_noret! { cbn_sel_end_cancel, co::CBN::SELENDCANCEL;
		/// [`CBN_SELENDCANCEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-selendcancel)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_sel_end_ok, co::CBN::SELENDOK;
		/// [`CBN_SELENDOK`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-selendok)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_set_focus, co::CBN::SETFOCUS;
		/// [`CBN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/cbn-setfocus)
		/// command notification.
	}
}
