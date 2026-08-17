use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::prelude::*;

struct Obj { // actual fields of DlgModal
	dlg_base: DlgBase,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A dialog-base modal window.
#[derive(Clone)]
pub(in crate::gui) struct DlgModal(Pin<Arc<Obj>>);

impl DlgModal {
	pub(in crate::gui) fn new(parent: &Base, dialog_id: u16) -> Self {
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(Some(parent), dialog_id),
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers();
		new_self
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.dlg_base.as_base()
	}

	pub(in crate::gui) fn hwnd(&self) -> &HWND {
		self.0.dlg_base.hwnd()
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.0.dlg_base.on()
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		self.0.dlg_base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static
	{
		self.0.dlg_base.run_ui_thread(func);
	}

	pub(in crate::gui) fn show_modal(&self) -> AnyResult<i32> {
		self.0.dlg_base.dialog_box_param()
			.map_err(|err| err.into())
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.dlg_base.privileged_on().wm_init_dialog(move |_| {
			let hwnd = self2.hwnd();
			let rc = hwnd.GetWindowRect()?;
			let rc_parent = hwnd.GetParent()?.GetWindowRect()?;
			hwnd.SetWindowPos( // center modal on parent
				HwndPlace::None,
				POINT::new(
					rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
					rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
				),
				SIZE::default(),
				co::SWP::NOSIZE | co::SWP::NOZORDER,
			)?;
			Ok(true) // not meaningful
		});

		let self2 = self.clone();
		self.on().wm_close(move || { // user clicked the X button
			self2.hwnd().EndDialog(0)?;
			Ok(())
		});
	}
}
