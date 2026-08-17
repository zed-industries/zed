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
impl AppEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct AppEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), width).ok()
    }
    pub unsafe fn DisplayPosition(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplayPosition(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisplayPosition)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Hidden(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Hidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHidden<P0>(&self, hidden: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHidden)(windows_core::Interface::as_raw(self), hidden.param().abi()).ok()
    }
    pub unsafe fn SetAsSortColumn(&self, sortorder: _ColumnSortOrder) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAsSortColumn)(windows_core::Interface::as_raw(self), sortorder).ok()
    }
    pub unsafe fn IsSortColumn(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSortColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Column_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisplayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDisplayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Hidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAsSortColumn: unsafe extern "system" fn(*mut core::ffi::c_void, _ColumnSortOrder) -> windows_core::HRESULT,
    pub IsSortColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<Column> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Columns_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, indexorpath: P0) -> windows_core::Result<MenuItem>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), indexorpath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ContextMenu_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SaveAs<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SaveAs)(windows_core::Interface::as_raw(self), filename.param().abi()).ok()
    }
    pub unsafe fn Close<P0>(&self, savechanges: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), savechanges.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Views(&self) -> windows_core::Result<Views> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Views)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SnapIns(&self) -> windows_core::Result<SnapIns> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SnapIns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActiveView(&self) -> windows_core::Result<View> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActiveView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Location(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Location)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsSaved(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSaved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Mode(&self) -> windows_core::Result<_DocumentMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Mode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMode(&self, mode: _DocumentMode) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootNode(&self) -> windows_core::Result<Node> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ScopeNamespace(&self) -> windows_core::Result<ScopeNamespace> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScopeNamespace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateProperties(&self) -> windows_core::Result<Properties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> windows_core::Result<_Application> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Document_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Views: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Views: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SnapIns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SnapIns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActiveView: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsSaved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut _DocumentMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, _DocumentMode) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ScopeNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ScopeNamespace: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Application: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Vendor(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Vendor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Extensions(&self) -> windows_core::Result<Extensions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SnapinCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SnapinCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnableAllExtensions<P0>(&self, enable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableAllExtensions)(windows_core::Interface::as_raw(self), enable.param().abi()).ok()
    }
    pub unsafe fn Enable<P0>(&self, enable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), enable.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Extension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Vendor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Extensions: usize,
    pub SnapinCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnableAllExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<Extension> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Extensions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Maximize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Minimize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Minimize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Restore(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Top(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Top)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTop)(windows_core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn Bottom(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Bottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBottom(&self, bottom: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBottom)(windows_core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn Left(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Left)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLeft)(windows_core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn Right(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Right)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRight(&self, right: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRight)(windows_core::Interface::as_raw(self), right).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
windows_core::imp::define_interface!(IColumnData, IColumnData_Vtbl, 0x547c1354_024d_11d3_a707_00c04f8ef4cb);
impl core::ops::Deref for IColumnData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IColumnData, windows_core::IUnknown);
impl IColumnData {
    pub unsafe fn SetColumnConfigData(&self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumnConfigData)(windows_core::Interface::as_raw(self), pcolid, pcolsetdata).ok()
    }
    pub unsafe fn GetColumnConfigData(&self, pcolid: *const SColumnSetID) -> windows_core::Result<*mut MMC_COLUMN_SET_DATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnConfigData)(windows_core::Interface::as_raw(self), pcolid, &mut result__).map(|| result__)
    }
    pub unsafe fn SetColumnSortData(&self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumnSortData)(windows_core::Interface::as_raw(self), pcolid, pcolsortdata).ok()
    }
    pub unsafe fn GetColumnSortData(&self, pcolid: *const SColumnSetID) -> windows_core::Result<*mut MMC_SORT_SET_DATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnSortData)(windows_core::Interface::as_raw(self), pcolid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IColumnData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetColumnConfigData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *const MMC_COLUMN_SET_DATA) -> windows_core::HRESULT,
    pub GetColumnConfigData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *mut *mut MMC_COLUMN_SET_DATA) -> windows_core::HRESULT,
    pub SetColumnSortData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *const MMC_SORT_SET_DATA) -> windows_core::HRESULT,
    pub GetColumnSortData: unsafe extern "system" fn(*mut core::ffi::c_void, *const SColumnSetID, *mut *mut MMC_SORT_SET_DATA) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComponent, IComponent_Vtbl, 0x43136eb2_d36c_11cf_adbc_00aa00a80033);
impl core::ops::Deref for IComponent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComponent, windows_core::IUnknown);
impl IComponent {
    pub unsafe fn Initialize<P0>(&self, lpconsole: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConsole>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lpconsole.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notify<P0, P1, P2>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: P1, param3: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), lpdataobject.param().abi(), event, arg.param().abi(), param3.param().abi()).ok()
    }
    pub unsafe fn Destroy(&self, cookie: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self), cookie).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDataObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDataObject)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut windows_core::PWSTR, pviewoptions: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResultViewType)(windows_core::Interface::as_raw(self), cookie, ppviewtype, pviewoptions).ok()
    }
    pub unsafe fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayInfo)(windows_core::Interface::as_raw(self), presultdataitem).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).CompareObjects)(windows_core::Interface::as_raw(self), lpdataobjecta.param().abi(), lpdataobjectb.param().abi()).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDispatch)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetResultViewType2(&self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResultViewType2)(windows_core::Interface::as_raw(self), cookie, presultviewtype).ok()
    }
    pub unsafe fn RestoreResultView(&self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestoreResultView)(windows_core::Interface::as_raw(self), cookie, presultviewtype).ok()
    }
}
#[repr(C)]
pub struct IComponent2_Vtbl {
    pub base__: IComponent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, isize, DATA_OBJECT_TYPES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
    pub GetResultViewType2: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *mut RESULT_VIEW_TYPE_INFO) -> windows_core::HRESULT,
    pub RestoreResultView: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *const RESULT_VIEW_TYPE_INFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComponentData, IComponentData_Vtbl, 0x955ab28a_5218_11d0_a985_00c04fd8d565);
impl core::ops::Deref for IComponentData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComponentData, windows_core::IUnknown);
impl IComponentData {
    pub unsafe fn Initialize<P0>(&self, punknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), punknown.param().abi()).ok()
    }
    pub unsafe fn CreateComponent(&self) -> windows_core::Result<IComponent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateComponent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notify<P0, P1, P2>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: P1, param3: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), lpdataobject.param().abi(), event, arg.param().abi(), param3.param().abi()).ok()
    }
    pub unsafe fn Destroy(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> windows_core::Result<super::Com::IDataObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDataObject)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayInfo)(windows_core::Interface::as_raw(self), pscopedataitem).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).CompareObjects)(windows_core::Interface::as_raw(self), lpdataobjecta.param().abi(), lpdataobjectb.param().abi()).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDispatch)(windows_core::Interface::as_raw(self), cookie, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IComponentData2_Vtbl {
    pub base__: IComponentData_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, isize, DATA_OBJECT_TYPES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
}
windows_core::imp::define_interface!(IConsole, IConsole_Vtbl, 0x43136eb1_d36c_11cf_adbc_00aa00a80033);
impl core::ops::Deref for IConsole {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsole, windows_core::IUnknown);
impl IConsole {
    pub unsafe fn SetHeader<P0>(&self, pheader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IHeaderCtrl>,
    {
        (windows_core::Interface::vtable(self).SetHeader)(windows_core::Interface::as_raw(self), pheader.param().abi()).ok()
    }
    pub unsafe fn SetToolbar<P0>(&self, ptoolbar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IToolbar>,
    {
        (windows_core::Interface::vtable(self).SetToolbar)(windows_core::Interface::as_raw(self), ptoolbar.param().abi()).ok()
    }
    pub unsafe fn QueryResultView(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryResultView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryScopeImageList(&self) -> windows_core::Result<IImageList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryScopeImageList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryResultImageList(&self) -> windows_core::Result<IImageList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryResultImageList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateAllViews<P0, P1>(&self, lpdataobject: P0, data: P1, hint: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).UpdateAllViews)(windows_core::Interface::as_raw(self), lpdataobject.param().abi(), data.param().abi(), hint).ok()
    }
    pub unsafe fn MessageBox<P0, P1>(&self, lpsztext: P0, lpsztitle: P1, fustyle: u32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MessageBox)(windows_core::Interface::as_raw(self), lpsztext.param().abi(), lpsztitle.param().abi(), fustyle, &mut result__).map(|| result__)
    }
    pub unsafe fn QueryConsoleVerb(&self) -> windows_core::Result<IConsoleVerb> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryConsoleVerb)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SelectScopeItem)(windows_core::Interface::as_raw(self), hscopeitem).ok()
    }
    pub unsafe fn GetMainWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMainWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NewWindow)(windows_core::Interface::as_raw(self), hscopeitem, loptions).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IConsole2, IConsole2_Vtbl, 0x103d842a_aa63_11d1_a7e1_00c04fd8d565);
impl core::ops::Deref for IConsole2 {
    type Target = IConsole;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsole2, windows_core::IUnknown, IConsole);
impl IConsole2 {
    pub unsafe fn Expand<P0>(&self, hitem: isize, bexpand: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), hitem, bexpand.param().abi()).ok()
    }
    pub unsafe fn IsTaskpadViewPreferred(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsTaskpadViewPreferred)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatusText<P0>(&self, pszstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetStatusText)(windows_core::Interface::as_raw(self), pszstatustext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IConsole2_Vtbl {
    pub base__: IConsole_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, isize, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsTaskpadViewPreferred: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatusText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).RenameScopeItem)(windows_core::Interface::as_raw(self), hscopeitem).ok()
    }
}
#[repr(C)]
pub struct IConsole3_Vtbl {
    pub base__: IConsole2_Vtbl,
    pub RenameScopeItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConsoleNameSpace, IConsoleNameSpace_Vtbl, 0xbedeb620_f24d_11cf_8afc_00aa003ca9f6);
impl core::ops::Deref for IConsoleNameSpace {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsoleNameSpace, windows_core::IUnknown);
impl IConsoleNameSpace {
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertItem)(windows_core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), hitem, fdeletethis).ok()
    }
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetItem)(windows_core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChildItem)(windows_core::Interface::as_raw(self), item, pitemchild, pcookie).ok()
    }
    pub unsafe fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextItem)(windows_core::Interface::as_raw(self), item, pitemnext, pcookie).ok()
    }
    pub unsafe fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParentItem)(windows_core::Interface::as_raw(self), item, pitemparent, pcookie).ok()
    }
}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), hitem).ok()
    }
    pub unsafe fn AddExtension(&self, hitem: isize, lpclsid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddExtension)(windows_core::Interface::as_raw(self), hitem, lpclsid).ok()
    }
}
#[repr(C)]
pub struct IConsoleNameSpace2_Vtbl {
    pub base__: IConsoleNameSpace_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub AddExtension: unsafe extern "system" fn(*mut core::ffi::c_void, isize, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConsolePower, IConsolePower_Vtbl, 0x1cfbdd0e_62ca_49ce_a3af_dbb2de61b068);
impl core::ops::Deref for IConsolePower {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsolePower, windows_core::IUnknown);
impl IConsolePower {
    pub unsafe fn SetExecutionState(&self, dwadd: u32, dwremove: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExecutionState)(windows_core::Interface::as_raw(self), dwadd, dwremove).ok()
    }
    pub unsafe fn ResetIdleTimer(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetIdleTimer)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
}
#[repr(C)]
pub struct IConsolePower_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetExecutionState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ResetIdleTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConsolePowerSink, IConsolePowerSink_Vtbl, 0x3333759f_fe4f_4975_b143_fec0a5dd6d65);
impl core::ops::Deref for IConsolePowerSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsolePowerSink, windows_core::IUnknown);
impl IConsolePowerSink {
    pub unsafe fn OnPowerBroadcast<P0>(&self, nevent: u32, lparam: P0) -> windows_core::Result<super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnPowerBroadcast)(windows_core::Interface::as_raw(self), nevent, lparam.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IConsolePowerSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPowerBroadcast: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::LPARAM, *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConsoleVerb, IConsoleVerb_Vtbl, 0xe49f7a60_74af_11d0_a286_00c04fd8fe93);
impl core::ops::Deref for IConsoleVerb {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConsoleVerb, windows_core::IUnknown);
impl IConsoleVerb {
    pub unsafe fn GetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVerbState)(windows_core::Interface::as_raw(self), ecmdid, nstate, &mut result__).map(|| result__)
    }
    pub unsafe fn SetVerbState<P0>(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVerbState)(windows_core::Interface::as_raw(self), ecmdid, nstate, bstate.param().abi()).ok()
    }
    pub unsafe fn SetDefaultVerb(&self, ecmdid: MMC_CONSOLE_VERB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultVerb)(windows_core::Interface::as_raw(self), ecmdid).ok()
    }
    pub unsafe fn GetDefaultVerb(&self) -> windows_core::Result<MMC_CONSOLE_VERB> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultVerb)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IConsoleVerb_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVerbState: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONSOLE_VERB, MMC_BUTTON_STATE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetVerbState: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONSOLE_VERB, MMC_BUTTON_STATE, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetDefaultVerb: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONSOLE_VERB) -> windows_core::HRESULT,
    pub GetDefaultVerb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MMC_CONSOLE_VERB) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextMenuCallback, IContextMenuCallback_Vtbl, 0x43136eb7_d36c_11cf_adbc_00aa00a80033);
impl core::ops::Deref for IContextMenuCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContextMenuCallback, windows_core::IUnknown);
impl IContextMenuCallback {
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pitem).ok()
    }
}
#[repr(C)]
pub struct IContextMenuCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTEXTMENUITEM) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextMenuCallback2, IContextMenuCallback2_Vtbl, 0xe178bc0e_2ed0_4b5e_8097_42c9087e8b33);
impl core::ops::Deref for IContextMenuCallback2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContextMenuCallback2, windows_core::IUnknown);
impl IContextMenuCallback2 {
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pitem).ok()
    }
}
#[repr(C)]
pub struct IContextMenuCallback2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTEXTMENUITEM2) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).EmptyMenuList)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPrimaryExtensionItems<P0, P1>(&self, piextension: P0, pidataobject: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).AddPrimaryExtensionItems)(windows_core::Interface::as_raw(self), piextension.param().abi(), pidataobject.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddThirdPartyExtensionItems<P0>(&self, pidataobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).AddThirdPartyExtensionItems)(windows_core::Interface::as_raw(self), pidataobject.param().abi()).ok()
    }
    pub unsafe fn ShowContextMenu<P0>(&self, hwndparent: P0, xpos: i32, ypos: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), xpos, ypos, &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IControlbar, IControlbar_Vtbl, 0x69fb811e_6c1c_11d0_a2cb_00c04fd909dd);
impl core::ops::Deref for IControlbar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IControlbar, windows_core::IUnknown);
impl IControlbar {
    pub unsafe fn Create<P0>(&self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IExtendControlbar>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), ntype, pextendcontrolbar.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Attach<P0>(&self, ntype: MMC_CONTROL_TYPE, lpunknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), ntype, lpunknown.param().abi()).ok()
    }
    pub unsafe fn Detach<P0>(&self, lpunknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self), lpunknown.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IControlbar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONTROL_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_CONTROL_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayHelp, IDisplayHelp_Vtbl, 0xcc593830_b926_11d1_8063_0000f875a9ce);
impl core::ops::Deref for IDisplayHelp {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDisplayHelp, windows_core::IUnknown);
impl IDisplayHelp {
    pub unsafe fn ShowTopic<P0>(&self, pszhelptopic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ShowTopic)(windows_core::Interface::as_raw(self), pszhelptopic.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDisplayHelp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShowTopic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumTASK, IEnumTASK_Vtbl, 0x338698b1_5a02_11d1_9fec_00600832db4a);
impl core::ops::Deref for IEnumTASK {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumTASK, windows_core::IUnknown);
impl IEnumTASK {
    pub unsafe fn Next(&self, rgelt: &mut [MMC_TASK], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTASK> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumTASK_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MMC_TASK, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExtendContextMenu, IExtendContextMenu_Vtbl, 0x4f3b7a4f_cfac_11cf_b8e3_00c04fd8d5b0);
impl core::ops::Deref for IExtendContextMenu {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExtendContextMenu, windows_core::IUnknown);
impl IExtendContextMenu {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddMenuItems<P0, P1>(&self, pidataobject: P0, picallback: P1, pinsertionallowed: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<IContextMenuCallback>,
    {
        (windows_core::Interface::vtable(self).AddMenuItems)(windows_core::Interface::as_raw(self), pidataobject.param().abi(), picallback.param().abi(), pinsertionallowed).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Command<P0>(&self, lcommandid: i32, pidataobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).Command)(windows_core::Interface::as_raw(self), lcommandid, pidataobject.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IExtendControlbar, IExtendControlbar_Vtbl, 0x49506520_6f40_11d0_a98b_00c04fd8d565);
impl core::ops::Deref for IExtendControlbar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExtendControlbar, windows_core::IUnknown);
impl IExtendControlbar {
    pub unsafe fn SetControlbar<P0>(&self, pcontrolbar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IControlbar>,
    {
        (windows_core::Interface::vtable(self).SetControlbar)(windows_core::Interface::as_raw(self), pcontrolbar.param().abi()).ok()
    }
    pub unsafe fn ControlbarNotify<P0, P1>(&self, event: MMC_NOTIFY_TYPE, arg: P0, param2: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).ControlbarNotify)(windows_core::Interface::as_raw(self), event, arg.param().abi(), param2.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IExtendControlbar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetControlbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlbarNotify: unsafe extern "system" fn(*mut core::ffi::c_void, MMC_NOTIFY_TYPE, super::super::Foundation::LPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExtendPropertySheet, IExtendPropertySheet_Vtbl, 0x85de64dc_ef21_11cf_a285_00c04fd8dbe6);
impl core::ops::Deref for IExtendPropertySheet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExtendPropertySheet, windows_core::IUnknown);
impl IExtendPropertySheet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<P0, P1>(&self, lpprovider: P0, handle: isize, lpidataobject: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPropertySheetCallback>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).CreatePropertyPages)(windows_core::Interface::as_raw(self), lpprovider.param().abi(), handle, lpidataobject.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<P0>(&self, lpdataobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).QueryPagesFor)(windows_core::Interface::as_raw(self), lpdataobject.param().abi()).ok()
    }
}
#[repr(C)]
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
    pub unsafe fn GetWatermarks<P0>(&self, lpidataobject: P0, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).GetWatermarks)(windows_core::Interface::as_raw(self), lpidataobject.param().abi(), lphwatermark, lphheader, lphpalette, bstretch).ok()
    }
}
#[repr(C)]
pub struct IExtendPropertySheet2_Vtbl {
    pub base__: IExtendPropertySheet_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetWatermarks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HPALETTE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetWatermarks: usize,
}
windows_core::imp::define_interface!(IExtendTaskPad, IExtendTaskPad_Vtbl, 0x8dee6511_554d_11d1_9fea_00600832db4a);
impl core::ops::Deref for IExtendTaskPad {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExtendTaskPad, windows_core::IUnknown);
impl IExtendTaskPad {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TaskNotify<P0>(&self, pdo: P0, arg: *const windows_core::VARIANT, param2: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).TaskNotify)(windows_core::Interface::as_raw(self), pdo.param().abi(), core::mem::transmute(arg), core::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTasks<P0, P1>(&self, pdo: P0, sztaskgroup: P1) -> windows_core::Result<IEnumTASK>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumTasks)(windows_core::Interface::as_raw(self), pdo.param().abi(), sztaskgroup.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTitle<P0>(&self, pszgroup: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTitle)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDescriptiveText<P0>(&self, pszgroup: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescriptiveText)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBackground<P0>(&self, pszgroup: P0) -> windows_core::Result<MMC_TASK_DISPLAY_OBJECT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBackground)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetListPadInfo<P0>(&self, pszgroup: P0) -> windows_core::Result<MMC_LISTPAD_INFO>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetListPadInfo)(windows_core::Interface::as_raw(self), pszgroup.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IExtendTaskPad_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub TaskNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
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
windows_core::imp::define_interface!(IExtendView, IExtendView_Vtbl, 0x89995cee_d2ed_4c0e_ae5e_df7e76f3fa53);
impl core::ops::Deref for IExtendView {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExtendView, windows_core::IUnknown);
impl IExtendView {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetViews<P0, P1>(&self, pdataobject: P0, pviewextensioncallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<IViewExtensionCallback>,
    {
        (windows_core::Interface::vtable(self).GetViews)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), pviewextensioncallback.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IExtendView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetViews: usize,
}
windows_core::imp::define_interface!(IHeaderCtrl, IHeaderCtrl_Vtbl, 0x43136eb3_d36c_11cf_adbc_00aa00a80033);
impl core::ops::Deref for IHeaderCtrl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHeaderCtrl, windows_core::IUnknown);
impl IHeaderCtrl {
    pub unsafe fn InsertColumn<P0>(&self, ncol: i32, title: P0, nformat: i32, nwidth: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InsertColumn)(windows_core::Interface::as_raw(self), ncol, title.param().abi(), nformat, nwidth).ok()
    }
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteColumn)(windows_core::Interface::as_raw(self), ncol).ok()
    }
    pub unsafe fn SetColumnText<P0>(&self, ncol: i32, title: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetColumnText)(windows_core::Interface::as_raw(self), ncol, title.param().abi()).ok()
    }
    pub unsafe fn GetColumnText(&self, ncol: i32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnText)(windows_core::Interface::as_raw(self), ncol, &mut result__).map(|| result__)
    }
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumnWidth)(windows_core::Interface::as_raw(self), ncol, nwidth).ok()
    }
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnWidth)(windows_core::Interface::as_raw(self), ncol, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IHeaderCtrl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InsertColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, i32, i32) -> windows_core::HRESULT,
    pub DeleteColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetColumnText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetColumnText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetColumnWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetColumnWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetChangeTimeOut)(windows_core::Interface::as_raw(self), utimeout).ok()
    }
    pub unsafe fn SetColumnFilter(&self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumnFilter)(windows_core::Interface::as_raw(self), ncolumn, dwtype, pfilterdata).ok()
    }
    pub unsafe fn GetColumnFilter(&self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnFilter)(windows_core::Interface::as_raw(self), ncolumn, pdwtype, pfilterdata).ok()
    }
}
#[repr(C)]
pub struct IHeaderCtrl2_Vtbl {
    pub base__: IHeaderCtrl_Vtbl,
    pub SetChangeTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetColumnFilter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const MMC_FILTERDATA) -> windows_core::HRESULT,
    pub GetColumnFilter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut MMC_FILTERDATA) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageList, IImageList_Vtbl, 0x43136eb8_d36c_11cf_adbc_00aa00a80033);
impl core::ops::Deref for IImageList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageList, windows_core::IUnknown);
impl IImageList {
    pub unsafe fn ImageListSetIcon(&self, picon: *const isize, nloc: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ImageListSetIcon)(windows_core::Interface::as_raw(self), picon, nloc).ok()
    }
    pub unsafe fn ImageListSetStrip<P0>(&self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::COLORREF>,
    {
        (windows_core::Interface::vtable(self).ImageListSetStrip)(windows_core::Interface::as_raw(self), pbmapsm, pbmaplg, nstartloc, cmask.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IImageList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ImageListSetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *const isize, i32) -> windows_core::HRESULT,
    pub ImageListSetStrip: unsafe extern "system" fn(*mut core::ffi::c_void, *const isize, *const isize, i32, super::super::Foundation::COLORREF) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMMCVersionInfo, IMMCVersionInfo_Vtbl, 0xa8d2c5fe_cdcb_4b9d_bde5_a27343ff54bc);
impl core::ops::Deref for IMMCVersionInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMMCVersionInfo, windows_core::IUnknown);
impl IMMCVersionInfo {
    pub unsafe fn GetMMCVersion(&self, pversionmajor: *mut i32, pversionminor: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMMCVersion)(windows_core::Interface::as_raw(self), pversionmajor, pversionminor).ok()
    }
}
#[repr(C)]
pub struct IMMCVersionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMMCVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMenuButton, IMenuButton_Vtbl, 0x951ed750_d080_11d0_b197_000000000000);
impl core::ops::Deref for IMenuButton {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMenuButton, windows_core::IUnknown);
impl IMenuButton {
    pub unsafe fn AddButton<P0, P1>(&self, idcommand: i32, lpbuttontext: P0, lptooltiptext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddButton)(windows_core::Interface::as_raw(self), idcommand, lpbuttontext.param().abi(), lptooltiptext.param().abi()).ok()
    }
    pub unsafe fn SetButton<P0, P1>(&self, idcommand: i32, lpbuttontext: P0, lptooltiptext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetButton)(windows_core::Interface::as_raw(self), idcommand, lpbuttontext.param().abi(), lptooltiptext.param().abi()).ok()
    }
    pub unsafe fn SetButtonState<P0>(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetButtonState)(windows_core::Interface::as_raw(self), idcommand, nstate, bstate.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMenuButton_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, MMC_BUTTON_STATE, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMessageView, IMessageView_Vtbl, 0x80f94174_fccc_11d2_b991_00c04f8ecd78);
impl core::ops::Deref for IMessageView {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMessageView, windows_core::IUnknown);
impl IMessageView {
    pub unsafe fn SetTitleText<P0>(&self, psztitletext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTitleText)(windows_core::Interface::as_raw(self), psztitletext.param().abi()).ok()
    }
    pub unsafe fn SetBodyText<P0>(&self, pszbodytext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetBodyText)(windows_core::Interface::as_raw(self), pszbodytext.param().abi()).ok()
    }
    pub unsafe fn SetIcon(&self, id: IconIdentifier) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIcon)(windows_core::Interface::as_raw(self), id).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMessageView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTitleText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetBodyText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, IconIdentifier) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INodeProperties, INodeProperties_Vtbl, 0x15bc4d24_a522_4406_aa55_0749537a6865);
impl core::ops::Deref for INodeProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INodeProperties, windows_core::IUnknown);
impl INodeProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperty<P0, P1>(&self, pdataobject: P0, szpropertyname: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), szpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct INodeProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProperty: usize,
}
windows_core::imp::define_interface!(IPropertySheetCallback, IPropertySheetCallback_Vtbl, 0x85de64dd_ef21_11cf_a285_00c04fd8dbe6);
impl core::ops::Deref for IPropertySheetCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertySheetCallback, windows_core::IUnknown);
impl IPropertySheetCallback {
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPage<P0>(&self, hpage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Controls::HPROPSHEETPAGE>,
    {
        (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), hpage.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn RemovePage<P0>(&self, hpage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Controls::HPROPSHEETPAGE>,
    {
        (windows_core::Interface::vtable(self).RemovePage)(windows_core::Interface::as_raw(self), hpage.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IPropertySheetProvider, IPropertySheetProvider_Vtbl, 0x85de64de_ef21_11cf_a285_00c04fd8dbe6);
impl core::ops::Deref for IPropertySheetProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertySheetProvider, windows_core::IUnknown);
impl IPropertySheetProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertySheet<P0, P1>(&self, title: P0, r#type: u8, cookie: isize, pidataobjectm: P1, dwoptions: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).CreatePropertySheet)(windows_core::Interface::as_raw(self), title.param().abi(), r#type, cookie, pidataobjectm.param().abi(), dwoptions).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindPropertySheet<P0, P1>(&self, hitem: isize, lpcomponent: P0, lpdataobject: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IComponent>,
        P1: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).FindPropertySheet)(windows_core::Interface::as_raw(self), hitem, lpcomponent.param().abi(), lpdataobject.param().abi()).ok()
    }
    pub unsafe fn AddPrimaryPages<P0, P1, P2, P3>(&self, lpunknown: P0, bcreatehandle: P1, hnotifywindow: P2, bscopepane: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::HWND>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddPrimaryPages)(windows_core::Interface::as_raw(self), lpunknown.param().abi(), bcreatehandle.param().abi(), hnotifywindow.param().abi(), bscopepane.param().abi()).ok()
    }
    pub unsafe fn AddExtensionPages(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddExtensionPages)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Show(&self, window: isize, page: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), window, page).ok()
    }
}
#[repr(C)]
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
    pub AddPrimaryPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::HWND, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub AddExtensionPages: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, isize, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRequiredExtensions, IRequiredExtensions_Vtbl, 0x72782d7a_a4a0_11d1_af0f_00c04fb6dd2c);
impl core::ops::Deref for IRequiredExtensions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRequiredExtensions, windows_core::IUnknown);
impl IRequiredExtensions {
    pub unsafe fn EnableAllExtensions(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableAllExtensions)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetFirstExtension(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFirstExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNextExtension(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRequiredExtensions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableAllExtensions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetNextExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResultData, IResultData_Vtbl, 0x31da5fa0_e0eb_11cf_9f21_00aa003ca9f6);
impl core::ops::Deref for IResultData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResultData, windows_core::IUnknown);
impl IResultData {
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertItem)(windows_core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), itemid, ncol).ok()
    }
    pub unsafe fn FindItemByLParam<P0>(&self, lparam: P0) -> windows_core::Result<isize>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindItemByLParam)(windows_core::Interface::as_raw(self), lparam.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn DeleteAllRsltItems(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAllRsltItems)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetItem)(windows_core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextItem)(windows_core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModifyItemState)(windows_core::Interface::as_raw(self), nindex, itemid, uadd, uremove).ok()
    }
    pub unsafe fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModifyViewStyle)(windows_core::Interface::as_raw(self), add, remove).ok()
    }
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetViewMode)(windows_core::Interface::as_raw(self), lviewmode).ok()
    }
    pub unsafe fn GetViewMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UpdateItem(&self, itemid: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateItem)(windows_core::Interface::as_raw(self), itemid).ok()
    }
    pub unsafe fn Sort<P0>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).Sort)(windows_core::Interface::as_raw(self), ncolumn, dwsortoptions, luserparam.param().abi()).ok()
    }
    pub unsafe fn SetDescBarText<P0>(&self, desctext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescBarText)(windows_core::Interface::as_raw(self), desctext.param().abi()).ok()
    }
    pub unsafe fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetItemCount)(windows_core::Interface::as_raw(self), nitemcount, dwoptions).ok()
    }
}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).RenameResultItem)(windows_core::Interface::as_raw(self), itemid).ok()
    }
}
#[repr(C)]
pub struct IResultData2_Vtbl {
    pub base__: IResultData_Vtbl,
    pub RenameResultItem: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResultDataCompare, IResultDataCompare_Vtbl, 0xe8315a52_7a1a_11d0_a2d2_00c04fd909dd);
impl core::ops::Deref for IResultDataCompare {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResultDataCompare, windows_core::IUnknown);
impl IResultDataCompare {
    pub unsafe fn Compare<P0>(&self, luserparam: P0, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), luserparam.param().abi(), cookiea, cookieb, pnresult).ok()
    }
}
#[repr(C)]
pub struct IResultDataCompare_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::LPARAM, isize, isize, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResultDataCompareEx, IResultDataCompareEx_Vtbl, 0x96933476_0251_11d3_aeb0_00c04f8ecd78);
impl core::ops::Deref for IResultDataCompareEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResultDataCompareEx, windows_core::IUnknown);
impl IResultDataCompareEx {
    pub unsafe fn Compare(&self, prdc: *const RDCOMPARE) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), prdc, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IResultDataCompareEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *const RDCOMPARE, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResultOwnerData, IResultOwnerData_Vtbl, 0x9cb396d8_ea83_11d0_aef1_00c04fb6dd2c);
impl core::ops::Deref for IResultOwnerData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResultOwnerData, windows_core::IUnknown);
impl IResultOwnerData {
    pub unsafe fn FindItem(&self, pfindinfo: *const RESULTFINDINFO) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindItem)(windows_core::Interface::as_raw(self), pfindinfo, &mut result__).map(|| result__)
    }
    pub unsafe fn CacheHint(&self, nstartindex: i32, nendindex: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CacheHint)(windows_core::Interface::as_raw(self), nstartindex, nendindex).ok()
    }
    pub unsafe fn SortItems<P0>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).SortItems)(windows_core::Interface::as_raw(self), ncolumn, dwsortoptions, luserparam.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IResultOwnerData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const RESULTFINDINFO, *mut i32) -> windows_core::HRESULT,
    pub CacheHint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SortItems: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISnapinAbout, ISnapinAbout_Vtbl, 0x1245208c_a151_11d0_a7d7_00c04fd909dd);
impl core::ops::Deref for ISnapinAbout {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISnapinAbout, windows_core::IUnknown);
impl ISnapinAbout {
    pub unsafe fn GetSnapinDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSnapinDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProvider(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSnapinVersion(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSnapinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetSnapinImage(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSnapinImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetStaticFolderImage(&self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStaticFolderImage)(windows_core::Interface::as_raw(self), hsmallimage, hsmallimageopen, hlargeimage, cmask).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(ISnapinHelp, ISnapinHelp_Vtbl, 0xa6b15ace_df59_11d0_a7dd_00c04fd909dd);
impl core::ops::Deref for ISnapinHelp {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISnapinHelp, windows_core::IUnknown);
impl ISnapinHelp {
    pub unsafe fn GetHelpTopic(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHelpTopic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISnapinHelp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHelpTopic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLinkedTopics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISnapinHelp2_Vtbl {
    pub base__: ISnapinHelp_Vtbl,
    pub GetLinkedTopics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISnapinProperties, ISnapinProperties_Vtbl, 0xf7889da9_4a02_4837_bf89_1a6f2a021010);
impl core::ops::Deref for ISnapinProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISnapinProperties, windows_core::IUnknown);
impl ISnapinProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Properties>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pproperties.param().abi()).ok()
    }
    pub unsafe fn QueryPropertyNames<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISnapinPropertiesCallback>,
    {
        (windows_core::Interface::vtable(self).QueryPropertyNames)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
    pub unsafe fn PropertiesChanged(&self, pproperties: &[MMC_SNAPIN_PROPERTY]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PropertiesChanged)(windows_core::Interface::as_raw(self), pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ISnapinProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub QueryPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const MMC_SNAPIN_PROPERTY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISnapinPropertiesCallback, ISnapinPropertiesCallback_Vtbl, 0xa50fa2e5_7e61_45eb_a8d4_9a07b3e851a8);
impl core::ops::Deref for ISnapinPropertiesCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISnapinPropertiesCallback, windows_core::IUnknown);
impl ISnapinPropertiesCallback {
    pub unsafe fn AddPropertyName<P0>(&self, pszpropname: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddPropertyName)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), dwflags).ok()
    }
}
#[repr(C)]
pub struct ISnapinPropertiesCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStringTable, IStringTable_Vtbl, 0xde40b7a4_0f65_11d2_8e25_00c04f8ecd78);
impl core::ops::Deref for IStringTable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStringTable, windows_core::IUnknown);
impl IStringTable {
    pub unsafe fn AddString<P0>(&self, pszadd: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddString)(windows_core::Interface::as_raw(self), pszadd.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetString(&self, stringid: u32, lpbuffer: &mut [u16], pcchout: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), stringid, lpbuffer.len().try_into().unwrap(), core::mem::transmute(lpbuffer.as_ptr()), pcchout).ok()
    }
    pub unsafe fn GetStringLength(&self, stringid: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), stringid, &mut result__).map(|| result__)
    }
    pub unsafe fn DeleteString(&self, stringid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteString)(windows_core::Interface::as_raw(self), stringid).ok()
    }
    pub unsafe fn DeleteAllStrings(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAllStrings)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FindString<P0>(&self, pszfind: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindString)(windows_core::Interface::as_raw(self), pszfind.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enumerate(&self) -> windows_core::Result<super::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enumerate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IToolbar, IToolbar_Vtbl, 0x43136eb9_d36c_11cf_adbc_00aa00a80033);
impl core::ops::Deref for IToolbar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IToolbar, windows_core::IUnknown);
impl IToolbar {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddBitmap<P0, P1>(&self, nimages: i32, hbmp: P0, cxsize: i32, cysize: i32, crmask: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HBITMAP>,
        P1: windows_core::Param<super::super::Foundation::COLORREF>,
    {
        (windows_core::Interface::vtable(self).AddBitmap)(windows_core::Interface::as_raw(self), nimages, hbmp.param().abi(), cxsize, cysize, crmask.param().abi()).ok()
    }
    pub unsafe fn AddButtons(&self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddButtons)(windows_core::Interface::as_raw(self), nbuttons, lpbuttons).ok()
    }
    pub unsafe fn InsertButton(&self, nindex: i32, lpbutton: *const MMCBUTTON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertButton)(windows_core::Interface::as_raw(self), nindex, lpbutton).ok()
    }
    pub unsafe fn DeleteButton(&self, nindex: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteButton)(windows_core::Interface::as_raw(self), nindex).ok()
    }
    pub unsafe fn GetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetButtonState)(windows_core::Interface::as_raw(self), idcommand, nstate, &mut result__).map(|| result__)
    }
    pub unsafe fn SetButtonState<P0>(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetButtonState)(windows_core::Interface::as_raw(self), idcommand, nstate, bstate.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IToolbar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Graphics::Gdi::HBITMAP, i32, i32, super::super::Foundation::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddBitmap: usize,
    pub AddButtons: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const MMCBUTTON) -> windows_core::HRESULT,
    pub InsertButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const MMCBUTTON) -> windows_core::HRESULT,
    pub DeleteButton: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, MMC_BUTTON_STATE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, MMC_BUTTON_STATE, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewExtensionCallback, IViewExtensionCallback_Vtbl, 0x34dd928a_7599_41e5_9f5e_d6bc3062c2da);
impl core::ops::Deref for IViewExtensionCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewExtensionCallback, windows_core::IUnknown);
impl IViewExtensionCallback {
    pub unsafe fn AddView(&self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddView)(windows_core::Interface::as_raw(self), pextviewdata).ok()
    }
}
#[repr(C)]
pub struct IViewExtensionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddView: unsafe extern "system" fn(*mut core::ffi::c_void, *const MMC_EXT_VIEW_DATA) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LanguageIndependentName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LanguageIndependentName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LanguageIndependentPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LanguageIndependentPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Execute(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct MenuItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LanguageIndependentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LanguageIndependentPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Property<P0>(&self, propertyname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), propertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Bookmark(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Bookmark)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsScopeNode(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsScopeNode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Nodetype(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Nodetype)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Node_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Bookmark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Nodetype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<Node> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Nodes_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, name: P0) -> windows_core::Result<Property>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Remove<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Properties_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Property_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParent<P0>(&self, node: P0) -> windows_core::Result<Node>
    where
        P0: windows_core::Param<Node>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChild<P0>(&self, node: P0) -> windows_core::Result<Node>
    where
        P0: windows_core::Param<Node>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNext<P0>(&self, node: P0) -> windows_core::Result<Node>
    where
        P0: windows_core::Param<Node>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRoot(&self) -> windows_core::Result<Node> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Expand<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ScopeNamespace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRoot: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Expand: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Vendor(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Vendor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Extensions(&self) -> windows_core::Result<Extensions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SnapinCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SnapinCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<Properties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnableAllExtensions<P0>(&self, enable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableAllExtensions)(windows_core::Interface::as_raw(self), enable.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SnapIn_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Vendor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Extensions: usize,
    pub SnapinCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub EnableAllExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<SnapIn> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1, P2>(&self, snapinnameorclsid: P0, parentsnapin: P1, properties: P2) -> windows_core::Result<SnapIn>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), snapinnameorclsid.param().abi(), parentsnapin.param().abi(), properties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove<P0>(&self, snapin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SnapIn>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), snapin.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SnapIns_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActiveScopeNode(&self) -> windows_core::Result<Node> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActiveScopeNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetActiveScopeNode<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        (windows_core::Interface::vtable(self).SetActiveScopeNode)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Selection(&self) -> windows_core::Result<Nodes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Selection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ListItems(&self) -> windows_core::Result<Nodes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ListItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SnapinScopeObject<P0>(&self, scopenode: P0) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SnapinScopeObject)(windows_core::Interface::as_raw(self), scopenode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SnapinSelectionObject(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SnapinSelectionObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Is<P0>(&self, view: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<View>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Is)(windows_core::Interface::as_raw(self), view.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Document(&self) -> windows_core::Result<Document> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Document)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SelectAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SelectAll)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Select<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deselect<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        (windows_core::Interface::vtable(self).Deselect)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsSelected<P0>(&self, node: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<Node>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSelected)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayScopeNodePropertySheet<P0>(&self, scopenode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DisplayScopeNodePropertySheet)(windows_core::Interface::as_raw(self), scopenode.param().abi()).ok()
    }
    pub unsafe fn DisplaySelectionPropertySheet(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisplaySelectionPropertySheet)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CopyScopeNode<P0>(&self, scopenode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).CopyScopeNode)(windows_core::Interface::as_raw(self), scopenode.param().abi()).ok()
    }
    pub unsafe fn CopySelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopySelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteScopeNode<P0>(&self, scopenode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteScopeNode)(windows_core::Interface::as_raw(self), scopenode.param().abi()).ok()
    }
    pub unsafe fn DeleteSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RenameScopeNode<P0, P1>(&self, newname: P0, scopenode: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RenameScopeNode)(windows_core::Interface::as_raw(self), newname.param().abi(), scopenode.param().abi()).ok()
    }
    pub unsafe fn RenameSelectedItem<P0>(&self, newname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RenameSelectedItem)(windows_core::Interface::as_raw(self), newname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ScopeNodeContextMenu<P0>(&self, scopenode: P0) -> windows_core::Result<ContextMenu>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ScopeNodeContextMenu)(windows_core::Interface::as_raw(self), scopenode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectionContextMenu(&self) -> windows_core::Result<ContextMenu> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SelectionContextMenu)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RefreshScopeNode<P0>(&self, scopenode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RefreshScopeNode)(windows_core::Interface::as_raw(self), scopenode.param().abi()).ok()
    }
    pub unsafe fn RefreshSelection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefreshSelection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExecuteSelectionMenuItem<P0>(&self, menuitempath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExecuteSelectionMenuItem)(windows_core::Interface::as_raw(self), menuitempath.param().abi()).ok()
    }
    pub unsafe fn ExecuteScopeNodeMenuItem<P0, P1>(&self, menuitempath: P0, scopenode: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).ExecuteScopeNodeMenuItem)(windows_core::Interface::as_raw(self), menuitempath.param().abi(), scopenode.param().abi()).ok()
    }
    pub unsafe fn ExecuteShellCommand<P0, P1, P2, P3>(&self, command: P0, directory: P1, parameters: P2, windowstate: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExecuteShellCommand)(windows_core::Interface::as_raw(self), command.param().abi(), directory.param().abi(), parameters.param().abi(), windowstate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Frame(&self) -> windows_core::Result<Frame> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Frame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ScopeTreeVisible(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScopeTreeVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScopeTreeVisible<P0>(&self, visible: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetScopeTreeVisible)(windows_core::Interface::as_raw(self), visible.param().abi()).ok()
    }
    pub unsafe fn Back(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Back)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forward(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Forward)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatusBarText<P0>(&self, statusbartext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetStatusBarText)(windows_core::Interface::as_raw(self), statusbartext.param().abi()).ok()
    }
    pub unsafe fn Memento(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Memento)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ViewMemento<P0>(&self, memento: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ViewMemento)(windows_core::Interface::as_raw(self), memento.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Columns(&self) -> windows_core::Result<Columns> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Columns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_CellContents<P0>(&self, node: P0, column: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<Node>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_CellContents)(windows_core::Interface::as_raw(self), node.param().abi(), column, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExportList<P0>(&self, file: P0, exportoptions: _ExportListOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExportList)(windows_core::Interface::as_raw(self), file.param().abi(), exportoptions).ok()
    }
    pub unsafe fn ListViewMode(&self) -> windows_core::Result<_ListViewMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ListViewMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetListViewMode(&self, mode: _ListViewMode) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetListViewMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ControlObject(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ControlObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct View_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActiveScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActiveScopeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetActiveScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetActiveScopeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Selection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Selection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ListItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ListItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SnapinScopeObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SnapinScopeObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SnapinSelectionObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SnapinSelectionObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Is: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Is: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Document: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Document: usize,
    pub SelectAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Select: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deselect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deselect: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsSelected: usize,
    pub DisplayScopeNodePropertySheet: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DisplaySelectionPropertySheet: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CopySelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RenameScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RenameSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ScopeNodeContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ScopeNodeContextMenu: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectionContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectionContextMenu: usize,
    pub RefreshScopeNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RefreshSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteSelectionMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ExecuteScopeNodeMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ExecuteShellCommand: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Frame: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScopeTreeVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetScopeTreeVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Back: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forward: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatusBarText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Memento: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ViewMemento: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Columns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Columns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_CellContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_CellContents: usize,
    pub ExportList: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, _ExportListOptions) -> windows_core::HRESULT,
    pub ListViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut _ListViewMode) -> windows_core::HRESULT,
    pub SetListViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, _ListViewMode) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ControlObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ControlObject: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<View> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, node: P0, viewoptions: _ViewOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Node>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), node.param().abi(), viewoptions).ok()
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Views_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, _ViewOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuit<P0>(&self, application: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<_Application>,
    {
        (windows_core::Interface::vtable(self).OnQuit)(windows_core::Interface::as_raw(self), application.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDocumentOpen<P0, P1>(&self, document: P0, new: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnDocumentOpen)(windows_core::Interface::as_raw(self), document.param().abi(), new.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDocumentClose<P0>(&self, document: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
    {
        (windows_core::Interface::vtable(self).OnDocumentClose)(windows_core::Interface::as_raw(self), document.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSnapInAdded<P0, P1>(&self, document: P0, snapin: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
        P1: windows_core::Param<SnapIn>,
    {
        (windows_core::Interface::vtable(self).OnSnapInAdded)(windows_core::Interface::as_raw(self), document.param().abi(), snapin.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSnapInRemoved<P0, P1>(&self, document: P0, snapin: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Document>,
        P1: windows_core::Param<SnapIn>,
    {
        (windows_core::Interface::vtable(self).OnSnapInRemoved)(windows_core::Interface::as_raw(self), document.param().abi(), snapin.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnNewView<P0>(&self, view: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
    {
        (windows_core::Interface::vtable(self).OnNewView)(windows_core::Interface::as_raw(self), view.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnViewClose<P0>(&self, view: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
    {
        (windows_core::Interface::vtable(self).OnViewClose)(windows_core::Interface::as_raw(self), view.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnViewChange<P0, P1>(&self, view: P0, newownernode: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
        P1: windows_core::Param<Node>,
    {
        (windows_core::Interface::vtable(self).OnViewChange)(windows_core::Interface::as_raw(self), view.param().abi(), newownernode.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSelectionChange<P0, P1>(&self, view: P0, newnodes: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
        P1: windows_core::Param<Nodes>,
    {
        (windows_core::Interface::vtable(self).OnSelectionChange)(windows_core::Interface::as_raw(self), view.param().abi(), newnodes.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnContextMenuExecuted<P0>(&self, menuitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MenuItem>,
    {
        (windows_core::Interface::vtable(self).OnContextMenuExecuted)(windows_core::Interface::as_raw(self), menuitem.param().abi()).ok()
    }
    pub unsafe fn OnToolbarButtonClicked(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnToolbarButtonClicked)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnListUpdated<P0>(&self, view: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<View>,
    {
        (windows_core::Interface::vtable(self).OnListUpdated)(windows_core::Interface::as_raw(self), view.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _AppEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQuit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQuit: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDocumentOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDocumentOpen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDocumentClose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDocumentClose: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSnapInAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSnapInAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSnapInRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSnapInRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnNewView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnNewView: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnViewClose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnViewClose: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnViewChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSelectionChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnContextMenuExecuted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnContextMenuExecuted: usize,
    pub OnToolbarButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnListUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnListUpdated: usize,
}
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
        (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Quit(&self) {
        (windows_core::Interface::vtable(self).Quit)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Document(&self) -> windows_core::Result<Document> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Document)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Load<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), filename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Frame(&self) -> windows_core::Result<Frame> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Frame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Visible(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Show(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Hide(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Hide)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UserControl(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUserControl<P0>(&self, usercontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUserControl)(windows_core::Interface::as_raw(self), usercontrol.param().abi()).ok()
    }
    pub unsafe fn VersionMajor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VersionMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VersionMinor(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VersionMinor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _Application_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Quit: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_System_Com")]
    pub Document: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Document: usize,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Frame: usize,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetUserControl: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub VersionMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VersionMinor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConnectTo<P0>(&self, application: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<_Application>,
    {
        (windows_core::Interface::vtable(self).ConnectTo)(windows_core::Interface::as_raw(self), application.param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _EventConnector_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ConnectTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConnectTo: usize,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const AUTO_WIDTH: i32 = -1i32;
pub const BUTTONPRESSED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(16i32);
pub const CCM_COMMANDID_MASK_RESERVED: CCM_COMMANDID_MASK_CONSTANTS = CCM_COMMANDID_MASK_CONSTANTS(4294901760u32);
pub const CCM_INSERTIONALLOWED_NEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(2i32);
pub const CCM_INSERTIONALLOWED_TASK: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(4i32);
pub const CCM_INSERTIONALLOWED_TOP: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(1i32);
pub const CCM_INSERTIONALLOWED_VIEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(8i32);
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
pub const DocumentMode_Author: _DocumentMode = _DocumentMode(0i32);
pub const DocumentMode_User: _DocumentMode = _DocumentMode(1i32);
pub const DocumentMode_User_MDI: _DocumentMode = _DocumentMode(2i32);
pub const DocumentMode_User_SDI: _DocumentMode = _DocumentMode(3i32);
pub const ENABLED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(1i32);
pub const ExportListOptions_Default: _ExportListOptions = _ExportListOptions(0i32);
pub const ExportListOptions_SelectedItemsOnly: _ExportListOptions = _ExportListOptions(4i32);
pub const ExportListOptions_TabDelimited: _ExportListOptions = _ExportListOptions(2i32);
pub const ExportListOptions_Unicode: _ExportListOptions = _ExportListOptions(1i32);
pub const HDI_HIDDEN: u32 = 1u32;
pub const HIDDEN: MMC_BUTTON_STATE = MMC_BUTTON_STATE(4i32);
pub const HIDE_COLUMN: i32 = -4i32;
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
pub const INDETERMINATE: MMC_BUTTON_STATE = MMC_BUTTON_STATE(8i32);
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
pub const MFCC_DISABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(0i32);
pub const MFCC_ENABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(1i32);
pub const MFCC_VALUE_CHANGE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(2i32);
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
pub const MMC_ACTION_ID: MMC_ACTION_TYPE = MMC_ACTION_TYPE(0i32);
pub const MMC_ACTION_LINK: MMC_ACTION_TYPE = MMC_ACTION_TYPE(1i32);
pub const MMC_ACTION_SCRIPT: MMC_ACTION_TYPE = MMC_ACTION_TYPE(2i32);
pub const MMC_ACTION_UNINITIALIZED: MMC_ACTION_TYPE = MMC_ACTION_TYPE(-1i32);
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
pub const MMC_ENSUREFOCUSVISIBLE: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(8i32);
pub const MMC_FILTER_NOVALUE: MMC_FILTER_TYPE = MMC_FILTER_TYPE(32768i32);
pub const MMC_IMAGECALLBACK: i32 = -1i32;
pub const MMC_INT_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(1i32);
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
pub const MMC_NOSORTHEADER: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(4i32);
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
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
pub const MMC_PROP_PERSIST: u32 = 8u32;
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
pub const MMC_PSO_HASHELP: u32 = 2u32;
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
pub const MMC_SCOPE_ITEM_STATE_BOLD: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(2i32);
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(3i32);
pub const MMC_SCOPE_ITEM_STATE_NORMAL: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(1i32);
pub const MMC_SHOWSELALWAYS: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(2i32);
pub const MMC_SINGLESEL: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(1i32);
pub const MMC_STRING_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(0i32);
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
pub const MMC_VIEW_TYPE_HTML: MMC_VIEW_TYPE = MMC_VIEW_TYPE(1i32);
pub const MMC_VIEW_TYPE_LIST: MMC_VIEW_TYPE = MMC_VIEW_TYPE(0i32);
pub const MMC_VIEW_TYPE_OCX: MMC_VIEW_TYPE = MMC_VIEW_TYPE(2i32);
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
pub const RDCI_ScopeItem: u32 = 2147483648u32;
pub const RDI_IMAGE: u32 = 4u32;
pub const RDI_INDENT: u32 = 64u32;
pub const RDI_INDEX: u32 = 32u32;
pub const RDI_PARAM: u32 = 16u32;
pub const RDI_STATE: u32 = 8u32;
pub const RDI_STR: u32 = 2u32;
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
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
pub const SortOrder_Ascending: _ColumnSortOrder = _ColumnSortOrder(0i32);
pub const SortOrder_Descending: _ColumnSortOrder = _ColumnSortOrder(1i32);
pub const TOOLBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(0i32);
pub const ViewOption_ActionPaneHidden: _ViewOptions = _ViewOptions(8i32);
pub const ViewOption_Default: _ViewOptions = _ViewOptions(0i32);
pub const ViewOption_NoToolBars: _ViewOptions = _ViewOptions(2i32);
pub const ViewOption_NotPersistable: _ViewOptions = _ViewOptions(4i32);
pub const ViewOption_ScopeTreeHidden: _ViewOptions = _ViewOptions(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CCM_COMMANDID_MASK_CONSTANTS(pub u32);
impl windows_core::TypeKind for CCM_COMMANDID_MASK_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CCM_COMMANDID_MASK_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CCM_COMMANDID_MASK_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CCM_INSERTIONALLOWED(pub i32);
impl windows_core::TypeKind for CCM_INSERTIONALLOWED {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CCM_INSERTIONALLOWED {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CCM_INSERTIONALLOWED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CCM_INSERTIONPOINTID(pub i32);
impl windows_core::TypeKind for CCM_INSERTIONPOINTID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CCM_INSERTIONPOINTID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CCM_INSERTIONPOINTID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CCM_SPECIAL(pub i32);
impl windows_core::TypeKind for CCM_SPECIAL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CCM_SPECIAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CCM_SPECIAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DATA_OBJECT_TYPES(pub i32);
impl windows_core::TypeKind for DATA_OBJECT_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DATA_OBJECT_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DATA_OBJECT_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IconIdentifier(pub i32);
impl windows_core::TypeKind for IconIdentifier {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IconIdentifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IconIdentifier").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_ACTION_TYPE(pub i32);
impl windows_core::TypeKind for MMC_ACTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_ACTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_ACTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_BUTTON_STATE(pub i32);
impl windows_core::TypeKind for MMC_BUTTON_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_BUTTON_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_BUTTON_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_CONSOLE_VERB(pub i32);
impl windows_core::TypeKind for MMC_CONSOLE_VERB {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_CONSOLE_VERB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_CONSOLE_VERB").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_CONTROL_TYPE(pub i32);
impl windows_core::TypeKind for MMC_CONTROL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_CONTROL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_CONTROL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_FILTER_CHANGE_CODE(pub i32);
impl windows_core::TypeKind for MMC_FILTER_CHANGE_CODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_FILTER_CHANGE_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_FILTER_CHANGE_CODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_FILTER_TYPE(pub i32);
impl windows_core::TypeKind for MMC_FILTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_FILTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_FILTER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_MENU_COMMAND_IDS(pub i32);
impl windows_core::TypeKind for MMC_MENU_COMMAND_IDS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_MENU_COMMAND_IDS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_MENU_COMMAND_IDS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_NOTIFY_TYPE(pub i32);
impl windows_core::TypeKind for MMC_NOTIFY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_NOTIFY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_NOTIFY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_PROPERTY_ACTION(pub i32);
impl windows_core::TypeKind for MMC_PROPERTY_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_PROPERTY_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_PROPERTY_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_RESULT_VIEW_STYLE(pub i32);
impl windows_core::TypeKind for MMC_RESULT_VIEW_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_RESULT_VIEW_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_RESULT_VIEW_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_SCOPE_ITEM_STATE(pub i32);
impl windows_core::TypeKind for MMC_SCOPE_ITEM_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_SCOPE_ITEM_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_SCOPE_ITEM_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_TASK_DISPLAY_TYPE(pub i32);
impl windows_core::TypeKind for MMC_TASK_DISPLAY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_TASK_DISPLAY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_TASK_DISPLAY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MMC_VIEW_TYPE(pub i32);
impl windows_core::TypeKind for MMC_VIEW_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MMC_VIEW_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MMC_VIEW_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _ColumnSortOrder(pub i32);
impl windows_core::TypeKind for _ColumnSortOrder {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _ColumnSortOrder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_ColumnSortOrder").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _DocumentMode(pub i32);
impl windows_core::TypeKind for _DocumentMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _DocumentMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DocumentMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _ExportListOptions(pub i32);
impl windows_core::TypeKind for _ExportListOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _ExportListOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_ExportListOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _ListViewMode(pub i32);
impl windows_core::TypeKind for _ListViewMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _ListViewMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_ListViewMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _ViewOptions(pub i32);
impl windows_core::TypeKind for _ViewOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _ViewOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_ViewOptions").field(&self.0).finish()
    }
}
pub const AppEventsDHTMLConnector: windows_core::GUID = windows_core::GUID::from_u128(0xade6444b_c91f_4e37_92a4_5bb430a33340);
pub const Application: windows_core::GUID = windows_core::GUID::from_u128(0x49b2791a_b1ae_4c90_9b8e_e860ba07f889);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONTEXTMENUITEM {
    pub strName: windows_core::PWSTR,
    pub strStatusBarText: windows_core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
}
impl windows_core::TypeKind for CONTEXTMENUITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONTEXTMENUITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONTEXTMENUITEM2 {
    pub strName: windows_core::PWSTR,
    pub strStatusBarText: windows_core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
    pub strLanguageIndependentName: windows_core::PWSTR,
}
impl windows_core::TypeKind for CONTEXTMENUITEM2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONTEXTMENUITEM2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ConsolePower: windows_core::GUID = windows_core::GUID::from_u128(0xf0285374_dff1_11d3_b433_00c04f8ecd78);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MENUBUTTONDATA {
    pub idCommand: i32,
    pub x: i32,
    pub y: i32,
}
impl windows_core::TypeKind for MENUBUTTONDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MENUBUTTONDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMCBUTTON {
    pub nBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsType: u8,
    pub lpButtonText: windows_core::PWSTR,
    pub lpTooltipText: windows_core::PWSTR,
}
impl windows_core::TypeKind for MMCBUTTON {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMCBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MMCVersionInfo: windows_core::GUID = windows_core::GUID::from_u128(0xd6fedb1d_cf21_4bd9_af3b_c5468e9c6684);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_COLUMN_DATA {
    pub nColIndex: i32,
    pub dwFlags: u32,
    pub nWidth: i32,
    pub ulReserved: usize,
}
impl windows_core::TypeKind for MMC_COLUMN_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_COLUMN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_COLUMN_SET_DATA {
    pub cbSize: i32,
    pub nNumCols: i32,
    pub pColData: *mut MMC_COLUMN_DATA,
}
impl windows_core::TypeKind for MMC_COLUMN_SET_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_COLUMN_SET_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_EXPANDSYNC_STRUCT {
    pub bHandled: super::super::Foundation::BOOL,
    pub bExpanding: super::super::Foundation::BOOL,
    pub hItem: isize,
}
impl windows_core::TypeKind for MMC_EXPANDSYNC_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_EXPANDSYNC_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_EXT_VIEW_DATA {
    pub viewID: windows_core::GUID,
    pub pszURL: windows_core::PCWSTR,
    pub pszViewTitle: windows_core::PCWSTR,
    pub pszTooltipText: windows_core::PCWSTR,
    pub bReplacesDefaultView: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for MMC_EXT_VIEW_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_EXT_VIEW_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_FILTERDATA {
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub lValue: i32,
}
impl windows_core::TypeKind for MMC_FILTERDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_FILTERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_LISTPAD_INFO {
    pub szTitle: windows_core::PWSTR,
    pub szButtonText: windows_core::PWSTR,
    pub nCommandID: isize,
}
impl windows_core::TypeKind for MMC_LISTPAD_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_LISTPAD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_RESTORE_VIEW {
    pub dwSize: u32,
    pub cookie: isize,
    pub pViewType: windows_core::PWSTR,
    pub lViewOptions: i32,
}
impl windows_core::TypeKind for MMC_RESTORE_VIEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_RESTORE_VIEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct MMC_SNAPIN_PROPERTY {
    pub pszPropName: windows_core::PCWSTR,
    pub varValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
    pub eAction: MMC_PROPERTY_ACTION,
}
impl Clone for MMC_SNAPIN_PROPERTY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MMC_SNAPIN_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_SNAPIN_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_SORT_DATA {
    pub nColIndex: i32,
    pub dwSortOptions: u32,
    pub ulReserved: usize,
}
impl windows_core::TypeKind for MMC_SORT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_SORT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_SORT_SET_DATA {
    pub cbSize: i32,
    pub nNumItems: i32,
    pub pSortData: *mut MMC_SORT_DATA,
}
impl windows_core::TypeKind for MMC_SORT_SET_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_SORT_SET_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MMC_TASK {
    pub sDisplayObject: MMC_TASK_DISPLAY_OBJECT,
    pub szText: windows_core::PWSTR,
    pub szHelpString: windows_core::PWSTR,
    pub eActionType: MMC_ACTION_TYPE,
    pub Anonymous: MMC_TASK_0,
}
impl windows_core::TypeKind for MMC_TASK {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MMC_TASK_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_TASK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_TASK_DISPLAY_BITMAP {
    pub szMouseOverBitmap: windows_core::PWSTR,
    pub szMouseOffBitmap: windows_core::PWSTR,
}
impl windows_core::TypeKind for MMC_TASK_DISPLAY_BITMAP {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_TASK_DISPLAY_BITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MMC_TASK_DISPLAY_OBJECT {
    pub eDisplayType: MMC_TASK_DISPLAY_TYPE,
    pub Anonymous: MMC_TASK_DISPLAY_OBJECT_0,
}
impl windows_core::TypeKind for MMC_TASK_DISPLAY_OBJECT {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MMC_TASK_DISPLAY_OBJECT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_TASK_DISPLAY_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_TASK_DISPLAY_SYMBOL {
    pub szFontFamilyName: windows_core::PWSTR,
    pub szURLtoEOT: windows_core::PWSTR,
    pub szSymbolString: windows_core::PWSTR,
}
impl windows_core::TypeKind for MMC_TASK_DISPLAY_SYMBOL {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_TASK_DISPLAY_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MMC_VISIBLE_COLUMNS {
    pub nVisibleColumns: i32,
    pub rgVisibleCols: [i32; 1],
}
impl windows_core::TypeKind for MMC_VISIBLE_COLUMNS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MMC_VISIBLE_COLUMNS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RDCOMPARE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub nColumn: i32,
    pub lUserParam: super::super::Foundation::LPARAM,
    pub prdch1: *mut RDITEMHDR,
    pub prdch2: *mut RDITEMHDR,
}
impl windows_core::TypeKind for RDCOMPARE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RDCOMPARE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RDITEMHDR {
    pub dwFlags: u32,
    pub cookie: isize,
    pub lpReserved: super::super::Foundation::LPARAM,
}
impl windows_core::TypeKind for RDITEMHDR {
    type TypeKind = windows_core::CopyType;
}
impl Default for RDITEMHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESULTDATAITEM {
    pub mask: u32,
    pub bScopeItem: super::super::Foundation::BOOL,
    pub itemID: isize,
    pub nIndex: i32,
    pub nCol: i32,
    pub str: windows_core::PWSTR,
    pub nImage: i32,
    pub nState: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
}
impl windows_core::TypeKind for RESULTDATAITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESULTDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESULTFINDINFO {
    pub psz: windows_core::PWSTR,
    pub nStart: i32,
    pub dwOptions: u32,
}
impl windows_core::TypeKind for RESULTFINDINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESULTFINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for RESULT_VIEW_TYPE_INFO {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for RESULT_VIEW_TYPE_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESULT_VIEW_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESULT_VIEW_TYPE_INFO_0_0 {
    pub dwHTMLOptions: u32,
    pub pstrURL: windows_core::PWSTR,
}
impl windows_core::TypeKind for RESULT_VIEW_TYPE_INFO_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESULT_VIEW_TYPE_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct RESULT_VIEW_TYPE_INFO_0_1 {
    pub dwOCXOptions: u32,
    pub pUnkControl: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for RESULT_VIEW_TYPE_INFO_0_1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for RESULT_VIEW_TYPE_INFO_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESULT_VIEW_TYPE_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for SCOPEDATAITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCOPEDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SColumnSetID {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl windows_core::TypeKind for SColumnSetID {
    type TypeKind = windows_core::CopyType;
}
impl Default for SColumnSetID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct SMMCDataObjects {
    pub count: u32,
    pub lpDataObject: [core::mem::ManuallyDrop<Option<super::Com::IDataObject>>; 1usize],
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for SMMCDataObjects {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SMMCDataObjects {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SMMCDataObjects {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SMMCObjectTypes {
    pub count: u32,
    pub guid: [windows_core::GUID; 1],
}
impl windows_core::TypeKind for SMMCObjectTypes {
    type TypeKind = windows_core::CopyType;
}
impl Default for SMMCObjectTypes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SNodeID {
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl windows_core::TypeKind for SNodeID {
    type TypeKind = windows_core::CopyType;
}
impl Default for SNodeID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SNodeID2 {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl windows_core::TypeKind for SNodeID2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SNodeID2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
