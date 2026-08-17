pub const AUTO_WIDTH: i32 = -1i32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(AppEvents, AppEvents_Vtbl, 0xfc7a4252_78ac_4532_8c5a_563cfe138863);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for AppEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(AppEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct AppEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait AppEvents_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl AppEvents_Vtbl {
    pub const fn new<Identity: AppEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AppEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for AppEvents {}
pub const AppEventsDHTMLConnector: windows_core::GUID = windows_core::GUID::from_u128(0xade6444b_c91f_4e37_92a4_5bb430a33340);
pub const Application: windows_core::GUID = windows_core::GUID::from_u128(0x49b2791a_b1ae_4c90_9b8e_e860ba07f889);
pub const BUTTONPRESSED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CCM_COMMANDID_MASK_CONSTANTS(pub u32);
pub const CCM_COMMANDID_MASK_RESERVED: CCM_COMMANDID_MASK_CONSTANTS = CCM_COMMANDID_MASK_CONSTANTS(4294901760u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CCM_INSERTIONALLOWED(pub i32);
pub const CCM_INSERTIONALLOWED_NEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(2i32);
pub const CCM_INSERTIONALLOWED_TASK: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(4i32);
pub const CCM_INSERTIONALLOWED_TOP: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(1i32);
pub const CCM_INSERTIONALLOWED_VIEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CCM_INSERTIONPOINTID(pub i32);
pub const CCM_INSERTIONPOINTID_3RDPARTY_NEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1879048191i32);
pub const CCM_INSERTIONPOINTID_3RDPARTY_TASK: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1879048190i32);
pub const CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(268435456i32);
pub const CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(536870912i32);
pub const CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(1073741824i32);
pub const CCM_INSERTIONPOINTID_MASK_FLAGINDEX: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(31i32);
pub const CCM_INSERTIONPOINTID_MASK_RESERVED: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(268369920i32);
pub const CCM_INSERTIONPOINTID_MASK_SHARED: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-2147483648i32);
pub const CCM_INSERTIONPOINTID_MASK_SPECIAL: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-65536i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_HELP: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612732i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_NEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612735i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_TASK: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612734i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_TOP: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612736i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_VIEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612733i32);
pub const CCM_INSERTIONPOINTID_ROOT_MENU: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-2147483648i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CCM_SPECIAL(pub i32);
pub const CCM_SPECIAL_DEFAULT_ITEM: CCM_SPECIAL = CCM_SPECIAL(4i32);
pub const CCM_SPECIAL_INSERTION_POINT: CCM_SPECIAL = CCM_SPECIAL(8i32);
pub const CCM_SPECIAL_SEPARATOR: CCM_SPECIAL = CCM_SPECIAL(1i32);
pub const CCM_SPECIAL_SUBMENU: CCM_SPECIAL = CCM_SPECIAL(2i32);
pub const CCM_SPECIAL_TESTONLY: CCM_SPECIAL = CCM_SPECIAL(16i32);
pub const CCT_RESULT: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32769i32);
pub const CCT_SCOPE: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32768i32);
pub const CCT_SNAPIN_MANAGER: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32770i32);
pub const CCT_UNINITIALIZED: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(65535i32);
pub const CHECKED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(2i32);
pub const COMBOBOXBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONTEXTMENUITEM {
    pub strName: windows_core::PWSTR,
    pub strStatusBarText: windows_core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONTEXTMENUITEM2 {
    pub strName: windows_core::PWSTR,
    pub strStatusBarText: windows_core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
    pub strLanguageIndependentName: windows_core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Column, Column_Vtbl, 0xfd1c5f63_2b16_4d06_9ab3_f45350b940ab);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Column {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Column, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Column {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWidth(&self, width: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), width).ok() }
    }
    pub unsafe fn DisplayPosition(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayPosition(&self, index: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayPosition)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn Hidden(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHidden(&self, hidden: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHidden)(windows_core::Interface::as_raw(self), hidden.into()).ok() }
    }
    pub unsafe fn SetAsSortColumn(&self, sortorder: _ColumnSortOrder) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAsSortColumn)(windows_core::Interface::as_raw(self), sortorder).ok() }
    }
    pub unsafe fn IsSortColumn(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSortColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Column_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisplayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDisplayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Hidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAsSortColumn: unsafe extern "system" fn(*mut core::ffi::c_void, _ColumnSortOrder) -> windows_core::HRESULT,
    pub IsSortColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Column_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn SetWidth(&self, width: i32) -> windows_core::Result<()>;
    fn DisplayPosition(&self) -> windows_core::Result<i32>;
    fn SetDisplayPosition(&self, index: i32) -> windows_core::Result<()>;
    fn Hidden(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetHidden(&self, hidden: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetAsSortColumn(&self, sortorder: _ColumnSortOrder) -> windows_core::Result<()>;
    fn IsSortColumn(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Column_Vtbl {
    pub const fn new<Identity: Column_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Column_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Width<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Column_Impl::Width(this) {
                    Ok(ok__) => {
                        width.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidth<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Column_Impl::SetWidth(this, core::mem::transmute_copy(&width)).into()
            }
        }
        unsafe extern "system" fn DisplayPosition<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Column_Impl::DisplayPosition(this) {
                    Ok(ok__) => {
                        displayposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayPosition<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Column_Impl::SetDisplayPosition(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn Hidden<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hidden: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Column_Impl::Hidden(this) {
                    Ok(ok__) => {
                        hidden.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHidden<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hidden: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Column_Impl::SetHidden(this, core::mem::transmute_copy(&hidden)).into()
            }
        }
        unsafe extern "system" fn SetAsSortColumn<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sortorder: _ColumnSortOrder) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Column_Impl::SetAsSortColumn(this, core::mem::transmute_copy(&sortorder)).into()
            }
        }
        unsafe extern "system" fn IsSortColumn<Identity: Column_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issortcolumn: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Column_Impl::IsSortColumn(this) {
                    Ok(ok__) => {
                        issortcolumn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            DisplayPosition: DisplayPosition::<Identity, OFFSET>,
            SetDisplayPosition: SetDisplayPosition::<Identity, OFFSET>,
            Hidden: Hidden::<Identity, OFFSET>,
            SetHidden: SetHidden::<Identity, OFFSET>,
            SetAsSortColumn: SetAsSortColumn::<Identity, OFFSET>,
            IsSortColumn: IsSortColumn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Column as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Column {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Columns, Columns_Vtbl, 0x383d4d97_fc44_478b_b139_6323dc48611c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Columns {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Columns, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Columns {
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<Column> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Columns_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Columns_Impl: super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<Column>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Columns_Vtbl {
    pub const fn new<Identity: Columns_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: Columns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, column: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Columns_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        column.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: Columns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Columns_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: Columns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Columns_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Columns as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Columns {}
pub const ConsolePower: windows_core::GUID = windows_core::GUID::from_u128(0xf0285374_dff1_11d3_b433_00c04f8ecd78);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ContextMenu, ContextMenu_Vtbl, 0xdab39ce0_25e6_4e07_8362_ba9c95706545);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ContextMenu {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ContextMenu, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ContextMenu {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, indexorpath: &super::Variant::VARIANT) -> windows_core::Result<MenuItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(indexorpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextMenu_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextMenu_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, indexorpath: &super::Variant::VARIANT) -> windows_core::Result<MenuItem>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ContextMenu_Vtbl {
    pub const fn new<Identity: ContextMenu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextMenu_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexorpath: super::Variant::VARIANT, menuitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextMenu_Impl::get_Item(this, core::mem::transmute(&indexorpath)) {
                    Ok(ok__) => {
                        menuitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextMenu_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ContextMenu as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ContextMenu {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DATA_OBJECT_TYPES(pub i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Document, Document_Vtbl, 0x225120d6_1e0f_40a3_93fe_1079e6a8017b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Document {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Document, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Document {
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SaveAs(&self, filename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveAs)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)).ok() }
    }
    pub unsafe fn Close(&self, savechanges: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), savechanges.into()).ok() }
    }
    pub unsafe fn Views(&self) -> windows_core::Result<Views> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Views)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SnapIns(&self) -> windows_core::Result<SnapIns> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SnapIns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActiveView(&self) -> windows_core::Result<View> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActiveView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Location(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Location)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsSaved(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSaved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Mode(&self) -> windows_core::Result<_DocumentMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Mode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMode(&self, mode: _DocumentMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn RootNode(&self) -> windows_core::Result<Node> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ScopeNamespace(&self) -> windows_core::Result<ScopeNamespace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScopeNamespace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateProperties(&self) -> windows_core::Result<Properties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Application(&self) -> windows_core::Result<_Application> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Document_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Views: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SnapIns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSaved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut _DocumentMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, _DocumentMode) -> windows_core::HRESULT,
    pub RootNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScopeNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Document_Impl: super::Com::IDispatch_Impl {
    fn Save(&self) -> windows_core::Result<()>;
    fn SaveAs(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Close(&self, savechanges: windows_core::BOOL) -> windows_core::Result<()>;
    fn Views(&self) -> windows_core::Result<Views>;
    fn SnapIns(&self) -> windows_core::Result<SnapIns>;
    fn ActiveView(&self) -> windows_core::Result<View>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Location(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsSaved(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Mode(&self) -> windows_core::Result<_DocumentMode>;
    fn SetMode(&self, mode: _DocumentMode) -> windows_core::Result<()>;
    fn RootNode(&self) -> windows_core::Result<Node>;
    fn ScopeNamespace(&self) -> windows_core::Result<ScopeNamespace>;
    fn CreateProperties(&self) -> windows_core::Result<Properties>;
    fn Application(&self) -> windows_core::Result<_Application>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Document_Vtbl {
    pub const fn new<Identity: Document_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Save<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Document_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn SaveAs<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Document_Impl::SaveAs(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, savechanges: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Document_Impl::Close(this, core::mem::transmute_copy(&savechanges)).into()
            }
        }
        unsafe extern "system" fn Views<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, views: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::Views(this) {
                    Ok(ok__) => {
                        views.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SnapIns<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapins: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::SnapIns(this) {
                    Ok(ok__) => {
                        snapins.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActiveView<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::ActiveView(this) {
                    Ok(ok__) => {
                        view.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Document_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Location<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, location: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::Location(this) {
                    Ok(ok__) => {
                        location.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSaved<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issaved: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::IsSaved(this) {
                    Ok(ok__) => {
                        issaved.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Mode<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut _DocumentMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::Mode(this) {
                    Ok(ok__) => {
                        mode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMode<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: _DocumentMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Document_Impl::SetMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn RootNode<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::RootNode(this) {
                    Ok(ok__) => {
                        node.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScopeNamespace<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scopenamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::ScopeNamespace(this) {
                    Ok(ok__) => {
                        scopenamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateProperties<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::CreateProperties(this) {
                    Ok(ok__) => {
                        properties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Application<Identity: Document_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, application: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Document_Impl::Application(this) {
                    Ok(ok__) => {
                        application.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Save: Save::<Identity, OFFSET>,
            SaveAs: SaveAs::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Views: Views::<Identity, OFFSET>,
            SnapIns: SnapIns::<Identity, OFFSET>,
            ActiveView: ActiveView::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Location: Location::<Identity, OFFSET>,
            IsSaved: IsSaved::<Identity, OFFSET>,
            Mode: Mode::<Identity, OFFSET>,
            SetMode: SetMode::<Identity, OFFSET>,
            RootNode: RootNode::<Identity, OFFSET>,
            ScopeNamespace: ScopeNamespace::<Identity, OFFSET>,
            CreateProperties: CreateProperties::<Identity, OFFSET>,
            Application: Application::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Document as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Document {}
pub const DocumentMode_Author: _DocumentMode = _DocumentMode(0i32);
pub const DocumentMode_User: _DocumentMode = _DocumentMode(1i32);
pub const DocumentMode_User_MDI: _DocumentMode = _DocumentMode(2i32);
pub const DocumentMode_User_SDI: _DocumentMode = _DocumentMode(3i32);
pub const ENABLED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(1i32);
pub const ExportListOptions_Default: _ExportListOptions = _ExportListOptions(0i32);
pub const ExportListOptions_SelectedItemsOnly: _ExportListOptions = _ExportListOptions(4i32);
pub const ExportListOptions_TabDelimited: _ExportListOptions = _ExportListOptions(2i32);
pub const ExportListOptions_Unicode: _ExportListOptions = _ExportListOptions(1i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Extension, Extension_Vtbl, 0xad4d6ca6_912f_409b_a26e_7fd234aef542);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Extension {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Extension, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Extension {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Vendor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Vendor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Extensions(&self) -> windows_core::Result<Extensions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SnapinCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SnapinCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnableAllExtensions(&self, enable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableAllExtensions)(windows_core::Interface::as_raw(self), enable.into()).ok() }
    }
    pub unsafe fn Enable(&self, enable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), enable.into()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Extension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Vendor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SnapinCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableAllExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Extension_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Vendor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Extensions(&self) -> windows_core::Result<Extensions>;
    fn SnapinCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnableAllExtensions(&self, enable: windows_core::BOOL) -> windows_core::Result<()>;
    fn Enable(&self, enable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Extension_Vtbl {
    pub const fn new<Identity: Extension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extension_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Vendor<Identity: Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extension_Impl::Vendor(this) {
                    Ok(ok__) => {
                        vendor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Version<Identity: Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extension_Impl::Version(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Extensions<Identity: Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extensions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extension_Impl::Extensions(this) {
                    Ok(ok__) => {
                        extensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SnapinCLSID<Identity: Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapinclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extension_Impl::SnapinCLSID(this) {
                    Ok(ok__) => {
                        snapinclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Identity: Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Extension_Impl::EnableAllExtensions(this, core::mem::transmute_copy(&enable)).into()
            }
        }
        unsafe extern "system" fn Enable<Identity: Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Extension_Impl::Enable(this, core::mem::transmute_copy(&enable)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Vendor: Vendor::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            Extensions: Extensions::<Identity, OFFSET>,
            SnapinCLSID: SnapinCLSID::<Identity, OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Extension as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Extension {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Extensions, Extensions_Vtbl, 0x82dbea43_8ca4_44bc_a2ca_d18741059ec8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Extensions {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Extensions, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Extensions {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<Extension> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Extensions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Extensions_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<Extension>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Extensions_Vtbl {
    pub const fn new<Identity: Extensions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extensions_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, extension: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extensions_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        extension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Extensions_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Extensions as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Extensions {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Frame, Frame_Vtbl, 0xe5e2d970_5bb3_4306_8804_b0968a31c8e6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Frame {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Frame, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Frame {
    pub unsafe fn Maximize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Maximize)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Minimize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Minimize)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Restore(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Top(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Top)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTop(&self, top: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTop)(windows_core::Interface::as_raw(self), top).ok() }
    }
    pub unsafe fn Bottom(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBottom(&self, bottom: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBottom)(windows_core::Interface::as_raw(self), bottom).ok() }
    }
    pub unsafe fn Left(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Left)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLeft(&self, left: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLeft)(windows_core::Interface::as_raw(self), left).ok() }
    }
    pub unsafe fn Right(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Right)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRight(&self, right: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRight)(windows_core::Interface::as_raw(self), right).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Frame_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Maximize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Minimize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Top: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Bottom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Left: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Right: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Frame_Impl: super::Com::IDispatch_Impl {
    fn Maximize(&self) -> windows_core::Result<()>;
    fn Minimize(&self) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn Top(&self) -> windows_core::Result<i32>;
    fn SetTop(&self, top: i32) -> windows_core::Result<()>;
    fn Bottom(&self) -> windows_core::Result<i32>;
    fn SetBottom(&self, bottom: i32) -> windows_core::Result<()>;
    fn Left(&self) -> windows_core::Result<i32>;
    fn SetLeft(&self, left: i32) -> windows_core::Result<()>;
    fn Right(&self) -> windows_core::Result<i32>;
    fn SetRight(&self, right: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Frame_Vtbl {
    pub const fn new<Identity: Frame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Maximize<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Frame_Impl::Maximize(this).into()
            }
        }
        unsafe extern "system" fn Minimize<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Frame_Impl::Minimize(this).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Frame_Impl::Restore(this).into()
            }
        }
        unsafe extern "system" fn Top<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Frame_Impl::Top(this) {
                    Ok(ok__) => {
                        top.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTop<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Frame_Impl::SetTop(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn Bottom<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Frame_Impl::Bottom(this) {
                    Ok(ok__) => {
                        bottom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBottom<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Frame_Impl::SetBottom(this, core::mem::transmute_copy(&bottom)).into()
            }
        }
        unsafe extern "system" fn Left<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Frame_Impl::Left(this) {
                    Ok(ok__) => {
                        left.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLeft<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Frame_Impl::SetLeft(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn Right<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Frame_Impl::Right(this) {
                    Ok(ok__) => {
                        right.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRight<Identity: Frame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Frame_Impl::SetRight(this, core::mem::transmute_copy(&right)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Maximize: Maximize::<Identity, OFFSET>,
            Minimize: Minimize::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            Top: Top::<Identity, OFFSET>,
            SetTop: SetTop::<Identity, OFFSET>,
            Bottom: Bottom::<Identity, OFFSET>,
            SetBottom: SetBottom::<Identity, OFFSET>,
            Left: Left::<Identity, OFFSET>,
            SetLeft: SetLeft::<Identity, OFFSET>,
            Right: Right::<Identity, OFFSET>,
            SetRight: SetRight::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Frame as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Frame {}
pub const HDI_HIDDEN: u32 = 1u32;
pub const HIDDEN: MMC_BUTTON_STATE = MMC_BUTTON_STATE(4i32);
pub const HIDE_COLUMN: i32 = -4i32;
windows_core::imp::define_interface!(IColumnData, IColumnData_Vtbl, 0x547c1354_024d_11d3_a707_00c04f8ef4cb);
windows_core::imp::interface_hierarchy!(IColumnData, windows_core::IUnknown);
impl IColumnData {
    pub unsafe fn SetColumnConfigData(&self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColumnConfigData)(windows_core::Interface::as_raw(self), pcolid, pcolsetdata).ok() }
    }
    pub unsafe fn GetColumnConfigData(&self, pcolid: *const SColumnSetID) -> windows_core::Result<*mut MMC_COLUMN_SET_DATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnConfigData)(windows_core::Interface::as_raw(self), pcolid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetColumnSortData(&self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColumnSortData)(windows_core::Interface::as_raw(self), pcolid, pcolsortdata).ok() }
    }
    pub unsafe fn GetColumnSortData(&self, pcolid: *const SColumnSetID) -> windows_core::Result<*mut MMC_SORT_SET_DATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnSortData)(windows_core::Interface::as_raw(self), pcolid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetColumnConfigData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *const MMC_COLUMN_SET_DATA) -> windows_core::HRESULT,
    pub GetColumnConfigData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *mut *mut MMC_COLUMN_SET_DATA) -> windows_core::HRESULT,
    pub SetColumnSortData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *const MMC_SORT_SET_DATA) -> windows_core::HRESULT,
    pub GetColumnSortData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *mut *mut MMC_SORT_SET_DATA) -> windows_core::HRESULT,
}
pub trait IColumnData_Impl: windows_core::IUnknownImpl {
    fn SetColumnConfigData(&self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> windows_core::Result<()>;
    fn GetColumnConfigData(&self, pcolid: *const SColumnSetID) -> windows_core::Result<*mut MMC_COLUMN_SET_DATA>;
    fn SetColumnSortData(&self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> windows_core::Result<()>;
    fn GetColumnSortData(&self, pcolid: *const SColumnSetID) -> windows_core::Result<*mut MMC_SORT_SET_DATA>;
}
impl IColumnData_Vtbl {
    pub const fn new<Identity: IColumnData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetColumnConfigData<Identity: IColumnData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnData_Impl::SetColumnConfigData(this, core::mem::transmute_copy(&pcolid), core::mem::transmute_copy(&pcolsetdata)).into()
            }
        }
        unsafe extern "system" fn GetColumnConfigData<Identity: IColumnData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IColumnData_Impl::GetColumnConfigData(this, core::mem::transmute_copy(&pcolid)) {
                    Ok(ok__) => {
                        ppcolsetdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColumnSortData<Identity: IColumnData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnData_Impl::SetColumnSortData(this, core::mem::transmute_copy(&pcolid), core::mem::transmute_copy(&pcolsortdata)).into()
            }
        }
        unsafe extern "system" fn GetColumnSortData<Identity: IColumnData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IColumnData_Impl::GetColumnSortData(this, core::mem::transmute_copy(&pcolid)) {
                    Ok(ok__) => {
                        ppcolsortdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetColumnConfigData: SetColumnConfigData::<Identity, OFFSET>,
            GetColumnConfigData: GetColumnConfigData::<Identity, OFFSET>,
            SetColumnSortData: SetColumnSortData::<Identity, OFFSET>,
            GetColumnSortData: GetColumnSortData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IColumnData {}
windows_core::imp::define_interface!(IComponent, IComponent_Vtbl, 0x43136eb2_d36c_11cf_adbc_00aa00a80033);
windows_core::imp::interface_hierarchy!(IComponent, windows_core::IUnknown);
impl IComponent {
    pub unsafe fn Initialize<P0>(&self, lpconsole: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConsole>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lpconsole.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notify<P0>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), lpdataobject.param().abi(), event, arg, param3).ok() }
    }
    pub unsafe fn Destroy(&self, cookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self), cookie).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryDataObject)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut windows_core::PWSTR, pviewoptions: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResultViewType)(windows_core::Interface::as_raw(self), cookie, ppviewtype as _, pviewoptions as _).ok() }
    }
    pub unsafe fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayInfo)(windows_core::Interface::as_raw(self), presultdataitem as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompareObjects)(windows_core::Interface::as_raw(self), lpdataobjecta.param().abi(), lpdataobjectb.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MMC_NOTIFY_TYPE, super::super::Foundation::LPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Notify: usize,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDataObject: unsafe extern "system" fn(*mut core::ffi::c_void, isize, DATA_OBJECT_TYPES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDataObject: usize,
    pub GetResultViewType: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *mut windows_core::PWSTR, *mut i32) -> windows_core::HRESULT,
    pub GetDisplayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RESULTDATAITEM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareObjects: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponent_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, lpconsole: windows_core::Ref<IConsole>) -> windows_core::Result<()>;
    fn Notify(&self, lpdataobject: windows_core::Ref<super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn Destroy(&self, cookie: isize) -> windows_core::Result<()>;
    fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDataObject>;
    fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut windows_core::PWSTR, pviewoptions: *mut i32) -> windows_core::Result<()>;
    fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> windows_core::Result<()>;
    fn CompareObjects(&self, lpdataobjecta: windows_core::Ref<super::Com::IDataObject>, lpdataobjectb: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IComponent_Vtbl {
    pub const fn new<Identity: IComponent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpconsole: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent_Impl::Initialize(this, core::mem::transmute_copy(&lpconsole)).into()
            }
        }
        unsafe extern "system" fn Notify<Identity: IComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobject: *mut core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent_Impl::Notify(this, core::mem::transmute_copy(&lpdataobject), core::mem::transmute_copy(&event), core::mem::transmute_copy(&arg), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn Destroy<Identity: IComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent_Impl::Destroy(this, core::mem::transmute_copy(&cookie)).into()
            }
        }
        unsafe extern "system" fn QueryDataObject<Identity: IComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComponent_Impl::QueryDataObject(this, core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppdataobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResultViewType<Identity: IComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize, ppviewtype: *mut windows_core::PWSTR, pviewoptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent_Impl::GetResultViewType(this, core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&ppviewtype), core::mem::transmute_copy(&pviewoptions)).into()
            }
        }
        unsafe extern "system" fn GetDisplayInfo<Identity: IComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent_Impl::GetDisplayInfo(this, core::mem::transmute_copy(&presultdataitem)).into()
            }
        }
        unsafe extern "system" fn CompareObjects<Identity: IComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobjecta: *mut core::ffi::c_void, lpdataobjectb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent_Impl::CompareObjects(this, core::mem::transmute_copy(&lpdataobjecta), core::mem::transmute_copy(&lpdataobjectb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            QueryDataObject: QueryDataObject::<Identity, OFFSET>,
            GetResultViewType: GetResultViewType::<Identity, OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Identity, OFFSET>,
            CompareObjects: CompareObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComponent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IComponent {}
windows_core::imp::define_interface!(IComponent2, IComponent2_Vtbl, 0x79a2d615_4a10_4ed4_8c65_8633f9335095);
impl core::ops::Deref for IComponent2 {
    type Target = IComponent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComponent2, windows_core::IUnknown, IComponent);
impl IComponent2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryDispatch)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetResultViewType2(&self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResultViewType2)(windows_core::Interface::as_raw(self), cookie, core::mem::transmute(presultviewtype)).ok() }
    }
    pub unsafe fn RestoreResultView(&self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestoreResultView)(windows_core::Interface::as_raw(self), cookie, core::mem::transmute(presultviewtype)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponent2_Vtbl {
    pub base__: IComponent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, isize, DATA_OBJECT_TYPES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
    pub GetResultViewType2: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *mut RESULT_VIEW_TYPE_INFO) -> windows_core::HRESULT,
    pub RestoreResultView: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *const RESULT_VIEW_TYPE_INFO) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponent2_Impl: IComponent_Impl {
    fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDispatch>;
    fn GetResultViewType2(&self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> windows_core::Result<()>;
    fn RestoreResultView(&self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IComponent2_Vtbl {
    pub const fn new<Identity: IComponent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryDispatch<Identity: IComponent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComponent2_Impl::QueryDispatch(this, core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppdispatch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResultViewType2<Identity: IComponent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent2_Impl::GetResultViewType2(this, core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&presultviewtype)).into()
            }
        }
        unsafe extern "system" fn RestoreResultView<Identity: IComponent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponent2_Impl::RestoreResultView(this, core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&presultviewtype)).into()
            }
        }
        Self {
            base__: IComponent_Vtbl::new::<Identity, OFFSET>(),
            QueryDispatch: QueryDispatch::<Identity, OFFSET>,
            GetResultViewType2: GetResultViewType2::<Identity, OFFSET>,
            RestoreResultView: RestoreResultView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComponent2 as windows_core::Interface>::IID || iid == &<IComponent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IComponent2 {}
windows_core::imp::define_interface!(IComponentData, IComponentData_Vtbl, 0x955ab28a_5218_11d0_a985_00c04fd8d565);
windows_core::imp::interface_hierarchy!(IComponentData, windows_core::IUnknown);
impl IComponentData {
    pub unsafe fn Initialize<P0>(&self, punknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), punknown.param().abi()).ok() }
    }
    pub unsafe fn CreateComponent(&self) -> windows_core::Result<IComponent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateComponent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notify<P0>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), lpdataobject.param().abi(), event, arg, param3).ok() }
    }
    pub unsafe fn Destroy(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryDataObject)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayInfo)(windows_core::Interface::as_raw(self), pscopedataitem as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompareObjects)(windows_core::Interface::as_raw(self), lpdataobjecta.param().abi(), lpdataobjectb.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MMC_NOTIFY_TYPE, super::super::Foundation::LPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Notify: usize,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDataObject: unsafe extern "system" fn(*mut core::ffi::c_void, isize, DATA_OBJECT_TYPES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDataObject: usize,
    pub GetDisplayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCOPEDATAITEM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareObjects: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentData_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateComponent(&self) -> windows_core::Result<IComponent>;
    fn Notify(&self, lpdataobject: windows_core::Ref<super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDataObject>;
    fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> windows_core::Result<()>;
    fn CompareObjects(&self, lpdataobjecta: windows_core::Ref<super::Com::IDataObject>, lpdataobjectb: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IComponentData_Vtbl {
    pub const fn new<Identity: IComponentData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IComponentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentData_Impl::Initialize(this, core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn CreateComponent<Identity: IComponentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComponentData_Impl::CreateComponent(this) {
                    Ok(ok__) => {
                        ppcomponent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notify<Identity: IComponentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobject: *mut core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentData_Impl::Notify(this, core::mem::transmute_copy(&lpdataobject), core::mem::transmute_copy(&event), core::mem::transmute_copy(&arg), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn Destroy<Identity: IComponentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentData_Impl::Destroy(this).into()
            }
        }
        unsafe extern "system" fn QueryDataObject<Identity: IComponentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComponentData_Impl::QueryDataObject(this, core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppdataobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayInfo<Identity: IComponentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentData_Impl::GetDisplayInfo(this, core::mem::transmute_copy(&pscopedataitem)).into()
            }
        }
        unsafe extern "system" fn CompareObjects<Identity: IComponentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobjecta: *mut core::ffi::c_void, lpdataobjectb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentData_Impl::CompareObjects(this, core::mem::transmute_copy(&lpdataobjecta), core::mem::transmute_copy(&lpdataobjectb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            CreateComponent: CreateComponent::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            QueryDataObject: QueryDataObject::<Identity, OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Identity, OFFSET>,
            CompareObjects: CompareObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComponentData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IComponentData {}
windows_core::imp::define_interface!(IComponentData2, IComponentData2_Vtbl, 0xcca0f2d2_82de_41b5_bf47_3b2076273d5c);
impl core::ops::Deref for IComponentData2 {
    type Target = IComponentData;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComponentData2, windows_core::IUnknown, IComponentData);
impl IComponentData2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryDispatch)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentData2_Vtbl {
    pub base__: IComponentData_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, isize, DATA_OBJECT_TYPES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IComponentData2_Impl: IComponentData_Impl {
    fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IComponentData2_Vtbl {
    pub const fn new<Identity: IComponentData2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryDispatch<Identity: IComponentData2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComponentData2_Impl::QueryDispatch(this, core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppdispatch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IComponentData_Vtbl::new::<Identity, OFFSET>(), QueryDispatch: QueryDispatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComponentData2 as windows_core::Interface>::IID || iid == &<IComponentData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IComponentData2 {}
windows_core::imp::define_interface!(IConsole, IConsole_Vtbl, 0x43136eb1_d36c_11cf_adbc_00aa00a80033);
windows_core::imp::interface_hierarchy!(IConsole, windows_core::IUnknown);
impl IConsole {
    pub unsafe fn SetHeader<P0>(&self, pheader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IHeaderCtrl>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHeader)(windows_core::Interface::as_raw(self), pheader.param().abi()).ok() }
    }
    pub unsafe fn SetToolbar<P0>(&self, ptoolbar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IToolbar>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetToolbar)(windows_core::Interface::as_raw(self), ptoolbar.param().abi()).ok() }
    }
    pub unsafe fn QueryResultView(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryResultView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryScopeImageList(&self) -> windows_core::Result<IImageList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryScopeImageList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryResultImageList(&self) -> windows_core::Result<IImageList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryResultImageList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateAllViews<P0>(&self, lpdataobject: P0, data: super::super::Foundation::LPARAM, hint: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateAllViews)(windows_core::Interface::as_raw(self), lpdataobject.param().abi(), data, hint).ok() }
    }
    pub unsafe fn MessageBox<P0, P1>(&self, lpsztext: P0, lpsztitle: P1, fustyle: u32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageBox)(windows_core::Interface::as_raw(self), lpsztext.param().abi(), lpsztitle.param().abi(), fustyle, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryConsoleVerb(&self) -> windows_core::Result<IConsoleVerb> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryConsoleVerb)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectScopeItem)(windows_core::Interface::as_raw(self), hscopeitem).ok() }
    }
    pub unsafe fn GetMainWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMainWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NewWindow)(windows_core::Interface::as_raw(self), hscopeitem, loptions).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryResultView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryScopeImageList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryResultImageList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateAllViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::LPARAM, isize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateAllViews: usize,
    pub MessageBox: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut i32) -> windows_core::HRESULT,
    pub QueryConsoleVerb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectScopeItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub GetMainWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub NewWindow: unsafe extern "system" fn(*mut core::ffi::c_void, isize, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConsole_Impl: windows_core::IUnknownImpl {
    fn SetHeader(&self, pheader: windows_core::Ref<IHeaderCtrl>) -> windows_core::Result<()>;
    fn SetToolbar(&self, ptoolbar: windows_core::Ref<IToolbar>) -> windows_core::Result<()>;
    fn QueryResultView(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryScopeImageList(&self) -> windows_core::Result<IImageList>;
    fn QueryResultImageList(&self) -> windows_core::Result<IImageList>;
    fn UpdateAllViews(&self, lpdataobject: windows_core::Ref<super::Com::IDataObject>, data: super::super::Foundation::LPARAM, hint: isize) -> windows_core::Result<()>;
    fn MessageBox(&self, lpsztext: &windows_core::PCWSTR, lpsztitle: &windows_core::PCWSTR, fustyle: u32) -> windows_core::Result<i32>;
    fn QueryConsoleVerb(&self) -> windows_core::Result<IConsoleVerb>;
    fn SelectScopeItem(&self, hscopeitem: isize) -> windows_core::Result<()>;
    fn GetMainWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IConsole_Vtbl {
    pub const fn new<Identity: IConsole_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHeader<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole_Impl::SetHeader(this, core::mem::transmute_copy(&pheader)).into()
            }
        }
        unsafe extern "system" fn SetToolbar<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoolbar: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole_Impl::SetToolbar(this, core::mem::transmute_copy(&ptoolbar)).into()
            }
        }
        unsafe extern "system" fn QueryResultView<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsole_Impl::QueryResultView(this) {
                    Ok(ok__) => {
                        punknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryScopeImageList<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimagelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsole_Impl::QueryScopeImageList(this) {
                    Ok(ok__) => {
                        ppimagelist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryResultImageList<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimagelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsole_Impl::QueryResultImageList(this) {
                    Ok(ok__) => {
                        ppimagelist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateAllViews<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobject: *mut core::ffi::c_void, data: super::super::Foundation::LPARAM, hint: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole_Impl::UpdateAllViews(this, core::mem::transmute_copy(&lpdataobject), core::mem::transmute_copy(&data), core::mem::transmute_copy(&hint)).into()
            }
        }
        unsafe extern "system" fn MessageBox<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsztext: windows_core::PCWSTR, lpsztitle: windows_core::PCWSTR, fustyle: u32, piretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsole_Impl::MessageBox(this, core::mem::transmute(&lpsztext), core::mem::transmute(&lpsztitle), core::mem::transmute_copy(&fustyle)) {
                    Ok(ok__) => {
                        piretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryConsoleVerb<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconsoleverb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsole_Impl::QueryConsoleVerb(this) {
                    Ok(ok__) => {
                        ppconsoleverb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectScopeItem<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hscopeitem: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole_Impl::SelectScopeItem(this, core::mem::transmute_copy(&hscopeitem)).into()
            }
        }
        unsafe extern "system" fn GetMainWindow<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsole_Impl::GetMainWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NewWindow<Identity: IConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hscopeitem: isize, loptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole_Impl::NewWindow(this, core::mem::transmute_copy(&hscopeitem), core::mem::transmute_copy(&loptions)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHeader: SetHeader::<Identity, OFFSET>,
            SetToolbar: SetToolbar::<Identity, OFFSET>,
            QueryResultView: QueryResultView::<Identity, OFFSET>,
            QueryScopeImageList: QueryScopeImageList::<Identity, OFFSET>,
            QueryResultImageList: QueryResultImageList::<Identity, OFFSET>,
            UpdateAllViews: UpdateAllViews::<Identity, OFFSET>,
            MessageBox: MessageBox::<Identity, OFFSET>,
            QueryConsoleVerb: QueryConsoleVerb::<Identity, OFFSET>,
            SelectScopeItem: SelectScopeItem::<Identity, OFFSET>,
            GetMainWindow: GetMainWindow::<Identity, OFFSET>,
            NewWindow: NewWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsole as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IConsole {}
windows_core::imp::define_interface!(IConsole2, IConsole2_Vtbl, 0x103d842a_aa63_11d1_a7e1_00c04fd8d565);
impl core::ops::Deref for IConsole2 {
    type Target = IConsole;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsole2, windows_core::IUnknown, IConsole);
impl IConsole2 {
    pub unsafe fn Expand(&self, hitem: isize, bexpand: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), hitem, bexpand.into()).ok() }
    }
    pub unsafe fn IsTaskpadViewPreferred(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsTaskpadViewPreferred)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetStatusText<P0>(&self, pszstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStatusText)(windows_core::Interface::as_raw(self), pszstatustext.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole2_Vtbl {
    pub base__: IConsole_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, isize, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsTaskpadViewPreferred: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatusText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConsole2_Impl: IConsole_Impl {
    fn Expand(&self, hitem: isize, bexpand: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsTaskpadViewPreferred(&self) -> windows_core::Result<()>;
    fn SetStatusText(&self, pszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IConsole2_Vtbl {
    pub const fn new<Identity: IConsole2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Expand<Identity: IConsole2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hitem: isize, bexpand: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole2_Impl::Expand(this, core::mem::transmute_copy(&hitem), core::mem::transmute_copy(&bexpand)).into()
            }
        }
        unsafe extern "system" fn IsTaskpadViewPreferred<Identity: IConsole2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole2_Impl::IsTaskpadViewPreferred(this).into()
            }
        }
        unsafe extern "system" fn SetStatusText<Identity: IConsole2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole2_Impl::SetStatusText(this, core::mem::transmute(&pszstatustext)).into()
            }
        }
        Self {
            base__: IConsole_Vtbl::new::<Identity, OFFSET>(),
            Expand: Expand::<Identity, OFFSET>,
            IsTaskpadViewPreferred: IsTaskpadViewPreferred::<Identity, OFFSET>,
            SetStatusText: SetStatusText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsole2 as windows_core::Interface>::IID || iid == &<IConsole as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IConsole2 {}
windows_core::imp::define_interface!(IConsole3, IConsole3_Vtbl, 0x4f85efdb_d0e1_498c_8d4a_d010dfdd404f);
impl core::ops::Deref for IConsole3 {
    type Target = IConsole2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsole3, windows_core::IUnknown, IConsole, IConsole2);
impl IConsole3 {
    pub unsafe fn RenameScopeItem(&self, hscopeitem: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RenameScopeItem)(windows_core::Interface::as_raw(self), hscopeitem).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole3_Vtbl {
    pub base__: IConsole2_Vtbl,
    pub RenameScopeItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConsole3_Impl: IConsole2_Impl {
    fn RenameScopeItem(&self, hscopeitem: isize) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IConsole3_Vtbl {
    pub const fn new<Identity: IConsole3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RenameScopeItem<Identity: IConsole3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hscopeitem: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsole3_Impl::RenameScopeItem(this, core::mem::transmute_copy(&hscopeitem)).into()
            }
        }
        Self { base__: IConsole2_Vtbl::new::<Identity, OFFSET>(), RenameScopeItem: RenameScopeItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsole3 as windows_core::Interface>::IID || iid == &<IConsole as windows_core::Interface>::IID || iid == &<IConsole2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IConsole3 {}
windows_core::imp::define_interface!(IConsoleNameSpace, IConsoleNameSpace_Vtbl, 0xbedeb620_f24d_11cf_8afc_00aa003ca9f6);
windows_core::imp::interface_hierarchy!(IConsoleNameSpace, windows_core::IUnknown);
impl IConsoleNameSpace {
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertItem)(windows_core::Interface::as_raw(self), item as _).ok() }
    }
    pub unsafe fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), hitem, fdeletethis).ok() }
    }
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetItem)(windows_core::Interface::as_raw(self), item).ok() }
    }
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), item as _).ok() }
    }
    pub unsafe fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetChildItem)(windows_core::Interface::as_raw(self), item, pitemchild as _, pcookie as _).ok() }
    }
    pub unsafe fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNextItem)(windows_core::Interface::as_raw(self), item, pitemnext as _, pcookie as _).ok() }
    }
    pub unsafe fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetParentItem)(windows_core::Interface::as_raw(self), item, pitemparent as _, pcookie as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleNameSpace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InsertItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCOPEDATAITEM) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize, i32) -> windows_core::HRESULT,
    pub SetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const SCOPEDATAITEM) -> windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCOPEDATAITEM) -> windows_core::HRESULT,
    pub GetChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *mut isize, *mut isize) -> windows_core::HRESULT,
    pub GetNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *mut isize, *mut isize) -> windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *mut isize, *mut isize) -> windows_core::HRESULT,
}
pub trait IConsoleNameSpace_Impl: windows_core::IUnknownImpl {
    fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> windows_core::Result<()>;
    fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> windows_core::Result<()>;
    fn SetItem(&self, item: *const SCOPEDATAITEM) -> windows_core::Result<()>;
    fn GetItem(&self, item: *mut SCOPEDATAITEM) -> windows_core::Result<()>;
    fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> windows_core::Result<()>;
    fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> windows_core::Result<()>;
    fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> windows_core::Result<()>;
}
impl IConsoleNameSpace_Vtbl {
    pub const fn new<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertItem<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut SCOPEDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace_Impl::InsertItem(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hitem: isize, fdeletethis: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace_Impl::DeleteItem(this, core::mem::transmute_copy(&hitem), core::mem::transmute_copy(&fdeletethis)).into()
            }
        }
        unsafe extern "system" fn SetItem<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *const SCOPEDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace_Impl::SetItem(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn GetItem<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut SCOPEDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace_Impl::GetItem(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn GetChildItem<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace_Impl::GetChildItem(this, core::mem::transmute_copy(&item), core::mem::transmute_copy(&pitemchild), core::mem::transmute_copy(&pcookie)).into()
            }
        }
        unsafe extern "system" fn GetNextItem<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace_Impl::GetNextItem(this, core::mem::transmute_copy(&item), core::mem::transmute_copy(&pitemnext), core::mem::transmute_copy(&pcookie)).into()
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: IConsoleNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace_Impl::GetParentItem(this, core::mem::transmute_copy(&item), core::mem::transmute_copy(&pitemparent), core::mem::transmute_copy(&pcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertItem: InsertItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            SetItem: SetItem::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            GetChildItem: GetChildItem::<Identity, OFFSET>,
            GetNextItem: GetNextItem::<Identity, OFFSET>,
            GetParentItem: GetParentItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsoleNameSpace as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConsoleNameSpace {}
windows_core::imp::define_interface!(IConsoleNameSpace2, IConsoleNameSpace2_Vtbl, 0x255f18cc_65db_11d1_a7dc_00c04fd8d565);
impl core::ops::Deref for IConsoleNameSpace2 {
    type Target = IConsoleNameSpace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsoleNameSpace2, windows_core::IUnknown, IConsoleNameSpace);
impl IConsoleNameSpace2 {
    pub unsafe fn Expand(&self, hitem: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), hitem).ok() }
    }
    pub unsafe fn AddExtension(&self, hitem: isize, lpclsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddExtension)(windows_core::Interface::as_raw(self), hitem, lpclsid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleNameSpace2_Vtbl {
    pub base__: IConsoleNameSpace_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub AddExtension: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IConsoleNameSpace2_Impl: IConsoleNameSpace_Impl {
    fn Expand(&self, hitem: isize) -> windows_core::Result<()>;
    fn AddExtension(&self, hitem: isize, lpclsid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IConsoleNameSpace2_Vtbl {
    pub const fn new<Identity: IConsoleNameSpace2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Expand<Identity: IConsoleNameSpace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hitem: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace2_Impl::Expand(this, core::mem::transmute_copy(&hitem)).into()
            }
        }
        unsafe extern "system" fn AddExtension<Identity: IConsoleNameSpace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hitem: isize, lpclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleNameSpace2_Impl::AddExtension(this, core::mem::transmute_copy(&hitem), core::mem::transmute_copy(&lpclsid)).into()
            }
        }
        Self { base__: IConsoleNameSpace_Vtbl::new::<Identity, OFFSET>(), Expand: Expand::<Identity, OFFSET>, AddExtension: AddExtension::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsoleNameSpace2 as windows_core::Interface>::IID || iid == &<IConsoleNameSpace as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConsoleNameSpace2 {}
windows_core::imp::define_interface!(IConsolePower, IConsolePower_Vtbl, 0x1cfbdd0e_62ca_49ce_a3af_dbb2de61b068);
windows_core::imp::interface_hierarchy!(IConsolePower, windows_core::IUnknown);
impl IConsolePower {
    pub unsafe fn SetExecutionState(&self, dwadd: u32, dwremove: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExecutionState)(windows_core::Interface::as_raw(self), dwadd, dwremove).ok() }
    }
    pub unsafe fn ResetIdleTimer(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetIdleTimer)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsolePower_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetExecutionState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ResetIdleTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IConsolePower_Impl: windows_core::IUnknownImpl {
    fn SetExecutionState(&self, dwadd: u32, dwremove: u32) -> windows_core::Result<()>;
    fn ResetIdleTimer(&self, dwflags: u32) -> windows_core::Result<()>;
}
impl IConsolePower_Vtbl {
    pub const fn new<Identity: IConsolePower_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetExecutionState<Identity: IConsolePower_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwadd: u32, dwremove: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsolePower_Impl::SetExecutionState(this, core::mem::transmute_copy(&dwadd), core::mem::transmute_copy(&dwremove)).into()
            }
        }
        unsafe extern "system" fn ResetIdleTimer<Identity: IConsolePower_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsolePower_Impl::ResetIdleTimer(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetExecutionState: SetExecutionState::<Identity, OFFSET>,
            ResetIdleTimer: ResetIdleTimer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsolePower as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConsolePower {}
windows_core::imp::define_interface!(IConsolePowerSink, IConsolePowerSink_Vtbl, 0x3333759f_fe4f_4975_b143_fec0a5dd6d65);
windows_core::imp::interface_hierarchy!(IConsolePowerSink, windows_core::IUnknown);
impl IConsolePowerSink {
    pub unsafe fn OnPowerBroadcast(&self, nevent: u32, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnPowerBroadcast)(windows_core::Interface::as_raw(self), nevent, lparam, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsolePowerSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPowerBroadcast: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::LPARAM, *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT,
}
pub trait IConsolePowerSink_Impl: windows_core::IUnknownImpl {
    fn OnPowerBroadcast(&self, nevent: u32, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::LRESULT>;
}
impl IConsolePowerSink_Vtbl {
    pub const fn new<Identity: IConsolePowerSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPowerBroadcast<Identity: IConsolePowerSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsolePowerSink_Impl::OnPowerBroadcast(this, core::mem::transmute_copy(&nevent), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plreturn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPowerBroadcast: OnPowerBroadcast::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsolePowerSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConsolePowerSink {}
windows_core::imp::define_interface!(IConsoleVerb, IConsoleVerb_Vtbl, 0xe49f7a60_74af_11d0_a286_00c04fd8fe93);
windows_core::imp::interface_hierarchy!(IConsoleVerb, windows_core::IUnknown);
impl IConsoleVerb {
    pub unsafe fn GetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVerbState)(windows_core::Interface::as_raw(self), ecmdid, nstate, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVerbState)(windows_core::Interface::as_raw(self), ecmdid, nstate, bstate.into()).ok() }
    }
    pub unsafe fn SetDefaultVerb(&self, ecmdid: MMC_CONSOLE_VERB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultVerb)(windows_core::Interface::as_raw(self), ecmdid).ok() }
    }
    pub unsafe fn GetDefaultVerb(&self) -> windows_core::Result<MMC_CONSOLE_VERB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultVerb)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleVerb_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVerbState: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONSOLE_VERB, MMC_BUTTON_STATE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetVerbState: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONSOLE_VERB, MMC_BUTTON_STATE, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetDefaultVerb: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONSOLE_VERB) -> windows_core::HRESULT,
    pub GetDefaultVerb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MMC_CONSOLE_VERB) -> windows_core::HRESULT,
}
pub trait IConsoleVerb_Impl: windows_core::IUnknownImpl {
    fn GetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> windows_core::Result<windows_core::BOOL>;
    fn SetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetDefaultVerb(&self, ecmdid: MMC_CONSOLE_VERB) -> windows_core::Result<()>;
    fn GetDefaultVerb(&self) -> windows_core::Result<MMC_CONSOLE_VERB>;
}
impl IConsoleVerb_Vtbl {
    pub const fn new<Identity: IConsoleVerb_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVerbState<Identity: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsoleVerb_Impl::GetVerbState(this, core::mem::transmute_copy(&ecmdid), core::mem::transmute_copy(&nstate)) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVerbState<Identity: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleVerb_Impl::SetVerbState(this, core::mem::transmute_copy(&ecmdid), core::mem::transmute_copy(&nstate), core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn SetDefaultVerb<Identity: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConsoleVerb_Impl::SetDefaultVerb(this, core::mem::transmute_copy(&ecmdid)).into()
            }
        }
        unsafe extern "system" fn GetDefaultVerb<Identity: IConsoleVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConsoleVerb_Impl::GetDefaultVerb(this) {
                    Ok(ok__) => {
                        pecmdid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVerbState: GetVerbState::<Identity, OFFSET>,
            SetVerbState: SetVerbState::<Identity, OFFSET>,
            SetDefaultVerb: SetDefaultVerb::<Identity, OFFSET>,
            GetDefaultVerb: GetDefaultVerb::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConsoleVerb as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConsoleVerb {}
windows_core::imp::define_interface!(IContextMenuCallback, IContextMenuCallback_Vtbl, 0x43136eb7_d36c_11cf_adbc_00aa00a80033);
windows_core::imp::interface_hierarchy!(IContextMenuCallback, windows_core::IUnknown);
impl IContextMenuCallback {
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pitem).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTEXTMENUITEM) -> windows_core::HRESULT,
}
pub trait IContextMenuCallback_Impl: windows_core::IUnknownImpl {
    fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> windows_core::Result<()>;
}
impl IContextMenuCallback_Vtbl {
    pub const fn new<Identity: IContextMenuCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddItem<Identity: IContextMenuCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextMenuCallback_Impl::AddItem(this, core::mem::transmute_copy(&pitem)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddItem: AddItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextMenuCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContextMenuCallback {}
windows_core::imp::define_interface!(IContextMenuCallback2, IContextMenuCallback2_Vtbl, 0xe178bc0e_2ed0_4b5e_8097_42c9087e8b33);
windows_core::imp::interface_hierarchy!(IContextMenuCallback2, windows_core::IUnknown);
impl IContextMenuCallback2 {
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pitem).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuCallback2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTEXTMENUITEM2) -> windows_core::HRESULT,
}
pub trait IContextMenuCallback2_Impl: windows_core::IUnknownImpl {
    fn AddItem(&self, pitem: *const CONTEXTMENUITEM2) -> windows_core::Result<()>;
}
impl IContextMenuCallback2_Vtbl {
    pub const fn new<Identity: IContextMenuCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddItem<Identity: IContextMenuCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextMenuCallback2_Impl::AddItem(this, core::mem::transmute_copy(&pitem)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddItem: AddItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextMenuCallback2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContextMenuCallback2 {}
windows_core::imp::define_interface!(IContextMenuProvider, IContextMenuProvider_Vtbl, 0x43136eb6_d36c_11cf_adbc_00aa00a80033);
impl core::ops::Deref for IContextMenuProvider {
    type Target = IContextMenuCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContextMenuProvider, windows_core::IUnknown, IContextMenuCallback);
impl IContextMenuProvider {
    pub unsafe fn EmptyMenuList(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EmptyMenuList)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPrimaryExtensionItems<P0, P1>(&self, piextension: P0, pidataobject: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPrimaryExtensionItems)(windows_core::Interface::as_raw(self), piextension.param().abi(), pidataobject.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddThirdPartyExtensionItems<P0>(&self, pidataobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddThirdPartyExtensionItems)(windows_core::Interface::as_raw(self), pidataobject.param().abi()).ok() }
    }
    pub unsafe fn ShowContextMenu(&self, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self), hwndparent, xpos, ypos, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuProvider_Vtbl {
    pub base__: IContextMenuCallback_Vtbl,
    pub EmptyMenuList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPrimaryExtensionItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPrimaryExtensionItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddThirdPartyExtensionItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddThirdPartyExtensionItems: usize,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IContextMenuProvider_Impl: IContextMenuCallback_Impl {
    fn EmptyMenuList(&self) -> windows_core::Result<()>;
    fn AddPrimaryExtensionItems(&self, piextension: windows_core::Ref<windows_core::IUnknown>, pidataobject: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
    fn AddThirdPartyExtensionItems(&self, pidataobject: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
    fn ShowContextMenu(&self, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IContextMenuProvider_Vtbl {
    pub const fn new<Identity: IContextMenuProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EmptyMenuList<Identity: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextMenuProvider_Impl::EmptyMenuList(this).into()
            }
        }
        unsafe extern "system" fn AddPrimaryExtensionItems<Identity: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piextension: *mut core::ffi::c_void, pidataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextMenuProvider_Impl::AddPrimaryExtensionItems(this, core::mem::transmute_copy(&piextension), core::mem::transmute_copy(&pidataobject)).into()
            }
        }
        unsafe extern "system" fn AddThirdPartyExtensionItems<Identity: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextMenuProvider_Impl::AddThirdPartyExtensionItems(this, core::mem::transmute_copy(&pidataobject)).into()
            }
        }
        unsafe extern "system" fn ShowContextMenu<Identity: IContextMenuProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContextMenuProvider_Impl::ShowContextMenu(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&xpos), core::mem::transmute_copy(&ypos)) {
                    Ok(ok__) => {
                        plselected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IContextMenuCallback_Vtbl::new::<Identity, OFFSET>(),
            EmptyMenuList: EmptyMenuList::<Identity, OFFSET>,
            AddPrimaryExtensionItems: AddPrimaryExtensionItems::<Identity, OFFSET>,
            AddThirdPartyExtensionItems: AddThirdPartyExtensionItems::<Identity, OFFSET>,
            ShowContextMenu: ShowContextMenu::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextMenuProvider as windows_core::Interface>::IID || iid == &<IContextMenuCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IContextMenuProvider {}
windows_core::imp::define_interface!(IControlbar, IControlbar_Vtbl, 0x69fb811e_6c1c_11d0_a2cb_00c04fd909dd);
windows_core::imp::interface_hierarchy!(IControlbar, windows_core::IUnknown);
impl IControlbar {
    pub unsafe fn Create<P1>(&self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P1: windows_core::Param<IExtendControlbar>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), ntype, pextendcontrolbar.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Attach<P1>(&self, ntype: MMC_CONTROL_TYPE, lpunknown: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), ntype, lpunknown.param().abi()).ok() }
    }
    pub unsafe fn Detach<P0>(&self, lpunknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self), lpunknown.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlbar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONTROL_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONTROL_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IControlbar_Impl: windows_core::IUnknownImpl {
    fn Create(&self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: windows_core::Ref<IExtendControlbar>) -> windows_core::Result<windows_core::IUnknown>;
    fn Attach(&self, ntype: MMC_CONTROL_TYPE, lpunknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Detach(&self, lpunknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IControlbar_Vtbl {
    pub const fn new<Identity: IControlbar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IControlbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: *mut core::ffi::c_void, ppunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlbar_Impl::Create(this, core::mem::transmute_copy(&ntype), core::mem::transmute_copy(&pextendcontrolbar)) {
                    Ok(ok__) => {
                        ppunknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Attach<Identity: IControlbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlbar_Impl::Attach(this, core::mem::transmute_copy(&ntype), core::mem::transmute_copy(&lpunknown)).into()
            }
        }
        unsafe extern "system" fn Detach<Identity: IControlbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlbar_Impl::Detach(this, core::mem::transmute_copy(&lpunknown)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlbar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IControlbar {}
windows_core::imp::define_interface!(IDisplayHelp, IDisplayHelp_Vtbl, 0xcc593830_b926_11d1_8063_0000f875a9ce);
windows_core::imp::interface_hierarchy!(IDisplayHelp, windows_core::IUnknown);
impl IDisplayHelp {
    pub unsafe fn ShowTopic<P0>(&self, pszhelptopic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShowTopic)(windows_core::Interface::as_raw(self), pszhelptopic.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayHelp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShowTopic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IDisplayHelp_Impl: windows_core::IUnknownImpl {
    fn ShowTopic(&self, pszhelptopic: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IDisplayHelp_Vtbl {
    pub const fn new<Identity: IDisplayHelp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowTopic<Identity: IDisplayHelp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelptopic: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDisplayHelp_Impl::ShowTopic(this, core::mem::transmute(&pszhelptopic)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowTopic: ShowTopic::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDisplayHelp as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDisplayHelp {}
windows_core::imp::define_interface!(IEnumTASK, IEnumTASK_Vtbl, 0x338698b1_5a02_11d1_9fec_00600832db4a);
windows_core::imp::interface_hierarchy!(IEnumTASK, windows_core::IUnknown);
impl IEnumTASK {
    pub unsafe fn Next(&self, rgelt: &mut [MMC_TASK], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTASK> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTASK_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MMC_TASK, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumTASK_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumTASK>;
}
impl IEnumTASK_Vtbl {
    pub const fn new<Identity: IEnumTASK_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumTASK_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTASK_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTASK_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTASK_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTASK_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTASK_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumTASK_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTASK_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTASK as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTASK {}
windows_core::imp::define_interface!(IExtendContextMenu, IExtendContextMenu_Vtbl, 0x4f3b7a4f_cfac_11cf_b8e3_00c04fd8d5b0);
windows_core::imp::interface_hierarchy!(IExtendContextMenu, windows_core::IUnknown);
impl IExtendContextMenu {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddMenuItems<P0, P1>(&self, pidataobject: P0, picallback: P1, pinsertionallowed: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<IContextMenuCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMenuItems)(windows_core::Interface::as_raw(self), pidataobject.param().abi(), picallback.param().abi(), pinsertionallowed as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Command<P1>(&self, lcommandid: i32, pidataobject: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).Command)(windows_core::Interface::as_raw(self), lcommandid, pidataobject.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendContextMenu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddMenuItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddMenuItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Command: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendContextMenu_Impl: windows_core::IUnknownImpl {
    fn AddMenuItems(&self, pidataobject: windows_core::Ref<super::Com::IDataObject>, picallback: windows_core::Ref<IContextMenuCallback>, pinsertionallowed: *mut i32) -> windows_core::Result<()>;
    fn Command(&self, lcommandid: i32, pidataobject: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendContextMenu_Vtbl {
    pub const fn new<Identity: IExtendContextMenu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddMenuItems<Identity: IExtendContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidataobject: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void, pinsertionallowed: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendContextMenu_Impl::AddMenuItems(this, core::mem::transmute_copy(&pidataobject), core::mem::transmute_copy(&picallback), core::mem::transmute_copy(&pinsertionallowed)).into()
            }
        }
        unsafe extern "system" fn Command<Identity: IExtendContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcommandid: i32, pidataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendContextMenu_Impl::Command(this, core::mem::transmute_copy(&lcommandid), core::mem::transmute_copy(&pidataobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMenuItems: AddMenuItems::<Identity, OFFSET>,
            Command: Command::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtendContextMenu as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IExtendContextMenu {}
windows_core::imp::define_interface!(IExtendControlbar, IExtendControlbar_Vtbl, 0x49506520_6f40_11d0_a98b_00c04fd8d565);
windows_core::imp::interface_hierarchy!(IExtendControlbar, windows_core::IUnknown);
impl IExtendControlbar {
    pub unsafe fn SetControlbar<P0>(&self, pcontrolbar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IControlbar>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetControlbar)(windows_core::Interface::as_raw(self), pcontrolbar.param().abi()).ok() }
    }
    pub unsafe fn ControlbarNotify(&self, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ControlbarNotify)(windows_core::Interface::as_raw(self), event, arg, param2).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendControlbar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetControlbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlbarNotify: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_NOTIFY_TYPE, super::super::Foundation::LPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
pub trait IExtendControlbar_Impl: windows_core::IUnknownImpl {
    fn SetControlbar(&self, pcontrolbar: windows_core::Ref<IControlbar>) -> windows_core::Result<()>;
    fn ControlbarNotify(&self, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
impl IExtendControlbar_Vtbl {
    pub const fn new<Identity: IExtendControlbar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetControlbar<Identity: IExtendControlbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontrolbar: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendControlbar_Impl::SetControlbar(this, core::mem::transmute_copy(&pcontrolbar)).into()
            }
        }
        unsafe extern "system" fn ControlbarNotify<Identity: IExtendControlbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendControlbar_Impl::ControlbarNotify(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&arg), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetControlbar: SetControlbar::<Identity, OFFSET>,
            ControlbarNotify: ControlbarNotify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtendControlbar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IExtendControlbar {}
windows_core::imp::define_interface!(IExtendPropertySheet, IExtendPropertySheet_Vtbl, 0x85de64dc_ef21_11cf_a285_00c04fd8dbe6);
windows_core::imp::interface_hierarchy!(IExtendPropertySheet, windows_core::IUnknown);
impl IExtendPropertySheet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<P0, P2>(&self, lpprovider: P0, handle: isize, lpidataobject: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPropertySheetCallback>,
        P2: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePropertyPages)(windows_core::Interface::as_raw(self), lpprovider.param().abi(), handle, lpidataobject.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<P0>(&self, lpdataobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryPagesFor)(windows_core::Interface::as_raw(self), lpdataobject.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendPropertySheet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyPages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryPagesFor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryPagesFor: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendPropertySheet_Impl: windows_core::IUnknownImpl {
    fn CreatePropertyPages(&self, lpprovider: windows_core::Ref<IPropertySheetCallback>, handle: isize, lpidataobject: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
    fn QueryPagesFor(&self, lpdataobject: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendPropertySheet_Vtbl {
    pub const fn new<Identity: IExtendPropertySheet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePropertyPages<Identity: IExtendPropertySheet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpprovider: *mut core::ffi::c_void, handle: isize, lpidataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendPropertySheet_Impl::CreatePropertyPages(this, core::mem::transmute_copy(&lpprovider), core::mem::transmute_copy(&handle), core::mem::transmute_copy(&lpidataobject)).into()
            }
        }
        unsafe extern "system" fn QueryPagesFor<Identity: IExtendPropertySheet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendPropertySheet_Impl::QueryPagesFor(this, core::mem::transmute_copy(&lpdataobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyPages: CreatePropertyPages::<Identity, OFFSET>,
            QueryPagesFor: QueryPagesFor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtendPropertySheet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IExtendPropertySheet {}
windows_core::imp::define_interface!(IExtendPropertySheet2, IExtendPropertySheet2_Vtbl, 0xb7a87232_4a51_11d1_a7ea_00c04fd909dd);
impl core::ops::Deref for IExtendPropertySheet2 {
    type Target = IExtendPropertySheet;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExtendPropertySheet2, windows_core::IUnknown, IExtendPropertySheet);
impl IExtendPropertySheet2 {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetWatermarks<P0>(&self, lpidataobject: P0, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetWatermarks)(windows_core::Interface::as_raw(self), lpidataobject.param().abi(), lphwatermark as _, lphheader as _, lphpalette as _, bstretch as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendPropertySheet2_Vtbl {
    pub base__: IExtendPropertySheet_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetWatermarks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HPALETTE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetWatermarks: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IExtendPropertySheet2_Impl: IExtendPropertySheet_Impl {
    fn GetWatermarks(&self, lpidataobject: windows_core::Ref<super::Com::IDataObject>, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IExtendPropertySheet2_Vtbl {
    pub const fn new<Identity: IExtendPropertySheet2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWatermarks<Identity: IExtendPropertySheet2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpidataobject: *mut core::ffi::c_void, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendPropertySheet2_Impl::GetWatermarks(this, core::mem::transmute_copy(&lpidataobject), core::mem::transmute_copy(&lphwatermark), core::mem::transmute_copy(&lphheader), core::mem::transmute_copy(&lphpalette), core::mem::transmute_copy(&bstretch)).into()
            }
        }
        Self { base__: IExtendPropertySheet_Vtbl::new::<Identity, OFFSET>(), GetWatermarks: GetWatermarks::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtendPropertySheet2 as windows_core::Interface>::IID || iid == &<IExtendPropertySheet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IExtendPropertySheet2 {}
windows_core::imp::define_interface!(IExtendTaskPad, IExtendTaskPad_Vtbl, 0x8dee6511_554d_11d1_9fea_00600832db4a);
windows_core::imp::interface_hierarchy!(IExtendTaskPad, windows_core::IUnknown);
impl IExtendTaskPad {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TaskNotify<P0>(&self, pdo: P0, arg: *const super::Variant::VARIANT, param2: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).TaskNotify)(windows_core::Interface::as_raw(self), pdo.param().abi(), core::mem::transmute(arg), core::mem::transmute(param2)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTasks<P0, P1>(&self, pdo: P0, sztaskgroup: P1) -> windows_core::Result<IEnumTASK>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumTasks)(windows_core::Interface::as_raw(self), pdo.param().abi(), sztaskgroup.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTitle<P0>(&self, pszgroup: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTitle)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescriptiveText<P0>(&self, pszgroup: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescriptiveText)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBackground<P0>(&self, pszgroup: P0) -> windows_core::Result<MMC_TASK_DISPLAY_OBJECT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackground)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetListPadInfo<P0>(&self, pszgroup: P0) -> windows_core::Result<MMC_LISTPAD_INFO>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListPadInfo)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendTaskPad_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TaskNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TaskNotify: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTasks: usize,
    pub GetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDescriptiveText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetBackground: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut MMC_TASK_DISPLAY_OBJECT) -> windows_core::HRESULT,
    pub GetListPadInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut MMC_LISTPAD_INFO) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IExtendTaskPad_Impl: windows_core::IUnknownImpl {
    fn TaskNotify(&self, pdo: windows_core::Ref<super::Com::IDataObject>, arg: *const super::Variant::VARIANT, param2: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn EnumTasks(&self, pdo: windows_core::Ref<super::Com::IDataObject>, sztaskgroup: &windows_core::PCWSTR) -> windows_core::Result<IEnumTASK>;
    fn GetTitle(&self, pszgroup: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDescriptiveText(&self, pszgroup: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn GetBackground(&self, pszgroup: &windows_core::PCWSTR) -> windows_core::Result<MMC_TASK_DISPLAY_OBJECT>;
    fn GetListPadInfo(&self, pszgroup: &windows_core::PCWSTR) -> windows_core::Result<MMC_LISTPAD_INFO>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IExtendTaskPad_Vtbl {
    pub const fn new<Identity: IExtendTaskPad_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TaskNotify<Identity: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdo: *mut core::ffi::c_void, arg: *const super::Variant::VARIANT, param2: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendTaskPad_Impl::TaskNotify(this, core::mem::transmute_copy(&pdo), core::mem::transmute_copy(&arg), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumTasks<Identity: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdo: *mut core::ffi::c_void, sztaskgroup: windows_core::PCWSTR, ppenumtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExtendTaskPad_Impl::EnumTasks(this, core::mem::transmute_copy(&pdo), core::mem::transmute(&sztaskgroup)) {
                    Ok(ok__) => {
                        ppenumtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTitle<Identity: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgroup: windows_core::PCWSTR, psztitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExtendTaskPad_Impl::GetTitle(this, core::mem::transmute(&pszgroup)) {
                    Ok(ok__) => {
                        psztitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescriptiveText<Identity: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgroup: windows_core::PCWSTR, pszdescriptivetext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExtendTaskPad_Impl::GetDescriptiveText(this, core::mem::transmute(&pszgroup)) {
                    Ok(ok__) => {
                        pszdescriptivetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackground<Identity: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgroup: windows_core::PCWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExtendTaskPad_Impl::GetBackground(this, core::mem::transmute(&pszgroup)) {
                    Ok(ok__) => {
                        ptdo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetListPadInfo<Identity: IExtendTaskPad_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgroup: windows_core::PCWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExtendTaskPad_Impl::GetListPadInfo(this, core::mem::transmute(&pszgroup)) {
                    Ok(ok__) => {
                        lplistpadinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TaskNotify: TaskNotify::<Identity, OFFSET>,
            EnumTasks: EnumTasks::<Identity, OFFSET>,
            GetTitle: GetTitle::<Identity, OFFSET>,
            GetDescriptiveText: GetDescriptiveText::<Identity, OFFSET>,
            GetBackground: GetBackground::<Identity, OFFSET>,
            GetListPadInfo: GetListPadInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtendTaskPad as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IExtendTaskPad {}
windows_core::imp::define_interface!(IExtendView, IExtendView_Vtbl, 0x89995cee_d2ed_4c0e_ae5e_df7e76f3fa53);
windows_core::imp::interface_hierarchy!(IExtendView, windows_core::IUnknown);
impl IExtendView {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetViews<P0, P1>(&self, pdataobject: P0, pviewextensioncallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<IViewExtensionCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetViews)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), pviewextensioncallback.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetViews: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendView_Impl: windows_core::IUnknownImpl {
    fn GetViews(&self, pdataobject: windows_core::Ref<super::Com::IDataObject>, pviewextensioncallback: windows_core::Ref<IViewExtensionCallback>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IExtendView_Vtbl {
    pub const fn new<Identity: IExtendView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetViews<Identity: IExtendView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, pviewextensioncallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtendView_Impl::GetViews(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pviewextensioncallback)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetViews: GetViews::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtendView as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IExtendView {}
windows_core::imp::define_interface!(IHeaderCtrl, IHeaderCtrl_Vtbl, 0x43136eb3_d36c_11cf_adbc_00aa00a80033);
windows_core::imp::interface_hierarchy!(IHeaderCtrl, windows_core::IUnknown);
impl IHeaderCtrl {
    pub unsafe fn InsertColumn<P1>(&self, ncol: i32, title: P1, nformat: i32, nwidth: i32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertColumn)(windows_core::Interface::as_raw(self), ncol, title.param().abi(), nformat, nwidth).ok() }
    }
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteColumn)(windows_core::Interface::as_raw(self), ncol).ok() }
    }
    pub unsafe fn SetColumnText<P1>(&self, ncol: i32, title: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetColumnText)(windows_core::Interface::as_raw(self), ncol, title.param().abi()).ok() }
    }
    pub unsafe fn GetColumnText(&self, ncol: i32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnText)(windows_core::Interface::as_raw(self), ncol, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColumnWidth)(windows_core::Interface::as_raw(self), ncol, nwidth).ok() }
    }
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnWidth)(windows_core::Interface::as_raw(self), ncol, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderCtrl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InsertColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, i32, i32) -> windows_core::HRESULT,
    pub DeleteColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetColumnText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetColumnText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetColumnWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetColumnWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IHeaderCtrl_Impl: windows_core::IUnknownImpl {
    fn InsertColumn(&self, ncol: i32, title: &windows_core::PCWSTR, nformat: i32, nwidth: i32) -> windows_core::Result<()>;
    fn DeleteColumn(&self, ncol: i32) -> windows_core::Result<()>;
    fn SetColumnText(&self, ncol: i32, title: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetColumnText(&self, ncol: i32) -> windows_core::Result<windows_core::PWSTR>;
    fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> windows_core::Result<()>;
    fn GetColumnWidth(&self, ncol: i32) -> windows_core::Result<i32>;
}
impl IHeaderCtrl_Vtbl {
    pub const fn new<Identity: IHeaderCtrl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertColumn<Identity: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncol: i32, title: windows_core::PCWSTR, nformat: i32, nwidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderCtrl_Impl::InsertColumn(this, core::mem::transmute_copy(&ncol), core::mem::transmute(&title), core::mem::transmute_copy(&nformat), core::mem::transmute_copy(&nwidth)).into()
            }
        }
        unsafe extern "system" fn DeleteColumn<Identity: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderCtrl_Impl::DeleteColumn(this, core::mem::transmute_copy(&ncol)).into()
            }
        }
        unsafe extern "system" fn SetColumnText<Identity: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncol: i32, title: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderCtrl_Impl::SetColumnText(this, core::mem::transmute_copy(&ncol), core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn GetColumnText<Identity: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncol: i32, ptext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderCtrl_Impl::GetColumnText(this, core::mem::transmute_copy(&ncol)) {
                    Ok(ok__) => {
                        ptext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColumnWidth<Identity: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncol: i32, nwidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderCtrl_Impl::SetColumnWidth(this, core::mem::transmute_copy(&ncol), core::mem::transmute_copy(&nwidth)).into()
            }
        }
        unsafe extern "system" fn GetColumnWidth<Identity: IHeaderCtrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderCtrl_Impl::GetColumnWidth(this, core::mem::transmute_copy(&ncol)) {
                    Ok(ok__) => {
                        pwidth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertColumn: InsertColumn::<Identity, OFFSET>,
            DeleteColumn: DeleteColumn::<Identity, OFFSET>,
            SetColumnText: SetColumnText::<Identity, OFFSET>,
            GetColumnText: GetColumnText::<Identity, OFFSET>,
            SetColumnWidth: SetColumnWidth::<Identity, OFFSET>,
            GetColumnWidth: GetColumnWidth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHeaderCtrl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHeaderCtrl {}
windows_core::imp::define_interface!(IHeaderCtrl2, IHeaderCtrl2_Vtbl, 0x9757abb8_1b32_11d1_a7ce_00c04fd8d565);
impl core::ops::Deref for IHeaderCtrl2 {
    type Target = IHeaderCtrl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHeaderCtrl2, windows_core::IUnknown, IHeaderCtrl);
impl IHeaderCtrl2 {
    pub unsafe fn SetChangeTimeOut(&self, utimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChangeTimeOut)(windows_core::Interface::as_raw(self), utimeout).ok() }
    }
    pub unsafe fn SetColumnFilter(&self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColumnFilter)(windows_core::Interface::as_raw(self), ncolumn, dwtype, pfilterdata).ok() }
    }
    pub unsafe fn GetColumnFilter(&self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetColumnFilter)(windows_core::Interface::as_raw(self), ncolumn, pdwtype as _, pfilterdata as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderCtrl2_Vtbl {
    pub base__: IHeaderCtrl_Vtbl,
    pub SetChangeTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetColumnFilter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const MMC_FILTERDATA) -> windows_core::HRESULT,
    pub GetColumnFilter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut MMC_FILTERDATA) -> windows_core::HRESULT,
}
pub trait IHeaderCtrl2_Impl: IHeaderCtrl_Impl {
    fn SetChangeTimeOut(&self, utimeout: u32) -> windows_core::Result<()>;
    fn SetColumnFilter(&self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> windows_core::Result<()>;
    fn GetColumnFilter(&self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> windows_core::Result<()>;
}
impl IHeaderCtrl2_Vtbl {
    pub const fn new<Identity: IHeaderCtrl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetChangeTimeOut<Identity: IHeaderCtrl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, utimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderCtrl2_Impl::SetChangeTimeOut(this, core::mem::transmute_copy(&utimeout)).into()
            }
        }
        unsafe extern "system" fn SetColumnFilter<Identity: IHeaderCtrl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderCtrl2_Impl::SetColumnFilter(this, core::mem::transmute_copy(&ncolumn), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pfilterdata)).into()
            }
        }
        unsafe extern "system" fn GetColumnFilter<Identity: IHeaderCtrl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderCtrl2_Impl::GetColumnFilter(this, core::mem::transmute_copy(&ncolumn), core::mem::transmute_copy(&pdwtype), core::mem::transmute_copy(&pfilterdata)).into()
            }
        }
        Self {
            base__: IHeaderCtrl_Vtbl::new::<Identity, OFFSET>(),
            SetChangeTimeOut: SetChangeTimeOut::<Identity, OFFSET>,
            SetColumnFilter: SetColumnFilter::<Identity, OFFSET>,
            GetColumnFilter: GetColumnFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHeaderCtrl2 as windows_core::Interface>::IID || iid == &<IHeaderCtrl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHeaderCtrl2 {}
windows_core::imp::define_interface!(IImageList, IImageList_Vtbl, 0x43136eb8_d36c_11cf_adbc_00aa00a80033);
windows_core::imp::interface_hierarchy!(IImageList, windows_core::IUnknown);
impl IImageList {
    pub unsafe fn ImageListSetIcon(&self, picon: *const isize, nloc: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImageListSetIcon)(windows_core::Interface::as_raw(self), picon, nloc).ok() }
    }
    pub unsafe fn ImageListSetStrip(&self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImageListSetStrip)(windows_core::Interface::as_raw(self), pbmapsm, pbmaplg, nstartloc, cmask).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ImageListSetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *const isize, i32) -> windows_core::HRESULT,
    pub ImageListSetStrip: unsafe extern "system" fn(*mut core::ffi::c_void, *const isize, *const isize, i32, super::super::Foundation::COLORREF) -> windows_core::HRESULT,
}
pub trait IImageList_Impl: windows_core::IUnknownImpl {
    fn ImageListSetIcon(&self, picon: *const isize, nloc: i32) -> windows_core::Result<()>;
    fn ImageListSetStrip(&self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> windows_core::Result<()>;
}
impl IImageList_Vtbl {
    pub const fn new<Identity: IImageList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImageListSetIcon<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, picon: *const isize, nloc: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::ImageListSetIcon(this, core::mem::transmute_copy(&picon), core::mem::transmute_copy(&nloc)).into()
            }
        }
        unsafe extern "system" fn ImageListSetStrip<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::ImageListSetStrip(this, core::mem::transmute_copy(&pbmapsm), core::mem::transmute_copy(&pbmaplg), core::mem::transmute_copy(&nstartloc), core::mem::transmute_copy(&cmask)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ImageListSetIcon: ImageListSetIcon::<Identity, OFFSET>,
            ImageListSetStrip: ImageListSetStrip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IImageList {}
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
windows_core::imp::define_interface!(IMMCVersionInfo, IMMCVersionInfo_Vtbl, 0xa8d2c5fe_cdcb_4b9d_bde5_a27343ff54bc);
windows_core::imp::interface_hierarchy!(IMMCVersionInfo, windows_core::IUnknown);
impl IMMCVersionInfo {
    pub unsafe fn GetMMCVersion(&self, pversionmajor: *mut i32, pversionminor: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMMCVersion)(windows_core::Interface::as_raw(self), pversionmajor as _, pversionminor as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMCVersionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMMCVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IMMCVersionInfo_Impl: windows_core::IUnknownImpl {
    fn GetMMCVersion(&self, pversionmajor: *mut i32, pversionminor: *mut i32) -> windows_core::Result<()>;
}
impl IMMCVersionInfo_Vtbl {
    pub const fn new<Identity: IMMCVersionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMMCVersion<Identity: IMMCVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMCVersionInfo_Impl::GetMMCVersion(this, core::mem::transmute_copy(&pversionmajor), core::mem::transmute_copy(&pversionminor)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMMCVersion: GetMMCVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMCVersionInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMCVersionInfo {}
windows_core::imp::define_interface!(IMenuButton, IMenuButton_Vtbl, 0x951ed750_d080_11d0_b197_000000000000);
windows_core::imp::interface_hierarchy!(IMenuButton, windows_core::IUnknown);
impl IMenuButton {
    pub unsafe fn AddButton<P1, P2>(&self, idcommand: i32, lpbuttontext: P1, lptooltiptext: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddButton)(windows_core::Interface::as_raw(self), idcommand, lpbuttontext.param().abi(), lptooltiptext.param().abi()).ok() }
    }
    pub unsafe fn SetButton<P1, P2>(&self, idcommand: i32, lpbuttontext: P1, lptooltiptext: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetButton)(windows_core::Interface::as_raw(self), idcommand, lpbuttontext.param().abi(), lptooltiptext.param().abi()).ok() }
    }
    pub unsafe fn SetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetButtonState)(windows_core::Interface::as_raw(self), idcommand, nstate, bstate.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuButton_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, MMC_BUTTON_STATE, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IMenuButton_Impl: windows_core::IUnknownImpl {
    fn AddButton(&self, idcommand: i32, lpbuttontext: &windows_core::PCWSTR, lptooltiptext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetButton(&self, idcommand: i32, lpbuttontext: &windows_core::PCWSTR, lptooltiptext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IMenuButton_Vtbl {
    pub const fn new<Identity: IMenuButton_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddButton<Identity: IMenuButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idcommand: i32, lpbuttontext: windows_core::PCWSTR, lptooltiptext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMenuButton_Impl::AddButton(this, core::mem::transmute_copy(&idcommand), core::mem::transmute(&lpbuttontext), core::mem::transmute(&lptooltiptext)).into()
            }
        }
        unsafe extern "system" fn SetButton<Identity: IMenuButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idcommand: i32, lpbuttontext: windows_core::PCWSTR, lptooltiptext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMenuButton_Impl::SetButton(this, core::mem::transmute_copy(&idcommand), core::mem::transmute(&lpbuttontext), core::mem::transmute(&lptooltiptext)).into()
            }
        }
        unsafe extern "system" fn SetButtonState<Identity: IMenuButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMenuButton_Impl::SetButtonState(this, core::mem::transmute_copy(&idcommand), core::mem::transmute_copy(&nstate), core::mem::transmute_copy(&bstate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddButton: AddButton::<Identity, OFFSET>,
            SetButton: SetButton::<Identity, OFFSET>,
            SetButtonState: SetButtonState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMenuButton as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMenuButton {}
windows_core::imp::define_interface!(IMessageView, IMessageView_Vtbl, 0x80f94174_fccc_11d2_b991_00c04f8ecd78);
windows_core::imp::interface_hierarchy!(IMessageView, windows_core::IUnknown);
impl IMessageView {
    pub unsafe fn SetTitleText<P0>(&self, psztitletext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitleText)(windows_core::Interface::as_raw(self), psztitletext.param().abi()).ok() }
    }
    pub unsafe fn SetBodyText<P0>(&self, pszbodytext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBodyText)(windows_core::Interface::as_raw(self), pszbodytext.param().abi()).ok() }
    }
    pub unsafe fn SetIcon(&self, id: IconIdentifier) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIcon)(windows_core::Interface::as_raw(self), id).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTitleText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetBodyText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, IconIdentifier) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMessageView_Impl: windows_core::IUnknownImpl {
    fn SetTitleText(&self, psztitletext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBodyText(&self, pszbodytext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetIcon(&self, id: IconIdentifier) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl IMessageView_Vtbl {
    pub const fn new<Identity: IMessageView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTitleText<Identity: IMessageView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitletext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageView_Impl::SetTitleText(this, core::mem::transmute(&psztitletext)).into()
            }
        }
        unsafe extern "system" fn SetBodyText<Identity: IMessageView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbodytext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageView_Impl::SetBodyText(this, core::mem::transmute(&pszbodytext)).into()
            }
        }
        unsafe extern "system" fn SetIcon<Identity: IMessageView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: IconIdentifier) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageView_Impl::SetIcon(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IMessageView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageView_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTitleText: SetTitleText::<Identity, OFFSET>,
            SetBodyText: SetBodyText::<Identity, OFFSET>,
            SetIcon: SetIcon::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessageView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMessageView {}
pub const INDETERMINATE: MMC_BUTTON_STATE = MMC_BUTTON_STATE(8i32);
windows_core::imp::define_interface!(INodeProperties, INodeProperties_Vtbl, 0x15bc4d24_a522_4406_aa55_0749537a6865);
windows_core::imp::interface_hierarchy!(INodeProperties, windows_core::IUnknown);
impl INodeProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperty<P0>(&self, pdataobject: P0, szpropertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), core::mem::transmute_copy(szpropertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INodeProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProperty: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait INodeProperties_Impl: windows_core::IUnknownImpl {
    fn GetProperty(&self, pdataobject: windows_core::Ref<super::Com::IDataObject>, szpropertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl INodeProperties_Vtbl {
    pub const fn new<Identity: INodeProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: INodeProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, szpropertyname: *mut core::ffi::c_void, pbstrproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INodeProperties_Impl::GetProperty(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute(&szpropertyname)) {
                    Ok(ok__) => {
                        pbstrproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperty: GetProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INodeProperties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INodeProperties {}
windows_core::imp::define_interface!(IPropertySheetCallback, IPropertySheetCallback_Vtbl, 0x85de64dd_ef21_11cf_a285_00c04fd8dbe6);
windows_core::imp::interface_hierarchy!(IPropertySheetCallback, windows_core::IUnknown);
impl IPropertySheetCallback {
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPage(&self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), hpage).ok() }
    }
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn RemovePage(&self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemovePage)(windows_core::Interface::as_raw(self), hpage).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySheetCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Controls")]
    pub AddPage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    AddPage: usize,
    #[cfg(feature = "Win32_UI_Controls")]
    pub RemovePage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    RemovePage: usize,
}
#[cfg(feature = "Win32_UI_Controls")]
pub trait IPropertySheetCallback_Impl: windows_core::IUnknownImpl {
    fn AddPage(&self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::Result<()>;
    fn RemovePage(&self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Controls")]
impl IPropertySheetCallback_Vtbl {
    pub const fn new<Identity: IPropertySheetCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPage<Identity: IPropertySheetCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySheetCallback_Impl::AddPage(this, core::mem::transmute_copy(&hpage)).into()
            }
        }
        unsafe extern "system" fn RemovePage<Identity: IPropertySheetCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySheetCallback_Impl::RemovePage(this, core::mem::transmute_copy(&hpage)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPage: AddPage::<Identity, OFFSET>, RemovePage: RemovePage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySheetCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Controls")]
impl windows_core::RuntimeName for IPropertySheetCallback {}
windows_core::imp::define_interface!(IPropertySheetProvider, IPropertySheetProvider_Vtbl, 0x85de64de_ef21_11cf_a285_00c04fd8dbe6);
windows_core::imp::interface_hierarchy!(IPropertySheetProvider, windows_core::IUnknown);
impl IPropertySheetProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertySheet<P0, P3>(&self, title: P0, r#type: u8, cookie: isize, pidataobjectm: P3, dwoptions: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePropertySheet)(windows_core::Interface::as_raw(self), title.param().abi(), r#type, cookie, pidataobjectm.param().abi(), dwoptions).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindPropertySheet<P1, P2>(&self, hitem: isize, lpcomponent: P1, lpdataobject: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IComponent>,
        P2: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindPropertySheet)(windows_core::Interface::as_raw(self), hitem, lpcomponent.param().abi(), lpdataobject.param().abi()).ok() }
    }
    pub unsafe fn AddPrimaryPages<P0>(&self, lpunknown: P0, bcreatehandle: bool, hnotifywindow: super::super::Foundation::HWND, bscopepane: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPrimaryPages)(windows_core::Interface::as_raw(self), lpunknown.param().abi(), bcreatehandle.into(), hnotifywindow, bscopepane.into()).ok() }
    }
    pub unsafe fn AddExtensionPages(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddExtensionPages)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Show(&self, window: isize, page: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), window, page).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySheetProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertySheet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u8, isize, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertySheet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FindPropertySheet: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindPropertySheet: usize,
    pub AddPrimaryPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, super::super::Foundation::HWND, windows_core::BOOL) -> windows_core::HRESULT,
    pub AddExtensionPages: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, isize, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPropertySheetProvider_Impl: windows_core::IUnknownImpl {
    fn CreatePropertySheet(&self, title: &windows_core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: windows_core::Ref<super::Com::IDataObject>, dwoptions: u32) -> windows_core::Result<()>;
    fn FindPropertySheet(&self, hitem: isize, lpcomponent: windows_core::Ref<IComponent>, lpdataobject: windows_core::Ref<super::Com::IDataObject>) -> windows_core::Result<()>;
    fn AddPrimaryPages(&self, lpunknown: windows_core::Ref<windows_core::IUnknown>, bcreatehandle: windows_core::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: windows_core::BOOL) -> windows_core::Result<()>;
    fn AddExtensionPages(&self) -> windows_core::Result<()>;
    fn Show(&self, window: isize, page: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPropertySheetProvider_Vtbl {
    pub const fn new<Identity: IPropertySheetProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePropertySheet<Identity: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: windows_core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: *mut core::ffi::c_void, dwoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySheetProvider_Impl::CreatePropertySheet(this, core::mem::transmute(&title), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&cookie), core::mem::transmute_copy(&pidataobjectm), core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        unsafe extern "system" fn FindPropertySheet<Identity: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hitem: isize, lpcomponent: *mut core::ffi::c_void, lpdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySheetProvider_Impl::FindPropertySheet(this, core::mem::transmute_copy(&hitem), core::mem::transmute_copy(&lpcomponent), core::mem::transmute_copy(&lpdataobject)).into()
            }
        }
        unsafe extern "system" fn AddPrimaryPages<Identity: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpunknown: *mut core::ffi::c_void, bcreatehandle: windows_core::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySheetProvider_Impl::AddPrimaryPages(this, core::mem::transmute_copy(&lpunknown), core::mem::transmute_copy(&bcreatehandle), core::mem::transmute_copy(&hnotifywindow), core::mem::transmute_copy(&bscopepane)).into()
            }
        }
        unsafe extern "system" fn AddExtensionPages<Identity: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySheetProvider_Impl::AddExtensionPages(this).into()
            }
        }
        unsafe extern "system" fn Show<Identity: IPropertySheetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: isize, page: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySheetProvider_Impl::Show(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&page)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertySheet: CreatePropertySheet::<Identity, OFFSET>,
            FindPropertySheet: FindPropertySheet::<Identity, OFFSET>,
            AddPrimaryPages: AddPrimaryPages::<Identity, OFFSET>,
            AddExtensionPages: AddExtensionPages::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySheetProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPropertySheetProvider {}
windows_core::imp::define_interface!(IRequiredExtensions, IRequiredExtensions_Vtbl, 0x72782d7a_a4a0_11d1_af0f_00c04fb6dd2c);
windows_core::imp::interface_hierarchy!(IRequiredExtensions, windows_core::IUnknown);
impl IRequiredExtensions {
    pub unsafe fn EnableAllExtensions(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableAllExtensions)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetFirstExtension(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNextExtension(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequiredExtensions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableAllExtensions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetNextExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IRequiredExtensions_Impl: windows_core::IUnknownImpl {
    fn EnableAllExtensions(&self) -> windows_core::Result<()>;
    fn GetFirstExtension(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetNextExtension(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IRequiredExtensions_Vtbl {
    pub const fn new<Identity: IRequiredExtensions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableAllExtensions<Identity: IRequiredExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRequiredExtensions_Impl::EnableAllExtensions(this).into()
            }
        }
        unsafe extern "system" fn GetFirstExtension<Identity: IRequiredExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRequiredExtensions_Impl::GetFirstExtension(this) {
                    Ok(ok__) => {
                        pextclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextExtension<Identity: IRequiredExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRequiredExtensions_Impl::GetNextExtension(this) {
                    Ok(ok__) => {
                        pextclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableAllExtensions: EnableAllExtensions::<Identity, OFFSET>,
            GetFirstExtension: GetFirstExtension::<Identity, OFFSET>,
            GetNextExtension: GetNextExtension::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRequiredExtensions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRequiredExtensions {}
windows_core::imp::define_interface!(IResultData, IResultData_Vtbl, 0x31da5fa0_e0eb_11cf_9f21_00aa003ca9f6);
windows_core::imp::interface_hierarchy!(IResultData, windows_core::IUnknown);
impl IResultData {
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertItem)(windows_core::Interface::as_raw(self), item as _).ok() }
    }
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), itemid, ncol).ok() }
    }
    pub unsafe fn FindItemByLParam(&self, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<isize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByLParam)(windows_core::Interface::as_raw(self), lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeleteAllRsltItems(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAllRsltItems)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetItem)(windows_core::Interface::as_raw(self), item).ok() }
    }
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), item as _).ok() }
    }
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNextItem)(windows_core::Interface::as_raw(self), item as _).ok() }
    }
    pub unsafe fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModifyItemState)(windows_core::Interface::as_raw(self), nindex, itemid, uadd, uremove).ok() }
    }
    pub unsafe fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModifyViewStyle)(windows_core::Interface::as_raw(self), add, remove).ok() }
    }
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetViewMode)(windows_core::Interface::as_raw(self), lviewmode).ok() }
    }
    pub unsafe fn GetViewMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateItem(&self, itemid: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateItem)(windows_core::Interface::as_raw(self), itemid).ok() }
    }
    pub unsafe fn Sort(&self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Sort)(windows_core::Interface::as_raw(self), ncolumn, dwsortoptions, luserparam).ok() }
    }
    pub unsafe fn SetDescBarText<P0>(&self, desctext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDescBarText)(windows_core::Interface::as_raw(self), desctext.param().abi()).ok() }
    }
    pub unsafe fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetItemCount)(windows_core::Interface::as_raw(self), nitemcount, dwoptions).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InsertItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RESULTDATAITEM) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize, i32) -> windows_core::HRESULT,
    pub FindItemByLParam: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::LPARAM, *mut isize) -> windows_core::HRESULT,
    pub DeleteAllRsltItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const RESULTDATAITEM) -> windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RESULTDATAITEM) -> windows_core::HRESULT,
    pub GetNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RESULTDATAITEM) -> windows_core::HRESULT,
    pub ModifyItemState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, isize, u32, u32) -> windows_core::HRESULT,
    pub ModifyViewStyle: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_RESULT_VIEW_STYLE, MMC_RESULT_VIEW_STYLE) -> windows_core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UpdateItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub Sort: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub SetDescBarText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
}
pub trait IResultData_Impl: windows_core::IUnknownImpl {
    fn InsertItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()>;
    fn DeleteItem(&self, itemid: isize, ncol: i32) -> windows_core::Result<()>;
    fn FindItemByLParam(&self, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<isize>;
    fn DeleteAllRsltItems(&self) -> windows_core::Result<()>;
    fn SetItem(&self, item: *const RESULTDATAITEM) -> windows_core::Result<()>;
    fn GetItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()>;
    fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()>;
    fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> windows_core::Result<()>;
    fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> windows_core::Result<()>;
    fn SetViewMode(&self, lviewmode: i32) -> windows_core::Result<()>;
    fn GetViewMode(&self) -> windows_core::Result<i32>;
    fn UpdateItem(&self, itemid: isize) -> windows_core::Result<()>;
    fn Sort(&self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn SetDescBarText(&self, desctext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> windows_core::Result<()>;
}
impl IResultData_Vtbl {
    pub const fn new<Identity: IResultData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertItem<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut RESULTDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::InsertItem(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: isize, ncol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::DeleteItem(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&ncol)).into()
            }
        }
        unsafe extern "system" fn FindItemByLParam<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResultData_Impl::FindItemByLParam(this, core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pitemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteAllRsltItems<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::DeleteAllRsltItems(this).into()
            }
        }
        unsafe extern "system" fn SetItem<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *const RESULTDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::SetItem(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn GetItem<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut RESULTDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::GetItem(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn GetNextItem<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut RESULTDATAITEM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::GetNextItem(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn ModifyItemState<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::ModifyItemState(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&uadd), core::mem::transmute_copy(&uremove)).into()
            }
        }
        unsafe extern "system" fn ModifyViewStyle<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::ModifyViewStyle(this, core::mem::transmute_copy(&add), core::mem::transmute_copy(&remove)).into()
            }
        }
        unsafe extern "system" fn SetViewMode<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lviewmode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::SetViewMode(this, core::mem::transmute_copy(&lviewmode)).into()
            }
        }
        unsafe extern "system" fn GetViewMode<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lviewmode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResultData_Impl::GetViewMode(this) {
                    Ok(ok__) => {
                        lviewmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateItem<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::UpdateItem(this, core::mem::transmute_copy(&itemid)).into()
            }
        }
        unsafe extern "system" fn Sort<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::Sort(this, core::mem::transmute_copy(&ncolumn), core::mem::transmute_copy(&dwsortoptions), core::mem::transmute_copy(&luserparam)).into()
            }
        }
        unsafe extern "system" fn SetDescBarText<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desctext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::SetDescBarText(this, core::mem::transmute(&desctext)).into()
            }
        }
        unsafe extern "system" fn SetItemCount<Identity: IResultData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData_Impl::SetItemCount(this, core::mem::transmute_copy(&nitemcount), core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertItem: InsertItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            FindItemByLParam: FindItemByLParam::<Identity, OFFSET>,
            DeleteAllRsltItems: DeleteAllRsltItems::<Identity, OFFSET>,
            SetItem: SetItem::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            GetNextItem: GetNextItem::<Identity, OFFSET>,
            ModifyItemState: ModifyItemState::<Identity, OFFSET>,
            ModifyViewStyle: ModifyViewStyle::<Identity, OFFSET>,
            SetViewMode: SetViewMode::<Identity, OFFSET>,
            GetViewMode: GetViewMode::<Identity, OFFSET>,
            UpdateItem: UpdateItem::<Identity, OFFSET>,
            Sort: Sort::<Identity, OFFSET>,
            SetDescBarText: SetDescBarText::<Identity, OFFSET>,
            SetItemCount: SetItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResultData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResultData {}
windows_core::imp::define_interface!(IResultData2, IResultData2_Vtbl, 0x0f36e0eb_a7f1_4a81_be5a_9247f7de4b1b);
impl core::ops::Deref for IResultData2 {
    type Target = IResultData;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResultData2, windows_core::IUnknown, IResultData);
impl IResultData2 {
    pub unsafe fn RenameResultItem(&self, itemid: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RenameResultItem)(windows_core::Interface::as_raw(self), itemid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultData2_Vtbl {
    pub base__: IResultData_Vtbl,
    pub RenameResultItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
}
pub trait IResultData2_Impl: IResultData_Impl {
    fn RenameResultItem(&self, itemid: isize) -> windows_core::Result<()>;
}
impl IResultData2_Vtbl {
    pub const fn new<Identity: IResultData2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RenameResultItem<Identity: IResultData2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultData2_Impl::RenameResultItem(this, core::mem::transmute_copy(&itemid)).into()
            }
        }
        Self { base__: IResultData_Vtbl::new::<Identity, OFFSET>(), RenameResultItem: RenameResultItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResultData2 as windows_core::Interface>::IID || iid == &<IResultData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResultData2 {}
windows_core::imp::define_interface!(IResultDataCompare, IResultDataCompare_Vtbl, 0xe8315a52_7a1a_11d0_a2d2_00c04fd909dd);
windows_core::imp::interface_hierarchy!(IResultDataCompare, windows_core::IUnknown);
impl IResultDataCompare {
    pub unsafe fn Compare(&self, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), luserparam, cookiea, cookieb, pnresult as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultDataCompare_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::LPARAM, isize, isize, *mut i32) -> windows_core::HRESULT,
}
pub trait IResultDataCompare_Impl: windows_core::IUnknownImpl {
    fn Compare(&self, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> windows_core::Result<()>;
}
impl IResultDataCompare_Vtbl {
    pub const fn new<Identity: IResultDataCompare_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compare<Identity: IResultDataCompare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultDataCompare_Impl::Compare(this, core::mem::transmute_copy(&luserparam), core::mem::transmute_copy(&cookiea), core::mem::transmute_copy(&cookieb), core::mem::transmute_copy(&pnresult)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Compare: Compare::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResultDataCompare as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResultDataCompare {}
windows_core::imp::define_interface!(IResultDataCompareEx, IResultDataCompareEx_Vtbl, 0x96933476_0251_11d3_aeb0_00c04f8ecd78);
windows_core::imp::interface_hierarchy!(IResultDataCompareEx, windows_core::IUnknown);
impl IResultDataCompareEx {
    pub unsafe fn Compare(&self, prdc: *const RDCOMPARE) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), prdc, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultDataCompareEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *const RDCOMPARE, *mut i32) -> windows_core::HRESULT,
}
pub trait IResultDataCompareEx_Impl: windows_core::IUnknownImpl {
    fn Compare(&self, prdc: *const RDCOMPARE) -> windows_core::Result<i32>;
}
impl IResultDataCompareEx_Vtbl {
    pub const fn new<Identity: IResultDataCompareEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compare<Identity: IResultDataCompareEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResultDataCompareEx_Impl::Compare(this, core::mem::transmute_copy(&prdc)) {
                    Ok(ok__) => {
                        pnresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Compare: Compare::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResultDataCompareEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResultDataCompareEx {}
windows_core::imp::define_interface!(IResultOwnerData, IResultOwnerData_Vtbl, 0x9cb396d8_ea83_11d0_aef1_00c04fb6dd2c);
windows_core::imp::interface_hierarchy!(IResultOwnerData, windows_core::IUnknown);
impl IResultOwnerData {
    pub unsafe fn FindItem(&self, pfindinfo: *const RESULTFINDINFO) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItem)(windows_core::Interface::as_raw(self), pfindinfo, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CacheHint(&self, nstartindex: i32, nendindex: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CacheHint)(windows_core::Interface::as_raw(self), nstartindex, nendindex).ok() }
    }
    pub unsafe fn SortItems(&self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SortItems)(windows_core::Interface::as_raw(self), ncolumn, dwsortoptions, luserparam).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultOwnerData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const RESULTFINDINFO, *mut i32) -> windows_core::HRESULT,
    pub CacheHint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SortItems: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
pub trait IResultOwnerData_Impl: windows_core::IUnknownImpl {
    fn FindItem(&self, pfindinfo: *const RESULTFINDINFO) -> windows_core::Result<i32>;
    fn CacheHint(&self, nstartindex: i32, nendindex: i32) -> windows_core::Result<()>;
    fn SortItems(&self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
impl IResultOwnerData_Vtbl {
    pub const fn new<Identity: IResultOwnerData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindItem<Identity: IResultOwnerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResultOwnerData_Impl::FindItem(this, core::mem::transmute_copy(&pfindinfo)) {
                    Ok(ok__) => {
                        pnfoundindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CacheHint<Identity: IResultOwnerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nstartindex: i32, nendindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultOwnerData_Impl::CacheHint(this, core::mem::transmute_copy(&nstartindex), core::mem::transmute_copy(&nendindex)).into()
            }
        }
        unsafe extern "system" fn SortItems<Identity: IResultOwnerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultOwnerData_Impl::SortItems(this, core::mem::transmute_copy(&ncolumn), core::mem::transmute_copy(&dwsortoptions), core::mem::transmute_copy(&luserparam)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindItem: FindItem::<Identity, OFFSET>,
            CacheHint: CacheHint::<Identity, OFFSET>,
            SortItems: SortItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResultOwnerData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResultOwnerData {}
windows_core::imp::define_interface!(ISnapinAbout, ISnapinAbout_Vtbl, 0x1245208c_a151_11d0_a7d7_00c04fd909dd);
windows_core::imp::interface_hierarchy!(ISnapinAbout, windows_core::IUnknown);
impl ISnapinAbout {
    pub unsafe fn GetSnapinDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapinDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProvider(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSnapinVersion(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetSnapinImage(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapinImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetStaticFolderImage(&self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStaticFolderImage)(windows_core::Interface::as_raw(self), hsmallimage as _, hsmallimageopen as _, hlargeimage as _, cmask as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinAbout_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSnapinDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSnapinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetSnapinImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetSnapinImage: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetStaticFolderImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetStaticFolderImage: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISnapinAbout_Impl: windows_core::IUnknownImpl {
    fn GetSnapinDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProvider(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSnapinVersion(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSnapinImage(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetStaticFolderImage(&self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ISnapinAbout_Vtbl {
    pub const fn new<Identity: ISnapinAbout_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSnapinDescription<Identity: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISnapinAbout_Impl::GetSnapinDescription(this) {
                    Ok(ok__) => {
                        lpdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProvider<Identity: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISnapinAbout_Impl::GetProvider(this) {
                    Ok(ok__) => {
                        lpname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSnapinVersion<Identity: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpversion: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISnapinAbout_Impl::GetSnapinVersion(this) {
                    Ok(ok__) => {
                        lpversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSnapinImage<Identity: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISnapinAbout_Impl::GetSnapinImage(this) {
                    Ok(ok__) => {
                        happicon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStaticFolderImage<Identity: ISnapinAbout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISnapinAbout_Impl::GetStaticFolderImage(this, core::mem::transmute_copy(&hsmallimage), core::mem::transmute_copy(&hsmallimageopen), core::mem::transmute_copy(&hlargeimage), core::mem::transmute_copy(&cmask)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSnapinDescription: GetSnapinDescription::<Identity, OFFSET>,
            GetProvider: GetProvider::<Identity, OFFSET>,
            GetSnapinVersion: GetSnapinVersion::<Identity, OFFSET>,
            GetSnapinImage: GetSnapinImage::<Identity, OFFSET>,
            GetStaticFolderImage: GetStaticFolderImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISnapinAbout as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for ISnapinAbout {}
windows_core::imp::define_interface!(ISnapinHelp, ISnapinHelp_Vtbl, 0xa6b15ace_df59_11d0_a7dd_00c04fd909dd);
windows_core::imp::interface_hierarchy!(ISnapinHelp, windows_core::IUnknown);
impl ISnapinHelp {
    pub unsafe fn GetHelpTopic(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpTopic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinHelp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHelpTopic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ISnapinHelp_Impl: windows_core::IUnknownImpl {
    fn GetHelpTopic(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISnapinHelp_Vtbl {
    pub const fn new<Identity: ISnapinHelp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHelpTopic<Identity: ISnapinHelp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcompiledhelpfile: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISnapinHelp_Impl::GetHelpTopic(this) {
                    Ok(ok__) => {
                        lpcompiledhelpfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHelpTopic: GetHelpTopic::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISnapinHelp as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISnapinHelp {}
windows_core::imp::define_interface!(ISnapinHelp2, ISnapinHelp2_Vtbl, 0x4861a010_20f9_11d2_a510_00c04fb6dd2c);
impl core::ops::Deref for ISnapinHelp2 {
    type Target = ISnapinHelp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISnapinHelp2, windows_core::IUnknown, ISnapinHelp);
impl ISnapinHelp2 {
    pub unsafe fn GetLinkedTopics(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLinkedTopics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinHelp2_Vtbl {
    pub base__: ISnapinHelp_Vtbl,
    pub GetLinkedTopics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ISnapinHelp2_Impl: ISnapinHelp_Impl {
    fn GetLinkedTopics(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISnapinHelp2_Vtbl {
    pub const fn new<Identity: ISnapinHelp2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLinkedTopics<Identity: ISnapinHelp2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcompiledhelpfiles: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISnapinHelp2_Impl::GetLinkedTopics(this) {
                    Ok(ok__) => {
                        lpcompiledhelpfiles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISnapinHelp_Vtbl::new::<Identity, OFFSET>(), GetLinkedTopics: GetLinkedTopics::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISnapinHelp2 as windows_core::Interface>::IID || iid == &<ISnapinHelp as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISnapinHelp2 {}
windows_core::imp::define_interface!(ISnapinProperties, ISnapinProperties_Vtbl, 0xf7889da9_4a02_4837_bf89_1a6f2a021010);
windows_core::imp::interface_hierarchy!(ISnapinProperties, windows_core::IUnknown);
impl ISnapinProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Properties>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pproperties.param().abi()).ok() }
    }
    pub unsafe fn QueryPropertyNames<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISnapinPropertiesCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryPropertyNames)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PropertiesChanged(&self, pproperties: &[MMC_SNAPIN_PROPERTY]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PropertiesChanged)(windows_core::Interface::as_raw(self), pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub QueryPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const MMC_SNAPIN_PROPERTY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PropertiesChanged: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISnapinProperties_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pproperties: windows_core::Ref<Properties>) -> windows_core::Result<()>;
    fn QueryPropertyNames(&self, pcallback: windows_core::Ref<ISnapinPropertiesCallback>) -> windows_core::Result<()>;
    fn PropertiesChanged(&self, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISnapinProperties_Vtbl {
    pub const fn new<Identity: ISnapinProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ISnapinProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISnapinProperties_Impl::Initialize(this, core::mem::transmute_copy(&pproperties)).into()
            }
        }
        unsafe extern "system" fn QueryPropertyNames<Identity: ISnapinProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISnapinProperties_Impl::QueryPropertyNames(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn PropertiesChanged<Identity: ISnapinProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISnapinProperties_Impl::PropertiesChanged(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&pproperties)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            QueryPropertyNames: QueryPropertyNames::<Identity, OFFSET>,
            PropertiesChanged: PropertiesChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISnapinProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISnapinProperties {}
windows_core::imp::define_interface!(ISnapinPropertiesCallback, ISnapinPropertiesCallback_Vtbl, 0xa50fa2e5_7e61_45eb_a8d4_9a07b3e851a8);
windows_core::imp::interface_hierarchy!(ISnapinPropertiesCallback, windows_core::IUnknown);
impl ISnapinPropertiesCallback {
    pub unsafe fn AddPropertyName<P0>(&self, pszpropname: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyName)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinPropertiesCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait ISnapinPropertiesCallback_Impl: windows_core::IUnknownImpl {
    fn AddPropertyName(&self, pszpropname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
}
impl ISnapinPropertiesCallback_Vtbl {
    pub const fn new<Identity: ISnapinPropertiesCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPropertyName<Identity: ISnapinPropertiesCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISnapinPropertiesCallback_Impl::AddPropertyName(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPropertyName: AddPropertyName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISnapinPropertiesCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISnapinPropertiesCallback {}
windows_core::imp::define_interface!(IStringTable, IStringTable_Vtbl, 0xde40b7a4_0f65_11d2_8e25_00c04f8ecd78);
windows_core::imp::interface_hierarchy!(IStringTable, windows_core::IUnknown);
impl IStringTable {
    pub unsafe fn AddString<P0>(&self, pszadd: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddString)(windows_core::Interface::as_raw(self), pszadd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetString(&self, stringid: u32, lpbuffer: &mut [u16], pcchout: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), stringid, lpbuffer.len().try_into().unwrap(), core::mem::transmute(lpbuffer.as_ptr()), pcchout as _).ok() }
    }
    pub unsafe fn GetStringLength(&self, stringid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), stringid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeleteString(&self, stringid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteString)(windows_core::Interface::as_raw(self), stringid).ok() }
    }
    pub unsafe fn DeleteAllStrings(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAllStrings)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FindString<P0>(&self, pszfind: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindString)(windows_core::Interface::as_raw(self), pszfind.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enumerate(&self) -> windows_core::Result<super::Com::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enumerate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub DeleteString: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAllStrings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Enumerate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Enumerate: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStringTable_Impl: windows_core::IUnknownImpl {
    fn AddString(&self, pszadd: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn GetString(&self, stringid: u32, cchbuffer: u32, lpbuffer: windows_core::PWSTR, pcchout: *mut u32) -> windows_core::Result<()>;
    fn GetStringLength(&self, stringid: u32) -> windows_core::Result<u32>;
    fn DeleteString(&self, stringid: u32) -> windows_core::Result<()>;
    fn DeleteAllStrings(&self) -> windows_core::Result<()>;
    fn FindString(&self, pszfind: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn Enumerate(&self) -> windows_core::Result<super::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl IStringTable_Vtbl {
    pub const fn new<Identity: IStringTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddString<Identity: IStringTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszadd: windows_core::PCWSTR, pstringid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringTable_Impl::AddString(this, core::mem::transmute(&pszadd)) {
                    Ok(ok__) => {
                        pstringid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IStringTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: windows_core::PWSTR, pcchout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringTable_Impl::GetString(this, core::mem::transmute_copy(&stringid), core::mem::transmute_copy(&cchbuffer), core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&pcchout)).into()
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IStringTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringTable_Impl::GetStringLength(this, core::mem::transmute_copy(&stringid)) {
                    Ok(ok__) => {
                        pcchstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteString<Identity: IStringTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stringid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringTable_Impl::DeleteString(this, core::mem::transmute_copy(&stringid)).into()
            }
        }
        unsafe extern "system" fn DeleteAllStrings<Identity: IStringTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringTable_Impl::DeleteAllStrings(this).into()
            }
        }
        unsafe extern "system" fn FindString<Identity: IStringTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfind: windows_core::PCWSTR, pstringid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringTable_Impl::FindString(this, core::mem::transmute(&pszfind)) {
                    Ok(ok__) => {
                        pstringid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enumerate<Identity: IStringTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringTable_Impl::Enumerate(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddString: AddString::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            DeleteString: DeleteString::<Identity, OFFSET>,
            DeleteAllStrings: DeleteAllStrings::<Identity, OFFSET>,
            FindString: FindString::<Identity, OFFSET>,
            Enumerate: Enumerate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStringTable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStringTable {}
windows_core::imp::define_interface!(IToolbar, IToolbar_Vtbl, 0x43136eb9_d36c_11cf_adbc_00aa00a80033);
windows_core::imp::interface_hierarchy!(IToolbar, windows_core::IUnknown);
impl IToolbar {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddBitmap(&self, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddBitmap)(windows_core::Interface::as_raw(self), nimages, hbmp, cxsize, cysize, crmask).ok() }
    }
    pub unsafe fn AddButtons(&self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddButtons)(windows_core::Interface::as_raw(self), nbuttons, lpbuttons).ok() }
    }
    pub unsafe fn InsertButton(&self, nindex: i32, lpbutton: *const MMCBUTTON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertButton)(windows_core::Interface::as_raw(self), nindex, lpbutton).ok() }
    }
    pub unsafe fn DeleteButton(&self, nindex: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteButton)(windows_core::Interface::as_raw(self), nindex).ok() }
    }
    pub unsafe fn GetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetButtonState)(windows_core::Interface::as_raw(self), idcommand, nstate, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetButtonState)(windows_core::Interface::as_raw(self), idcommand, nstate, bstate.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolbar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Graphics::Gdi::HBITMAP, i32, i32, super::super::Foundation::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddBitmap: usize,
    pub AddButtons: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const MMCBUTTON) -> windows_core::HRESULT,
    pub InsertButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const MMCBUTTON) -> windows_core::HRESULT,
    pub DeleteButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, MMC_BUTTON_STATE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, MMC_BUTTON_STATE, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IToolbar_Impl: windows_core::IUnknownImpl {
    fn AddBitmap(&self, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> windows_core::Result<()>;
    fn AddButtons(&self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> windows_core::Result<()>;
    fn InsertButton(&self, nindex: i32, lpbutton: *const MMCBUTTON) -> windows_core::Result<()>;
    fn DeleteButton(&self, nindex: i32) -> windows_core::Result<()>;
    fn GetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE) -> windows_core::Result<windows_core::BOOL>;
    fn SetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IToolbar_Vtbl {
    pub const fn new<Identity: IToolbar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddBitmap<Identity: IToolbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToolbar_Impl::AddBitmap(this, core::mem::transmute_copy(&nimages), core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&cxsize), core::mem::transmute_copy(&cysize), core::mem::transmute_copy(&crmask)).into()
            }
        }
        unsafe extern "system" fn AddButtons<Identity: IToolbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToolbar_Impl::AddButtons(this, core::mem::transmute_copy(&nbuttons), core::mem::transmute_copy(&lpbuttons)).into()
            }
        }
        unsafe extern "system" fn InsertButton<Identity: IToolbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToolbar_Impl::InsertButton(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&lpbutton)).into()
            }
        }
        unsafe extern "system" fn DeleteButton<Identity: IToolbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToolbar_Impl::DeleteButton(this, core::mem::transmute_copy(&nindex)).into()
            }
        }
        unsafe extern "system" fn GetButtonState<Identity: IToolbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToolbar_Impl::GetButtonState(this, core::mem::transmute_copy(&idcommand), core::mem::transmute_copy(&nstate)) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetButtonState<Identity: IToolbar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToolbar_Impl::SetButtonState(this, core::mem::transmute_copy(&idcommand), core::mem::transmute_copy(&nstate), core::mem::transmute_copy(&bstate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddBitmap: AddBitmap::<Identity, OFFSET>,
            AddButtons: AddButtons::<Identity, OFFSET>,
            InsertButton: InsertButton::<Identity, OFFSET>,
            DeleteButton: DeleteButton::<Identity, OFFSET>,
            GetButtonState: GetButtonState::<Identity, OFFSET>,
            SetButtonState: SetButtonState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToolbar as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IToolbar {}
windows_core::imp::define_interface!(IViewExtensionCallback, IViewExtensionCallback_Vtbl, 0x34dd928a_7599_41e5_9f5e_d6bc3062c2da);
windows_core::imp::interface_hierarchy!(IViewExtensionCallback, windows_core::IUnknown);
impl IViewExtensionCallback {
    pub unsafe fn AddView(&self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddView)(windows_core::Interface::as_raw(self), pextviewdata).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewExtensionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddView: unsafe extern "system" fn(*mut core::ffi::c_void, *const MMC_EXT_VIEW_DATA) -> windows_core::HRESULT,
}
pub trait IViewExtensionCallback_Impl: windows_core::IUnknownImpl {
    fn AddView(&self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> windows_core::Result<()>;
}
impl IViewExtensionCallback_Vtbl {
    pub const fn new<Identity: IViewExtensionCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddView<Identity: IViewExtensionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewExtensionCallback_Impl::AddView(this, core::mem::transmute_copy(&pextviewdata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddView: AddView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewExtensionCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IViewExtensionCallback {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IconIdentifier(pub i32);
pub const Icon_Error: IconIdentifier = IconIdentifier(32513i32);
pub const Icon_First: IconIdentifier = IconIdentifier(32513i32);
pub const Icon_Information: IconIdentifier = IconIdentifier(32516i32);
pub const Icon_Last: IconIdentifier = IconIdentifier(32516i32);
pub const Icon_None: IconIdentifier = IconIdentifier(0i32);
pub const Icon_Question: IconIdentifier = IconIdentifier(32514i32);
pub const Icon_Warning: IconIdentifier = IconIdentifier(32515i32);
pub const ListMode_Detail: _ListViewMode = _ListViewMode(3i32);
pub const ListMode_Filtered: _ListViewMode = _ListViewMode(4i32);
pub const ListMode_Large_Icons: _ListViewMode = _ListViewMode(1i32);
pub const ListMode_List: _ListViewMode = _ListViewMode(2i32);
pub const ListMode_Small_Icons: _ListViewMode = _ListViewMode(0i32);
pub const MENUBUTTON: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MENUBUTTONDATA {
    pub idCommand: i32,
    pub x: i32,
    pub y: i32,
}
pub const MFCC_DISABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(0i32);
pub const MFCC_ENABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(1i32);
pub const MFCC_VALUE_CHANGE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMCBUTTON {
    pub nBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsType: u8,
    pub lpButtonText: windows_core::PWSTR,
    pub lpTooltipText: windows_core::PWSTR,
}
pub const MMCC_STANDARD_VIEW_SELECT: MMC_MENU_COMMAND_IDS = MMC_MENU_COMMAND_IDS(-1i32);
pub const MMCLV_AUTO: i32 = -1i32;
pub const MMCLV_NOICON: i32 = -1i32;
pub const MMCLV_NOPARAM: i32 = -2i32;
pub const MMCLV_NOPTR: u32 = 0u32;
pub const MMCLV_UPDATE_NOINVALIDATEALL: u32 = 1u32;
pub const MMCLV_UPDATE_NOSCROLL: u32 = 2u32;
pub const MMCLV_VIEWSTYLE_FILTERED: u32 = 4u32;
pub const MMCLV_VIEWSTYLE_ICON: u32 = 0u32;
pub const MMCLV_VIEWSTYLE_LIST: u32 = 3u32;
pub const MMCLV_VIEWSTYLE_REPORT: u32 = 1u32;
pub const MMCLV_VIEWSTYLE_SMALLICON: u32 = 2u32;
pub const MMCN_ACTIVATE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32769i32);
pub const MMCN_ADD_IMAGES: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32770i32);
pub const MMCN_BTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32771i32);
pub const MMCN_CANPASTE_OUTOFPROC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32803i32);
pub const MMCN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32772i32);
pub const MMCN_COLUMNS_CHANGED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32802i32);
pub const MMCN_COLUMN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32773i32);
pub const MMCN_CONTEXTHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32793i32);
pub const MMCN_CONTEXTMENU: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32774i32);
pub const MMCN_CUTORMOVE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32775i32);
pub const MMCN_DBLCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32776i32);
pub const MMCN_DELETE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32777i32);
pub const MMCN_DESELECT_ALL: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32778i32);
pub const MMCN_EXPAND: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32779i32);
pub const MMCN_EXPANDSYNC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32801i32);
pub const MMCN_FILTERBTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32796i32);
pub const MMCN_FILTER_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32795i32);
pub const MMCN_HELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32780i32);
pub const MMCN_INITOCX: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32794i32);
pub const MMCN_LISTPAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32800i32);
pub const MMCN_MENU_BTNCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32781i32);
pub const MMCN_MINIMIZED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32782i32);
pub const MMCN_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32783i32);
pub const MMCN_PRELOAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32799i32);
pub const MMCN_PRINT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32798i32);
pub const MMCN_PROPERTY_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32784i32);
pub const MMCN_QUERY_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32785i32);
pub const MMCN_REFRESH: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32786i32);
pub const MMCN_REMOVE_CHILDREN: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32787i32);
pub const MMCN_RENAME: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32788i32);
pub const MMCN_RESTORE_VIEW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32797i32);
pub const MMCN_SELECT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32789i32);
pub const MMCN_SHOW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32790i32);
pub const MMCN_SNAPINHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32792i32);
pub const MMCN_VIEW_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32791i32);
pub const MMCVersionInfo: windows_core::GUID = windows_core::GUID::from_u128(0xd6fedb1d_cf21_4bd9_af3b_c5468e9c6684);
pub const MMC_ACTION_ID: MMC_ACTION_TYPE = MMC_ACTION_TYPE(0i32);
pub const MMC_ACTION_LINK: MMC_ACTION_TYPE = MMC_ACTION_TYPE(1i32);
pub const MMC_ACTION_SCRIPT: MMC_ACTION_TYPE = MMC_ACTION_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_ACTION_TYPE(pub i32);
pub const MMC_ACTION_UNINITIALIZED: MMC_ACTION_TYPE = MMC_ACTION_TYPE(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_BUTTON_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_COLUMN_DATA {
    pub nColIndex: i32,
    pub dwFlags: u32,
    pub nWidth: i32,
    pub ulReserved: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MMC_COLUMN_SET_DATA {
    pub cbSize: i32,
    pub nNumCols: i32,
    pub pColData: *mut MMC_COLUMN_DATA,
}
impl Default for MMC_COLUMN_SET_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_CONSOLE_VERB(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_CONTROL_TYPE(pub i32);
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
pub const MMC_ENSUREFOCUSVISIBLE: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(8i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_EXPANDSYNC_STRUCT {
    pub bHandled: windows_core::BOOL,
    pub bExpanding: windows_core::BOOL,
    pub hItem: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_EXT_VIEW_DATA {
    pub viewID: windows_core::GUID,
    pub pszURL: windows_core::PCWSTR,
    pub pszViewTitle: windows_core::PCWSTR,
    pub pszTooltipText: windows_core::PCWSTR,
    pub bReplacesDefaultView: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_FILTERDATA {
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub lValue: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_FILTER_CHANGE_CODE(pub i32);
pub const MMC_FILTER_NOVALUE: MMC_FILTER_TYPE = MMC_FILTER_TYPE(32768i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_FILTER_TYPE(pub i32);
pub const MMC_IMAGECALLBACK: i32 = -1i32;
pub const MMC_INT_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(1i32);
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_LISTPAD_INFO {
    pub szTitle: windows_core::PWSTR,
    pub szButtonText: windows_core::PWSTR,
    pub nCommandID: isize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_MENU_COMMAND_IDS(pub i32);
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
pub const MMC_NOSORTHEADER: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_NOTIFY_TYPE(pub i32);
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
pub const MMC_PROPACT_CHANGING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(2i32);
pub const MMC_PROPACT_DELETING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(1i32);
pub const MMC_PROPACT_INITIALIZED: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_PROPERTY_ACTION(pub i32);
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
pub const MMC_PROP_PERSIST: u32 = 8u32;
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
pub const MMC_PSO_HASHELP: u32 = 2u32;
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_RESTORE_VIEW {
    pub dwSize: u32,
    pub cookie: isize,
    pub pViewType: windows_core::PWSTR,
    pub lViewOptions: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_RESULT_VIEW_STYLE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_SCOPE_ITEM_STATE(pub i32);
pub const MMC_SCOPE_ITEM_STATE_BOLD: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(2i32);
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(3i32);
pub const MMC_SCOPE_ITEM_STATE_NORMAL: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(1i32);
pub const MMC_SHOWSELALWAYS: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(2i32);
pub const MMC_SINGLESEL: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(1i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub struct MMC_SNAPIN_PROPERTY {
    pub pszPropName: windows_core::PCWSTR,
    pub varValue: super::Variant::VARIANT,
    pub eAction: MMC_PROPERTY_ACTION,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Clone for MMC_SNAPIN_PROPERTY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for MMC_SNAPIN_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_SORT_DATA {
    pub nColIndex: i32,
    pub dwSortOptions: u32,
    pub ulReserved: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MMC_SORT_SET_DATA {
    pub cbSize: i32,
    pub nNumItems: i32,
    pub pSortData: *mut MMC_SORT_DATA,
}
impl Default for MMC_SORT_SET_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MMC_STRING_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MMC_TASK {
    pub sDisplayObject: MMC_TASK_DISPLAY_OBJECT,
    pub szText: windows_core::PWSTR,
    pub szHelpString: windows_core::PWSTR,
    pub eActionType: MMC_ACTION_TYPE,
    pub Anonymous: MMC_TASK_0,
}
impl Default for MMC_TASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MMC_TASK_0 {
    pub nCommandID: isize,
    pub szActionURL: windows_core::PWSTR,
    pub szScript: windows_core::PWSTR,
}
impl Default for MMC_TASK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_TASK_DISPLAY_BITMAP {
    pub szMouseOverBitmap: windows_core::PWSTR,
    pub szMouseOffBitmap: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MMC_TASK_DISPLAY_OBJECT {
    pub eDisplayType: MMC_TASK_DISPLAY_TYPE,
    pub Anonymous: MMC_TASK_DISPLAY_OBJECT_0,
}
impl Default for MMC_TASK_DISPLAY_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MMC_TASK_DISPLAY_OBJECT_0 {
    pub uBitmap: MMC_TASK_DISPLAY_BITMAP,
    pub uSymbol: MMC_TASK_DISPLAY_SYMBOL,
}
impl Default for MMC_TASK_DISPLAY_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MMC_TASK_DISPLAY_SYMBOL {
    pub szFontFamilyName: windows_core::PWSTR,
    pub szURLtoEOT: windows_core::PWSTR,
    pub szSymbolString: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_TASK_DISPLAY_TYPE(pub i32);
pub const MMC_TASK_DISPLAY_TYPE_BITMAP: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(4i32);
pub const MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(3i32);
pub const MMC_TASK_DISPLAY_TYPE_SYMBOL: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(1i32);
pub const MMC_TASK_DISPLAY_TYPE_VANILLA_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(2i32);
pub const MMC_TASK_DISPLAY_UNINITIALIZED: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(0i32);
pub const MMC_VER: u32 = 512u32;
pub const MMC_VERB_COPY: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32769i32);
pub const MMC_VERB_CUT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
pub const MMC_VERB_DELETE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32771i32);
pub const MMC_VERB_FIRST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
pub const MMC_VERB_LAST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
pub const MMC_VERB_MAX: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32777i32);
pub const MMC_VERB_NONE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(0i32);
pub const MMC_VERB_OPEN: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
pub const MMC_VERB_PASTE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32770i32);
pub const MMC_VERB_PRINT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32775i32);
pub const MMC_VERB_PROPERTIES: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32772i32);
pub const MMC_VERB_REFRESH: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32774i32);
pub const MMC_VERB_RENAME: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32773i32);
pub const MMC_VIEW_OPTIONS_CREATENEW: u32 = 16u32;
pub const MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
pub const MMC_VIEW_OPTIONS_FILTERED: u32 = 8u32;
pub const MMC_VIEW_OPTIONS_LEXICAL_SORT: u32 = 128u32;
pub const MMC_VIEW_OPTIONS_MULTISELECT: u32 = 2u32;
pub const MMC_VIEW_OPTIONS_NOLISTVIEWS: u32 = 1u32;
pub const MMC_VIEW_OPTIONS_NONE: u32 = 0u32;
pub const MMC_VIEW_OPTIONS_OWNERDATALIST: u32 = 4u32;
pub const MMC_VIEW_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MMC_VIEW_TYPE(pub i32);
pub const MMC_VIEW_TYPE_HTML: MMC_VIEW_TYPE = MMC_VIEW_TYPE(1i32);
pub const MMC_VIEW_TYPE_LIST: MMC_VIEW_TYPE = MMC_VIEW_TYPE(0i32);
pub const MMC_VIEW_TYPE_OCX: MMC_VIEW_TYPE = MMC_VIEW_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MMC_VISIBLE_COLUMNS {
    pub nVisibleColumns: i32,
    pub rgVisibleCols: [i32; 1],
}
impl Default for MMC_VISIBLE_COLUMNS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(MenuItem, MenuItem_Vtbl, 0x0178fad1_b361_4b27_96ad_67c57ebf2e1d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for MenuItem {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(MenuItem, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl MenuItem {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LanguageIndependentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageIndependentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LanguageIndependentPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageIndependentPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Execute(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct MenuItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LanguageIndependentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LanguageIndependentPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait MenuItem_Impl: super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LanguageIndependentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LanguageIndependentPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Execute(&self) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl MenuItem_Vtbl {
    pub const fn new<Identity: MenuItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayName<Identity: MenuItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match MenuItem_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LanguageIndependentName<Identity: MenuItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageindependentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match MenuItem_Impl::LanguageIndependentName(this) {
                    Ok(ok__) => {
                        languageindependentname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: MenuItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match MenuItem_Impl::Path(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LanguageIndependentPath<Identity: MenuItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageindependentpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match MenuItem_Impl::LanguageIndependentPath(this) {
                    Ok(ok__) => {
                        languageindependentpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Execute<Identity: MenuItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                MenuItem_Impl::Execute(this).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: MenuItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match MenuItem_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisplayName: DisplayName::<Identity, OFFSET>,
            LanguageIndependentName: LanguageIndependentName::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            LanguageIndependentPath: LanguageIndependentPath::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<MenuItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for MenuItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Node, Node_Vtbl, 0xf81ed800_7839_4447_945d_8e15da59ca55);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Node {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Node, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Node {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Property(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Bookmark(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bookmark)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsScopeNode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsScopeNode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Nodetype(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Nodetype)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Node_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Bookmark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Nodetype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Node_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Property(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn Bookmark(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsScopeNode(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Nodetype(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Node_Vtbl {
    pub const fn new<Identity: Node_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: Node_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Node_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Property<Identity: Node_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, propertyvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Node_Impl::get_Property(this, core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        propertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Bookmark<Identity: Node_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bookmark: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Node_Impl::Bookmark(this) {
                    Ok(ok__) => {
                        bookmark.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsScopeNode<Identity: Node_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isscopenode: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Node_Impl::IsScopeNode(this) {
                    Ok(ok__) => {
                        isscopenode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Nodetype<Identity: Node_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Node_Impl::Nodetype(this) {
                    Ok(ok__) => {
                        nodetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            get_Property: get_Property::<Identity, OFFSET>,
            Bookmark: Bookmark::<Identity, OFFSET>,
            IsScopeNode: IsScopeNode::<Identity, OFFSET>,
            Nodetype: Nodetype::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Node as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Node {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Nodes, Nodes_Vtbl, 0x313b01df_b22f_4d42_b1b8_483cdcf51d35);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Nodes {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Nodes, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Nodes {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<Node> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Nodes_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Nodes_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<Node>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Nodes_Vtbl {
    pub const fn new<Identity: Nodes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: Nodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Nodes_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: Nodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, node: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Nodes_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        node.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: Nodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Nodes_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Nodes as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Nodes {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Properties, Properties_Vtbl, 0x2886abc2_a425_42b2_91c6_e25c0e04581c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Properties {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Properties, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Properties {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<Property> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Remove(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Properties_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Properties_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<Property>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Properties_Vtbl {
    pub const fn new<Identity: Properties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Properties_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Properties_Impl::Item(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Properties_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: Properties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Properties_Impl::Remove(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Properties as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Properties {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Property, Property_Vtbl, 0x4600c3a5_e301_41d8_b6d0_ef2e4212e0ca);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Property {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Property, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Property {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Property_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Property_Impl: super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Property_Vtbl {
    pub const fn new<Identity: Property_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: Property_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Property_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: Property_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Property_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: Property_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Property_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Property as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Property {}
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RDCOMPARE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub nColumn: i32,
    pub lUserParam: super::super::Foundation::LPARAM,
    pub prdch1: *mut RDITEMHDR,
    pub prdch2: *mut RDITEMHDR,
}
impl Default for RDCOMPARE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RDITEMHDR {
    pub dwFlags: u32,
    pub cookie: isize,
    pub lpReserved: super::super::Foundation::LPARAM,
}
pub const RDI_IMAGE: u32 = 4u32;
pub const RDI_INDENT: u32 = 64u32;
pub const RDI_INDEX: u32 = 32u32;
pub const RDI_PARAM: u32 = 16u32;
pub const RDI_STATE: u32 = 8u32;
pub const RDI_STR: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESULTDATAITEM {
    pub mask: u32,
    pub bScopeItem: windows_core::BOOL,
    pub itemID: isize,
    pub nIndex: i32,
    pub nCol: i32,
    pub str: windows_core::PWSTR,
    pub nImage: i32,
    pub nState: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESULTFINDINFO {
    pub psz: windows_core::PWSTR,
    pub nStart: i32,
    pub dwOptions: u32,
}
#[repr(C)]
pub struct RESULT_VIEW_TYPE_INFO {
    pub pstrPersistableViewDescription: windows_core::PWSTR,
    pub eViewType: MMC_VIEW_TYPE,
    pub dwMiscOptions: u32,
    pub Anonymous: RESULT_VIEW_TYPE_INFO_0,
}
impl Clone for RESULT_VIEW_TYPE_INFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for RESULT_VIEW_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union RESULT_VIEW_TYPE_INFO_0 {
    pub dwListOptions: u32,
    pub Anonymous1: RESULT_VIEW_TYPE_INFO_0_0,
    pub Anonymous2: core::mem::ManuallyDrop<RESULT_VIEW_TYPE_INFO_0_1>,
}
impl Clone for RESULT_VIEW_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for RESULT_VIEW_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESULT_VIEW_TYPE_INFO_0_0 {
    pub dwHTMLOptions: u32,
    pub pstrURL: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RESULT_VIEW_TYPE_INFO_0_1 {
    pub dwOCXOptions: u32,
    pub pUnkControl: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
pub const RFI_PARTIAL: u32 = 1u32;
pub const RFI_WRAP: u32 = 2u32;
pub const RSI_DESCENDING: u32 = 1u32;
pub const RSI_NOSORTICON: u32 = 2u32;
pub const RVTI_HTML_OPTIONS_NOLISTVIEW: u32 = 1u32;
pub const RVTI_HTML_OPTIONS_NONE: u32 = 0u32;
pub const RVTI_LIST_OPTIONS_ALLOWPASTE: u32 = 256u32;
pub const RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
pub const RVTI_LIST_OPTIONS_FILTERED: u32 = 8u32;
pub const RVTI_LIST_OPTIONS_LEXICAL_SORT: u32 = 128u32;
pub const RVTI_LIST_OPTIONS_MULTISELECT: u32 = 4u32;
pub const RVTI_LIST_OPTIONS_NONE: u32 = 0u32;
pub const RVTI_LIST_OPTIONS_OWNERDATALIST: u32 = 2u32;
pub const RVTI_LIST_OPTIONS_USEFONTLINKING: u32 = 32u32;
pub const RVTI_MISC_OPTIONS_NOLISTVIEWS: u32 = 1u32;
pub const RVTI_OCX_OPTIONS_CACHE_OCX: u32 = 2u32;
pub const RVTI_OCX_OPTIONS_NOLISTVIEW: u32 = 1u32;
pub const RVTI_OCX_OPTIONS_NONE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPEDATAITEM {
    pub mask: u32,
    pub displayname: windows_core::PWSTR,
    pub nImage: i32,
    pub nOpenImage: i32,
    pub nState: u32,
    pub cChildren: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub relativeID: isize,
    pub ID: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SColumnSetID {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl Default for SColumnSetID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SDI_CHILDREN: u32 = 64u32;
pub const SDI_FIRST: u32 = 134217728u32;
pub const SDI_IMAGE: u32 = 4u32;
pub const SDI_NEXT: u32 = 536870912u32;
pub const SDI_OPENIMAGE: u32 = 8u32;
pub const SDI_PARAM: u32 = 32u32;
pub const SDI_PARENT: u32 = 0u32;
pub const SDI_PREVIOUS: u32 = 268435456u32;
pub const SDI_STATE: u32 = 16u32;
pub const SDI_STR: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, PartialEq)]
pub struct SMMCDataObjects {
    pub count: u32,
    pub lpDataObject: [core::mem::ManuallyDrop<Option<super::Com::IDataObject>>; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SMMCDataObjects {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SMMCObjectTypes {
    pub count: u32,
    pub guid: [windows_core::GUID; 1],
}
impl Default for SMMCObjectTypes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SNodeID {
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl Default for SNodeID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SNodeID2 {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl Default for SNodeID2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ScopeNamespace, ScopeNamespace_Vtbl, 0xebbb48dc_1a3b_4d86_b786_c21b28389012);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ScopeNamespace {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ScopeNamespace, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ScopeNamespace {
    pub unsafe fn GetParent<P0>(&self, node: P0) -> windows_core::Result<Node>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetChild<P0>(&self, node: P0) -> windows_core::Result<Node>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNext<P0>(&self, node: P0) -> windows_core::Result<Node>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRoot(&self) -> windows_core::Result<Node> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Expand<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe { (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), node.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ScopeNamespace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ScopeNamespace_Impl: super::Com::IDispatch_Impl {
    fn GetParent(&self, node: windows_core::Ref<Node>) -> windows_core::Result<Node>;
    fn GetChild(&self, node: windows_core::Ref<Node>) -> windows_core::Result<Node>;
    fn GetNext(&self, node: windows_core::Ref<Node>) -> windows_core::Result<Node>;
    fn GetRoot(&self) -> windows_core::Result<Node>;
    fn Expand(&self, node: windows_core::Ref<Node>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ScopeNamespace_Vtbl {
    pub const fn new<Identity: ScopeNamespace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParent<Identity: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ScopeNamespace_Impl::GetParent(this, core::mem::transmute_copy(&node)) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChild<Identity: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, child: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ScopeNamespace_Impl::GetChild(this, core::mem::transmute_copy(&node)) {
                    Ok(ok__) => {
                        child.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNext<Identity: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, next: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ScopeNamespace_Impl::GetNext(this, core::mem::transmute_copy(&node)) {
                    Ok(ok__) => {
                        next.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRoot<Identity: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, root: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ScopeNamespace_Impl::GetRoot(this) {
                    Ok(ok__) => {
                        root.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Expand<Identity: ScopeNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ScopeNamespace_Impl::Expand(this, core::mem::transmute_copy(&node)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetParent: GetParent::<Identity, OFFSET>,
            GetChild: GetChild::<Identity, OFFSET>,
            GetNext: GetNext::<Identity, OFFSET>,
            GetRoot: GetRoot::<Identity, OFFSET>,
            Expand: Expand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ScopeNamespace as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ScopeNamespace {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(SnapIn, SnapIn_Vtbl, 0x3be910f6_3459_49c6_a1bb_41e6be9df3ea);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for SnapIn {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(SnapIn, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl SnapIn {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Vendor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Vendor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Extensions(&self) -> windows_core::Result<Extensions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SnapinCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SnapinCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<Properties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnableAllExtensions(&self, enable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableAllExtensions)(windows_core::Interface::as_raw(self), enable.into()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct SnapIn_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Vendor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SnapinCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableAllExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SnapIn_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Vendor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Extensions(&self) -> windows_core::Result<Extensions>;
    fn SnapinCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Properties(&self) -> windows_core::Result<Properties>;
    fn EnableAllExtensions(&self, enable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl SnapIn_Vtbl {
    pub const fn new<Identity: SnapIn_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: SnapIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIn_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Vendor<Identity: SnapIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIn_Impl::Vendor(this) {
                    Ok(ok__) => {
                        vendor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Version<Identity: SnapIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIn_Impl::Version(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Extensions<Identity: SnapIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extensions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIn_Impl::Extensions(this) {
                    Ok(ok__) => {
                        extensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SnapinCLSID<Identity: SnapIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapinclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIn_Impl::SnapinCLSID(this) {
                    Ok(ok__) => {
                        snapinclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: SnapIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIn_Impl::Properties(this) {
                    Ok(ok__) => {
                        properties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableAllExtensions<Identity: SnapIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                SnapIn_Impl::EnableAllExtensions(this, core::mem::transmute_copy(&enable)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Vendor: Vendor::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            Extensions: Extensions::<Identity, OFFSET>,
            SnapinCLSID: SnapinCLSID::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<SnapIn as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for SnapIn {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(SnapIns, SnapIns_Vtbl, 0x2ef3de1d_b12a_49d1_92c5_0b00798768f1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for SnapIns {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(SnapIns, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl SnapIns {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<SnapIn> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, snapinnameorclsid: &windows_core::BSTR, parentsnapin: &super::Variant::VARIANT, properties: &super::Variant::VARIANT) -> windows_core::Result<SnapIn> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(snapinnameorclsid), core::mem::transmute_copy(parentsnapin), core::mem::transmute_copy(properties), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove<P0>(&self, snapin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SnapIn>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), snapin.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct SnapIns_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SnapIns_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<SnapIn>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, snapinnameorclsid: &windows_core::BSTR, parentsnapin: &super::Variant::VARIANT, properties: &super::Variant::VARIANT) -> windows_core::Result<SnapIn>;
    fn Remove(&self, snapin: windows_core::Ref<SnapIn>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl SnapIns_Vtbl {
    pub const fn new<Identity: SnapIns_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: SnapIns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIns_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: SnapIns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, snapin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIns_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        snapin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: SnapIns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIns_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: SnapIns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapinnameorclsid: *mut core::ffi::c_void, parentsnapin: super::Variant::VARIANT, properties: super::Variant::VARIANT, snapin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SnapIns_Impl::Add(this, core::mem::transmute(&snapinnameorclsid), core::mem::transmute(&parentsnapin), core::mem::transmute(&properties)) {
                    Ok(ok__) => {
                        snapin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: SnapIns_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                SnapIns_Impl::Remove(this, core::mem::transmute_copy(&snapin)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<SnapIns as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for SnapIns {}
pub const SortOrder_Ascending: _ColumnSortOrder = _ColumnSortOrder(0i32);
pub const SortOrder_Descending: _ColumnSortOrder = _ColumnSortOrder(1i32);
pub const TOOLBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(0i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(View, View_Vtbl, 0x6efc2da2_b38c_457e_9abb_ed2d189b8c38);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for View {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(View, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl View {
    pub unsafe fn ActiveScopeNode(&self) -> windows_core::Result<Node> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActiveScopeNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetActiveScopeNode<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActiveScopeNode)(windows_core::Interface::as_raw(self), node.param().abi()).ok() }
    }
    pub unsafe fn Selection(&self) -> windows_core::Result<Nodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Selection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ListItems(&self) -> windows_core::Result<Nodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SnapinScopeObject(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SnapinScopeObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(scopenode), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SnapinSelectionObject(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SnapinSelectionObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Is<P0>(&self, view: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<View>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Is)(windows_core::Interface::as_raw(self), view.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Document(&self) -> windows_core::Result<Document> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Document)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SelectAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectAll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Select<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), node.param().abi()).ok() }
    }
    pub unsafe fn Deselect<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe { (windows_core::Interface::vtable(self).Deselect)(windows_core::Interface::as_raw(self), node.param().abi()).ok() }
    }
    pub unsafe fn IsSelected<P0>(&self, node: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSelected)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DisplayScopeNodePropertySheet(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayScopeNodePropertySheet)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(scopenode)).ok() }
    }
    pub unsafe fn DisplaySelectionPropertySheet(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplaySelectionPropertySheet)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyScopeNode(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyScopeNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(scopenode)).ok() }
    }
    pub unsafe fn CopySelection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopySelection)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteScopeNode(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteScopeNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(scopenode)).ok() }
    }
    pub unsafe fn DeleteSelection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteSelection)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RenameScopeNode(&self, newname: &windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RenameScopeNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newname), core::mem::transmute_copy(scopenode)).ok() }
    }
    pub unsafe fn RenameSelectedItem(&self, newname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RenameSelectedItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_ScopeNodeContextMenu(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<ContextMenu> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ScopeNodeContextMenu)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(scopenode), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SelectionContextMenu(&self) -> windows_core::Result<ContextMenu> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectionContextMenu)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RefreshScopeNode(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RefreshScopeNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(scopenode)).ok() }
    }
    pub unsafe fn RefreshSelection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RefreshSelection)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExecuteSelectionMenuItem(&self, menuitempath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecuteSelectionMenuItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(menuitempath)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExecuteScopeNodeMenuItem(&self, menuitempath: &windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecuteScopeNodeMenuItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(menuitempath), core::mem::transmute_copy(scopenode)).ok() }
    }
    pub unsafe fn ExecuteShellCommand(&self, command: &windows_core::BSTR, directory: &windows_core::BSTR, parameters: &windows_core::BSTR, windowstate: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecuteShellCommand)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(command), core::mem::transmute_copy(directory), core::mem::transmute_copy(parameters), core::mem::transmute_copy(windowstate)).ok() }
    }
    pub unsafe fn Frame(&self) -> windows_core::Result<Frame> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Frame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ScopeTreeVisible(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScopeTreeVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScopeTreeVisible(&self, visible: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScopeTreeVisible)(windows_core::Interface::as_raw(self), visible.into()).ok() }
    }
    pub unsafe fn Back(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Back)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Forward(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Forward)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetStatusBarText(&self, statusbartext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStatusBarText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(statusbartext)).ok() }
    }
    pub unsafe fn Memento(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Memento)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ViewMemento(&self, memento: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ViewMemento)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(memento)).ok() }
    }
    pub unsafe fn Columns(&self) -> windows_core::Result<Columns> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Columns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CellContents<P0>(&self, node: P0, column: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CellContents)(windows_core::Interface::as_raw(self), node.param().abi(), column, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExportList(&self, file: &windows_core::BSTR, exportoptions: _ExportListOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExportList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(file), exportoptions).ok() }
    }
    pub unsafe fn ListViewMode(&self) -> windows_core::Result<_ListViewMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListViewMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListViewMode(&self, mode: _ListViewMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetListViewMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn ControlObject(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ControlObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct View_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ActiveScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActiveScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Selection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ListItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SnapinScopeObject: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SnapinScopeObject: usize,
    pub SnapinSelectionObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Is: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Document: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deselect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DisplayScopeNodePropertySheet: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DisplayScopeNodePropertySheet: usize,
    pub DisplaySelectionPropertySheet: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CopyScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CopyScopeNode: usize,
    pub CopySelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteScopeNode: usize,
    pub DeleteSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RenameScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RenameScopeNode: usize,
    pub RenameSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_ScopeNodeContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_ScopeNodeContextMenu: usize,
    pub SelectionContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RefreshScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RefreshScopeNode: usize,
    pub RefreshSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteSelectionMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExecuteScopeNodeMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExecuteScopeNodeMenuItem: usize,
    pub ExecuteShellCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScopeTreeVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetScopeTreeVisible: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Back: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forward: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatusBarText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Memento: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ViewMemento: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Columns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CellContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExportList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, _ExportListOptions) -> windows_core::HRESULT,
    pub ListViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut _ListViewMode) -> windows_core::HRESULT,
    pub SetListViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, _ListViewMode) -> windows_core::HRESULT,
    pub ControlObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait View_Impl: super::Com::IDispatch_Impl {
    fn ActiveScopeNode(&self) -> windows_core::Result<Node>;
    fn SetActiveScopeNode(&self, node: windows_core::Ref<Node>) -> windows_core::Result<()>;
    fn Selection(&self) -> windows_core::Result<Nodes>;
    fn ListItems(&self) -> windows_core::Result<Nodes>;
    fn SnapinScopeObject(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<super::Com::IDispatch>;
    fn SnapinSelectionObject(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Is(&self, view: windows_core::Ref<View>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Document(&self) -> windows_core::Result<Document>;
    fn SelectAll(&self) -> windows_core::Result<()>;
    fn Select(&self, node: windows_core::Ref<Node>) -> windows_core::Result<()>;
    fn Deselect(&self, node: windows_core::Ref<Node>) -> windows_core::Result<()>;
    fn IsSelected(&self, node: windows_core::Ref<Node>) -> windows_core::Result<windows_core::BOOL>;
    fn DisplayScopeNodePropertySheet(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn DisplaySelectionPropertySheet(&self) -> windows_core::Result<()>;
    fn CopyScopeNode(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn CopySelection(&self) -> windows_core::Result<()>;
    fn DeleteScopeNode(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteSelection(&self) -> windows_core::Result<()>;
    fn RenameScopeNode(&self, newname: &windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn RenameSelectedItem(&self, newname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ScopeNodeContextMenu(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<ContextMenu>;
    fn SelectionContextMenu(&self) -> windows_core::Result<ContextMenu>;
    fn RefreshScopeNode(&self, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn RefreshSelection(&self) -> windows_core::Result<()>;
    fn ExecuteSelectionMenuItem(&self, menuitempath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExecuteScopeNodeMenuItem(&self, menuitempath: &windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ExecuteShellCommand(&self, command: &windows_core::BSTR, directory: &windows_core::BSTR, parameters: &windows_core::BSTR, windowstate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Frame(&self) -> windows_core::Result<Frame>;
    fn Close(&self) -> windows_core::Result<()>;
    fn ScopeTreeVisible(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetScopeTreeVisible(&self, visible: windows_core::BOOL) -> windows_core::Result<()>;
    fn Back(&self) -> windows_core::Result<()>;
    fn Forward(&self) -> windows_core::Result<()>;
    fn SetStatusBarText(&self, statusbartext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Memento(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ViewMemento(&self, memento: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Columns(&self) -> windows_core::Result<Columns>;
    fn get_CellContents(&self, node: windows_core::Ref<Node>, column: i32) -> windows_core::Result<windows_core::BSTR>;
    fn ExportList(&self, file: &windows_core::BSTR, exportoptions: _ExportListOptions) -> windows_core::Result<()>;
    fn ListViewMode(&self) -> windows_core::Result<_ListViewMode>;
    fn SetListViewMode(&self, mode: _ListViewMode) -> windows_core::Result<()>;
    fn ControlObject(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl View_Vtbl {
    pub const fn new<Identity: View_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActiveScopeNode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::ActiveScopeNode(this) {
                    Ok(ok__) => {
                        node.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActiveScopeNode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::SetActiveScopeNode(this, core::mem::transmute_copy(&node)).into()
            }
        }
        unsafe extern "system" fn Selection<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::Selection(this) {
                    Ok(ok__) => {
                        nodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ListItems<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::ListItems(this) {
                    Ok(ok__) => {
                        nodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SnapinScopeObject<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT, scopenodeobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::SnapinScopeObject(this, core::mem::transmute(&scopenode)) {
                    Ok(ok__) => {
                        scopenodeobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SnapinSelectionObject<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::SnapinSelectionObject(this) {
                    Ok(ok__) => {
                        selectionobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Is<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: *mut core::ffi::c_void, thesame: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::Is(this, core::mem::transmute_copy(&view)) {
                    Ok(ok__) => {
                        thesame.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Document<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::Document(this) {
                    Ok(ok__) => {
                        document.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectAll<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::SelectAll(this).into()
            }
        }
        unsafe extern "system" fn Select<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::Select(this, core::mem::transmute_copy(&node)).into()
            }
        }
        unsafe extern "system" fn Deselect<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::Deselect(this, core::mem::transmute_copy(&node)).into()
            }
        }
        unsafe extern "system" fn IsSelected<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, isselected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::IsSelected(this, core::mem::transmute_copy(&node)) {
                    Ok(ok__) => {
                        isselected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayScopeNodePropertySheet<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::DisplayScopeNodePropertySheet(this, core::mem::transmute(&scopenode)).into()
            }
        }
        unsafe extern "system" fn DisplaySelectionPropertySheet<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::DisplaySelectionPropertySheet(this).into()
            }
        }
        unsafe extern "system" fn CopyScopeNode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::CopyScopeNode(this, core::mem::transmute(&scopenode)).into()
            }
        }
        unsafe extern "system" fn CopySelection<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::CopySelection(this).into()
            }
        }
        unsafe extern "system" fn DeleteScopeNode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::DeleteScopeNode(this, core::mem::transmute(&scopenode)).into()
            }
        }
        unsafe extern "system" fn DeleteSelection<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::DeleteSelection(this).into()
            }
        }
        unsafe extern "system" fn RenameScopeNode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newname: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::RenameScopeNode(this, core::mem::transmute(&newname), core::mem::transmute(&scopenode)).into()
            }
        }
        unsafe extern "system" fn RenameSelectedItem<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::RenameSelectedItem(this, core::mem::transmute(&newname)).into()
            }
        }
        unsafe extern "system" fn get_ScopeNodeContextMenu<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT, contextmenu: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::get_ScopeNodeContextMenu(this, core::mem::transmute(&scopenode)) {
                    Ok(ok__) => {
                        contextmenu.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectionContextMenu<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextmenu: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::SelectionContextMenu(this) {
                    Ok(ok__) => {
                        contextmenu.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RefreshScopeNode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::RefreshScopeNode(this, core::mem::transmute(&scopenode)).into()
            }
        }
        unsafe extern "system" fn RefreshSelection<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::RefreshSelection(this).into()
            }
        }
        unsafe extern "system" fn ExecuteSelectionMenuItem<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, menuitempath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::ExecuteSelectionMenuItem(this, core::mem::transmute(&menuitempath)).into()
            }
        }
        unsafe extern "system" fn ExecuteScopeNodeMenuItem<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, menuitempath: *mut core::ffi::c_void, scopenode: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::ExecuteScopeNodeMenuItem(this, core::mem::transmute(&menuitempath), core::mem::transmute(&scopenode)).into()
            }
        }
        unsafe extern "system" fn ExecuteShellCommand<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: *mut core::ffi::c_void, directory: *mut core::ffi::c_void, parameters: *mut core::ffi::c_void, windowstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::ExecuteShellCommand(this, core::mem::transmute(&command), core::mem::transmute(&directory), core::mem::transmute(&parameters), core::mem::transmute(&windowstate)).into()
            }
        }
        unsafe extern "system" fn Frame<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frame: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::Frame(this) {
                    Ok(ok__) => {
                        frame.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn ScopeTreeVisible<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::ScopeTreeVisible(this) {
                    Ok(ok__) => {
                        visible.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScopeTreeVisible<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::SetScopeTreeVisible(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn Back<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::Back(this).into()
            }
        }
        unsafe extern "system" fn Forward<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::Forward(this).into()
            }
        }
        unsafe extern "system" fn SetStatusBarText<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statusbartext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::SetStatusBarText(this, core::mem::transmute(&statusbartext)).into()
            }
        }
        unsafe extern "system" fn Memento<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memento: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::Memento(this) {
                    Ok(ok__) => {
                        memento.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ViewMemento<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memento: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::ViewMemento(this, core::mem::transmute(&memento)).into()
            }
        }
        unsafe extern "system" fn Columns<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, columns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::Columns(this) {
                    Ok(ok__) => {
                        columns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CellContents<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, column: i32, cellcontents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::get_CellContents(this, core::mem::transmute_copy(&node), core::mem::transmute_copy(&column)) {
                    Ok(ok__) => {
                        cellcontents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportList<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, exportoptions: _ExportListOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::ExportList(this, core::mem::transmute(&file), core::mem::transmute_copy(&exportoptions)).into()
            }
        }
        unsafe extern "system" fn ListViewMode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut _ListViewMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::ListViewMode(this) {
                    Ok(ok__) => {
                        mode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListViewMode<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: _ListViewMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                View_Impl::SetListViewMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn ControlObject<Identity: View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, control: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match View_Impl::ControlObject(this) {
                    Ok(ok__) => {
                        control.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ActiveScopeNode: ActiveScopeNode::<Identity, OFFSET>,
            SetActiveScopeNode: SetActiveScopeNode::<Identity, OFFSET>,
            Selection: Selection::<Identity, OFFSET>,
            ListItems: ListItems::<Identity, OFFSET>,
            SnapinScopeObject: SnapinScopeObject::<Identity, OFFSET>,
            SnapinSelectionObject: SnapinSelectionObject::<Identity, OFFSET>,
            Is: Is::<Identity, OFFSET>,
            Document: Document::<Identity, OFFSET>,
            SelectAll: SelectAll::<Identity, OFFSET>,
            Select: Select::<Identity, OFFSET>,
            Deselect: Deselect::<Identity, OFFSET>,
            IsSelected: IsSelected::<Identity, OFFSET>,
            DisplayScopeNodePropertySheet: DisplayScopeNodePropertySheet::<Identity, OFFSET>,
            DisplaySelectionPropertySheet: DisplaySelectionPropertySheet::<Identity, OFFSET>,
            CopyScopeNode: CopyScopeNode::<Identity, OFFSET>,
            CopySelection: CopySelection::<Identity, OFFSET>,
            DeleteScopeNode: DeleteScopeNode::<Identity, OFFSET>,
            DeleteSelection: DeleteSelection::<Identity, OFFSET>,
            RenameScopeNode: RenameScopeNode::<Identity, OFFSET>,
            RenameSelectedItem: RenameSelectedItem::<Identity, OFFSET>,
            get_ScopeNodeContextMenu: get_ScopeNodeContextMenu::<Identity, OFFSET>,
            SelectionContextMenu: SelectionContextMenu::<Identity, OFFSET>,
            RefreshScopeNode: RefreshScopeNode::<Identity, OFFSET>,
            RefreshSelection: RefreshSelection::<Identity, OFFSET>,
            ExecuteSelectionMenuItem: ExecuteSelectionMenuItem::<Identity, OFFSET>,
            ExecuteScopeNodeMenuItem: ExecuteScopeNodeMenuItem::<Identity, OFFSET>,
            ExecuteShellCommand: ExecuteShellCommand::<Identity, OFFSET>,
            Frame: Frame::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            ScopeTreeVisible: ScopeTreeVisible::<Identity, OFFSET>,
            SetScopeTreeVisible: SetScopeTreeVisible::<Identity, OFFSET>,
            Back: Back::<Identity, OFFSET>,
            Forward: Forward::<Identity, OFFSET>,
            SetStatusBarText: SetStatusBarText::<Identity, OFFSET>,
            Memento: Memento::<Identity, OFFSET>,
            ViewMemento: ViewMemento::<Identity, OFFSET>,
            Columns: Columns::<Identity, OFFSET>,
            get_CellContents: get_CellContents::<Identity, OFFSET>,
            ExportList: ExportList::<Identity, OFFSET>,
            ListViewMode: ListViewMode::<Identity, OFFSET>,
            SetListViewMode: SetListViewMode::<Identity, OFFSET>,
            ControlObject: ControlObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<View as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for View {}
pub const ViewOption_ActionPaneHidden: _ViewOptions = _ViewOptions(8i32);
pub const ViewOption_Default: _ViewOptions = _ViewOptions(0i32);
pub const ViewOption_NoToolBars: _ViewOptions = _ViewOptions(2i32);
pub const ViewOption_NotPersistable: _ViewOptions = _ViewOptions(4i32);
pub const ViewOption_ScopeTreeHidden: _ViewOptions = _ViewOptions(1i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Views, Views_Vtbl, 0xd6b8c29d_a1ff_4d72_aab0_e381e9b9338d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Views {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Views, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Views {
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<View> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, node: P0, viewoptions: _ViewOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), node.param().abi(), viewoptions).ok() }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Views_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, _ViewOptions) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Views_Impl: super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<View>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, node: windows_core::Ref<Node>, viewoptions: _ViewOptions) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Views_Vtbl {
    pub const fn new<Identity: Views_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: Views_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, view: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Views_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        view.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: Views_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Views_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: Views_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, viewoptions: _ViewOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Views_Impl::Add(this, core::mem::transmute_copy(&node), core::mem::transmute_copy(&viewoptions)).into()
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: Views_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Views_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Views as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Views {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_AppEvents, _AppEvents_Vtbl, 0xde46cbdd_53f5_4635_af54_4fe71e923d3f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _AppEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_AppEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _AppEvents {
    pub unsafe fn OnQuit<P0>(&self, application: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<_Application>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnQuit)(windows_core::Interface::as_raw(self), application.param().abi()).ok() }
    }
    pub unsafe fn OnDocumentOpen<P0>(&self, document: P0, new: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDocumentOpen)(windows_core::Interface::as_raw(self), document.param().abi(), new.into()).ok() }
    }
    pub unsafe fn OnDocumentClose<P0>(&self, document: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDocumentClose)(windows_core::Interface::as_raw(self), document.param().abi()).ok() }
    }
    pub unsafe fn OnSnapInAdded<P0, P1>(&self, document: P0, snapin: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
        P1: windows_core::Param<SnapIn>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSnapInAdded)(windows_core::Interface::as_raw(self), document.param().abi(), snapin.param().abi()).ok() }
    }
    pub unsafe fn OnSnapInRemoved<P0, P1>(&self, document: P0, snapin: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
        P1: windows_core::Param<SnapIn>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSnapInRemoved)(windows_core::Interface::as_raw(self), document.param().abi(), snapin.param().abi()).ok() }
    }
    pub unsafe fn OnNewView<P0>(&self, view: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNewView)(windows_core::Interface::as_raw(self), view.param().abi()).ok() }
    }
    pub unsafe fn OnViewClose<P0>(&self, view: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnViewClose)(windows_core::Interface::as_raw(self), view.param().abi()).ok() }
    }
    pub unsafe fn OnViewChange<P0, P1>(&self, view: P0, newownernode: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
        P1: windows_core::Param<Node>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnViewChange)(windows_core::Interface::as_raw(self), view.param().abi(), newownernode.param().abi()).ok() }
    }
    pub unsafe fn OnSelectionChange<P0, P1>(&self, view: P0, newnodes: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
        P1: windows_core::Param<Nodes>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSelectionChange)(windows_core::Interface::as_raw(self), view.param().abi(), newnodes.param().abi()).ok() }
    }
    pub unsafe fn OnContextMenuExecuted<P0>(&self, menuitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MenuItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnContextMenuExecuted)(windows_core::Interface::as_raw(self), menuitem.param().abi()).ok() }
    }
    pub unsafe fn OnToolbarButtonClicked(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnToolbarButtonClicked)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnListUpdated<P0>(&self, view: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnListUpdated)(windows_core::Interface::as_raw(self), view.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _AppEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub OnQuit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDocumentOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnDocumentClose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSnapInAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSnapInRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnNewView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnViewClose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnContextMenuExecuted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnToolbarButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnListUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _AppEvents_Impl: super::Com::IDispatch_Impl {
    fn OnQuit(&self, application: windows_core::Ref<_Application>) -> windows_core::Result<()>;
    fn OnDocumentOpen(&self, document: windows_core::Ref<Document>, new: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnDocumentClose(&self, document: windows_core::Ref<Document>) -> windows_core::Result<()>;
    fn OnSnapInAdded(&self, document: windows_core::Ref<Document>, snapin: windows_core::Ref<SnapIn>) -> windows_core::Result<()>;
    fn OnSnapInRemoved(&self, document: windows_core::Ref<Document>, snapin: windows_core::Ref<SnapIn>) -> windows_core::Result<()>;
    fn OnNewView(&self, view: windows_core::Ref<View>) -> windows_core::Result<()>;
    fn OnViewClose(&self, view: windows_core::Ref<View>) -> windows_core::Result<()>;
    fn OnViewChange(&self, view: windows_core::Ref<View>, newownernode: windows_core::Ref<Node>) -> windows_core::Result<()>;
    fn OnSelectionChange(&self, view: windows_core::Ref<View>, newnodes: windows_core::Ref<Nodes>) -> windows_core::Result<()>;
    fn OnContextMenuExecuted(&self, menuitem: windows_core::Ref<MenuItem>) -> windows_core::Result<()>;
    fn OnToolbarButtonClicked(&self) -> windows_core::Result<()>;
    fn OnListUpdated(&self, view: windows_core::Ref<View>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _AppEvents_Vtbl {
    pub const fn new<Identity: _AppEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnQuit<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, application: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnQuit(this, core::mem::transmute_copy(&application)).into()
            }
        }
        unsafe extern "system" fn OnDocumentOpen<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut core::ffi::c_void, new: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnDocumentOpen(this, core::mem::transmute_copy(&document), core::mem::transmute_copy(&new)).into()
            }
        }
        unsafe extern "system" fn OnDocumentClose<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnDocumentClose(this, core::mem::transmute_copy(&document)).into()
            }
        }
        unsafe extern "system" fn OnSnapInAdded<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut core::ffi::c_void, snapin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnSnapInAdded(this, core::mem::transmute_copy(&document), core::mem::transmute_copy(&snapin)).into()
            }
        }
        unsafe extern "system" fn OnSnapInRemoved<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut core::ffi::c_void, snapin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnSnapInRemoved(this, core::mem::transmute_copy(&document), core::mem::transmute_copy(&snapin)).into()
            }
        }
        unsafe extern "system" fn OnNewView<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnNewView(this, core::mem::transmute_copy(&view)).into()
            }
        }
        unsafe extern "system" fn OnViewClose<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnViewClose(this, core::mem::transmute_copy(&view)).into()
            }
        }
        unsafe extern "system" fn OnViewChange<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: *mut core::ffi::c_void, newownernode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnViewChange(this, core::mem::transmute_copy(&view), core::mem::transmute_copy(&newownernode)).into()
            }
        }
        unsafe extern "system" fn OnSelectionChange<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: *mut core::ffi::c_void, newnodes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnSelectionChange(this, core::mem::transmute_copy(&view), core::mem::transmute_copy(&newnodes)).into()
            }
        }
        unsafe extern "system" fn OnContextMenuExecuted<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, menuitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnContextMenuExecuted(this, core::mem::transmute_copy(&menuitem)).into()
            }
        }
        unsafe extern "system" fn OnToolbarButtonClicked<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnToolbarButtonClicked(this).into()
            }
        }
        unsafe extern "system" fn OnListUpdated<Identity: _AppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _AppEvents_Impl::OnListUpdated(this, core::mem::transmute_copy(&view)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OnQuit: OnQuit::<Identity, OFFSET>,
            OnDocumentOpen: OnDocumentOpen::<Identity, OFFSET>,
            OnDocumentClose: OnDocumentClose::<Identity, OFFSET>,
            OnSnapInAdded: OnSnapInAdded::<Identity, OFFSET>,
            OnSnapInRemoved: OnSnapInRemoved::<Identity, OFFSET>,
            OnNewView: OnNewView::<Identity, OFFSET>,
            OnViewClose: OnViewClose::<Identity, OFFSET>,
            OnViewChange: OnViewChange::<Identity, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, OFFSET>,
            OnContextMenuExecuted: OnContextMenuExecuted::<Identity, OFFSET>,
            OnToolbarButtonClicked: OnToolbarButtonClicked::<Identity, OFFSET>,
            OnListUpdated: OnListUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_AppEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _AppEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_Application, _Application_Vtbl, 0xa3afb9cc_b653_4741_86ab_f0470ec1384c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _Application {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_Application, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _Application {
    pub unsafe fn Help(&self) {
        unsafe { (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Quit(&self) {
        unsafe { (windows_core::Interface::vtable(self).Quit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Document(&self) -> windows_core::Result<Document> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Document)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Load(&self, filename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)).ok() }
    }
    pub unsafe fn Frame(&self) -> windows_core::Result<Frame> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Frame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Visible(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Show(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Hide(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Hide)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UserControl(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUserControl(&self, usercontrol: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserControl)(windows_core::Interface::as_raw(self), usercontrol.into()).ok() }
    }
    pub unsafe fn VersionMajor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VersionMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VersionMinor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VersionMinor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _Application_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Quit: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Document: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetUserControl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub VersionMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VersionMinor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _Application_Impl: super::Com::IDispatch_Impl {
    fn Help(&self);
    fn Quit(&self);
    fn Document(&self) -> windows_core::Result<Document>;
    fn Load(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Frame(&self) -> windows_core::Result<Frame>;
    fn Visible(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Show(&self) -> windows_core::Result<()>;
    fn Hide(&self) -> windows_core::Result<()>;
    fn UserControl(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetUserControl(&self, usercontrol: windows_core::BOOL) -> windows_core::Result<()>;
    fn VersionMajor(&self) -> windows_core::Result<i32>;
    fn VersionMinor(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _Application_Vtbl {
    pub const fn new<Identity: _Application_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Help<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _Application_Impl::Help(this)
            }
        }
        unsafe extern "system" fn Quit<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _Application_Impl::Quit(this)
            }
        }
        unsafe extern "system" fn Document<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _Application_Impl::Document(this) {
                    Ok(ok__) => {
                        document.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Load<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _Application_Impl::Load(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn Frame<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frame: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _Application_Impl::Frame(this) {
                    Ok(ok__) => {
                        frame.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Visible<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _Application_Impl::Visible(this) {
                    Ok(ok__) => {
                        visible.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Show<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _Application_Impl::Show(this).into()
            }
        }
        unsafe extern "system" fn Hide<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _Application_Impl::Hide(this).into()
            }
        }
        unsafe extern "system" fn UserControl<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usercontrol: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _Application_Impl::UserControl(this) {
                    Ok(ok__) => {
                        usercontrol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserControl<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usercontrol: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _Application_Impl::SetUserControl(this, core::mem::transmute_copy(&usercontrol)).into()
            }
        }
        unsafe extern "system" fn VersionMajor<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, versionmajor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _Application_Impl::VersionMajor(this) {
                    Ok(ok__) => {
                        versionmajor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VersionMinor<Identity: _Application_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, versionminor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _Application_Impl::VersionMinor(this) {
                    Ok(ok__) => {
                        versionminor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Help: Help::<Identity, OFFSET>,
            Quit: Quit::<Identity, OFFSET>,
            Document: Document::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Frame: Frame::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            Hide: Hide::<Identity, OFFSET>,
            UserControl: UserControl::<Identity, OFFSET>,
            SetUserControl: SetUserControl::<Identity, OFFSET>,
            VersionMajor: VersionMajor::<Identity, OFFSET>,
            VersionMinor: VersionMinor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_Application as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _Application {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _ColumnSortOrder(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DocumentMode(pub i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_EventConnector, _EventConnector_Vtbl, 0xc0bccd30_de44_4528_8403_a05a6a1cc8ea);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _EventConnector {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_EventConnector, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _EventConnector {
    pub unsafe fn ConnectTo<P0>(&self, application: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<_Application>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectTo)(windows_core::Interface::as_raw(self), application.param().abi()).ok() }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _EventConnector_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConnectTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _EventConnector_Impl: super::Com::IDispatch_Impl {
    fn ConnectTo(&self, application: windows_core::Ref<_Application>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _EventConnector_Vtbl {
    pub const fn new<Identity: _EventConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectTo<Identity: _EventConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, application: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _EventConnector_Impl::ConnectTo(this, core::mem::transmute_copy(&application)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: _EventConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _EventConnector_Impl::Disconnect(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ConnectTo: ConnectTo::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_EventConnector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _EventConnector {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _ExportListOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _ListViewMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _ViewOptions(pub i32);
