#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_SELECTION_DEVICE_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ERROR_ADVISE_MESSAGE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ERROR_ADVISE_RESULT(pub i32);
windows_core::imp::define_interface!(IPhotoAcquire, IPhotoAcquire_Vtbl, 0x00f23353_e31b_4955_a8ad_ca5ebf31e2ce);
windows_core::imp::interface_hierarchy!(IPhotoAcquire, windows_core::IUnknown);
impl IPhotoAcquire {
    pub unsafe fn CreatePhotoSource<P0>(&self, pszdevice: P0) -> windows_core::Result<IPhotoAcquireSource>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePhotoSource)(windows_core::Interface::as_raw(self), pszdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Acquire<P0, P3, P4>(&self, pphotoacquiresource: P0, fshowprogress: bool, hwndparent: Option<super::super::Foundation::HWND>, pszapplicationname: P3, pphotoacquireprogresscb: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<IPhotoAcquireProgressCB>,
    {
        unsafe { (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi(), fshowprogress.into(), hwndparent.unwrap_or(core::mem::zeroed()) as _, pszapplicationname.param().abi(), pphotoacquireprogresscb.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumResults(&self) -> windows_core::Result<super::super::System::Com::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumResults)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquire_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePhotoSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, super::super::Foundation::HWND, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumResults: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPhotoAcquire_Impl: windows_core::IUnknownImpl {
    fn CreatePhotoSource(&self, pszdevice: &windows_core::PCWSTR) -> windows_core::Result<IPhotoAcquireSource>;
    fn Acquire(&self, pphotoacquiresource: windows_core::Ref<IPhotoAcquireSource>, fshowprogress: windows_core::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: &windows_core::PCWSTR, pphotoacquireprogresscb: windows_core::Ref<IPhotoAcquireProgressCB>) -> windows_core::Result<()>;
    fn EnumResults(&self) -> windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPhotoAcquire_Vtbl {
    pub const fn new<Identity: IPhotoAcquire_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePhotoSource<Identity: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdevice: windows_core::PCWSTR, ppphotoacquiresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquire_Impl::CreatePhotoSource(this, core::mem::transmute(&pszdevice)) {
                    Ok(ok__) => {
                        ppphotoacquiresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Acquire<Identity: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void, fshowprogress: windows_core::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: windows_core::PCWSTR, pphotoacquireprogresscb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquire_Impl::Acquire(this, core::mem::transmute_copy(&pphotoacquiresource), core::mem::transmute_copy(&fshowprogress), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&pszapplicationname), core::mem::transmute_copy(&pphotoacquireprogresscb)).into()
            }
        }
        unsafe extern "system" fn EnumResults<Identity: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumfilepaths: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquire_Impl::EnumResults(this) {
                    Ok(ok__) => {
                        ppenumfilepaths.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePhotoSource: CreatePhotoSource::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            EnumResults: EnumResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquire as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPhotoAcquire {}
windows_core::imp::define_interface!(IPhotoAcquireDeviceSelectionDialog, IPhotoAcquireDeviceSelectionDialog_Vtbl, 0x00f28837_55dd_4f37_aaf5_6855a9640467);
windows_core::imp::interface_hierarchy!(IPhotoAcquireDeviceSelectionDialog, windows_core::IUnknown);
impl IPhotoAcquireDeviceSelectionDialog {
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), psztitle.param().abi()).ok() }
    }
    pub unsafe fn SetSubmitButtonText<P0>(&self, pszsubmitbuttontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSubmitButtonText)(windows_core::Interface::as_raw(self), pszsubmitbuttontext.param().abi()).ok() }
    }
    pub unsafe fn DoModal(&self, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: Option<*mut windows_core::BSTR>, pndevicetype: Option<*mut DEVICE_SELECTION_DEVICE_TYPE>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoModal)(windows_core::Interface::as_raw(self), hwndparent, dwdeviceflags, pbstrdeviceid.unwrap_or(core::mem::zeroed()) as _, pndevicetype.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSubmitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DoModal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, *mut *mut core::ffi::c_void, *mut DEVICE_SELECTION_DEVICE_TYPE) -> windows_core::HRESULT,
}
pub trait IPhotoAcquireDeviceSelectionDialog_Impl: windows_core::IUnknownImpl {
    fn SetTitle(&self, psztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSubmitButtonText(&self, pszsubmitbuttontext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DoModal(&self, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut windows_core::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> windows_core::Result<()>;
}
impl IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub const fn new<Identity: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTitle<Identity: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireDeviceSelectionDialog_Impl::SetTitle(this, core::mem::transmute(&psztitle)).into()
            }
        }
        unsafe extern "system" fn SetSubmitButtonText<Identity: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubmitbuttontext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireDeviceSelectionDialog_Impl::SetSubmitButtonText(this, core::mem::transmute(&pszsubmitbuttontext)).into()
            }
        }
        unsafe extern "system" fn DoModal<Identity: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut *mut core::ffi::c_void, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireDeviceSelectionDialog_Impl::DoModal(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwdeviceflags), core::mem::transmute_copy(&pbstrdeviceid), core::mem::transmute_copy(&pndevicetype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTitle: SetTitle::<Identity, OFFSET>,
            SetSubmitButtonText: SetSubmitButtonText::<Identity, OFFSET>,
            DoModal: DoModal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireDeviceSelectionDialog as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPhotoAcquireDeviceSelectionDialog {}
windows_core::imp::define_interface!(IPhotoAcquireItem, IPhotoAcquireItem_Vtbl, 0x00f21c97_28bf_4c02_b842_5e4e90139a30);
windows_core::imp::interface_hierarchy!(IPhotoAcquireItem, windows_core::IUnknown);
impl IPhotoAcquireItem {
    pub unsafe fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetThumbnail(&self, sizethumbnail: super::super::Foundation::SIZE) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThumbnail)(windows_core::Interface::as_raw(self), core::mem::transmute(sizethumbnail), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, key: *const super::super::Foundation::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), key, core::mem::transmute(pv)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CanDelete(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanDelete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetSubItemCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSubItemAt(&self, nitemindex: u32) -> windows_core::Result<IPhotoAcquireItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubItemAt)(windows_core::Interface::as_raw(self), nitemindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::SIZE, *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetThumbnail: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    pub CanDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSubItemAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPhotoAcquireItem_Impl: windows_core::IUnknownImpl {
    fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetThumbnail(&self, sizethumbnail: &super::super::Foundation::SIZE) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
    fn GetProperty(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&self, key: *const super::super::Foundation::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn CanDelete(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetSubItemCount(&self) -> windows_core::Result<u32>;
    fn GetSubItemAt(&self, nitemindex: u32) -> windows_core::Result<IPhotoAcquireItem>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPhotoAcquireItem_Vtbl {
    pub const fn new<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemName<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstritemname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireItem_Impl::GetItemName(this) {
                    Ok(ok__) => {
                        pbstritemname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThumbnail<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireItem_Impl::GetThumbnail(this, core::mem::transmute(&sizethumbnail)) {
                    Ok(ok__) => {
                        phbmpthumbnail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireItem_Impl::GetProperty(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireItem_Impl::SetProperty(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pv)).into()
            }
        }
        unsafe extern "system" fn GetStream<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireItem_Impl::GetStream(this) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanDelete<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcandelete: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireItem_Impl::CanDelete(this) {
                    Ok(ok__) => {
                        pfcandelete.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireItem_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn GetSubItemCount<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireItem_Impl::GetSubItemCount(this) {
                    Ok(ok__) => {
                        pncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubItemAt<Identity: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireItem_Impl::GetSubItemAt(this, core::mem::transmute_copy(&nitemindex)) {
                    Ok(ok__) => {
                        ppphotoacquireitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemName: GetItemName::<Identity, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            CanDelete: CanDelete::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetSubItemCount: GetSubItemCount::<Identity, OFFSET>,
            GetSubItemAt: GetSubItemAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPhotoAcquireItem {}
windows_core::imp::define_interface!(IPhotoAcquireOptionsDialog, IPhotoAcquireOptionsDialog_Vtbl, 0x00f2b3ee_bf64_47ee_89f4_4dedd79643f2);
windows_core::imp::interface_hierarchy!(IPhotoAcquireOptionsDialog, windows_core::IUnknown);
impl IPhotoAcquireOptionsDialog {
    pub unsafe fn Initialize<P0>(&self, pszregistryroot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pszregistryroot.param().abi()).ok() }
    }
    pub unsafe fn Create(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), hwndparent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Destroy(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DoModal(&self, hwndparent: super::super::Foundation::HWND, ppnreturncode: Option<*mut isize>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoModal)(windows_core::Interface::as_raw(self), hwndparent, ppnreturncode.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SaveData(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveData)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireOptionsDialog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoModal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut isize) -> windows_core::HRESULT,
    pub SaveData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPhotoAcquireOptionsDialog_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pszregistryroot: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<super::super::Foundation::HWND>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn DoModal(&self, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> windows_core::Result<()>;
    fn SaveData(&self) -> windows_core::Result<()>;
}
impl IPhotoAcquireOptionsDialog_Vtbl {
    pub const fn new<Identity: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszregistryroot: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireOptionsDialog_Impl::Initialize(this, core::mem::transmute(&pszregistryroot)).into()
            }
        }
        unsafe extern "system" fn Create<Identity: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireOptionsDialog_Impl::Create(this, core::mem::transmute_copy(&hwndparent)) {
                    Ok(ok__) => {
                        phwnddialog.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Destroy<Identity: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireOptionsDialog_Impl::Destroy(this).into()
            }
        }
        unsafe extern "system" fn DoModal<Identity: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireOptionsDialog_Impl::DoModal(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ppnreturncode)).into()
            }
        }
        unsafe extern "system" fn SaveData<Identity: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireOptionsDialog_Impl::SaveData(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            DoModal: DoModal::<Identity, OFFSET>,
            SaveData: SaveData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireOptionsDialog as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPhotoAcquireOptionsDialog {}
windows_core::imp::define_interface!(IPhotoAcquirePlugin, IPhotoAcquirePlugin_Vtbl, 0x00f2dceb_ecb8_4f77_8e47_e7a987c83dd0);
windows_core::imp::interface_hierarchy!(IPhotoAcquirePlugin, windows_core::IUnknown);
impl IPhotoAcquirePlugin {
    pub unsafe fn Initialize<P0, P1>(&self, pphotoacquiresource: P0, pphotoacquireprogresscb: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
        P1: windows_core::Param<IPhotoAcquireProgressCB>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi(), pphotoacquireprogresscb.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn ProcessItem<P1, P2, P3, P4>(&self, dwacquirestage: u32, pphotoacquireitem: P1, poriginalitemstream: P2, pszfinalfilename: P3, ppropertystore: P4) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPhotoAcquireItem>,
        P2: windows_core::Param<super::super::System::Com::IStream>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessItem)(windows_core::Interface::as_raw(self), dwacquirestage, pphotoacquireitem.param().abi(), poriginalitemstream.param().abi(), pszfinalfilename.param().abi(), ppropertystore.param().abi()).ok() }
    }
    pub unsafe fn TransferComplete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransferComplete)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn DisplayConfigureDialog(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayConfigureDialog)(windows_core::Interface::as_raw(self), hwndparent).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquirePlugin_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pphotoacquiresource: windows_core::Ref<IPhotoAcquireSource>, pphotoacquireprogresscb: windows_core::Ref<IPhotoAcquireProgressCB>) -> windows_core::Result<()>;
    fn ProcessItem(&self, dwacquirestage: u32, pphotoacquireitem: windows_core::Ref<IPhotoAcquireItem>, poriginalitemstream: windows_core::Ref<super::super::System::Com::IStream>, pszfinalfilename: &windows_core::PCWSTR, ppropertystore: windows_core::Ref<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn TransferComplete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn DisplayConfigureDialog(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquirePlugin_Vtbl {
    pub const fn new<Identity: IPhotoAcquirePlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void, pphotoacquireprogresscb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquirePlugin_Impl::Initialize(this, core::mem::transmute_copy(&pphotoacquiresource), core::mem::transmute_copy(&pphotoacquireprogresscb)).into()
            }
        }
        unsafe extern "system" fn ProcessItem<Identity: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: *mut core::ffi::c_void, poriginalitemstream: *mut core::ffi::c_void, pszfinalfilename: windows_core::PCWSTR, ppropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquirePlugin_Impl::ProcessItem(this, core::mem::transmute_copy(&dwacquirestage), core::mem::transmute_copy(&pphotoacquireitem), core::mem::transmute_copy(&poriginalitemstream), core::mem::transmute(&pszfinalfilename), core::mem::transmute_copy(&ppropertystore)).into()
            }
        }
        unsafe extern "system" fn TransferComplete<Identity: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquirePlugin_Impl::TransferComplete(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn DisplayConfigureDialog<Identity: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquirePlugin_Impl::DisplayConfigureDialog(this, core::mem::transmute_copy(&hwndparent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ProcessItem: ProcessItem::<Identity, OFFSET>,
            TransferComplete: TransferComplete::<Identity, OFFSET>,
            DisplayConfigureDialog: DisplayConfigureDialog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquirePlugin as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IPhotoAcquirePlugin {}
windows_core::imp::define_interface!(IPhotoAcquireProgressCB, IPhotoAcquireProgressCB_Vtbl, 0x00f2ce1e_935e_4248_892c_130f32c45cb4);
windows_core::imp::interface_hierarchy!(IPhotoAcquireProgressCB, windows_core::IUnknown);
impl IPhotoAcquireProgressCB {
    pub unsafe fn Cancelled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancelled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartEnumeration<P0>(&self, pphotoacquiresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartEnumeration)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi()).ok() }
    }
    pub unsafe fn FoundItem<P0>(&self, pphotoacquireitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).FoundItem)(windows_core::Interface::as_raw(self), pphotoacquireitem.param().abi()).ok() }
    }
    pub unsafe fn EndEnumeration(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn StartTransfer<P0>(&self, pphotoacquiresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartTransfer)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi()).ok() }
    }
    pub unsafe fn StartItemTransfer<P1>(&self, nitemindex: u32, pphotoacquireitem: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPhotoAcquireItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartItemTransfer)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi()).ok() }
    }
    pub unsafe fn DirectoryCreated<P0>(&self, pszdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DirectoryCreated)(windows_core::Interface::as_raw(self), pszdirectory.param().abi()).ok() }
    }
    pub unsafe fn UpdateTransferPercent(&self, foverall: bool, npercent: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateTransferPercent)(windows_core::Interface::as_raw(self), foverall.into(), npercent).ok() }
    }
    pub unsafe fn EndItemTransfer<P1>(&self, nitemindex: u32, pphotoacquireitem: P1, hr: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPhotoAcquireItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndItemTransfer)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi(), hr).ok() }
    }
    pub unsafe fn EndTransfer(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndTransfer)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn StartDelete<P0>(&self, pphotoacquiresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoAcquireSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartDelete)(windows_core::Interface::as_raw(self), pphotoacquiresource.param().abi()).ok() }
    }
    pub unsafe fn StartItemDelete<P1>(&self, nitemindex: u32, pphotoacquireitem: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPhotoAcquireItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartItemDelete)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi()).ok() }
    }
    pub unsafe fn UpdateDeletePercent(&self, npercent: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateDeletePercent)(windows_core::Interface::as_raw(self), npercent).ok() }
    }
    pub unsafe fn EndItemDelete<P1>(&self, nitemindex: u32, pphotoacquireitem: P1, hr: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPhotoAcquireItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndItemDelete)(windows_core::Interface::as_raw(self), nitemindex, pphotoacquireitem.param().abi(), hr).ok() }
    }
    pub unsafe fn EndDelete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndDelete)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn EndSession(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn GetDeleteAfterAcquire(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeleteAfterAcquire)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ErrorAdvise<P1>(&self, hr: windows_core::HRESULT, pszerrormessage: P1, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> windows_core::Result<ERROR_ADVISE_RESULT>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorAdvise)(windows_core::Interface::as_raw(self), hr, pszerrormessage.param().abi(), nmessagetype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetUserInput<P1>(&self, riidtype: *const windows_core::GUID, punknown: P1, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetUserInput)(windows_core::Interface::as_raw(self), riidtype, punknown.param().abi(), core::mem::transmute(ppropvarresult), ppropvardefault.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireProgressCB_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancelled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub StartEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FoundItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub StartTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartItemTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DirectoryCreated: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UpdateTransferPercent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub EndItemTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub EndTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub StartDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartItemDelete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateDeletePercent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EndItemDelete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub EndDelete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetDeleteAfterAcquire: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ErrorAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR, ERROR_ADVISE_MESSAGE_TYPE, *mut ERROR_ADVISE_RESULT) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut super::super::System::Com::StructuredStorage::PROPVARIANT, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetUserInput: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPhotoAcquireProgressCB_Impl: windows_core::IUnknownImpl {
    fn Cancelled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn StartEnumeration(&self, pphotoacquiresource: windows_core::Ref<IPhotoAcquireSource>) -> windows_core::Result<()>;
    fn FoundItem(&self, pphotoacquireitem: windows_core::Ref<IPhotoAcquireItem>) -> windows_core::Result<()>;
    fn EndEnumeration(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn StartTransfer(&self, pphotoacquiresource: windows_core::Ref<IPhotoAcquireSource>) -> windows_core::Result<()>;
    fn StartItemTransfer(&self, nitemindex: u32, pphotoacquireitem: windows_core::Ref<IPhotoAcquireItem>) -> windows_core::Result<()>;
    fn DirectoryCreated(&self, pszdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UpdateTransferPercent(&self, foverall: windows_core::BOOL, npercent: u32) -> windows_core::Result<()>;
    fn EndItemTransfer(&self, nitemindex: u32, pphotoacquireitem: windows_core::Ref<IPhotoAcquireItem>, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn EndTransfer(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn StartDelete(&self, pphotoacquiresource: windows_core::Ref<IPhotoAcquireSource>) -> windows_core::Result<()>;
    fn StartItemDelete(&self, nitemindex: u32, pphotoacquireitem: windows_core::Ref<IPhotoAcquireItem>) -> windows_core::Result<()>;
    fn UpdateDeletePercent(&self, npercent: u32) -> windows_core::Result<()>;
    fn EndItemDelete(&self, nitemindex: u32, pphotoacquireitem: windows_core::Ref<IPhotoAcquireItem>, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn EndDelete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn EndSession(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetDeleteAfterAcquire(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ErrorAdvise(&self, hr: windows_core::HRESULT, pszerrormessage: &windows_core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> windows_core::Result<ERROR_ADVISE_RESULT>;
    fn GetUserInput(&self, riidtype: *const windows_core::GUID, punknown: windows_core::Ref<windows_core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPhotoAcquireProgressCB_Vtbl {
    pub const fn new<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancelled<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcancelled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireProgressCB_Impl::Cancelled(this) {
                    Ok(ok__) => {
                        pfcancelled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartEnumeration<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::StartEnumeration(this, core::mem::transmute_copy(&pphotoacquiresource)).into()
            }
        }
        unsafe extern "system" fn FoundItem<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquireitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::FoundItem(this, core::mem::transmute_copy(&pphotoacquireitem)).into()
            }
        }
        unsafe extern "system" fn EndEnumeration<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::EndEnumeration(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn StartTransfer<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::StartTransfer(this, core::mem::transmute_copy(&pphotoacquiresource)).into()
            }
        }
        unsafe extern "system" fn StartItemTransfer<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::StartItemTransfer(this, core::mem::transmute_copy(&nitemindex), core::mem::transmute_copy(&pphotoacquireitem)).into()
            }
        }
        unsafe extern "system" fn DirectoryCreated<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdirectory: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::DirectoryCreated(this, core::mem::transmute(&pszdirectory)).into()
            }
        }
        unsafe extern "system" fn UpdateTransferPercent<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverall: windows_core::BOOL, npercent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::UpdateTransferPercent(this, core::mem::transmute_copy(&foverall), core::mem::transmute_copy(&npercent)).into()
            }
        }
        unsafe extern "system" fn EndItemTransfer<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::EndItemTransfer(this, core::mem::transmute_copy(&nitemindex), core::mem::transmute_copy(&pphotoacquireitem), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn EndTransfer<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::EndTransfer(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn StartDelete<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::StartDelete(this, core::mem::transmute_copy(&pphotoacquiresource)).into()
            }
        }
        unsafe extern "system" fn StartItemDelete<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::StartItemDelete(this, core::mem::transmute_copy(&nitemindex), core::mem::transmute_copy(&pphotoacquireitem)).into()
            }
        }
        unsafe extern "system" fn UpdateDeletePercent<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, npercent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::UpdateDeletePercent(this, core::mem::transmute_copy(&npercent)).into()
            }
        }
        unsafe extern "system" fn EndItemDelete<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::EndItemDelete(this, core::mem::transmute_copy(&nitemindex), core::mem::transmute_copy(&pphotoacquireitem), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn EndDelete<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::EndDelete(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn EndSession<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::EndSession(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn GetDeleteAfterAcquire<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdeleteafteracquire: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireProgressCB_Impl::GetDeleteAfterAcquire(this) {
                    Ok(ok__) => {
                        pfdeleteafteracquire.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ErrorAdvise<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pszerrormessage: windows_core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireProgressCB_Impl::ErrorAdvise(this, core::mem::transmute_copy(&hr), core::mem::transmute(&pszerrormessage), core::mem::transmute_copy(&nmessagetype)) {
                    Ok(ok__) => {
                        pnerroradviseresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserInput<Identity: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riidtype: *const windows_core::GUID, punknown: *mut core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireProgressCB_Impl::GetUserInput(this, core::mem::transmute_copy(&riidtype), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&ppropvarresult), core::mem::transmute_copy(&ppropvardefault)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancelled: Cancelled::<Identity, OFFSET>,
            StartEnumeration: StartEnumeration::<Identity, OFFSET>,
            FoundItem: FoundItem::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
            StartTransfer: StartTransfer::<Identity, OFFSET>,
            StartItemTransfer: StartItemTransfer::<Identity, OFFSET>,
            DirectoryCreated: DirectoryCreated::<Identity, OFFSET>,
            UpdateTransferPercent: UpdateTransferPercent::<Identity, OFFSET>,
            EndItemTransfer: EndItemTransfer::<Identity, OFFSET>,
            EndTransfer: EndTransfer::<Identity, OFFSET>,
            StartDelete: StartDelete::<Identity, OFFSET>,
            StartItemDelete: StartItemDelete::<Identity, OFFSET>,
            UpdateDeletePercent: UpdateDeletePercent::<Identity, OFFSET>,
            EndItemDelete: EndItemDelete::<Identity, OFFSET>,
            EndDelete: EndDelete::<Identity, OFFSET>,
            EndSession: EndSession::<Identity, OFFSET>,
            GetDeleteAfterAcquire: GetDeleteAfterAcquire::<Identity, OFFSET>,
            ErrorAdvise: ErrorAdvise::<Identity, OFFSET>,
            GetUserInput: GetUserInput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireProgressCB as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPhotoAcquireProgressCB {}
windows_core::imp::define_interface!(IPhotoAcquireSettings, IPhotoAcquireSettings_Vtbl, 0x00f2b868_dd67_487c_9553_049240767e91);
windows_core::imp::interface_hierarchy!(IPhotoAcquireSettings, windows_core::IUnknown);
impl IPhotoAcquireSettings {
    pub unsafe fn InitializeFromRegistry<P0>(&self, pszregistrykey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromRegistry)(windows_core::Interface::as_raw(self), pszregistrykey.param().abi()).ok() }
    }
    pub unsafe fn SetFlags(&self, dwphotoacquireflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), dwphotoacquireflags).ok() }
    }
    pub unsafe fn SetOutputFilenameTemplate<P0>(&self, psztemplate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFilenameTemplate)(windows_core::Interface::as_raw(self), psztemplate.param().abi()).ok() }
    }
    pub unsafe fn SetSequencePaddingWidth(&self, dwwidth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSequencePaddingWidth)(windows_core::Interface::as_raw(self), dwwidth).ok() }
    }
    pub unsafe fn SetSequenceZeroPadding(&self, fzeropad: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSequenceZeroPadding)(windows_core::Interface::as_raw(self), fzeropad.into()).ok() }
    }
    pub unsafe fn SetGroupTag<P0>(&self, pszgrouptag: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGroupTag)(windows_core::Interface::as_raw(self), pszgrouptag.param().abi()).ok() }
    }
    pub unsafe fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAcquisitionTime)(windows_core::Interface::as_raw(self), pftacquisitiontime).ok() }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputFilenameTemplate(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputFilenameTemplate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSequencePaddingWidth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSequencePaddingWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSequenceZeroPadding(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSequenceZeroPadding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGroupTag(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGroupTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAcquisitionTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAcquisitionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFromRegistry: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetOutputFilenameTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSequencePaddingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSequenceZeroPadding: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetGroupTag: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetAcquisitionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutputFilenameTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSequencePaddingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSequenceZeroPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetGroupTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAcquisitionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
pub trait IPhotoAcquireSettings_Impl: windows_core::IUnknownImpl {
    fn InitializeFromRegistry(&self, pszregistrykey: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFlags(&self, dwphotoacquireflags: u32) -> windows_core::Result<()>;
    fn SetOutputFilenameTemplate(&self, psztemplate: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSequencePaddingWidth(&self, dwwidth: u32) -> windows_core::Result<()>;
    fn SetSequenceZeroPadding(&self, fzeropad: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetGroupTag(&self, pszgrouptag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn GetOutputFilenameTemplate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSequencePaddingWidth(&self) -> windows_core::Result<u32>;
    fn GetSequenceZeroPadding(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetGroupTag(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAcquisitionTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
}
impl IPhotoAcquireSettings_Vtbl {
    pub const fn new<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFromRegistry<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszregistrykey: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSettings_Impl::InitializeFromRegistry(this, core::mem::transmute(&pszregistrykey)).into()
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwphotoacquireflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSettings_Impl::SetFlags(this, core::mem::transmute_copy(&dwphotoacquireflags)).into()
            }
        }
        unsafe extern "system" fn SetOutputFilenameTemplate<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztemplate: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSettings_Impl::SetOutputFilenameTemplate(this, core::mem::transmute(&psztemplate)).into()
            }
        }
        unsafe extern "system" fn SetSequencePaddingWidth<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwidth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSettings_Impl::SetSequencePaddingWidth(this, core::mem::transmute_copy(&dwwidth)).into()
            }
        }
        unsafe extern "system" fn SetSequenceZeroPadding<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fzeropad: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSettings_Impl::SetSequenceZeroPadding(this, core::mem::transmute_copy(&fzeropad)).into()
            }
        }
        unsafe extern "system" fn SetGroupTag<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgrouptag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSettings_Impl::SetGroupTag(this, core::mem::transmute(&pszgrouptag)).into()
            }
        }
        unsafe extern "system" fn SetAcquisitionTime<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSettings_Impl::SetAcquisitionTime(this, core::mem::transmute_copy(&pftacquisitiontime)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSettings_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pdwphotoacquireflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputFilenameTemplate<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSettings_Impl::GetOutputFilenameTemplate(this) {
                    Ok(ok__) => {
                        pbstrtemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSequencePaddingWidth<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSettings_Impl::GetSequencePaddingWidth(this) {
                    Ok(ok__) => {
                        pdwwidth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSequenceZeroPadding<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfzeropad: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSettings_Impl::GetSequenceZeroPadding(this) {
                    Ok(ok__) => {
                        pfzeropad.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGroupTag<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrgrouptag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSettings_Impl::GetGroupTag(this) {
                    Ok(ok__) => {
                        pbstrgrouptag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAcquisitionTime<Identity: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSettings_Impl::GetAcquisitionTime(this) {
                    Ok(ok__) => {
                        pftacquisitiontime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromRegistry: InitializeFromRegistry::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            SetOutputFilenameTemplate: SetOutputFilenameTemplate::<Identity, OFFSET>,
            SetSequencePaddingWidth: SetSequencePaddingWidth::<Identity, OFFSET>,
            SetSequenceZeroPadding: SetSequenceZeroPadding::<Identity, OFFSET>,
            SetGroupTag: SetGroupTag::<Identity, OFFSET>,
            SetAcquisitionTime: SetAcquisitionTime::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetOutputFilenameTemplate: GetOutputFilenameTemplate::<Identity, OFFSET>,
            GetSequencePaddingWidth: GetSequencePaddingWidth::<Identity, OFFSET>,
            GetSequenceZeroPadding: GetSequenceZeroPadding::<Identity, OFFSET>,
            GetGroupTag: GetGroupTag::<Identity, OFFSET>,
            GetAcquisitionTime: GetAcquisitionTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPhotoAcquireSettings {}
windows_core::imp::define_interface!(IPhotoAcquireSource, IPhotoAcquireSource_Vtbl, 0x00f2c703_8613_4282_a53b_6ec59c5883ac);
windows_core::imp::interface_hierarchy!(IPhotoAcquireSource, windows_core::IUnknown);
impl IPhotoAcquireSource {
    pub unsafe fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcons(&self, nsize: u32, phlargeicon: Option<*mut super::super::UI::WindowsAndMessaging::HICON>, phsmallicon: Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceIcons)(windows_core::Interface::as_raw(self), nsize, phlargeicon.unwrap_or(core::mem::zeroed()) as _, phsmallicon.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn InitializeItemList<P1>(&self, fforceenumeration: bool, pphotoacquireprogresscb: P1, pnitemcount: Option<*mut u32>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPhotoAcquireProgressCB>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeItemList)(windows_core::Interface::as_raw(self), fforceenumeration.into(), pphotoacquireprogresscb.param().abi(), pnitemcount.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetItemCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemAt(&self, nindex: u32) -> windows_core::Result<IPhotoAcquireItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemAt)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPhotoAcquireSettings(&self) -> windows_core::Result<IPhotoAcquireSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPhotoAcquireSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BindToObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BindToObject)(windows_core::Interface::as_raw(self), riid, ppv as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcons: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::UI::WindowsAndMessaging::HICON, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcons: usize,
    pub InitializeItemList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPhotoAcquireSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IPhotoAcquireSource_Impl: windows_core::IUnknownImpl {
    fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDeviceIcons(&self, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<()>;
    fn InitializeItemList(&self, fforceenumeration: windows_core::BOOL, pphotoacquireprogresscb: windows_core::Ref<IPhotoAcquireProgressCB>, pnitemcount: *mut u32) -> windows_core::Result<()>;
    fn GetItemCount(&self) -> windows_core::Result<u32>;
    fn GetItemAt(&self, nindex: u32) -> windows_core::Result<IPhotoAcquireItem>;
    fn GetPhotoAcquireSettings(&self) -> windows_core::Result<IPhotoAcquireSettings>;
    fn GetDeviceId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BindToObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IPhotoAcquireSource_Vtbl {
    pub const fn new<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFriendlyName<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfriendlyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSource_Impl::GetFriendlyName(this) {
                    Ok(ok__) => {
                        pbstrfriendlyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceIcons<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSource_Impl::GetDeviceIcons(this, core::mem::transmute_copy(&nsize), core::mem::transmute_copy(&phlargeicon), core::mem::transmute_copy(&phsmallicon)).into()
            }
        }
        unsafe extern "system" fn InitializeItemList<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforceenumeration: windows_core::BOOL, pphotoacquireprogresscb: *mut core::ffi::c_void, pnitemcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSource_Impl::InitializeItemList(this, core::mem::transmute_copy(&fforceenumeration), core::mem::transmute_copy(&pphotoacquireprogresscb), core::mem::transmute_copy(&pnitemcount)).into()
            }
        }
        unsafe extern "system" fn GetItemCount<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnitemcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSource_Impl::GetItemCount(this) {
                    Ok(ok__) => {
                        pnitemcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemAt<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSource_Impl::GetItemAt(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppphotoacquireitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPhotoAcquireSettings<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphotoacquiresettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSource_Impl::GetPhotoAcquireSettings(this) {
                    Ok(ok__) => {
                        ppphotoacquiresettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoAcquireSource_Impl::GetDeviceId(this) {
                    Ok(ok__) => {
                        pbstrdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BindToObject<Identity: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoAcquireSource_Impl::BindToObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            GetDeviceIcons: GetDeviceIcons::<Identity, OFFSET>,
            InitializeItemList: InitializeItemList::<Identity, OFFSET>,
            GetItemCount: GetItemCount::<Identity, OFFSET>,
            GetItemAt: GetItemAt::<Identity, OFFSET>,
            GetPhotoAcquireSettings: GetPhotoAcquireSettings::<Identity, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, OFFSET>,
            BindToObject: BindToObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IPhotoAcquireSource {}
windows_core::imp::define_interface!(IPhotoProgressActionCB, IPhotoProgressActionCB_Vtbl, 0x00f242d0_b206_4e7d_b4c1_4755bcbb9c9f);
windows_core::imp::interface_hierarchy!(IPhotoProgressActionCB, windows_core::IUnknown);
impl IPhotoProgressActionCB {
    pub unsafe fn DoAction(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoAction)(windows_core::Interface::as_raw(self), hwndparent).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressActionCB_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IPhotoProgressActionCB_Impl: windows_core::IUnknownImpl {
    fn DoAction(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl IPhotoProgressActionCB_Vtbl {
    pub const fn new<Identity: IPhotoProgressActionCB_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoAction<Identity: IPhotoProgressActionCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressActionCB_Impl::DoAction(this, core::mem::transmute_copy(&hwndparent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoAction: DoAction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoProgressActionCB as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPhotoProgressActionCB {}
windows_core::imp::define_interface!(IPhotoProgressDialog, IPhotoProgressDialog_Vtbl, 0x00f246f9_0750_4f08_9381_2cd8e906a4ae);
windows_core::imp::interface_hierarchy!(IPhotoProgressDialog, windows_core::IUnknown);
impl IPhotoProgressDialog {
    pub unsafe fn Create(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), hwndparent).ok() }
    }
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Destroy(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), psztitle.param().abi()).ok() }
    }
    pub unsafe fn ShowCheckbox(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowCheckbox)(windows_core::Interface::as_raw(self), ncheckboxid, fshow.into()).ok() }
    }
    pub unsafe fn SetCheckboxText<P1>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCheckboxText)(windows_core::Interface::as_raw(self), ncheckboxid, pszcheckboxtext.param().abi()).ok() }
    }
    pub unsafe fn SetCheckboxCheck(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCheckboxCheck)(windows_core::Interface::as_raw(self), ncheckboxid, fchecked.into()).ok() }
    }
    pub unsafe fn SetCheckboxTooltip<P1>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCheckboxTooltip)(windows_core::Interface::as_raw(self), ncheckboxid, pszcheckboxtooltiptext.param().abi()).ok() }
    }
    pub unsafe fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCheckboxChecked)(windows_core::Interface::as_raw(self), ncheckboxid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCaption<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCaption)(windows_core::Interface::as_raw(self), psztitle.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetImage(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: Option<super::super::UI::WindowsAndMessaging::HICON>, hbitmap: Option<super::super::Graphics::Gdi::HBITMAP>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetImage)(windows_core::Interface::as_raw(self), nimagetype, hicon.unwrap_or(core::mem::zeroed()) as _, hbitmap.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPercentComplete(&self, npercent: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPercentComplete)(windows_core::Interface::as_raw(self), npercent).ok() }
    }
    pub unsafe fn SetProgressText<P0>(&self, pszprogresstext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProgressText)(windows_core::Interface::as_raw(self), pszprogresstext.param().abi()).ok() }
    }
    pub unsafe fn SetActionLinkCallback<P0>(&self, pphotoprogressactioncb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPhotoProgressActionCB>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActionLinkCallback)(windows_core::Interface::as_raw(self), pphotoprogressactioncb.param().abi()).ok() }
    }
    pub unsafe fn SetActionLinkText<P0>(&self, pszcaption: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActionLinkText)(windows_core::Interface::as_raw(self), pszcaption.param().abi()).ok() }
    }
    pub unsafe fn ShowActionLink(&self, fshow: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowActionLink)(windows_core::Interface::as_raw(self), fshow.into()).ok() }
    }
    pub unsafe fn IsCancelled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCancelled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetUserInput<P1>(&self, riidtype: *const windows_core::GUID, punknown: P1, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetUserInput)(windows_core::Interface::as_raw(self), riidtype, punknown.param().abi(), core::mem::transmute(ppropvarresult), ppropvardefault.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressDialog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ShowCheckbox: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCheckboxText: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetCheckboxCheck: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCheckboxTooltip: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsCheckboxChecked: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_CHECKBOX_ID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, PROGRESS_DIALOG_IMAGE_TYPE, super::super::UI::WindowsAndMessaging::HICON, super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    SetImage: usize,
    pub SetPercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetProgressText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetActionLinkCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActionLinkText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ShowActionLink: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsCancelled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut super::super::System::Com::StructuredStorage::PROPVARIANT, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetUserInput: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoProgressDialog_Impl: windows_core::IUnknownImpl {
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn SetTitle(&self, psztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShowCheckbox(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetCheckboxText(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetCheckboxCheck(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetCheckboxTooltip(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> windows_core::Result<windows_core::BOOL>;
    fn SetCaption(&self, psztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetImage(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
    fn SetPercentComplete(&self, npercent: i32) -> windows_core::Result<()>;
    fn SetProgressText(&self, pszprogresstext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetActionLinkCallback(&self, pphotoprogressactioncb: windows_core::Ref<IPhotoProgressActionCB>) -> windows_core::Result<()>;
    fn SetActionLinkText(&self, pszcaption: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShowActionLink(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsCancelled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetUserInput(&self, riidtype: *const windows_core::GUID, punknown: windows_core::Ref<windows_core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoProgressDialog_Vtbl {
    pub const fn new<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::Create(this, core::mem::transmute_copy(&hwndparent)).into()
            }
        }
        unsafe extern "system" fn GetWindow<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoProgressDialog_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwndprogressdialog.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Destroy<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::Destroy(this).into()
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetTitle(this, core::mem::transmute(&psztitle)).into()
            }
        }
        unsafe extern "system" fn ShowCheckbox<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::ShowCheckbox(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn SetCheckboxText<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetCheckboxText(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute(&pszcheckboxtext)).into()
            }
        }
        unsafe extern "system" fn SetCheckboxCheck<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetCheckboxCheck(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute_copy(&fchecked)).into()
            }
        }
        unsafe extern "system" fn SetCheckboxTooltip<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetCheckboxTooltip(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute(&pszcheckboxtooltiptext)).into()
            }
        }
        unsafe extern "system" fn IsCheckboxChecked<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoProgressDialog_Impl::IsCheckboxChecked(this, core::mem::transmute_copy(&ncheckboxid)) {
                    Ok(ok__) => {
                        pfchecked.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaption<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetCaption(this, core::mem::transmute(&psztitle)).into()
            }
        }
        unsafe extern "system" fn SetImage<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetImage(this, core::mem::transmute_copy(&nimagetype), core::mem::transmute_copy(&hicon), core::mem::transmute_copy(&hbitmap)).into()
            }
        }
        unsafe extern "system" fn SetPercentComplete<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, npercent: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetPercentComplete(this, core::mem::transmute_copy(&npercent)).into()
            }
        }
        unsafe extern "system" fn SetProgressText<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprogresstext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetProgressText(this, core::mem::transmute(&pszprogresstext)).into()
            }
        }
        unsafe extern "system" fn SetActionLinkCallback<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoprogressactioncb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetActionLinkCallback(this, core::mem::transmute_copy(&pphotoprogressactioncb)).into()
            }
        }
        unsafe extern "system" fn SetActionLinkText<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcaption: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::SetActionLinkText(this, core::mem::transmute(&pszcaption)).into()
            }
        }
        unsafe extern "system" fn ShowActionLink<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::ShowActionLink(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn IsCancelled<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcancelled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhotoProgressDialog_Impl::IsCancelled(this) {
                    Ok(ok__) => {
                        pfcancelled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserInput<Identity: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riidtype: *const windows_core::GUID, punknown: *mut core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhotoProgressDialog_Impl::GetUserInput(this, core::mem::transmute_copy(&riidtype), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&ppropvarresult), core::mem::transmute_copy(&ppropvardefault)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            GetWindow: GetWindow::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            ShowCheckbox: ShowCheckbox::<Identity, OFFSET>,
            SetCheckboxText: SetCheckboxText::<Identity, OFFSET>,
            SetCheckboxCheck: SetCheckboxCheck::<Identity, OFFSET>,
            SetCheckboxTooltip: SetCheckboxTooltip::<Identity, OFFSET>,
            IsCheckboxChecked: IsCheckboxChecked::<Identity, OFFSET>,
            SetCaption: SetCaption::<Identity, OFFSET>,
            SetImage: SetImage::<Identity, OFFSET>,
            SetPercentComplete: SetPercentComplete::<Identity, OFFSET>,
            SetProgressText: SetProgressText::<Identity, OFFSET>,
            SetActionLinkCallback: SetActionLinkCallback::<Identity, OFFSET>,
            SetActionLinkText: SetActionLinkText::<Identity, OFFSET>,
            ShowActionLink: ShowActionLink::<Identity, OFFSET>,
            IsCancelled: IsCancelled::<Identity, OFFSET>,
            GetUserInput: GetUserInput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoProgressDialog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IPhotoProgressDialog {}
windows_core::imp::define_interface!(IUserInputString, IUserInputString_Vtbl, 0x00f243a1_205b_45ba_ae26_abbc53aa7a6f);
windows_core::imp::interface_hierarchy!(IUserInputString, windows_core::IUnknown);
impl IUserInputString {
    pub unsafe fn GetSubmitButtonText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubmitButtonText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPrompt(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrompt)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetStringId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetStringType(&self) -> windows_core::Result<USER_INPUT_STRING_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTooltipText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTooltipText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDefault(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetMruCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMruCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMruEntryAt(&self, nindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMruEntryAt)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetImage(&self, nsize: u32, phbitmap: Option<*mut super::super::Graphics::Gdi::HBITMAP>, phicon: Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImage)(windows_core::Interface::as_raw(self), nsize, phbitmap.unwrap_or(core::mem::zeroed()) as _, phicon.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserInputString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSubmitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStringId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStringType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut USER_INPUT_STRING_TYPE) -> windows_core::HRESULT,
    pub GetTooltipText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMruCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMruEntryAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetImage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    GetImage: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IUserInputString_Impl: windows_core::IUnknownImpl {
    fn GetSubmitButtonText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPrompt(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetStringId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetStringType(&self) -> windows_core::Result<USER_INPUT_STRING_TYPE>;
    fn GetTooltipText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetMaxLength(&self) -> windows_core::Result<u32>;
    fn GetDefault(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetMruCount(&self) -> windows_core::Result<u32>;
    fn GetMruEntryAt(&self, nindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetImage(&self, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IUserInputString_Vtbl {
    pub const fn new<Identity: IUserInputString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSubmitButtonText<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmitbuttontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetSubmitButtonText(this) {
                    Ok(ok__) => {
                        pbstrsubmitbuttontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrompt<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprompttitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetPrompt(this) {
                    Ok(ok__) => {
                        pbstrprompttitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringId<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstringid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetStringId(this) {
                    Ok(ok__) => {
                        pbstrstringid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringType<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetStringType(this) {
                    Ok(ok__) => {
                        pnstringtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTooltipText<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtooltiptext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetTooltipText(this) {
                    Ok(ok__) => {
                        pbstrtooltiptext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchmaxlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetMaxLength(this) {
                    Ok(ok__) => {
                        pcchmaxlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefault<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdefault: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetDefault(this) {
                    Ok(ok__) => {
                        pbstrdefault.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMruCount<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnmrucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetMruCount(this) {
                    Ok(ok__) => {
                        pnmrucount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMruEntryAt<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pbstrmruentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserInputString_Impl::GetMruEntryAt(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        pbstrmruentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImage<Identity: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserInputString_Impl::GetImage(this, core::mem::transmute_copy(&nsize), core::mem::transmute_copy(&phbitmap), core::mem::transmute_copy(&phicon)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSubmitButtonText: GetSubmitButtonText::<Identity, OFFSET>,
            GetPrompt: GetPrompt::<Identity, OFFSET>,
            GetStringId: GetStringId::<Identity, OFFSET>,
            GetStringType: GetStringType::<Identity, OFFSET>,
            GetTooltipText: GetTooltipText::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetDefault: GetDefault::<Identity, OFFSET>,
            GetMruCount: GetMruCount::<Identity, OFFSET>,
            GetMruEntryAt: GetMruEntryAt::<Identity, OFFSET>,
            GetImage: GetImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserInputString as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IUserInputString {}
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
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 7 };
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 10 };
pub const PKEY_PhotoAcquire_FinalFilename: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 3 };
pub const PKEY_PhotoAcquire_GroupTag: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 4 };
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 8 };
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 6 };
pub const PKEY_PhotoAcquire_RelativePathname: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 2 };
pub const PKEY_PhotoAcquire_SkipImport: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 9 };
pub const PKEY_PhotoAcquire_TransferResult: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 5 };
pub const PROGRESS_DIALOG_BITMAP_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROGRESS_DIALOG_CHECKBOX_ID(pub i32);
pub const PROGRESS_DIALOG_CHECKBOX_ID_DEFAULT: PROGRESS_DIALOG_CHECKBOX_ID = PROGRESS_DIALOG_CHECKBOX_ID(0i32);
pub const PROGRESS_DIALOG_ICON_LARGE: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(1i32);
pub const PROGRESS_DIALOG_ICON_SMALL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(0i32);
pub const PROGRESS_DIALOG_ICON_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROGRESS_DIALOG_IMAGE_TYPE(pub i32);
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
pub const PhotoAcquire: windows_core::GUID = windows_core::GUID::from_u128(0x00f26e02_e9f2_4a9f_9fdd_5a962fb26a98);
pub const PhotoAcquireAutoPlayDropTarget: windows_core::GUID = windows_core::GUID::from_u128(0x00f20eb5_8fd6_4d9d_b75e_36801766c8f1);
pub const PhotoAcquireAutoPlayHWEventHandler: windows_core::GUID = windows_core::GUID::from_u128(0x00f2b433_44e4_4d88_b2b0_2698a0a91dba);
pub const PhotoAcquireDeviceSelectionDialog: windows_core::GUID = windows_core::GUID::from_u128(0x00f29a34_b8a1_482c_bcf8_3ac7b0fe8f62);
pub const PhotoAcquireOptionsDialog: windows_core::GUID = windows_core::GUID::from_u128(0x00f210a1_62f0_438b_9f7e_9618d72a1831);
pub const PhotoProgressDialog: windows_core::GUID = windows_core::GUID::from_u128(0x00f24ca0_748f_4e8a_894f_0e0357c6799f);
pub const USER_INPUT_DEFAULT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(0i32);
pub const USER_INPUT_PATH_ELEMENT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INPUT_STRING_TYPE(pub i32);
