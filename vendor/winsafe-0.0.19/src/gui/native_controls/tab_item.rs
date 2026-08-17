use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

/// A single item of a [`Tab`](crate::gui::Tab) control.
///
/// **Note:** Each object keeps the zero-based index of an item. If new items
/// are added/removed from the tab control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[derive(Clone, Copy)]
pub struct TabItem<'a> {
	owner: &'a Tab,
	index: u32,
}

impl<'a> TabItem<'a> {
	pub(in crate::gui) const fn new(owner: &'a Tab, index: u32) -> Self {
		Self { owner, index }
	}

	/// Returns the zero-based index of the item.
	#[must_use]
	pub const fn index(&self) -> u32 {
		self.index
	}

	/// Deletes the item by sending a
	/// [`tcm::DeleteItem`](crate::msg::tcm::DeleteItem) message.
	///
	/// # Safety
	///
	/// If you delete a tab automatically created, which has a container window
	/// attached to it, the rendering will be out-of-order.
	pub unsafe fn delete(&self) {
		self.owner.hwnd()
			.SendMessage(tcm::DeleteItem { index: self.index })
			.unwrap();
	}

	/// Retrieves the user-defined value by sending an
	/// [`tcm::GetItem`](crate::msg::tcm::GetItem) message.
	#[must_use]
	pub fn lparam(&self) -> isize {
		let mut tci = TCITEM::default();
		tci.mask = co::TCIF::PARAM;

		self.owner.hwnd()
			.SendMessage(tcm::GetItem {
				index: self.index,
				item: &mut tci,
			})
			.unwrap();
		tci.lParam
	}

	/// Sets the user-defined value by sending an
	/// [`lvm::SetItem`](crate::msg::lvm::SetItem) message.
	pub fn set_lparam(&self, lparam: isize) {
		let mut tci = TCITEM::default();
		tci.mask = co::TCIF::PARAM;
		tci.lParam = lparam;

		self.owner.hwnd()
			.SendMessage(tcm::SetItem {
				index: self.index,
				item: &mut tci,
			})
			.unwrap();
	}

	/// Sets the text by sending a
	/// [`tcm:SetItem`](crate::msg::tcm::SetItem) message.
	pub fn set_text(&self, text: &str) {
		let mut wtext = WString::from_str(text);
		let mut tci = TCITEM::default();
		tci.mask = co::TCIF::TEXT;
		tci.set_pszText(Some(&mut wtext));

		self.owner.hwnd()
			.SendMessage(tcm::SetItem {
				index: self.index,
				item: &mut tci,
			})
			.unwrap();
	}

	/// Retrieves the text by sending a
	/// [`tcm:GetItem`](crate::msg::tcm::GetItem) message.
	#[must_use]
	pub fn text(&self) -> String {
		let mut buf = WString::new_alloc_buf(64); // arbitrary
		let mut tci = TCITEM::default();
		tci.mask = co::TCIF::TEXT;
		tci.set_pszText(Some(&mut buf));

		self.owner.hwnd()
			.SendMessage(tcm::GetItem {
				index: self.index,
				item: &mut tci,
			})
			.unwrap();
		buf.to_string()
	}
}
