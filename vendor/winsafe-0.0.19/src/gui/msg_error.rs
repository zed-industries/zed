use crate::msg::*;

/// An error that occurred within a closure of a window message handling.
/// Usually these errors are thrown by the user closures.
///
/// This error types wraps the actual user error along with the parameters of
/// the message where the error happened.
pub struct MsgError {
	src_msg: WndMsg,
	source: Box<dyn std::error::Error + Send + Sync>,
}

impl std::error::Error for MsgError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		Some(self.source.as_ref())
	}
}

impl std::fmt::Display for MsgError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "WM {} - {}",
			self.src_msg.msg_id, self.source.to_string())
	}
}
impl std::fmt::Debug for MsgError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self, f)
	}
}

impl MsgError {
	/// Constructs a new `MsgError` by wrapping the given error.
	#[must_use]
	pub const fn new(
		src_msg: WndMsg,
		source: Box<dyn std::error::Error + Send + Sync>,
	) -> MsgError
	{
		Self { src_msg, source }
	}

	/// The source message information where the error originated from.
	#[must_use]
	pub const fn src_msg(&self) -> WndMsg {
		self.src_msg
	}
}
