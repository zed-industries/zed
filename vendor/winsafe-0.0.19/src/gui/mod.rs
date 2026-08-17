//! High-level GUI abstractions for user windows and native controls. They can
//! be created programmatically or by loading resources from a `.res` file.
//! These files can be created with a WYSIWYG
//! [resource editor](https://en.wikipedia.org/wiki/Resource_(Windows)#Resource_software).
//!
//! You'll probably want to start your GUI application using the
//! [`WindowMain`].

#![cfg(feature = "gui")]

mod base;
mod dlg_base;
mod dlg_control;
mod dlg_main;
mod dlg_modal;
mod dlg_modeless;
mod gui_traits;
mod layout_arranger;
mod msg_error;
mod native_controls;
mod privs_gui;
mod raw_base;
mod raw_control;
mod raw_main;
mod raw_modal;
mod raw_modeless;
mod window_control;
mod window_main;
mod window_message_only;
mod window_modal;
mod window_modeless;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::base::Base;
	pub(in crate::gui) use super::dlg_base::DlgBase;
	pub(in crate::gui) use super::dlg_control::DlgControl;
	pub(in crate::gui) use super::dlg_main::DlgMain;
	pub(in crate::gui) use super::dlg_modal::DlgModal;
	pub(in crate::gui) use super::dlg_modeless::DlgModeless;
	pub(in crate::gui) use super::events::privs::*;
	pub(in crate::gui) use super::layout_arranger::LayoutArranger;
	pub(in crate::gui) use super::native_controls::privs::*;
	pub(in crate::gui) use super::privs_gui::*;
	pub(in crate::gui) use super::raw_base::RawBase;
	pub(in crate::gui) use super::raw_control::RawControl;
	pub(in crate::gui) use super::raw_main::RawMain;
	pub(in crate::gui) use super::raw_modal::RawModal;
	pub(in crate::gui) use super::raw_modeless::RawModeless;
}

pub mod events;

pub use layout_arranger::{Horz, Vert};
pub use msg_error::MsgError;
pub use native_controls::*;
pub use raw_base::{Brush, Cursor, Icon};
pub use raw_control::WindowControlOpts;
pub use raw_main::WindowMainOpts;
pub use raw_modal::WindowModalOpts;
pub use raw_modeless::WindowModelessOpts;
pub use window_control::WindowControl;
pub use window_main::WindowMain;
pub use window_message_only::WindowMessageOnly;
pub use window_modal::WindowModal;
pub use window_modeless::WindowModeless;

pub(crate) mod traits {
	pub use super::events::traits::*;
	pub use super::gui_traits::*;
}
