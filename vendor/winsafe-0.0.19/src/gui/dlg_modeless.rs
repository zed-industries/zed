use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::prelude::*;

struct Obj { // actual fields of DlgModeless
	dlg_base: DlgBase,
	position: POINT,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A dialog-based modeless window.
#[derive(Clone)]
pub(in crate::gui) struct DlgModeless(Pin<Arc<Obj>>);

impl DlgModeless {
	pub(in crate::gui) fn new(
		parent: &Base,
		dialog_id: u16,
		position: POINT,
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(Some(parent), dialog_id),
					position,
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers(parent);
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

	fn default_message_handlers(&self, parent: &Base) {
		let self2 = self.clone();
		parent.privileged_on().wm(parent.wm_create_or_initdialog(), move |_| {
			self2.0.dlg_base.create_dialog_param()?;
			self2.hwnd().ShowWindow(co::SW::SHOW);

			let dlg_pos = adjust_modeless_pos(
				self2.0.dlg_base.parent().unwrap(), self2.0.position)?;

			self2.hwnd().SetWindowPos(
				HwndPlace::None,
				dlg_pos, SIZE::default(),
				co::SWP::NOZORDER | co::SWP::NOSIZE,
			)?;
			Ok(None) // not meaningful
		});
	}
}
