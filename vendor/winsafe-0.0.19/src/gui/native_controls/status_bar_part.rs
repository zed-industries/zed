use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

/// A single part of a [`StatusBar`](crate::gui::StatusBar) control.
///
/// **Note:** Each object keeps the zero-based index of a part. If new parts are
/// added/removed from the status bar control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[derive(Clone, Copy)]
pub struct StatusBarPart<'a> {
	owner: &'a StatusBar,
	index: u8,
}

impl<'a> StatusBarPart<'a> {
	pub(in crate::gui) const fn new(owner: &'a StatusBar, index: u8) -> Self {
		Self { owner, index }
	}

	/// Sets the icon of a part by sending an
	/// [`sb::SetIcon`](crate::msg::sb::SetIcon) message.
	pub fn set_icon(&self, hicon: Option<&HICON>) {
		self.owner.hwnd()
			.SendMessage(sb::SetIcon {
				part_index: self.index,
				hicon,
			})
			.unwrap();
	}

	/// Sets the text of a part by sending an
	/// [`sb::SetText`](crate::msg::sb::SetText) message.
	pub fn set_text(&self, text: &str) {
		self.owner.hwnd()
			.SendMessage(sb::SetText {
				part_index: self.index,
				draw_operation: co::SBT::BORDER,
				text: WString::from_str(text),
			})
			.unwrap();
	}

	/// Retrieves the text of the item by sending a
	/// [`sb::GetText`](crate::msg::sb::GetText) message.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_sb: gui::StatusBar; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_sb = gui::StatusBar::new(&wnd, &[]);
	///
	/// println!("Text: {}", my_sb.parts().get(0).text());
	/// ```
	#[must_use]
	pub fn text(&self) -> String {
		let (num_chars, _) = self.owner.hwnd()
			.SendMessage(sb::GetTextLength { part_index: self.index });

		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);
		self.owner.hwnd()
			.SendMessage(sb::GetText {
				part_index: self.index,
				text: &mut buf,
			});
		buf.to_string()
	}
}
