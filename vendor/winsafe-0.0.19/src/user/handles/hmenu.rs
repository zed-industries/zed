#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::{iterators::*, ffi};

impl_handle! { HMENU;
	/// Handle to a
	/// [menu](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
}

impl user_Hmenu for HMENU {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HMENU`](crate::HMENU).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hmenu: Handle {
	/// A more convenient
	/// [`HMENU::AppendMenu`](crate::prelude::user_Hmenu::AppendMenu).
	///
	/// # Examples
	///
	/// Adding multiple entries at once, with their command IDs:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 2001;
	///     ID_FILE_SAVE
	///     ID_FILE_EXIT
	/// }
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// hmenu.append_item(&[
	///     w::MenuItem::Entry(ID_FILE_OPEN, "&Open"),
	///     w::MenuItem::Entry(ID_FILE_OPEN, "&Save"),
	///     w::MenuItem::Separator,
	///     w::MenuItem::Entry(ID_FILE_EXIT, "E&xit"),
	/// ])?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn append_item(&self, items: &[MenuItem]) -> SysResult<()> {
		items.iter().map(|item| {
			match item {
				MenuItem::Entry(cmd_id, text) => self.AppendMenu(
					co::MF::STRING,
					IdMenu::Id(*cmd_id),
					BmpPtrStr::from_str(*text),
				),
				MenuItem::Separator => self.AppendMenu(
					co::MF::SEPARATOR,
					IdMenu::None,
					BmpPtrStr::None,
				),
				MenuItem::Submenu(hmenu, text) => self.AppendMenu(
					co::MF::POPUP,
					IdMenu::Menu(hmenu),
					BmpPtrStr::from_str(*text),
				),
			}
		}).collect::<Result<Vec<_>, _>>()?;

		Ok(())
	}

	/// Simpler version of
	/// [`HMENU::GetMenuItemInfo`](crate::prelude::user_Hmenu::GetMenuItemInfo),
	/// which returns a [`MenuItemInfo`](crate::MenuItemInfo) instead of the
	/// tricky [`MENUITEMINFO`](crate::MENUITEMINFO).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// let item_info = hmenu.item_info(w::IdPos::Id(0))?;
	/// match item_info {
	///     w::MenuItemInfo::Entry(id, txt) => println!("item {} {}", id, txt),
	///     w::MenuItemInfo::Separator => println!("separator"),
	///     w::MenuItemInfo::Submenu(hsub, txt) => println!("submenu {} {}", hsub, txt),
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn item_info(&self, id_or_pos: IdPos) -> SysResult<MenuItemInfo> {
		let mut mii = MENUITEMINFO::default();
		mii.fMask = co::MIIM::FTYPE | co::MIIM::ID | co::MIIM::STATE | co::MIIM::SUBMENU;
		self.GetMenuItemInfo(id_or_pos, &mut mii)?;

		let nfo = if mii.fType == co::MFT::SEPARATOR {
			MenuItemInfo::Separator
		} else {
			let text = self.GetMenuString(id_or_pos)?;
			if mii.hSubMenu != HMENU::NULL {
				MenuItemInfo::Submenu(mii.hSubMenu, text)
			} else {
				MenuItemInfo::Entry(mii.wID as _, text)
			}
		};

		Ok(nfo)
	}

	/// Returns an iterator over all menu items, including submenus and
	/// separators.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// for item_info in hmenu.iter_items() {
	///     let item_info = item_info?;
	///     match item_info {
	///         w::MenuItemInfo::Entry(id, txt) => println!("item {} {}", id, txt),
	///         w::MenuItemInfo::Separator => println!("separator"),
	///         w::MenuItemInfo::Submenu(hsub, txt) => println!("submenu {} {}", hsub, txt),
	///     }
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_items(&self,
	) -> Box<dyn Iterator<Item = SysResult<MenuItemInfo>> + '_>
	{
		Box::new(HmenuIteritems::new(self))
	}

	/// [`AppendMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
	/// function.
	///
	/// This method is rather tricky, consider using
	/// [`HMENU::append_item`](crate::prelude::user_Hmenu::append_item).
	fn AppendMenu(&self,
		flags: co::MF,
		new_item: IdMenu,
		content: BmpPtrStr,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::AppendMenuW(
					self.ptr(),
					flags.raw(),
					new_item.as_usize(),
					content.as_ptr(),
				)
			},
		)
	}

	/// [`CheckMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuitem)
	/// function.
	fn CheckMenuItem(&self,
		id_or_pos: IdPos,
		check: bool,
	) -> SysResult<co::MF>
	{
		match unsafe {
			ffi::CheckMenuItem(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag() | if check {
					co::MF::CHECKED
				} else {
					co::MF::UNCHECKED
				}).raw(),
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(unsafe { co::MF::from_raw(ret as _) }),
		}
	}

	/// [`CheckMenuRadioItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuradioitem)
	/// function.
	///
	/// # Panics
	///
	/// Panics if `first`, `last` and `check` don't use the same enum field.
	fn CheckMenuRadioItem(&self,
		first: IdPos,
		last: IdPos,
		check: IdPos,
	) -> SysResult<()>
	{
		if !(first.is_by_pos() == last.is_by_pos()
			&& last.is_by_pos() == check.is_by_pos())
		{
			panic!("Different enum fields.");
		}

		bool_to_sysresult(
			unsafe {
				ffi::CheckMenuRadioItem(
					self.ptr(),
					first.id_or_pos_u32(),
					last.id_or_pos_u32(),
					check.id_or_pos_u32(),
					check.mf_flag().raw(),
				)
			},
		)
	}

	/// [`CreateMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// function.
	///
	/// **Note:** If not attached to a window, must be paired with an
	/// [`HMENU::DestroyMenu`](crate::prelude::user_Hmenu::DestroyMenu) call.
	#[must_use]
	fn CreateMenu() -> SysResult<HMENU> {
		ptr_to_sysresult_handle(unsafe { ffi::CreateMenu() })
	}

	/// [`CreatePopupMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// function.
	///
	/// **Note:** When a menu is attached to a window, it's automatically
	/// destroyed along with the window. However, if the menu is not attached to
	/// any window, you must call
	/// [`HMENU::DestroyMenu`](crate::prelude::user_Hmenu::DestroyMenu).
	#[must_use]
	fn CreatePopupMenu() -> SysResult<HMENU> {
		ptr_to_sysresult_handle(unsafe { ffi::CreatePopupMenu() })
	}

	/// [`DeleteMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deletemenu)
	/// function.
	fn DeleteMenu(&self, id_or_pos: IdPos) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::DeleteMenu(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().raw(),
				)
			},
		)
	}

	/// [`DestroyMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// function.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn DestroyMenu(&mut self) -> SysResult<()> {
		let ret = bool_to_sysresult(unsafe { ffi::DestroyMenu(self.ptr()) });
		*self = Self::INVALID;
		ret
	}

	/// [`EnableMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablemenuitem)
	/// function.
	///
	/// # Examples
	///
	/// Disabling a menu item:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 101;
	/// }
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// hmenu.EnableMenuItem(
	///     w::IdPos::Id(ID_FILE_OPEN),
	///     false,
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// Disabling multiple menu items at once:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 201;
	///     ID_FILE_SAVE
	/// }
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// [
	///     (ID_FILE_OPEN, "Open\tCtrl+O"),
	///     (ID_FILE_SAVE, "&Save"),
	/// ].iter()
	///     .map(|(id, txt)| hmenu.EnableMenuItem(
	///         w::IdPos::Id(*id),
	///         false,
	///     ))
	///     .collect::<w::SysResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn EnableMenuItem(&self,
		id_or_pos: IdPos,
		enable: bool,
	) -> SysResult<co::MF>
	{
		match unsafe {
			ffi::EnableMenuItem(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag()
					| if enable { co::MF::ENABLED } else { co::MF::DISABLED }).raw(),
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(unsafe { co::MF::from_raw(ret as _) }),
		}
	}

	/// [`GetMenuDefaultItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenudefaultitem)
	/// function.
	#[must_use]
	fn GetMenuDefaultItem(&self,
		by_pos: bool,
		flags: co::GMDI,
	) -> SysResult<IdPos>
	{
		match unsafe {
			ffi::GetMenuDefaultItem(self.ptr(), by_pos as _, flags.raw()) as i32
		} {
			-1 => Err(GetLastError()),
			n => Ok(if by_pos { IdPos::Pos(n as _) } else { IdPos::Id(n as _) }),
		}
	}

	/// [`GetMenuInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuinfo)
	/// function.
	fn GetMenuInfo(&self, mi: &mut MENUINFO) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::GetMenuInfo(self.ptr(), mi as *mut _ as _) },
		)
	}

	/// [`GetMenuItemCount`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemcount)
	/// function.
	#[must_use]
	fn GetMenuItemCount(&self) -> SysResult<u32> {
		match unsafe { ffi::GetMenuItemCount(self.ptr()) } {
			-1 => Err(GetLastError()),
			count => Ok(count as _),
		}
	}

	/// [`GetMenuItemID`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemid)
	/// function.
	///
	/// If `item_index` corresponds to a submenu, returns `None`.
	#[must_use]
	fn GetMenuItemID(&self, item_index: i32) -> Option<u16> {
		match unsafe { ffi::GetMenuItemID(self.ptr(), item_index) } {
			-1 => None,
			id => Some(id as _),
		}
	}

	/// [`GetMenuItemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuiteminfow)
	/// function.
	///
	/// This method is rather tricky, consider using
	/// [`HMENU::item_info`](crate::prelude::user_Hmenu::item_info).
	fn GetMenuItemInfo(&self,
		id_or_pos: IdPos,
		mii: &mut MENUITEMINFO,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::GetMenuItemInfoW(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *mut _ as _,
				)
			},
		)
	}

	/// [`GetMenuState`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustate)
	/// function.
	#[must_use]
	fn GetMenuState(&self, id_or_pos: IdPos) -> SysResult<co::MF> {
		match unsafe {
			ffi::GetMenuState(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.is_by_pos() as _,
			) as i32
		} {
			-1 => Err(GetLastError()),
			mf => Ok(unsafe { co::MF::from_raw(mf as _) }),
		}
	}

	/// [`GetMenuString`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustringw)
	/// function.
	#[must_use]
	fn GetMenuString(&self, id_or_pos: IdPos) -> SysResult<String> {
		let mut buf_sz = SSO_LEN; // start with no string heap allocation
		loop {
			let mut buf = WString::new_alloc_buf(buf_sz);

			let returned_chars = match unsafe { // char count without terminating null
				ffi::GetMenuStringW(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
					id_or_pos.mf_flag().raw(),
				)
			} {
				0 => return Err(GetLastError()),
				n => n + 1, // plus terminating null count
			};

			if (returned_chars as usize) < buf_sz { // to break, must have at least 1 char gap
				return Ok(buf.to_string());
			}

			buf_sz *= 2; // double the buffer size to try again
		}
	}

	/// [`GetSubMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsubmenu)
	/// function.
	#[must_use]
	fn GetSubMenu(&self, pos: u32) -> Option<HMENU> {
		ptr_to_option_handle(unsafe { ffi::GetSubMenu(self.ptr(), pos as _) })
	}

	/// [`InsertMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuitemw)
	/// function.
	fn InsertMenuItem(&self,
		id_or_pos: IdPos,
		mii: &MENUITEMINFO,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::InsertMenuItemW(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *const _ as _,
				)
			},
		)
	}

	/// [`IsMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ismenu)
	/// function.
	#[must_use]
	fn IsMenu(&self) -> bool {
		unsafe { ffi::IsMenu(self.ptr()) != 0 }
	}

	/// [`RemoveMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removemenu)
	/// function.
	fn RemoveMenu(&self, id_or_pos: IdPos) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::RemoveMenu(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().raw(),
				)
			},
		)
	}

	/// [`SetMenuDefaultItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenudefaultitem)
	/// function.
	fn SetMenuDefaultItem(&self, id_or_pos: IdPos) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::SetMenuDefaultItem(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
				)
			},
		)
	}

	/// [`SetMenuInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// function.
	fn SetMenuInfo(&self, mi: &MENUINFO) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::SetMenuInfo(self.ptr(), mi as *const _ as _) },
		)
	}

	/// [`SetMenuItemBitmaps`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuitembitmaps)
	/// function.
	fn SetMenuItemBitmaps(&self,
		id_or_pos: IdPos,
		hbmp_unchecked: Option<&HBITMAP>,
		hbmp_checked: Option<&HBITMAP>,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetMenuItemBitmaps(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().raw(),
					hbmp_unchecked.map_or(std::ptr::null_mut(), |h| h.ptr()),
					hbmp_checked.map_or(std::ptr::null_mut(), |h| h.ptr()),
				)
			},
		)
	}

	/// [`SetMenuItemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuiteminfow)
	/// function.
	fn SetMenuItemInfo(&self,
		id_or_pos: IdPos,
		mii: &MENUITEMINFO,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetMenuItemInfoW(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *const _ as _,
				)
			},
		)
	}

	/// [`TrackPopupMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// function.
	///
	/// **Note:** If you just want to display a popup menu, consider the simpler
	/// [`HMENU::TrackPopupMenuAtPoint`](crate::prelude::user_Hmenu::TrackPopupMenuAtPoint).
	fn TrackPopupMenu(&self,
		flags: co::TPM,
		location: POINT,
		hwnd: &HWND,
	) -> SysResult<Option<i32>>
	{
		let ret = unsafe {
			ffi::TrackPopupMenu(
				self.ptr(),
				flags.raw(),
				location.x, location.y,
				0,
				hwnd.ptr(),
				std::ptr::null(),
			)
		};

		if flags.has(co::TPM::RETURNCMD) {
			match ret {
				0 => match GetLastError() {
					co::ERROR::SUCCESS => Ok(None), // success, user cancelled the menu
					error => Err(error),
				},
				id => Ok(Some(id)), // success, menu item identifier
			}
		} else {
			match ret {
				0 => Err(GetLastError()),
				_ => Ok(None),
			}
		}
	}

	/// Shows the popup menu anchored at the given coordinates using
	/// [`TrackPopupMenu`](crate::prelude::user_Hmenu::TrackPopupMenu), and
	/// performs other needed operations.
	///
	/// This method will block until the menu disappears.
	fn TrackPopupMenuAtPoint(&self,
		pos: POINT,
		hwnd_parent: &HWND,
		hwnd_coords_relative_to: &HWND,
	) -> SysResult<()>
	{
		let mut pos = pos;
		hwnd_coords_relative_to.ClientToScreen(&mut pos)?; // now relative to screen
		hwnd_parent.SetForegroundWindow();
		self.TrackPopupMenu(co::TPM::LEFTBUTTON, pos, hwnd_parent)?;
		hwnd_parent.PostMessage(wm::Null {})?; // necessary according to TrackPopupMenu docs
		Ok(())
	}
}
