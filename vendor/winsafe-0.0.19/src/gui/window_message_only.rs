use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::prelude::*;
use crate::user::privs::*;

/// A
/// [message-only](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows)
/// window, which can handle events.
#[derive(Clone)]
pub struct WindowMessageOnly(Pin<Arc<RawBase>>);

unsafe impl Send for WindowMessageOnly {}

impl GuiWindow for WindowMessageOnly {
	fn hwnd(&self) -> &HWND {
		self.0.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for WindowMessageOnly {
	fn on(&self) -> &WindowEventsAll {
		self.0.on()
	}

	unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.as_base()
	}
}

impl WindowMessageOnly {
	/// Instantiates a new `WindowMessageOnly` object, to be created internally
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new(parent: Option<&WindowMessageOnly>) -> Self {
		let parent_base_ref = parent.map(|parent| {
			let base_ptr = unsafe { parent.as_base() } as *mut Base;
			unsafe { base_ptr.as_ref() }.unwrap()
		});

		let new_self = Self(
			Arc::pin(RawBase::new(parent_base_ref)),
		);
		new_self.create();
		new_self
	}

	fn create(&self) {
		let hinst = HINSTANCE::GetModuleHandle(None).unwrap();
		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::default();
		RawBase::fill_wndclassex(
			&hinst,
			co::CS::default(), &Icon::None, &Icon::None,
			&Brush::None, &Cursor::None, &mut wcx,
			&mut class_name_buf).unwrap();
		let atom = self.0.register_class(&mut wcx).unwrap();

		let hparent_msg = unsafe { HWND::from_ptr(HWND_MESSAGE as _) };

		self.0.create_window(
			Some(match self.0.parent() {
				Some(parent) => parent.hwnd(),
				None => &hparent_msg, // special case: message-only window with no parent
			}),
			atom, None, IdMenu::None,
			POINT::default(), SIZE::default(),
			co::WS_EX::NoValue, co::WS::NoValue,
		).unwrap();
	}
}
