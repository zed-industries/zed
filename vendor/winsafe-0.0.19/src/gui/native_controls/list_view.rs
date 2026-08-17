use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // atual fields of ListView
	base: BaseNativeControl,
	events: ListViewEvents,
	context_menu: Option<HMENU>,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [list view](https://learn.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
/// control. Not to be confused with the simpler [list box](crate::gui::ListBox)
/// control.
#[derive(Clone)]
pub struct ListView(Pin<Arc<Obj>>);

unsafe impl Send for ListView {}

impl GuiWindow for ListView {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for ListView {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for ListView {}

impl GuiNativeControl for ListView {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<ListViewEvents> for ListView {
	fn on(&self) -> &ListViewEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl ListView {
	/// Instantiates a new `ListView` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ListView` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: ListViewOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let opts = ListViewOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;
		let context_menu = opts.context_menu.as_ref().map(|h| unsafe { h.raw_copy() });

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: ListViewEvents::new(parent_base_ref, ctrl_id),
					context_menu,
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm(parent_base_ref.wm_create_or_initdialog(), move |_| {
			self2.create(OptsResz::Wnd(&opts))?;
			Ok(None) // not meaningful
		});

		new_self.default_message_handlers(parent_base_ref, ctrl_id);
		new_self
	}

	/// Instantiates a new `ListView` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// **Note:** The optional `context_menu` is shared: it must be destroyed
	/// manually after the control is destroyed. But note that menus loaded from
	/// resources don't need to be destroyed.
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ListView` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
		context_menu: Option<HMENU>,
	) -> Self
	{
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: ListViewEvents::new(parent_base_ref, ctrl_id),
					context_menu,
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(true) // not meaningful
		});

		new_self.default_message_handlers(parent_base_ref, ctrl_id);
		new_self
	}

	fn create(&self, opts_resz: OptsResz<&ListViewOpts>) -> SysResult<()> {
		let resize_behavior = match opts_resz {
			OptsResz::Wnd(opts) => opts.resize_behavior,
			OptsResz::Dlg(resize_behavior) => resize_behavior,
		};

		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window(
					"SysListView32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.list_view_style.into(),
				)?;

				if opts.list_view_ex_style != co::LVS_EX::NoValue {
					self.set_extended_style(true, opts.list_view_ex_style);
				}

				self.columns().add(&opts.columns);
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), resize_behavior)
	}

	fn default_message_handlers(&self, parent: &Base, ctrl_id: u16) {
		let self2 = self.clone();
		self.on_subclass().wm_get_dlg_code(move |p| {
			if !p.is_query && p.vkey_code == co::VK::RETURN {
				let mut nmlvkd = NMLVKEYDOWN::default();
				nmlvkd.hdr.hwndFrom = unsafe { self2.hwnd().raw_copy() };
				nmlvkd.hdr.set_idFrom(self2.ctrl_id());
				nmlvkd.hdr.code = co::LVN::KEYDOWN.into();
				nmlvkd.wVKey = co::VK::RETURN;

				self2.hwnd().GetAncestor(co::GA::PARENT).unwrap()
					.SendMessage(wm::Notify { nmhdr: &mut nmlvkd.hdr }); // send Enter key to parent
			}
			let dlgc_system = self2.hwnd().DefSubclassProc::<wm::GetDlgCode>(p.into());
			Ok(dlgc_system)
		});

		let self2 = self.clone();
		parent.privileged_on().wm_notify(ctrl_id, co::LVN::KEYDOWN, move |p| {
			let lvnk = unsafe { p.cast_nmhdr::<NMLVKEYDOWN>() };
			let has_ctrl = GetAsyncKeyState(co::VK::CONTROL);
			let has_shift = GetAsyncKeyState(co::VK::SHIFT);

			if has_ctrl && lvnk.wVKey == co::VK::CHAR_A { // Ctrl+A
				self2.items().select_all(true);
			} else if lvnk.wVKey == co::VK::APPS { // context menu key
				self2.show_context_menu(false, has_ctrl, has_shift);
			}
			Ok(None) // not meaningful
		});

		let self2 = self.clone();
		parent.privileged_on().wm_notify(ctrl_id, co::NM::RCLICK, move |p| {
			let nmia = unsafe { p.cast_nmhdr::<NMITEMACTIVATE>() };
			let has_ctrl = nmia.uKeyFlags.has(co::LVKF::CONTROL);
			let has_shift = nmia.uKeyFlags.has(co::LVKF::SHIFT);

			self2.show_context_menu(true, has_ctrl, has_shift);
			Ok(None) // not meaningful
		});
	}

	/// Exposes the column methods.
	#[must_use]
	pub const fn columns(&self) -> ListViewColumns {
		ListViewColumns::new(self)
	}

	/// Returns the context menu attached to this list view, if any.
	///
	/// The context menu is attached when the list view is created, either by
	/// calling [`ListView::new`](crate::gui::ListView::new) or
	/// [`ListView::new_dlg`](crate::gui::ListView::new_dlg).
	#[must_use]
	pub fn context_menu(&self) -> Option<&HMENU> {
		self.0.context_menu.as_ref()
	}

	/// Retrieves one of the associated image lists by sending an
	/// [`lvm::GetImageList`](crate::msg::lvm::GetImageList) message.
	#[must_use]
	pub fn image_list(&self, kind: co::LVSIL) -> Option<HIMAGELIST> {
		self.hwnd().SendMessage(lvm::GetImageList { kind })
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> ListViewItems {
		ListViewItems::new(self)
	}

	/// Retrieves the current view by sending an
	/// [`lvm::GetView`](crate::msg::lvm::GetView) message.
	#[must_use]
	pub fn current_view(&self) -> co::LV_VIEW {
		self.hwnd().SendMessage(lvm::GetView {})
	}

	/// Sets the current view by sending an
	/// [`lvm::SetView`](crate::msg::lvm::SetView) message.
	pub fn set_current_view(&self, view: co::LV_VIEW) {
		self.hwnd().SendMessage(lvm::SetView { view }).unwrap();
	}

	/// Sets or unsets the given extended list view styles by sending an
	/// [`lvm::SetExtendedListViewStyle`](crate::msg::lvm::SetExtendedListViewStyle)
	/// message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::LVS_EX) {
		self.hwnd().SendMessage(lvm::SetExtendedListViewStyle {
			mask: ex_style,
			style: if set { ex_style } else { co::LVS_EX::NoValue },
		});
	}

	/// Sets the one of the associated image lists by sending an
	/// [`lvm::SetImageList`](crate::msg::lvm::SetImageList) message.
	///
	/// Returns the previous image list, if any.
	pub fn set_image_list(&self,
		kind: co::LVSIL, himagelist: &HIMAGELIST) -> Option<HIMAGELIST>
	{
		self.hwnd().SendMessage(lvm::SetImageList { kind, himagelist })
	}

	/// Allows or disallows the redrawing of the control by sending a
	/// [`wm::SetRedraw`](crate::msg::wm::SetRedraw) message.
	pub fn set_redraw(&self, can_redraw: bool) {
		self.hwnd().SendMessage(wm::SetRedraw { can_redraw });
	}

	fn show_context_menu(&self,
		follow_cursor: bool, has_ctrl: bool, has_shift: bool)
	{
		let hmenu = match self.0.context_menu.as_ref() {
			Some(h) => h,
			None => return, // no menu, nothing to do
		};

		let menu_pos = if follow_cursor { // usually when fired by a right-click
			let mut menu_pos = GetCursorPos().unwrap(); // relative to screen
			self.hwnd().ScreenToClient(&mut menu_pos).unwrap(); // now relative to list view

			match self.items().hit_test(menu_pos) {
				Some(item_over) => {
					if !has_ctrl && !has_shift {
						item_over.select(true); // if not yet
						item_over.focus();
					}
				},
				None => self.items().select_all(false), // no item was right-clicked
			}

			self.hwnd().SetFocus(); // because a right-click won't set the focus by itself
			menu_pos

		} else { // usually fired by the context meny key
			let focused_opt = self.items().focused();

			if focused_opt.is_some() && focused_opt.unwrap().is_visible() {
				let focused = focused_opt.unwrap();
				let rc_item = focused.rect(co::LVIR::BOUNDS);
				POINT::new(rc_item.left + 16,
					rc_item.top + (rc_item.bottom - rc_item.top) / 2)

			} else { // no item is focused and visible
				POINT::new(6, 10) // arbitrary coordinates
			}
		};

		hmenu.TrackPopupMenuAtPoint(
			menu_pos, &self.hwnd().GetParent().unwrap(), self.hwnd())
			.unwrap();
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ListView`](crate::gui::ListView) programmatically with
/// [`ListView::new`](crate::gui::ListView::new).
pub struct ListViewOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(50, 50)`.
	pub size: (u32, u32),
	/// List view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS::REPORT | LVS::NOSORTHEADER | LVS::SHOWSELALWAYS | LVS::SHAREIMAGELISTS`.
	pub list_view_style: co::LVS,
	/// Extended list view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS_EX::NoValue`.
	pub list_view_ex_style: co::LVS_EX,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),

	/// Context popup menu.
	///
	/// This menu is shared: it must be destroyed manually after the control is
	/// destroyed. But note that menus loaded from resources don't need to be
	/// destroyed.
	///
	/// Defaults to `None`.
	pub context_menu: Option<HMENU>,
	/// Text and width of columns to be added right away. The columns only show
	/// in report mode.
	///
	/// Defaults to none.
	pub columns: Vec<(String, u32)>,
}

impl Default for ListViewOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (50, 50),
			list_view_style: co::LVS::REPORT | co::LVS::NOSORTHEADER | co::LVS::SHOWSELALWAYS | co::LVS::SHAREIMAGELISTS,
			list_view_ex_style: co::LVS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			context_menu: None,
			columns: Vec::<(String, u32)>::default(),
		}
	}
}

impl ListViewOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
