use std::any::Any;

use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::prelude::*;

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawModal), Dlg(DlgModal) }

//------------------------------------------------------------------------------

/// An user modal window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowModal(RawDlg);

unsafe impl Send for WindowModal {}

impl GuiWindow for WindowModal {
	fn hwnd(&self) -> &HWND {
		match &self.0 {
			RawDlg::Raw(r) => r.hwnd(),
			RawDlg::Dlg(d) => d.hwnd(),
		}
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindowText for WindowModal {}

impl GuiParent for WindowModal {
	fn on(&self) -> &WindowEventsAll {
		match &self.0 {
			RawDlg::Raw(r) => r.on(),
			RawDlg::Dlg(d) => d.on(),
		}
	}

	unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		match &self.0 {
			RawDlg::Raw(r) => r.as_base(),
			RawDlg::Dlg(d) => d.as_base(),
		}
	}
}

impl GuiThread for WindowModal {
	fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		match &self.0 {
			RawDlg::Raw(r) => r.spawn_new_thread(func),
			RawDlg::Dlg(d) => d.spawn_new_thread(func),
		}
	}

	fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static
	{
		match &self.0 {
			RawDlg::Raw(r) => r.run_ui_thread(func),
			RawDlg::Dlg(d) => d.run_ui_thread(func),
		}
	}
}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created internally with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: WindowModalOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Raw(
				RawModal::new(parent_base_ref, opts),
			),
		)
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with
	/// [`HINSTANCE::DialogBoxParam`](crate::prelude::user_Hinstance::DialogBoxParam).
	#[must_use]
	pub fn new_dlg(parent: &impl GuiParent, dialog_id: u16) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Dlg(
				DlgModal::new(parent_base_ref, dialog_id),
			),
		)
	}

	/// Physically creates the window, then runs the modal loop. This method
	/// will block until the window is closed.
	///
	/// For a modal created with
	/// [`WindowModal::new`](crate::gui::WindowModal::new), the returned `i32`
	/// is always zero.
	///
	/// For a modal created with
	/// [`WindowModal::new_dlg`](crate::gui::WindowModal::new_dlg), the returned
	/// `i32` is the value passed to
	/// [`HWND::EndDialog`](crate::prelude::user_Hwnd::EndDialog). Note that, if
	/// the user clicks the "X" to close the modal, the default behavior is to
	/// call `EndDialog(0)`. To override this behavior, simply handle the
	/// modal's [`wm_close`](crate::prelude::GuiEvents::wm_close) yourself.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn show_modal(&self) -> AnyResult<i32> {
		match &self.0 {
			RawDlg::Raw(r) => r.show_modal(),
			RawDlg::Dlg(d) => d.show_modal(),
		}
	}
}
