use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// A single item of a [`TreeView`](crate::gui::TreeView) control.
///
/// Each object keeps an unique [`HTREEITEM`](crate::HTREEITEM) handle.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TreeViewItem<'a> {
	owner: &'a TreeView,
	hitem: HTREEITEM,
}

impl<'a> TreeViewItem<'a> {
	pub(in crate::gui) const fn new(
		owner: &'a TreeView, hitem: HTREEITEM) -> Self
	{
		Self { owner, hitem }
	}

	pub(in crate::gui) fn raw_clone(&self) -> Self {
		Self {
			owner: self.owner,
			hitem: unsafe { self.hitem.raw_copy() },
		}
	}

	/// Adds a new child item by sending a
	/// [`tvm::InsertItem`](crate::msg::tvm::InsertItem) message, and returns
	/// the newly added item.
	pub fn add_child(&self,
		text: &str, icon_index: Option<u32>) -> Self
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
		tvis.hParent = unsafe { self.hitem.raw_copy() };
		tvis.set_hInsertAfter(TreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		let new_hitem = self.owner.hwnd()
			.SendMessage(tvm::InsertItem { item: &mut tvis })
			.unwrap();
		Self::new(self.owner, new_hitem)
	}

	/// Deletes the item by sending a
	/// [`tvm::DeleteItem`](crate::msg::tvm::DeleteItem) message.
	pub fn delete(&self) {
		self.owner.hwnd()
			.SendMessage(tvm::DeleteItem { hitem: &self.hitem })
			.unwrap();
	}

	/// Begins in-place editing of the item's text by sending a
	/// [`tvm::EditLabel`](crate::msg::tvm::EditLabel) message.
	///
	/// Returns a handle to the edit control.
	pub fn edit_label(&self) -> HWND {
		self.owner.hwnd()
			.SendMessage(tvm::EditLabel { hitem: &self.hitem })
			.unwrap()
	}

	/// Ensures that a tree-view item is visible, expanding the parent item or
	/// scrolling the tree-view control, if necessary, by sending a
	/// [`tvm::EnsureVisible`](crate::msg::tvm::EnsureVisible) message.
	///
	/// Returns whether a scroll occurred and no items were expanded.
	pub fn ensure_visible(&self) -> bool {
		self.owner.hwnd()
			.SendMessage(tvm::EnsureVisible { hitem: &self.hitem }) != 0
	}

	/// Expands or collapse the item by sending a
	/// [`tvm::Expand`](crate::msg::tvm::Expand) message.
	pub fn expand(&self, expand: bool) {
		self.owner.hwnd()
			.SendMessage(tvm::Expand {
				hitem: &self.hitem,
				action: if expand { co::TVE::EXPAND } else { co::TVE::COLLAPSE },
			})
			.unwrap();
	}

	/// Returns the underlying handle of the item.
	#[must_use]
	pub const fn htreeitem(&self) -> &HTREEITEM {
		&self.hitem
	}

	/// Tells if the item is expanded by sending a
	/// [`tvm::GetItemState`](crate::msg::tvm::GetItemState) message.
	#[must_use]
	pub fn is_expanded(&self) -> bool {
		self.owner.hwnd()
			.SendMessage(tvm::GetItemState {
				hitem: &self.hitem,
				mask: co::TVIS::EXPANDED,
			})
			.has(co::TVIS::EXPANDED)
	}

	/// Tells if the item is a root by sending a
	/// [`tvm::GetNextItem`](crate::msg::tvm::GetNextItem) message.
	#[must_use]
	pub fn is_root(&self) -> bool {
		self.parent().is_none()
	}

	/// Returns an iterator over the child items.
	#[must_use]
	pub fn iter_children(&self) -> impl Iterator<Item = TreeViewItem<'a>> + 'a {
		TreeViewChildItemIter::new(self.owner, Some(self.raw_clone()))
	}

	/// Returns an iterator over the next sibling items.
	#[must_use]
	pub fn iter_next_siblings(&self,
	) -> impl Iterator<Item = TreeViewItem<'a>> + 'a
	{
		TreeViewItemIter::new(self.owner, Some(self.raw_clone()), co::TVGN::NEXT)
	}

	/// Returns an iterator over the previous sibling items.
	#[must_use]
	pub fn iter_prev_siblings(&self,
	) -> impl Iterator<Item = TreeViewItem<'a>> + 'a
	{
		TreeViewItemIter::new(self.owner, Some(self.raw_clone()), co::TVGN::PREVIOUS)
	}

	/// Retrieves the parent of the item by sending a
	/// [`tvm::GetNextItem`](crate::msg::tvm::GetNextItem) message.
	#[must_use]
	pub fn parent(&self) -> Option<Self> {
		self.owner.hwnd()
			.SendMessage(tvm::GetNextItem {
				relationship: co::TVGN::PARENT,
				hitem: Some(&self.hitem),
			})
			.map(|hitem| TreeViewItem::new(self.owner, hitem))
	}

	/// Sets the text of the item by sending a
	/// [`tvm::SetItem`](crate::msg::tvm::SetItem) message.
	pub fn set_text(&self, text: &str) {
		let mut buf = WString::from_str(text);

		let mut tvi = TVITEMEX::default();
		tvi.hItem = unsafe { self.hitem.raw_copy() };
		tvi.mask = co::TVIF::TEXT;
		tvi.set_pszText(Some(&mut buf));

		self.owner.hwnd()
			.SendMessage(tvm::SetItem { tvitem: &tvi })
			.unwrap();
	}

	/// Retrieves the text of the item by sending a
	/// [`tvm::GetItem`](crate::msg::tvm::GetItem) message.
	#[must_use]
	pub fn text(&self) -> String {
		let mut tvi = TVITEMEX::default();
		tvi.hItem = unsafe { self.hitem.raw_copy() };
		tvi.mask = co::TVIF::TEXT;

		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		tvi.set_pszText(Some(&mut buf));

		self.owner.hwnd()
			.SendMessage(tvm::GetItem { tvitem: &mut tvi })
			.unwrap();
		buf.to_string()
	}
}
