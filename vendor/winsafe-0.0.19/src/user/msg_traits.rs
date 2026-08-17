use crate::msg::*;

/// Parameters of a message which can be sent. Implemented by [all undefined
/// messages](crate::msg).
///
/// Allows the conversion to the generic [`WndMsg`](crate::msg::WndMsg)
/// parameters, and also defines the return type of the message.
///
/// Used in functions like
/// [`SendMessage`](crate::prelude::user_Hwnd::SendMessage) and
/// [`DefWindowProc`](`crate::prelude::user_Hwnd::DefWindowProc`).
pub unsafe trait MsgSend {
	/// The specific type of the value returned by the message.
	type RetType;

	/// Converts the generic `isize` return value to the specific type returned
	/// by the message.
	#[must_use]
	fn convert_ret(&self, v: isize) -> Self::RetType;

	/// Converts the specific message parameters struct into the generic
	/// [`WndMsg`](crate::msg::WndMsg) message struct.
	#[must_use]
	fn as_generic_wm(&mut self) -> WndMsg;
}

/// Parameters of a message which can be sent and handled (received).
/// Implemented by [`WndMsg`](crate::msg::WndMsg) and all
/// [msg::wm](`crate::msg::wm`) messages.
///
/// Allows the conversion from and to the generic [`WndMsg`](crate::msg::WndMsg)
/// parameters, and also defines the return type of the message.
pub unsafe trait MsgSendRecv: MsgSend {
	/// Converts the generic [`WndMsg`](crate::msg::WndMsg) parameters struct
	/// into the specific message struct.
	#[must_use]
	fn from_generic_wm(parm: WndMsg) -> Self;
}
