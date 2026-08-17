windows_core::imp::define_interface!(IPhotoAcquire, IPhotoAcquire_Vtbl, 0x00f23353_e31b_4955_a8ad_ca5ebf31e2ce);
impl core::ops::Deref for IPhotoAcquire {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquire, windows_core::IUnknown);
impl IPhotoAcquire {
    pub unsafe fn CreatePhotoSource<P0>(&self, pszdevice: P0) -> windows_core::Result<IPhotoAcquireSource>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePhotoSource)(windows_core::Interface::as_raw(self), pszdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Acquire<P0, P1, P2, P3, P4>(&self, pphotoacquiresource: P0, fshowprogress: P1, hwndparent: P2, pszapplicationname: P3, pphotoacquireprogresscb: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::HWND>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<IPhotoAcquireProgressCB>,
    {
        (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi(), fshowprogress.param().abi(), hwndparent.param().abi(), pszapplicationname.param().abi(), pphotoacquireprogresscb.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumResults(&self) -> windows_core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumResults)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPhotoAcquire_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePhotoSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::HWND, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumResults: usize,
}
windows_core::imp::define_interface!(IPhotoAcquireDeviceSelectionDialog, IPhotoAcquireDeviceSelectionDialog_Vtbl, 0x00f28837_55dd_4f37_aaf5_6855a9640467);
impl core::ops::Deref for IPhotoAcquireDeviceSelectionDialog {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquireDeviceSelectionDialog, windows_core::IUnknown);
impl IPhotoAcquireDeviceSelectionDialog {
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), psztitle.param().abi()).ok()
    }
    pub unsafe fn SetSubmitButtonText<P0>(&self, pszsubmitbuttontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubmitButtonText)(windows_core::Interface::as_raw(self), pszsubmitbuttontext.param().abi()).ok()
    }
    pub unsafe fn DoModal<P0>(&self, hwndparent: P0, dwdeviceflags: u32, pbstrdeviceid: Option<*mut windows_core::BSTR>, pndevicetype: Option<*mut DEVICE_SELECTION_DEVICE_TYPE>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DoModal)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), dwdeviceflags, core::mem::transmute(pbstrdeviceid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pndevicetype.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSubmitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DoModal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut DEVICE_SELECTION_DEVICE_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoAcquireItem, IPhotoAcquireItem_Vtbl, 0x00f21c97_28bf_4c02_b842_5e4e90139a30);
impl core::ops::Deref for IPhotoAcquireItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquireItem, windows_core::IUnknown);
impl IPhotoAcquireItem {
    pub unsafe fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetThumbnail(&self, sizethumbnail: super::super::Foundation::SIZE) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThumbnail)(windows_core::Interface::as_raw(self), core::mem::transmute(sizethumbnail), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), key, core::mem::transmute(pv)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CanDelete(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanDelete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSubItemCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSubItemAt(&self, nitemindex: u32) -> windows_core::Result<IPhotoAcquireItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubItemAt)(windows_core::Interface::as_raw(self), nitemindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPhotoAcquireItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::SIZE, *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetThumbnail: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    SetProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    pub CanDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSubItemAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoAcquireOptionsDialog, IPhotoAcquireOptionsDialog_Vtbl, 0x00f2b3ee_bf64_47ee_89f4_4dedd79643f2);
impl core::ops::Deref for IPhotoAcquireOptionsDialog {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquireOptionsDialog, windows_core::IUnknown);
impl IPhotoAcquireOptionsDialog {
    pub unsafe fn Initialize<P0>(&self, pszregistryroot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pszregistryroot.param().abi()).ok()
    }
    pub unsafe fn Create<P0>(&self, hwndparent: P0) -> windows_core::Result<super::super::Foundation::HWND>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Destroy(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DoModal<P0>(&self, hwndparent: P0, ppnreturncode: Option<*mut isize>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DoModal)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), core::mem::transmute(ppnreturncode.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveData(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveData)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPhotoAcquireOptionsDialog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoModal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut isize) -> windows_core::HRESULT,
    pub SaveData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoAcquirePlugin, IPhotoAcquirePlugin_Vtbl, 0x00f2dceb_ecb8_4f77_8e47_e7a987c83dd0);
impl core::ops::Deref for IPhotoAcquirePlugin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquirePlugin, windows_core::IUnknown);
impl IPhotoAcquirePlugin {
    pub unsafe fn Initialize<P0, P1>(&self, pphotoacquiresource: P0, pphotoacquireprogresscb: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
        P1: windows_core::Param<IPhotoAcquireProgressCB>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi(), pphotoacquireprogresscb.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn ProcessItem<P0, P1, P2, P3>(&self, dwacquirestage: u32, pphotoacquireitem: P0, poriginalitemstream: P1, pszfinalfilename: P2, ppropertystore: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireItem>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (windows_core::Interface::vtable(self).ProcessItem)(windows_core::Interface::as_raw(self), dwacquirestage, pphotoacquireitem.param().abi(), poriginalitemstream.param().abi(), pszfinalfilename.param().abi(), ppropertystore.param().abi()).ok()
    }
    pub unsafe fn TransferComplete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransferComplete)(windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn DisplayConfigureDialog<P0>(&self, hwndparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DisplayConfigureDialog)(windows_core::Interface::as_raw(self), hwndparent.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPhotoAcquirePlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub ProcessItem: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    ProcessItem: usize,
    pub TransferComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub DisplayConfigureDialog: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoAcquireProgressCB, IPhotoAcquireProgressCB_Vtbl, 0x00f2ce1e_935e_4248_892c_130f32c45cb4);
impl core::ops::Deref for IPhotoAcquireProgressCB {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquireProgressCB, windows_core::IUnknown);
impl IPhotoAcquireProgressCB {
    pub unsafe fn Cancelled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cancelled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StartEnumeration<P0>(&self, pphotoacquiresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
    {
        (windows_core::Interface::vtable(self).StartEnumeration)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi()).ok()
    }
    pub unsafe fn FoundItem<P0>(&self, pphotoacquireitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireItem>,
    {
        (windows_core::Interface::vtable(self).FoundItem)(windows_core::Interface::as_raw(self), pphotoacquireitem.param().abi()).ok()
    }
    pub unsafe fn EndEnumeration(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn StartTransfer<P0>(&self, pphotoacquiresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
    {
        (windows_core::Interface::vtable(self).StartTransfer)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi()).ok()
    }
    pub unsafe fn StartItemTransfer<P0>(&self, nitemindex: u32, pphotoacquireitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireItem>,
    {
        (windows_core::Interface::vtable(self).StartItemTransfer)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi()).ok()
    }
    pub unsafe fn DirectoryCreated<P0>(&self, pszdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DirectoryCreated)(windows_core::Interface::as_raw(self), pszdirectory.param().abi()).ok()
    }
    pub unsafe fn UpdateTransferPercent<P0>(&self, foverall: P0, npercent: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UpdateTransferPercent)(windows_core::Interface::as_raw(self), foverall.param().abi(), npercent).ok()
    }
    pub unsafe fn EndItemTransfer<P0>(&self, nitemindex: u32, pphotoacquireitem: P0, hr: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireItem>,
    {
        (windows_core::Interface::vtable(self).EndItemTransfer)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi(), hr).ok()
    }
    pub unsafe fn EndTransfer(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndTransfer)(windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn StartDelete<P0>(&self, pphotoacquiresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
    {
        (windows_core::Interface::vtable(self).StartDelete)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi()).ok()
    }
    pub unsafe fn StartItemDelete<P0>(&self, nitemindex: u32, pphotoacquireitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireItem>,
    {
        (windows_core::Interface::vtable(self).StartItemDelete)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi()).ok()
    }
    pub unsafe fn UpdateDeletePercent(&self, npercent: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateDeletePercent)(windows_core::Interface::as_raw(self), npercent).ok()
    }
    pub unsafe fn EndItemDelete<P0>(&self, nitemindex: u32, pphotoacquireitem: P0, hr: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireItem>,
    {
        (windows_core::Interface::vtable(self).EndItemDelete)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi(), hr).ok()
    }
    pub unsafe fn EndDelete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndDelete)(windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn EndSession(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn GetDeleteAfterAcquire(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeleteAfterAcquire)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ErrorAdvise<P0>(&self, hr: windows_core::HRESULT, pszerrormessage: P0, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> windows_core::Result<ERROR_ADVISE_RESULT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ErrorAdvise)(windows_core::Interface::as_raw(self), hr, pszerrormessage.param().abi(), nmessagetype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetUserInput<P0>(&self, riidtype: *const windows_core::GUID, punknown: P0, ppropvarresult: *mut windows_core::PROPVARIANT, ppropvardefault: Option<*const windows_core::PROPVARIANT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetUserInput)(windows_core::Interface::as_raw(self), riidtype, punknown.param().abi(), core::mem::transmute(ppropvarresult), core::mem::transmute(ppropvardefault.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IPhotoAcquireProgressCB_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancelled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub StartEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FoundItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub StartTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartItemTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DirectoryCreated: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UpdateTransferPercent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    pub EndItemTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub EndTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub StartDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartItemDelete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateDeletePercent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EndItemDelete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub EndDelete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetDeleteAfterAcquire: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ErrorAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR, ERROR_ADVISE_MESSAGE_TYPE, *mut ERROR_ADVISE_RESULT) -> windows_core::HRESULT,
    pub GetUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoAcquireSettings, IPhotoAcquireSettings_Vtbl, 0x00f2b868_dd67_487c_9553_049240767e91);
impl core::ops::Deref for IPhotoAcquireSettings {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquireSettings, windows_core::IUnknown);
impl IPhotoAcquireSettings {
    pub unsafe fn InitializeFromRegistry<P0>(&self, pszregistrykey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromRegistry)(windows_core::Interface::as_raw(self), pszregistrykey.param().abi()).ok()
    }
    pub unsafe fn SetFlags(&self, dwphotoacquireflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), dwphotoacquireflags).ok()
    }
    pub unsafe fn SetOutputFilenameTemplate<P0>(&self, psztemplate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetOutputFilenameTemplate)(windows_core::Interface::as_raw(self), psztemplate.param().abi()).ok()
    }
    pub unsafe fn SetSequencePaddingWidth(&self, dwwidth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSequencePaddingWidth)(windows_core::Interface::as_raw(self), dwwidth).ok()
    }
    pub unsafe fn SetSequenceZeroPadding<P0>(&self, fzeropad: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSequenceZeroPadding)(windows_core::Interface::as_raw(self), fzeropad.param().abi()).ok()
    }
    pub unsafe fn SetGroupTag<P0>(&self, pszgrouptag: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetGroupTag)(windows_core::Interface::as_raw(self), pszgrouptag.param().abi()).ok()
    }
    pub unsafe fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAcquisitionTime)(windows_core::Interface::as_raw(self), pftacquisitiontime).ok()
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOutputFilenameTemplate(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputFilenameTemplate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSequencePaddingWidth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSequencePaddingWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSequenceZeroPadding(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSequenceZeroPadding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGroupTag(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGroupTag)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAcquisitionTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAcquisitionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPhotoAcquireSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFromRegistry: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetOutputFilenameTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSequencePaddingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSequenceZeroPadding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetGroupTag: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetAcquisitionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutputFilenameTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSequencePaddingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSequenceZeroPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetGroupTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAcquisitionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoAcquireSource, IPhotoAcquireSource_Vtbl, 0x00f2c703_8613_4282_a53b_6ec59c5883ac);
impl core::ops::Deref for IPhotoAcquireSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoAcquireSource, windows_core::IUnknown);
impl IPhotoAcquireSource {
    pub unsafe fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcons(&self, nsize: u32, phlargeicon: Option<*mut super::super::UI::WindowsAndMessaging::HICON>, phsmallicon: Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceIcons)(windows_core::Interface::as_raw(self), nsize, core::mem::transmute(phlargeicon.unwrap_or(std::ptr::null_mut())), core::mem::transmute(phsmallicon.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn InitializeItemList<P0, P1>(&self, fforceenumeration: P0, pphotoacquireprogresscb: P1, pnitemcount: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<IPhotoAcquireProgressCB>,
    {
        (windows_core::Interface::vtable(self).InitializeItemList)(windows_core::Interface::as_raw(self), fforceenumeration.param().abi(), pphotoacquireprogresscb.param().abi(), core::mem::transmute(pnitemcount.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetItemAt(&self, nindex: u32) -> windows_core::Result<IPhotoAcquireItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemAt)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPhotoAcquireSettings(&self) -> windows_core::Result<IPhotoAcquireSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPhotoAcquireSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDeviceId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BindToObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindToObject)(windows_core::Interface::as_raw(self), riid, ppv).ok()
    }
}
#[repr(C)]
pub struct IPhotoAcquireSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcons: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::UI::WindowsAndMessaging::HICON, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcons: usize,
    pub InitializeItemList: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPhotoAcquireSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoProgressActionCB, IPhotoProgressActionCB_Vtbl, 0x00f242d0_b206_4e7d_b4c1_4755bcbb9c9f);
impl core::ops::Deref for IPhotoProgressActionCB {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoProgressActionCB, windows_core::IUnknown);
impl IPhotoProgressActionCB {
    pub unsafe fn DoAction<P0>(&self, hwndparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DoAction)(windows_core::Interface::as_raw(self), hwndparent.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPhotoProgressActionCB_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoProgressDialog, IPhotoProgressDialog_Vtbl, 0x00f246f9_0750_4f08_9381_2cd8e906a4ae);
impl core::ops::Deref for IPhotoProgressDialog {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPhotoProgressDialog, windows_core::IUnknown);
impl IPhotoProgressDialog {
    pub unsafe fn Create<P0>(&self, hwndparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), hwndparent.param().abi()).ok()
    }
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Destroy(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), psztitle.param().abi()).ok()
    }
    pub unsafe fn ShowCheckbox<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ShowCheckbox)(windows_core::Interface::as_raw(self), ncheckboxid, fshow.param().abi()).ok()
    }
    pub unsafe fn SetCheckboxText<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCheckboxText)(windows_core::Interface::as_raw(self), ncheckboxid, pszcheckboxtext.param().abi()).ok()
    }
    pub unsafe fn SetCheckboxCheck<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCheckboxCheck)(windows_core::Interface::as_raw(self), ncheckboxid, fchecked.param().abi()).ok()
    }
    pub unsafe fn SetCheckboxTooltip<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCheckboxTooltip)(windows_core::Interface::as_raw(self), ncheckboxid, pszcheckboxtooltiptext.param().abi()).ok()
    }
    pub unsafe fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCheckboxChecked)(windows_core::Interface::as_raw(self), ncheckboxid, &mut result__).map(|| result__)
    }
    pub unsafe fn SetCaption<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCaption)(windows_core::Interface::as_raw(self), psztitle.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetImage<P0, P1>(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: P0, hbitmap: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HICON>,
        P1: windows_core::Param<super::super::Graphics::Gdi::HBITMAP>,
    {
        (windows_core::Interface::vtable(self).SetImage)(windows_core::Interface::as_raw(self), nimagetype, hicon.param().abi(), hbitmap.param().abi()).ok()
    }
    pub unsafe fn SetPercentComplete(&self, npercent: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPercentComplete)(windows_core::Interface::as_raw(self), npercent).ok()
    }
    pub unsafe fn SetProgressText<P0>(&self, pszprogresstext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProgressText)(windows_core::Interface::as_raw(self), pszprogresstext.param().abi()).ok()
    }
    pub unsafe fn SetActionLinkCallback<P0>(&self, pphotoprogressactioncb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoProgressActionCB>,
    {
        (windows_core::Interface::vtable(self).SetActionLinkCallback)(windows_core::Interface::as_raw(self), pphotoprogressactioncb.param().abi()).ok()
    }
    pub unsafe fn SetActionLinkText<P0>(&self, pszcaption: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetActionLinkText)(windows_core::Interface::as_raw(self), pszcaption.param().abi()).ok()
    }
    pub unsafe fn ShowActionLink<P0>(&self, fshow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ShowActionLink)(windows_core::Interface::as_raw(self), fshow.param().abi()).ok()
    }
    pub unsafe fn IsCancelled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCancelled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUserInput<P0>(&self, riidtype: *const windows_core::GUID, punknown: P0, ppropvarresult: *mut windows_core::PROPVARIANT, ppropvardefault: Option<*const windows_core::PROPVARIANT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetUserInput)(windows_core::Interface::as_raw(self), riidtype, punknown.param().abi(), core::mem::transmute(ppropvarresult), core::mem::transmute(ppropvardefault.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IPhotoProgressDialog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ShowCheckbox: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCheckboxText: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetCheckboxCheck: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCheckboxTooltip: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsCheckboxChecked: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_IMAGE_TYPE, super::super::UI::WindowsAndMessaging::HICON, super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    SetImage: usize,
    pub SetPercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetProgressText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetActionLinkCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActionLinkText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ShowActionLink: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsCancelled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserInputString, IUserInputString_Vtbl, 0x00f243a1_205b_45ba_ae26_abbc53aa7a6f);
impl core::ops::Deref for IUserInputString {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserInputString, windows_core::IUnknown);
impl IUserInputString {
    pub unsafe fn GetSubmitButtonText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubmitButtonText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPrompt(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrompt)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStringId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStringType(&self) -> windows_core::Result<USER_INPUT_STRING_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTooltipText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTooltipText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDefault(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefault)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMruCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMruCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMruEntryAt(&self, nindex: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMruEntryAt)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetImage(&self, nsize: u32, phbitmap: Option<*mut super::super::Graphics::Gdi::HBITMAP>, phicon: Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetImage)(windows_core::Interface::as_raw(self), nsize, core::mem::transmute(phbitmap.unwrap_or(std::ptr::null_mut())), core::mem::transmute(phicon.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IUserInputString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSubmitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetStringId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetStringType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut USER_INPUT_STRING_TYPE) -> windows_core::HRESULT,
    pub GetTooltipText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetMruCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMruEntryAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetImage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    GetImage: usize,
}
pub const DSF_ALL_DEVICES: u32 = 65535u32;
pub const DSF_CPL_MODE: u32 = 65536u32;
pub const DSF_DV_DEVICES: u32 = 64u32;
pub const DSF_FS_DEVICES: u32 = 32u32;
pub const DSF_SHOW_OFFLINE: u32 = 131072u32;
pub const DSF_STI_DEVICES: u32 = 8u32;
pub const DSF_TWAIN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(4i32);
pub const DSF_TWAIN_DEVICES: u32 = 16u32;
pub const DSF_WIA_CAMERAS: u32 = 2u32;
pub const DSF_WIA_SCANNERS: u32 = 4u32;
pub const DSF_WPD_DEVICES: u32 = 1u32;
pub const DST_DV_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(6i32);
pub const DST_FS_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(5i32);
pub const DST_STI_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(3i32);
pub const DST_UNKNOWN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(0i32);
pub const DST_WIA_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(2i32);
pub const DST_WPD_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(1i32);
pub const PAPS_CLEANUP: u32 = 2u32;
pub const PAPS_POSTSAVE: u32 = 1u32;
pub const PAPS_PRESAVE: u32 = 0u32;
pub const PHOTOACQUIRE_ERROR_OK: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(3i32);
pub const PHOTOACQUIRE_ERROR_RETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(1i32);
pub const PHOTOACQUIRE_ERROR_SKIPRETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(0i32);
pub const PHOTOACQUIRE_ERROR_YESNO: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(2i32);
pub const PHOTOACQUIRE_RESULT_ABORT: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(6i32);
pub const PHOTOACQUIRE_RESULT_NO: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(1i32);
pub const PHOTOACQUIRE_RESULT_OK: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(2i32);
pub const PHOTOACQUIRE_RESULT_RETRY: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(5i32);
pub const PHOTOACQUIRE_RESULT_SKIP: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(3i32);
pub const PHOTOACQUIRE_RESULT_SKIP_ALL: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(4i32);
pub const PHOTOACQUIRE_RESULT_YES: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(0i32);
pub const PHOTOACQ_ABORT_ON_SETTINGS_UPDATE: u32 = 2048u32;
pub const PHOTOACQ_DELETE_AFTER_ACQUIRE: u32 = 32u32;
pub const PHOTOACQ_DISABLE_AUTO_ROTATE: u32 = 2u32;
pub const PHOTOACQ_DISABLE_DB_INTEGRATION: u32 = 16u32;
pub const PHOTOACQ_DISABLE_DUPLICATE_DETECTION: u32 = 64u32;
pub const PHOTOACQ_DISABLE_GROUP_TAG_PROMPT: u32 = 8u32;
pub const PHOTOACQ_DISABLE_METADATA_WRITE: u32 = 256u32;
pub const PHOTOACQ_DISABLE_PLUGINS: u32 = 4u32;
pub const PHOTOACQ_DISABLE_SETTINGS_LINK: u32 = 1024u32;
pub const PHOTOACQ_DISABLE_THUMBNAIL_PROGRESS: u32 = 512u32;
pub const PHOTOACQ_ENABLE_THUMBNAIL_CACHING: u32 = 128u32;
pub const PHOTOACQ_ERROR_RESTART_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8004A001_u32 as _);
pub const PHOTOACQ_IMPORT_VIDEO_AS_MULTIPLE_FILES: u32 = 4096u32;
pub const PHOTOACQ_NO_GALLERY_LAUNCH: u32 = 1u32;
pub const PHOTOACQ_RUN_DEFAULT: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_FinalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_GroupTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_RelativePathname: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_SkipImport: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_TransferResult: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 5 };
pub const PROGRESS_DIALOG_BITMAP_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(3i32);
pub const PROGRESS_DIALOG_CHECKBOX_ID_DEFAULT: PROGRESS_DIALOG_CHECKBOX_ID = PROGRESS_DIALOG_CHECKBOX_ID(0i32);
pub const PROGRESS_DIALOG_ICON_LARGE: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(1i32);
pub const PROGRESS_DIALOG_ICON_SMALL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(0i32);
pub const PROGRESS_DIALOG_ICON_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(2i32);
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
pub const USER_INPUT_DEFAULT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(0i32);
pub const USER_INPUT_PATH_ELEMENT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEVICE_SELECTION_DEVICE_TYPE(pub i32);
impl windows_core::TypeKind for DEVICE_SELECTION_DEVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEVICE_SELECTION_DEVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEVICE_SELECTION_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ERROR_ADVISE_MESSAGE_TYPE(pub i32);
impl windows_core::TypeKind for ERROR_ADVISE_MESSAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ERROR_ADVISE_MESSAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ERROR_ADVISE_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ERROR_ADVISE_RESULT(pub i32);
impl windows_core::TypeKind for ERROR_ADVISE_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ERROR_ADVISE_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ERROR_ADVISE_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROGRESS_DIALOG_CHECKBOX_ID(pub i32);
impl windows_core::TypeKind for PROGRESS_DIALOG_CHECKBOX_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROGRESS_DIALOG_CHECKBOX_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROGRESS_DIALOG_CHECKBOX_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROGRESS_DIALOG_IMAGE_TYPE(pub i32);
impl windows_core::TypeKind for PROGRESS_DIALOG_IMAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROGRESS_DIALOG_IMAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROGRESS_DIALOG_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USER_INPUT_STRING_TYPE(pub i32);
impl windows_core::TypeKind for USER_INPUT_STRING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USER_INPUT_STRING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USER_INPUT_STRING_TYPE").field(&self.0).finish()
    }
}
pub const PhotoAcquire: windows_core::GUID = windows_core::GUID::from_u128(0x00f26e02_e9f2_4a9f_9fdd_5a962fb26a98);
pub const PhotoAcquireAutoPlayDropTarget: windows_core::GUID = windows_core::GUID::from_u128(0x00f20eb5_8fd6_4d9d_b75e_36801766c8f1);
pub const PhotoAcquireAutoPlayHWEventHandler: windows_core::GUID = windows_core::GUID::from_u128(0x00f2b433_44e4_4d88_b2b0_2698a0a91dba);
pub const PhotoAcquireDeviceSelectionDialog: windows_core::GUID = windows_core::GUID::from_u128(0x00f29a34_b8a1_482c_bcf8_3ac7b0fe8f62);
pub const PhotoAcquireOptionsDialog: windows_core::GUID = windows_core::GUID::from_u128(0x00f210a1_62f0_438b_9f7e_9618d72a1831);
pub const PhotoProgressDialog: windows_core::GUID = windows_core::GUID::from_u128(0x00f24ca0_748f_4e8a_894f_0e0357c6799f);
#[cfg(feature = "implement")]
core::include!("impl.rs");
