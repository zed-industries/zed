use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// A single column of a [`ListView`](crate::gui::ListView) control.
///
/// **Note:** Each object keeps the zero-based index of a column. If new columns
/// are added/removed from the list view control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[derive(Clone, Copy)]
pub struct ListViewColumn<'a> {
	owner: &'a ListView,
	index: u32,
}

impl<'a> ListViewColumn<'a> {
	pub(in crate::gui) const fn new(owner: &'a ListView, index: u32) -> Self {
		Self { owner, index }
	}

	/// Retrieves information about the column by sending an
	/// [`lvm::GetColumn`](crate::msg::lvm::GetColumn) message.
	pub fn info(&self, lvc: &mut LVCOLUMN) {
		self.owner.hwnd()
			.SendMessage(lvm::GetColumn {
				index: self.index,
				lvcolumn: lvc,
			})
			.unwrap();
	}

	/// Sets information of the column by sending an
	/// [`lvm::SetColumn`](crate::msg::lvm::SetColumn) message.
	pub fn set_info(&self, lvc: &LVCOLUMN) {
		self.owner.hwnd()
			.SendMessage(lvm::SetColumn {
				index: self.index,
				lvcolumn: lvc,
			})
			.unwrap();
	}

	/// Sets the title of the column by calling
	/// [`set_info`](crate::gui::spec::ListViewColumn::set_info).
	pub fn set_title(&self, text: &str) {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = self.index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::from_str(text);
		lvc.set_pszText(Some(&mut buf));

		self.set_info(&lvc);
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message.
	///
	/// Width will be adjusted to match current system DPI.
	pub fn set_width(&self, width: u32) {
		let mut col_cx = SIZE::new(width as _, 0);
		multiply_dpi(None, Some(&mut col_cx)).unwrap();

		self.owner.hwnd()
			.SendMessage(lvm::SetColumnWidth {
				index: self.index,
				width: col_cx.cx as _,
			})
			.unwrap();
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message. The
	/// width will be calculated to fill the remaining space.
	pub fn set_width_to_fill(&self) {
		let num_cols = self.owner.columns().count();
		if num_cols > 0 {
			let mut cx_used = 0;

			for i in 0..num_cols {
				if i != self.index {
					cx_used += self.owner.columns().get(i).width(); // retrieve cx of each column, but us
				}
			}

			let rc = self.owner.hwnd().GetClientRect().unwrap(); // list view client area
			self.owner.hwnd()
				.SendMessage(lvm::SetColumnWidth {
					index: self.index,
					width: rc.right as u32 - cx_used,
				})
				.unwrap();
		}
	}

	/// Retrieves the title of the column by calling
	/// [`info`](crate::gui::spec::ListViewColumn::info).
	#[must_use]
	pub fn title(&self) -> String {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = self.index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::new_alloc_buf(128); // arbitrary
		lvc.set_pszText(Some(&mut buf));

		self.info(&mut lvc);
		buf.to_string()
	}

	/// Retrieves the width of the column by sending an
	/// [`lvm::GetColumnWidth`](crate::msg::lvm::GetColumnWidth) message.
	#[must_use]
	pub fn width(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(lvm::GetColumnWidth { index: self.index })
			.unwrap()
	}
}
