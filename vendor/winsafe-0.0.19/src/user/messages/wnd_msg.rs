use crate::co;
use crate::prelude::*;

/// Generic
/// [window message](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters: `WPARAM` and `LPARAM`.
///
/// All message types can be converted to `WndMsg` via the
/// [`as_generic_wm`](crate::prelude::MsgSend::as_generic_wm) method.
///
/// Return type: `isize`.
#[derive(Clone, Copy, Debug)]
pub struct WndMsg {
	/// The [`co::WM`](crate::co::WM) constant that identifies the window
	/// message.
	pub msg_id: co::WM,
	/// First message parameter.
	pub wparam: usize,
	/// Second message parameter.
	pub lparam: isize,
}

unsafe impl MsgSend for WndMsg {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&mut self) -> Self {
		*self
	}
}

unsafe impl MsgSendRecv for WndMsg {
	fn from_generic_wm(p: Self) -> Self {
		p
	}
}

impl WndMsg {
	/// Creates a new `WndMsg` from the given message parameters.
	#[must_use]
	pub const fn new(msg_id: co::WM, wparam: usize, lparam: isize) -> WndMsg {
		Self { msg_id, wparam, lparam }
	}
}
