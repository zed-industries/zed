use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes column methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewColumns<'a> {
	owner: &'a ListView,
}

impl<'a> ListViewColumns<'a> {
	pub(in crate::gui) const fn new(owner: &'a ListView) -> Self {
		Self { owner }
	}

	/// Adds many columns at once by sending an
	/// [`lvm::InsertColumn`](crate::msg::lvm::InsertColumn) message.
	///
	/// Widths will be adjusted to match current system DPI.
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
	/// my_list.columns().add(&[
	///     ("Name", 300),
	///     ("Address", 500),
	/// ]);
	/// ```
	pub fn add(&self, texts_and_widths: &[(impl AsRef<str>, u32)]) {
		for (text, width) in texts_and_widths.iter() {
			let mut col_cx = SIZE::new(*width as _, 0);
			multiply_dpi(None, Some(&mut col_cx)).unwrap();

			let mut lvc = LVCOLUMN::default();
			lvc.mask = co::LVCF::TEXT | co::LVCF::WIDTH;
			lvc.cx = col_cx.cx;

			let mut wtext = WString::from_str(text.as_ref());
			lvc.set_pszText(Some(&mut wtext));

			self.owner.hwnd()
				.SendMessage(lvm::InsertColumn {
					index: 0xffff, // insert as the last columns
					column: &lvc,
				})
				.unwrap();
		}
	}

	/// Retrieves the number of columns by sending an
	/// [`hdm::GetItemCount`](crate::msg::hdm::GetItemCount) message to the
	/// handle returned by [`lvm::GetHeader`](crate::msg::lvm::GetHeader).
	#[must_use]
	pub fn count(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(lvm::GetHeader {})
			.unwrap()
			.SendMessage(hdm::GetItemCount {})
			.unwrap()
	}

	/// Retrieves the column at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing columns, an object will still be returned. However, operations
	/// upon this object will fail.
	#[must_use]
	pub const fn get(&self, index: u32) -> ListViewColumn<'a> {
		ListViewColumn::new(self.owner, index)
	}
}
