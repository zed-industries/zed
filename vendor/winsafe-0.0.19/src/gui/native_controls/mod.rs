//! Native Win32 controls.

mod base_native_control;
mod button;
mod check_box;
mod combo_box_items;
mod combo_box;
mod date_time_picker;
mod edit;
mod label;
mod list_box_items;
mod list_box;
mod list_view_column;
mod list_view_columns;
mod list_view_item;
mod list_view_items;
mod list_view;
mod month_calendar;
mod progress_bar;
mod radio_button;
mod radio_group;
mod status_bar_part;
mod status_bar_parts;
mod status_bar;
mod tab_item;
mod tab_items;
mod tab;
mod trackbar;
mod tree_view_item;
mod tree_view_items;
mod tree_view;
mod up_down;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::base_native_control::*;
	pub(in crate::gui) use super::tree_view_items::{TreeViewChildItemIter, TreeViewItemIter};
}

pub use button::{Button, ButtonOpts};
pub use check_box::{CheckBox, CheckBoxOpts, CheckState};
pub use combo_box::{ComboBox, ComboBoxOpts};
pub use date_time_picker::{DateTimePicker, DateTimePickerOpts};
pub use edit::{Edit, EditOpts};
pub use label::{Label, LabelOpts};
pub use list_box::{ListBox, ListBoxOpts};
pub use list_view::{ListView, ListViewOpts};
pub use month_calendar::{MonthCalendar, MonthCalendarOpts};
pub use progress_bar::{ProgressBar, ProgressBarOpts};
pub use radio_button::{RadioButton, RadioButtonOpts};
pub use radio_group::RadioGroup;
pub use status_bar::{StatusBar, SbPart};
pub use tab::{Tab, TabOpts};
pub use trackbar::{Trackbar, TrackbarOpts};
pub use tree_view::{TreeView, TreeViewOpts};
pub use up_down::{UpDown, UpDownOpts};

pub mod spec {
	//! Structs which expose specialized methods of controls.

	pub use super::combo_box_items::ComboBoxItems;
	pub use super::list_box_items::ListBoxItems;
	pub use super::list_view_column::ListViewColumn;
	pub use super::list_view_columns::ListViewColumns;
	pub use super::list_view_item::ListViewItem;
	pub use super::list_view_items::ListViewItems;
	pub use super::status_bar_part::StatusBarPart;
	pub use super::status_bar_parts::StatusBarParts;
	pub use super::tab_item::TabItem;
	pub use super::tab_items::TabItems;
	pub use super::tree_view_item::TreeViewItem;
	pub use super::tree_view_items::TreeViewItems;
}
