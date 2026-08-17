use crate::co;
use crate::decl::*;
use crate::gui::{*, spec::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes the methods of a [`Tab`](crate::gui::Tab) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TabItems<'a> {
	owner: &'a Tab,
}

impl<'a> TabItems<'a> {
	pub(in crate::gui) const fn new(owner: &'a Tab) -> Self {
		Self { owner }
	}

	/// Manually appends a new tab by sending a
	/// [`tcm::InsertItem`](crate::msg::tcm::InsertItem) message, and returns
	/// the newly added item.
	///
	/// # Safety
	///
	/// By adding a tab item manually, you are responsible for all the message
	/// handling. Prefer adding items automatically by filling the
	/// [`TabOpts::items`](crate::gui::TabOpts::items) member when calling the
	/// [`Tab::new`](crate::gui::Tab::new) function.
	pub unsafe fn add(&self, title: &str) -> TabItem<'a> {
		let mut wtitle = WString::from_str(title);
		let mut tci = TCITEM::default();
		tci.mask = co::TCIF::TEXT;
		tci.set_pszText(Some(&mut wtitle));

		self.get( // retrieve the item newly added
			self.owner.hwnd()
				.SendMessage(tcm::InsertItem {
					index: 0x0fff_ffff, // insert as the last item
					item: &tci,
				})
				.unwrap(),
		)
	}

	/// Retrieves the total number of items by sending an
	/// [`tcm::GetItemCount`](crate::msg::tcm::GetItemCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(tcm::GetItemCount {})
			.unwrap()
	}

	/// Deletes all items by sending a
	/// [`tcm::DeleteAllItems`](crate::msg::tcm::DeleteAllItems) message.
	///
	/// # Safety
	///
	/// If you delete a tab automatically created, which has a container window
	/// attached to it, the rendering will be out-of-order.
	pub unsafe fn delete_all(&self) {
		self.owner.hwnd()
			.SendMessage(tcm::DeleteAllItems {})
			.unwrap();
	}

	/// Retrieves the item at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing items, an object will still be returned. However, operations
	/// upon this object will fail.
	#[must_use]
	pub const fn get(&self, index: u32) -> TabItem<'a> {
		TabItem::new(self.owner, index)
	}

	/// Returns the focused item by sending a
	/// [`tcm::GetCurSel`](crate::msg::tcm::GetCurSel) message.
	#[must_use]
	pub fn focused(&self) -> Option<TabItem<'a>> {
		self.owner.hwnd()
			.SendMessage(tcm::GetCurFocus {})
			.map(|i| self.get(i))
	}

	/// Returns the selected item by sending a
	/// [`tcm::GetCurSel`](crate::msg::tcm::GetCurSel) message.
	#[must_use]
	pub fn selected(&self) -> Option<TabItem<'a>> {
		self.owner.hwnd()
			.SendMessage(tcm::GetCurSel {})
			.map(|i| self.get(i))
	}
}
