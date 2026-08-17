use std::cell::UnsafeCell;
use std::ptr::NonNull;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

static mut BASE_SUBCLASS_ID: usize = 0;

/// Variant field for creating child controls.
pub(in crate::gui) enum OptsResz<T> {
	/// Options for a raw control creation.
	Wnd(T),
	/// Resize behavior for a dialog control creation.
	Dlg((Horz, Vert)),
}

//------------------------------------------------------------------------------

/// Base to all native child controls.
///
/// Owns the window procedure for all subclassed native child controls.
pub(in crate::gui) struct BaseNativeControl {
	ctrl_id: u16,
	hwnd: UnsafeCell<HWND>,
	parent_ptr: NonNull<Base>, // base of WindowControl, WindowMain or WindowModal
	subclass_events: WindowEvents, // for control subclassing
}

impl BaseNativeControl {
	pub(in crate::gui) fn new(parent: &Base, ctrl_id: u16) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a child control after the parent window is created.");
		}

		Self {
			ctrl_id,
			hwnd: UnsafeCell::new(HWND::NULL),
			parent_ptr: NonNull::from(parent),
			subclass_events: WindowEvents::new(),
		}
	}

	pub(in crate::gui) const fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}

	pub(in crate::gui) fn hwnd(&self) -> &HWND {
		unsafe { &mut *self.hwnd.get() }
	}

	pub(in crate::gui) fn parent(&self) -> &Base {
		unsafe { self.parent_ptr.as_ref() }
	}

	pub(in crate::gui) fn on_subclass(&self) -> &WindowEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add subclass events after control creation.");
		} else if *self.parent().hwnd() != HWND::NULL {
			panic!("Cannot add subclass events after parent window creation.");
		}
		&self.subclass_events
	}

	/// Creates the child control with `CreateWindowEx`.
	pub(in crate::gui) fn create_window(&self,
		class_name: &str,
		title: Option<&str>,
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS,
	) -> SysResult<()>
	{
		let hparent = self.parent().hwnd();

		if *self.hwnd() != HWND::NULL {
			panic!("Cannot create control twice.");
		} else if *hparent == HWND::NULL {
			panic!("Cannot create control before parent window creation.");
		}

		unsafe {
			*&mut *self.hwnd.get() = HWND::CreateWindowEx(
				ex_styles,
				AtomStr::from_str(class_name),
				title, styles,
				pos, sz,
				Some(hparent),
				IdMenu::Id(self.ctrl_id),
				&hparent.hinstance(),
				None,
			)?;
		}

		self.install_subclass_if_needed()?;
		Ok(())
	}

	/// Assigns the control ID with `GetDlgItem`.
	pub(in crate::gui) fn create_dlg(&self) -> SysResult<()> {
		if !self.parent().is_dialog() {
			panic!("Parent window is not a dialog, cannot create control.");
		}

		let hparent = self.parent().hwnd();

		if *self.hwnd() != HWND::NULL {
			panic!("Cannot create control twice.");
		} else if *hparent == HWND::NULL {
			panic!("Cannot create control before parent window creation.");
		}

		*unsafe { &mut *self.hwnd.get() } = hparent.GetDlgItem(self.ctrl_id)?;
		self.install_subclass_if_needed()?;
		Ok(())
	}

	fn install_subclass_if_needed(&self) -> SysResult<()> {
		if !self.subclass_events.is_empty() {
			let subclass_id = unsafe {
				BASE_SUBCLASS_ID += 1;
				BASE_SUBCLASS_ID
			};

			unsafe {
				self.hwnd().SetWindowSubclass(
					Self::subclass_proc, subclass_id,
					self as *const _ as _, // pass pointer to self
				)?;
			}
		}
		Ok(())
	}

	extern "system" fn subclass_proc(
		hwnd: HWND,
		msg: co::WM,
		wparam: usize,
		lparam: isize,
		subclass_id: usize,
		ref_data: usize,
	) -> isize
	{
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::subclass_proc_proc(hwnd, wm_any, subclass_id, ref_data)
			.unwrap_or_else(|err| { post_quit_error(wm_any, err); 0 })
	}

	fn subclass_proc_proc(
		hwnd: HWND,
		wm_any: WndMsg,
		subclass_id: usize,
		ref_data: usize,
	) -> AnyResult<isize>
	{
		let ptr_self = ref_data as *mut Self; // retrieve
		let mut process_result = ProcessResult::NotHandled;

		if !ptr_self.is_null() {
			let ref_self = unsafe { &mut *ptr_self };
			if *ref_self.hwnd() != HWND::NULL {
				process_result = ref_self.subclass_events.process_one_message(wm_any)?;
			}
		}

		if wm_any.msg_id == co::WM::NCDESTROY { // always check
			hwnd.RemoveWindowSubclass(Self::subclass_proc, subclass_id)?;
			if !ptr_self.is_null() {
				let ref_self = unsafe { &mut *ptr_self };
				ref_self.subclass_events.clear_events(); // prevents circular references
			}
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefSubclassProc(wm_any).into(),
		})
	}
}
