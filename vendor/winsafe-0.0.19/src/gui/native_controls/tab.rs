use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of Tab
	base: BaseNativeControl,
	events: TabEvents,
	children: Vec<(String, Box<dyn GuiTab>)>,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [tab](https://learn.microsoft.com/en-us/windows/win32/controls/tab-controls)
/// control.
#[derive(Clone)]
pub struct Tab(Pin<Arc<Obj>>);

unsafe impl Send for Tab {}

impl GuiWindow for Tab {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for Tab {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for Tab {}

impl GuiNativeControl for Tab {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<TabEvents> for Tab {
	fn on(&self) -> &TabEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl Tab {
	/// Instantiates a new `Tab` object, to be created on the parent window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `TreeView` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: TabOpts) -> Self {
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };
		let mut opts = TabOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;
		let children = opts.items.drain(..).collect::<Vec<_>>();

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: TabEvents::new(parent_base_ref, ctrl_id),
					children,
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

	/// Instantiates a new `Tab` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `TreeView` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
		items: Vec<(String, Box<dyn GuiTab>)>,
	) -> Self
	{
		let parent_base_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: TabEvents::new(parent_base_ref, ctrl_id),
					children: items,
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

	fn create(&self, opts_resz: OptsResz<&TabOpts>) -> SysResult<()> {
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

				self.0.base.create_window( // may panic
					"SysTabControl32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.tab_style.into(),
				)?;

				self.hwnd().SendMessage(wm::SetFont {
					hfont: unsafe { ui_font().raw_copy() },
					redraw: true,
				});

				if opts.tab_ex_style != co::TCS_EX::NoValue {
					self.set_extended_style(true, opts.tab_ex_style);
				}
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.children.iter()
			.for_each(|(text, _)| unsafe { self.items().add(text); }); // add the tabs
		self.display_tab(0)?; // 1st tab selected by default

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), resize_behavior)
	}

	fn default_message_handlers(&self, parent: &Base, ctrl_id: u16) {
		let self2 = self.clone();
		parent.privileged_on().wm_notify(ctrl_id, co::TCN::SELCHANGE, move |_| {
			if let Some(sel_item) = self2.items().selected() {
				self2.display_tab(sel_item.index())?;
			}
			Ok(None) // not meaningful
		})
	}

	fn display_tab(&self, index: u32) -> SysResult<()> {
		self.0.children.iter()
			.enumerate()
			.filter(|(i, _)| *i != index as usize)
			.for_each(|(_, (_, item))| {
				item.as_ctrl().hwnd().ShowWindow(co::SW::HIDE); // hide all others
			});

		if let Some((_, item)) = self.0.children.get(index as usize) {
			let mut rc = self.hwnd().GetWindowRect()?;
			self.hwnd().GetParent()?.ScreenToClientRc(&mut rc)?;
			self.hwnd().SendMessage(tcm::AdjustRect {
				display_rect: false,
				rect: &mut rc,
			});
			item.as_ctrl().hwnd().SetWindowPos(
				HwndPlace::None,
				POINT::new(rc.left, rc.top),
				SIZE::new(rc.right - rc.left, rc.bottom - rc.top),
				co::SWP::NOZORDER | co::SWP::SHOWWINDOW,
			)?;
		}

		Ok(())
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> TabItems {
		TabItems::new(self)
	}

	/// Sets or unsets the given extended list view styles by sending a
	/// [`tcm::SetExtendedStyle`](crate::msg::tcm::SetExtendedStyle) message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::TCS_EX) {
		self.hwnd()
			.SendMessage(tcm::SetExtendedStyle {
				mask: ex_style,
				style: if set { ex_style } else { co::TCS_EX::NoValue },
			});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Tab`](crate::gui::Tab) programmatically with
/// [`Tab::new`](crate::gui::Tab::new).
pub struct TabOpts {
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
	/// Defaults to `(80, 50)`.
	pub size: (u32, u32),
	/// Tab styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TCS::NoValue`.
	pub tab_style: co::TCS,
	/// Extended tab styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TCS_EX::NoValue`.
	pub tab_ex_style: co::TCS_EX,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::NoValue`.
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

	/// Items to be added as soon as the control is created. The tuple contains
	/// the title of the tab and the window to be rendered inside of it.
	///
	/// Note that, in o order to make the focus rotation work properly, the
	/// child windows must be created with the
	/// [`co::WS_EX::CONTROLPARENT`](crate::co::WS_EX::CONTROLPARENT) extended
	/// style.
	///
	/// Defaults to none.
	pub items: Vec<(String, Box<dyn GuiTab>)>,
}

impl Default for TabOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (80, 50),
			tab_style: co::TCS::NoValue,
			tab_ex_style: co::TCS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::NoValue,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			items: Vec::default(),
		}
	}
}

impl TabOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
