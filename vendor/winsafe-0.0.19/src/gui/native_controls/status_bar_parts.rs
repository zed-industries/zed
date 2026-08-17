use crate::gui::{*, spec::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes the part methods of a [`StatusBar`](crate::gui::StatusBar) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct StatusBarParts<'a> {
	owner: &'a StatusBar,
}

impl<'a> StatusBarParts<'a> {
	pub(in crate::gui) const fn new(owner: &'a StatusBar) -> Self {
		Self { owner }
	}

	/// Retrieves the number of parts by sending an
	/// [`sb::GetParts`](crate::msg::sb::GetParts) message.
	#[must_use]
	pub fn count(&self) -> u8 {
		self.owner.hwnd()
			.SendMessage(sb::GetParts { right_edges: None })
	}

	/// Retrieves the part at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing parts, an object will still be returned. However, operations
	/// upon this object will fail.
	#[must_use]
	pub const fn get(&self, index: u8) -> StatusBarPart<'a> {
		StatusBarPart::new(self.owner, index)
	}

	/// Sets the texts of multiple parts at once.
	///
	/// # Panics
	///
	/// Panics if `texts` is empty, or if the number of texts is greater than
	/// the number of columns.
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
	/// my_sb.parts().set_texts(&[
	///     Some("First"),
	///     None, // 2nd part won't have its text changed
	///     Some("Third"),
	///     Some("Fourth"),
	/// ]);
	/// ```
	pub fn set_texts(&self, texts: &[Option<impl AsRef<str>>]) {
		if texts.is_empty() {
			panic!("No texts passed when setting StatusBar parts text.");
		} else if texts.len() > self.count() as usize {
			panic!("Cannot set {} text(s) to {} part(s).", texts.len(), self.count());
		}

		texts.iter()
			.enumerate()
			.for_each(|(idx, maybe_text)| {
				maybe_text.as_ref()
					.map(|text| self.get(idx as _).set_text(text.as_ref()));
			});
	}
}
