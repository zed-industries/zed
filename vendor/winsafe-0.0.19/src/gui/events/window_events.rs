use std::cell::UnsafeCell;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// The result of processing a message.
pub(in crate::gui) enum ProcessResult {
	/// Message was not handled because no function was found.
	NotHandled,
	/// Message handled, and return value is meaningful.
	HandledWithRet(isize),
	/// Message handled, but you should return the default value (0 or FALSE).
	HandledWithoutRet,
}

//------------------------------------------------------------------------------

/// Exposes window
/// [messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct WindowEvents {
	msgs: UnsafeCell< // ordinary WM messages
		FuncStore<
			co::WM,
			Box<dyn Fn(WndMsg) -> AnyResult<Option<isize>>>, // return value may be meaningful
		>,
	>,
}

impl GuiEvents for WindowEvents {
	fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> AnyResult<Option<isize>> + 'static,
	{
		unsafe { &mut *self.msgs.get() }.push(ident, Box::new(func));
	}
}

impl WindowEvents {
	pub(in crate::gui) fn new() -> Self {
		Self { msgs: UnsafeCell::new(FuncStore::new()) }
	}

	pub(in crate::gui) fn is_empty(&self) -> bool {
		unsafe { &mut *self.msgs.get() }.is_empty()
	}

	/// Removes all stored events.
	pub(in crate::gui) fn clear_events(&self) {
		unsafe { &mut *self.msgs.get() }.clear();
	}

	/// Searches for the last added user function for the given message, and
	/// runs if it exists, returning the result.
	pub(in crate::gui) fn process_one_message(&self,
		wm_any: WndMsg,
	) -> AnyResult<ProcessResult>
	{
		let msgs = unsafe { &mut *self.msgs.get() };
		Ok(match msgs.find(wm_any.msg_id) {
			Some(func) => { // we have a stored function to handle this message
				match func(wm_any)? { // execute user function
					Some(res) => ProcessResult::HandledWithRet(res), // meaningful return value
					None => ProcessResult::HandledWithoutRet,
				}
			},
			None => ProcessResult::NotHandled, // no stored function
		})
	}

	/// Searches for all user functions for the given message, and runs all of
	/// them, discarding the results.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_all_messages(&self,
		wm_any: WndMsg,
	) -> AnyResult<bool>
	{
		let mut at_least_one = false;
		let msgs = unsafe { &mut *self.msgs.get() };

		for func in msgs.find_all(wm_any.msg_id) {
			at_least_one = true;
			func(wm_any)?; // execute each stored function
		}
		Ok(at_least_one)
	}
}
