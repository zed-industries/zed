use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

/// Exposes item methods of a [`ComboBox`](crate::gui::ComboBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ComboBoxItems<'a> {
	owner: &'a ComboBox,
}

impl<'a> ComboBoxItems<'a> {
	pub(in crate::gui) const fn new(owner: &'a ComboBox) -> Self {
		Self { owner }
	}

	/// Adds new texts by sending [`cb::AddString`](crate::msg::cb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_combo: gui::ComboBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_combo = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
	///
	/// my_combo.items().add(&["John", "Mary"]);
	/// ```
	pub fn add(&self, items: &[impl AsRef<str>]) {
		for text in items.iter() {
			self.owner.hwnd()
				.SendMessage(cb::AddString {
					text: WString::from_str(text.as_ref()),
				})
				.unwrap();
		}
	}

	/// Retrieves the number of items by sending a
	/// [`cb::GetCount`](crate::msg::cb::GetCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(cb::GetCount {})
			.unwrap()
	}

	/// Deletes the item at the given index by sending a
	/// [`cb::DeleteString`](crate::msg::cb::DeleteString) message.
	///
	/// # Panics
	///
	/// Panics if the index is invalid.
	pub fn delete(&self, index: u32) {
		self.owner.hwnd()
			.SendMessage(cb::DeleteString { index })
			.unwrap();
	}

	/// Deletes all items by sending a
	/// [`cb::ResetContent`](crate::msg::cb::ResetContent) message.
	pub fn delete_all(&self) {
		self.owner.hwnd().SendMessage(cb::ResetContent {});
	}

	/// Returns an iterator over the texts.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_combo: gui::ComboBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_combo = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
	///
	/// for text in my_combo.items().iter() {
	///     println!("Text {}", text);
	/// }
	/// ```
	#[must_use]
	pub fn iter(&self) -> impl Iterator<Item = String> + 'a {
		ComboBoxItemIter::new(self.owner)
	}

	/// Sets the currently selected index, or clears it, by sending a
	/// [`cb::SetCurSel`](crate::msg::cb::SetCurSel) message.
	pub fn select(&self, index: Option<u32>) {
		self.owner.hwnd().SendMessage(cb::SetCurSel { index });
	}

	/// Retrieves the index of the currently selected item, if any, by sending a
	/// [`cb::GetCurSel`](crate::msg::cb::GetCurSel) message.
	#[must_use]
	pub fn selected_index(&self) -> Option<u32> {
		self.owner.hwnd().SendMessage(cb::GetCurSel {})
	}

	/// Retrieves the currently selected text, if any, by calling
	/// [`selected_index`](crate::gui::spec::ComboBoxItems::selected_index) and
	/// [`text`](crate::gui::spec::ComboBoxItems::text) methods.
	#[must_use]
	pub fn selected_text(&self) -> Option<String> {
		self.selected_index()
			.map(|idx| self.text(idx))
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`cb::GetLbText`](crate::msg::cb::GetLbText) message.
	///
	/// # Panics
	///
	/// Panics if the index is invalid.
	#[must_use]
	pub fn text(&self, index: u32) -> String {
		let num_chars = self.owner.hwnd()
			.SendMessage(cb::GetLbTextLen { index })
			.unwrap();
		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);
		self.owner.hwnd()
			.SendMessage(cb::GetLbText { index, text: &mut buf })
			.unwrap();
		buf.to_string()
	}
}

//------------------------------------------------------------------------------

struct ComboBoxItemIter<'a> {
	owner: &'a ComboBox,
	count: u32,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for ComboBoxItemIter<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let num_chars = self.owner.hwnd()
			.SendMessage(cb::GetLbTextLen { index: self.current })
			.unwrap();
		self.buffer = WString::new_alloc_buf(num_chars as usize + 1);

		self.owner.hwnd()
			.SendMessage(cb::GetLbText {
				index: self.current,
				text: &mut self.buffer,
			})
			.unwrap();
		self.current += 1;
		Some(self.buffer.to_string())
	}
}

impl<'a> ComboBoxItemIter<'a> {
	fn new(owner: &'a ComboBox) -> Self {
		Self {
			owner,
			count: owner.items().count(),
			current: 0,
			buffer: WString::new_alloc_buf(40), // arbitrary
		}
	}
}
