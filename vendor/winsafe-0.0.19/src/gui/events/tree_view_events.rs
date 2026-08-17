use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes tree view control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TreeViewEvents(BaseEventsProxy);

impl TreeViewEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_withparm_noret! { tvn_delete_item, co::TVN::DELETEITEM, NMTREEVIEW;
		/// [`TVN_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvn-deleteitem)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tvn_item_changed, co::TVN::ITEMCHANGED, NMTREEVIEW;
		/// [`TVN_ITEMCHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/tvn-itemchanged)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { tvn_item_changing, co::TVN::ITEMCHANGING, NMTREEVIEW;
		/// [`TVN_ITEMCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/tvn-itemchanging)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tvn_item_expanded, co::TVN::ITEMEXPANDED, NMTREEVIEW;
		/// [`TVN_ITEMEXPANDED`](https://learn.microsoft.com/en-us/windows/win32/controls/tvn-itemexpanded)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { tvn_item_expanding, co::TVN::ITEMEXPANDING, NMTREEVIEW;
		/// [`TVN_ITEMEXPANDING`](https://learn.microsoft.com/en-us/windows/win32/controls/tvn-itemexpanding)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tvn_sel_changed, co::TVN::SELCHANGED, NMTREEVIEW;
		/// [`TVN_SELCHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/tvn-selchanged)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui, co};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let tree: gui::TreeView;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let tree = gui::TreeView::new(&wnd, gui::TreeViewOpts::default());
		///
		/// tree.on().tvn_sel_changed(
		///     move |p: &w::NMTREEVIEW| -> w::AnyResult<()> {
		///         println!(
		///             "Old item: {} - new item: {} - action: {}",
		///             p.itemOld.pszText().unwrap(),
		///             p.itemNew.pszText().unwrap(),
		///             unsafe { co::TVC::from_raw(p.action) },
		///         );
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_nfy_withparm_boolret! { tvn_sel_changing, co::TVN::SELCHANGING, NMTREEVIEW;
		/// [`TVN_SELCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/tvn-selchanging)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_click, co::NM::CLICK;
		/// [`NM_CLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-click-tree-view)
		/// notification.
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-tree-view)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&mut NMTVCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.0.wm_notify(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr_mut::<NMTVCUSTOMDRAW>() })?.raw() as _)));
	}

	pub_fn_nfy_noparm_i32ret! { nm_dbl_clk, co::NM::DBLCLK;
		/// [`NM_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-dblclk-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_kill_focus, co::NM::KILLFOCUS;
		/// [`NM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-killfocus-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_r_click, co::NM::RCLICK;
		/// [`NM_RCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rclick-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_r_dbl_clk, co::NM::RDBLCLK;
		/// [`NM_RDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_return, co::NM::RETURN;
		/// [`NM_RETURN`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-return-tree-view-)
		/// notification.
	}

	pub_fn_nfy_withparm_i32ret! { nm_set_cursor, co::NM::SETCURSOR, NMMOUSE;
		/// [`NM_MOUSE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-setcursor-tree-view-)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_set_focus, co::NM::SETFOCUS;
		/// [`NM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-setfocus-tree-view-)
		/// notification.
	}
}
