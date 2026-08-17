use crate::co;
use crate::decl::*;
use crate::gui::{*, spec::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes item methods of a [`TreeView`](crate::gui::TreeView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TreeViewItems<'a> {
	owner: &'a TreeView,
}

impl<'a> TreeViewItems<'a> {
	pub(in crate::gui) const fn new(owner: &'a TreeView) -> Self {
		Self { owner }
	}

	/// Adds a new root item by sending a
	/// [`tvm::InsertItem`](crate::msg::tvm::InsertItem) message, and returns
	/// the newly added item.
	pub fn add_root(&self,
		text: &str,
		icon_index: Option<u32>,
	) -> TreeViewItem<'a>
	{
		let mut buf = WString::from_str(text);

		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;
		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}
		tvix.set_pszText(Some(&mut buf));

		let mut tvis = TVINSERTSTRUCT::default();
		tvis.set_hInsertAfter(TreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		let new_hitem = self.owner.hwnd()
			.SendMessage(tvm::InsertItem { item: &mut tvis })
			.unwrap();
		self.get(new_hitem)
	}

	/// Deletes all items by sending a
	/// [`tvm::DeleteItem`](crate::msg::tvm::DeleteItem) message.
	pub fn delete_all(&self) {
		self.owner.hwnd()
			.SendMessage(tvm::DeleteItem { hitem: &HTREEITEM::NULL })
			.unwrap();
	}

	/// Retrieves the total number of items by sending a
	/// [`tvm::GetCount`](crate::msg::tvm::GetCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(tvm::GetCount {})
	}

	/// Retrieves the number of visible items by sending a
	/// [`tvm::GetVisibleCount`](crate::msg::tvm::GetVisibleCount) message.
	#[must_use]
	pub fn count_visible(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(tvm::GetVisibleCount {})
	}

	/// Ends the editing of the item's text by sending a
	/// [`tvm::EndEditLabelNow`](crate::msg::tvm::EndEditLabelNow) message.
	pub fn end_edit_label_now(&self, save: bool) {
		self.owner.hwnd()
			.SendMessage(tvm::EndEditLabelNow { save })
			.unwrap();
	}

	/// Retrieves the item of the given handle.
	///
	/// **Note:** This method is cheap – even if `htreeitem` is invalid, an
	/// object will still be returned. However, operations upon this object will
	/// fail.
	#[must_use]
	pub const fn get(&self, hitem: HTREEITEM) -> TreeViewItem<'a> {
		TreeViewItem::new(self.owner, hitem)
	}

	/// Returns an iterator over the selected items.
	#[must_use]
	pub fn iter_selected(&self) -> impl Iterator<Item = TreeViewItem<'a>> + 'a {
		TreeViewItemIter::new(self.owner, None, co::TVGN::CARET)
	}

	/// Returns an iterator over the root items.
	#[must_use]
	pub fn iter_root(&self) -> impl Iterator<Item = TreeViewItem<'a>> + 'a {
		TreeViewChildItemIter::new(self.owner, None)
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct TreeViewItemIter<'a> {
	owner: &'a TreeView,
	current: Option<TreeViewItem<'a>>,
	relationship: co::TVGN,
}

impl<'a> Iterator for TreeViewItemIter<'a> {
	type Item = TreeViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = self.owner.hwnd()
			.SendMessage(tvm::GetNextItem {
				relationship: self.relationship,
				hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
			})
			.map(|hitem| self.owner.items().get(hitem));

		self.current.as_ref()
			.map(|tvi| TreeViewItem::new(
				self.owner,
				unsafe { tvi.htreeitem().raw_copy() },
			))
	}
}

impl<'a> TreeViewItemIter<'a> {
	pub(in crate::gui) const fn new(
		owner: &'a TreeView,
		current: Option<TreeViewItem<'a>>,
		relationship: co::TVGN,
	) -> Self
	{
		Self { owner, current, relationship }
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct TreeViewChildItemIter<'a> {
	owner: &'a TreeView,
	current: Option<TreeViewItem<'a>>,
	first_call: bool,
}

impl<'a> Iterator for TreeViewChildItemIter<'a> {
	type Item = TreeViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.first_call { // search for the first child
			self.current = self.owner.hwnd()
				.SendMessage(tvm::GetNextItem {
					relationship: co::TVGN::CHILD,
					hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
				})
				.map(|hitem| self.owner.items().get(hitem));

			self.first_call = false;

		} else { // search for next siblings
			self.current = self.owner.hwnd()
				.SendMessage(tvm::GetNextItem {
					relationship: co::TVGN::NEXT,
					hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
				})
				.map(|hitem| self.owner.items().get(hitem));
		}

		self.current.as_ref()
			.map(|tvi| TreeViewItem::new(
				self.owner,
				unsafe { tvi.htreeitem().raw_copy() },
			))
	}
}

impl<'a> TreeViewChildItemIter<'a> {
	pub(in crate::gui) fn new(
		owner: &'a TreeView,
		current: Option<TreeViewItem<'a>>,
	) -> Self
	{
		Self {
			owner,
			current,
			first_call: true,
		}
	}
}
