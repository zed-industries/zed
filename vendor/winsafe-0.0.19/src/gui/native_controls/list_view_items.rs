use crate::co;
use crate::decl::*;
use crate::gui::{*, spec::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes item methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewItems<'a> {
	owner: &'a ListView,
}

impl<'a> ListViewItems<'a> {
	pub(in crate::gui) const fn new(owner: &'a ListView) -> Self {
		Self { owner }
	}

	/// Appends a new item by sending an
	/// [`lvm::InsertItem`](crate::msg::lvm::InsertItem) message, and returns
	/// the newly added item.
	///
	/// The texts are relative to each column.
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
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
	///
	/// let new_item = my_list.items().add(
	///     &[
	///         "First column text",
	///         "Second column text",
	///     ],
	///     None, // no icon; requires a previous set_image_list()
	/// );
	/// ```
	pub fn add(&self,
		texts: &[impl AsRef<str>],
		icon_index: Option<u32>,
	) -> ListViewItem<'a>
	{
		if texts.is_empty() {
			panic!("No texts passed when adding a ListView item.");
		} else if texts.len() > self.owner.columns().count() as usize {
			panic!("Cannot set {} text(s) to {} column(s).",
				texts.len(), self.owner.columns().count());
		}

		let mut lvi = LVITEM::default();
		lvi.mask = co::LVIF::TEXT;
		lvi.iItem = 0x0fff_ffff; // insert as the last item

		if let Some(icon_index) = icon_index { // will it have an icon?
			lvi.mask |= co::LVIF::IMAGE;
			lvi.iImage = icon_index as _;
		}

		let mut wtext = WString::from_str(texts[0].as_ref()); // text of 1st column
		lvi.set_pszText(Some(&mut wtext));

		let new_item = self.get( // insert new item; retrieve newly added
			self.owner.hwnd()
				.SendMessage(lvm::InsertItem { item: &lvi })
				.unwrap(),
		);

		texts.iter()
			.enumerate()
			.skip(1) // iterate over subsequent columns
			.for_each(|(idx, text)|
				new_item.set_text(idx as _, text.as_ref()), // set the text ordinarily
			);

		new_item
	}

	/// Retrieves the total number of items by sending an
	/// [`lvm::GetItemCount`](crate::msg::lvm::GetItemCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(lvm::GetItemCount {})
	}

	/// Deletes all items by sending an
	/// [`lvm::DeleteAllItems`](crate::msg::lvm::DeleteAllItems) message.
	pub fn delete_all(&self) {
		self.owner.hwnd()
			.SendMessage(lvm::DeleteAllItems {})
			.unwrap();
	}

	/// Deletes all selected items by sending
	/// [`lvm::DeleteItem`](crate::msg::lvm::DeleteItem) messages.
	pub fn delete_selected(&self) {
		loop {
			let next_idx = self.owner.hwnd()
				.SendMessage(lvm::GetNextItem {
					initial_index: None,
					relationship: co::LVNI::SELECTED,
				});
			match next_idx {
				Some(next_idx) => self.get(next_idx).delete(),
				None => break,
			}
		}
	}

	/// Searches for an item with the given text, case-insensitive, by sending
	/// an [`lvm::FindItem`](crate::msg::lvm::FindItem) message.
	#[must_use]
	pub fn find(&self, text: &str) -> Option<ListViewItem<'a>> {
		let mut buf = WString::from_str(text);

		let mut lvfi = LVFINDINFO::default();
		lvfi.flags = co::LVFI::STRING;
		lvfi.set_psz(Some(&mut buf));

		self.owner.hwnd()
			.SendMessage(lvm::FindItem {
				start_index: None,
				lvfindinfo: &mut lvfi,
			})
			.map(|idx| self.get(idx))
	}

	/// Retrieves the focused item by sending an
	/// [`lvm::GetNextItem`](crate::msg::lvm::GetNextItem) message.
	#[must_use]
	pub fn focused(&self) -> Option<ListViewItem<'a>> {
		self.owner.hwnd()
			.SendMessage(lvm::GetNextItem {
				initial_index: None,
				relationship: co::LVNI::FOCUSED,
			})
			.map(|idx| self.get(idx))
	}

	/// Retrieves the item at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing items, an object will still be returned. However, operations
	/// upon this object will fail.
	#[must_use]
	pub const fn get(&self, index: u32) -> ListViewItem<'a> {
		ListViewItem::new(self.owner, index)
	}

	/// Retrieves the item at the specified position by sending an
	/// [`lvm::HitTest`](crate::msg::lvm::HitTest) message
	///
	/// `coords` must be relative to the list view.
	#[must_use]
	pub fn hit_test(&self, coords: POINT) -> Option<ListViewItem<'a>> {
		let mut lvhti = LVHITTESTINFO::default();
		lvhti.pt = coords;

		self.owner.hwnd()
			.SendMessage(lvm::HitTest { info: &mut lvhti })
			.map(|index| self.get(index))
	}

	/// Returns an iterator over all items.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
	///
	/// for item in my_list.items().iter() {
	///     println!("Item {}, text of the first column: {}",
	///         item.index(), item.text(0));
	/// }
	/// ```
	#[must_use]
	pub fn iter(&self) -> impl Iterator<Item = ListViewItem> {
		ListViewItemIter::new(self.owner, co::LVNI::ALL)
	}

	/// Returns an iterator over the selected items.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
	///
	/// for item in my_list.items().iter_selected() {
	///     println!("Selected item {}, text of the first column: {}",
	///         item.index(), item.text(0));
	/// }
	/// ```
	#[must_use]
	pub fn iter_selected(&self) -> impl Iterator<Item = ListViewItem<'a>> + 'a {
		ListViewItemIter::new(self.owner, co::LVNI::ALL | co::LVNI::SELECTED)
	}

	/// Retrieves the item of the unique ID by sending an
	/// [`lvm::MapIdToIndex`](crate::msg::lvm::MapIdToIndex) message.
	///
	/// If the item of the given unique ID doesn't exist anymore, returns
	/// `None`.
	///
	/// # Panics
	///
	/// Panics if the given ID doesn't exist among the items.
	#[must_use]
	pub fn map_id_to_index(&self, item_id: u32) -> ListViewItem<'a> {
		self.get(
			self.owner.hwnd()
				.SendMessage(lvm::MapIdToIndex { id: item_id })
				.unwrap(),
		)
	}

	/// Sets or remove the selection for all items by sending an
	/// [`lvm::SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn select_all(&self, set: bool) {
		let styles = unsafe {
			co::LVS::from_raw(self.owner.hwnd().GetWindowLongPtr(co::GWLP::STYLE) as _)
		};
		if styles.has(co::LVS::SINGLESEL) {
			return; // LVM_SETITEMSTATE fails for all items in single-sel list views
		}

		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		self.owner.hwnd()
			.SendMessage(lvm::SetItemState {
				index: None,
				lvitem: &lvi,
			})
			.unwrap();
	}

	/// Retrieves the number of selected items by sending an
	/// [`lvm::GetSelectedCount`](crate::msg::lvm::GetSelectedCount) message.
	#[must_use]
	pub fn selected_count(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(lvm::GetSelectedCount {})
	}

	/// Sets the number of items in a virtual list view – that is, a list view
	/// created with [`LVS::OWNERDATA`](crate::co::LVS::OWNERDATA) style – by
	/// sending an [`lvm::SetItemCount`](crate::msg::lvm::SetItemCount) message.
	pub fn set_count(&self, count: u32, behavior: Option<co::LVSICF>) {
		self.owner.hwnd()
			.SendMessage(lvm::SetItemCount { count, behavior })
			.unwrap();
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct ListViewItemIter<'a> {
	owner: &'a ListView,
	current: Option<ListViewItem<'a>>,
	relationship: co::LVNI,
}

impl<'a> Iterator for ListViewItemIter<'a> {
	type Item = ListViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = self.owner.hwnd()
			.SendMessage(lvm::GetNextItem {
				initial_index: self.current.map(|item| item.index()),
				relationship: self.relationship,
			})
			.map(|index| self.owner.items().get(index));

		self.current
	}
}

impl<'a> ListViewItemIter<'a> {
	pub(in crate::gui) const fn new(
		owner: &'a ListView,
		relationship: co::LVNI,
	) -> Self
	{
		Self {
			owner,
			current: None,
			relationship,
		}
	}
}
