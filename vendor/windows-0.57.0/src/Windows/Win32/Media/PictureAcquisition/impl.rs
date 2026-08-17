#[cfg(feature = "Win32_System_Com")]
pub trait IPhotoAcquire_Impl: Sized {
    fn CreatePhotoSource(&self, pszdevice: &windows_core::PCWSTR) -> windows_core::Result<IPhotoAcquireSource>;
    fn Acquire(&self, pphotoacquiresource: Option<&IPhotoAcquireSource>, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: &windows_core::PCWSTR, pphotoacquireprogresscb: Option<&IPhotoAcquireProgressCB>) -> windows_core::Result<()>;
    fn EnumResults(&self) -> windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPhotoAcquire {}
#[cfg(feature = "Win32_System_Com")]
impl IPhotoAcquire_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>() -> IPhotoAcquire_Vtbl {
        unsafe extern "system" fn CreatePhotoSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdevice: windows_core::PCWSTR, ppphotoacquiresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquire_Impl::CreatePhotoSource(this, core::mem::transmute(&pszdevice)) {
                Ok(ok__) => {
                    core::ptr::write(ppphotoacquiresource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: windows_core::PCWSTR, pphotoacquireprogresscb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquire_Impl::Acquire(this, windows_core::from_raw_borrowed(&pphotoacquiresource), core::mem::transmute_copy(&fshowprogress), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&pszapplicationname), windows_core::from_raw_borrowed(&pphotoacquireprogresscb)).into()
        }
        unsafe extern "system" fn EnumResults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumfilepaths: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquire_Impl::EnumResults(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumfilepaths, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePhotoSource: CreatePhotoSource::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            EnumResults: EnumResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquire as windows_core::Interface>::IID
    }
}
pub trait IPhotoAcquireDeviceSelectionDialog_Impl: Sized {
    fn SetTitle(&self, psztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSubmitButtonText(&self, pszsubmitbuttontext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DoModal(&self, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut windows_core::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPhotoAcquireDeviceSelectionDialog {}
impl IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>() -> IPhotoAcquireDeviceSelectionDialog_Vtbl {
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireDeviceSelectionDialog_Impl::SetTitle(this, core::mem::transmute(&psztitle)).into()
        }
        unsafe extern "system" fn SetSubmitButtonText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubmitbuttontext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireDeviceSelectionDialog_Impl::SetSubmitButtonText(this, core::mem::transmute(&pszsubmitbuttontext)).into()
        }
        unsafe extern "system" fn DoModal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireDeviceSelectionDialog_Impl::DoModal(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwdeviceflags), core::mem::transmute_copy(&pbstrdeviceid), core::mem::transmute_copy(&pndevicetype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            SetSubmitButtonText: SetSubmitButtonText::<Identity, Impl, OFFSET>,
            DoModal: DoModal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireDeviceSelectionDialog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquireItem_Impl: Sized {
    fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetThumbnail(&self, sizethumbnail: &super::super::Foundation::SIZE) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
    fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn CanDelete(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetSubItemCount(&self) -> windows_core::Result<u32>;
    fn GetSubItemAt(&self, nitemindex: u32) -> windows_core::Result<IPhotoAcquireItem>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IPhotoAcquireItem {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquireItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>() -> IPhotoAcquireItem_Vtbl {
        unsafe extern "system" fn GetItemName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstritemname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireItem_Impl::GetItemName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstritemname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireItem_Impl::GetThumbnail(this, core::mem::transmute(&sizethumbnail)) {
                Ok(ok__) => {
                    core::ptr::write(phbmpthumbnail, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireItem_Impl::GetProperty(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    core::ptr::write(pv, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireItem_Impl::SetProperty(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pv)).into()
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireItem_Impl::GetStream(this) {
                Ok(ok__) => {
                    core::ptr::write(ppstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireItem_Impl::CanDelete(this) {
                Ok(ok__) => {
                    core::ptr::write(pfcandelete, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireItem_Impl::Delete(this).into()
        }
        unsafe extern "system" fn GetSubItemCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireItem_Impl::GetSubItemCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pncount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubItemAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireItem_Impl::GetSubItemAt(this, core::mem::transmute_copy(&nitemindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppphotoacquireitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemName: GetItemName::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            CanDelete: CanDelete::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetSubItemCount: GetSubItemCount::<Identity, Impl, OFFSET>,
            GetSubItemAt: GetSubItemAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireItem as windows_core::Interface>::IID
    }
}
pub trait IPhotoAcquireOptionsDialog_Impl: Sized {
    fn Initialize(&self, pszregistryroot: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<super::super::Foundation::HWND>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn DoModal(&self, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> windows_core::Result<()>;
    fn SaveData(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPhotoAcquireOptionsDialog {}
impl IPhotoAcquireOptionsDialog_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>() -> IPhotoAcquireOptionsDialog_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszregistryroot: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireOptionsDialog_Impl::Initialize(this, core::mem::transmute(&pszregistryroot)).into()
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireOptionsDialog_Impl::Create(this, core::mem::transmute_copy(&hwndparent)) {
                Ok(ok__) => {
                    core::ptr::write(phwnddialog, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireOptionsDialog_Impl::Destroy(this).into()
        }
        unsafe extern "system" fn DoModal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireOptionsDialog_Impl::DoModal(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ppnreturncode)).into()
        }
        unsafe extern "system" fn SaveData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireOptionsDialog_Impl::SaveData(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            DoModal: DoModal::<Identity, Impl, OFFSET>,
            SaveData: SaveData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireOptionsDialog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquirePlugin_Impl: Sized {
    fn Initialize(&self, pphotoacquiresource: Option<&IPhotoAcquireSource>, pphotoacquireprogresscb: Option<&IPhotoAcquireProgressCB>) -> windows_core::Result<()>;
    fn ProcessItem(&self, dwacquirestage: u32, pphotoacquireitem: Option<&IPhotoAcquireItem>, poriginalitemstream: Option<&super::super::System::Com::IStream>, pszfinalfilename: &windows_core::PCWSTR, ppropertystore: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn TransferComplete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn DisplayConfigureDialog(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IPhotoAcquirePlugin {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquirePlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>() -> IPhotoAcquirePlugin_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void, pphotoacquireprogresscb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquirePlugin_Impl::Initialize(this, windows_core::from_raw_borrowed(&pphotoacquiresource), windows_core::from_raw_borrowed(&pphotoacquireprogresscb)).into()
        }
        unsafe extern "system" fn ProcessItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: *mut core::ffi::c_void, poriginalitemstream: *mut core::ffi::c_void, pszfinalfilename: windows_core::PCWSTR, ppropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquirePlugin_Impl::ProcessItem(this, core::mem::transmute_copy(&dwacquirestage), windows_core::from_raw_borrowed(&pphotoacquireitem), windows_core::from_raw_borrowed(&poriginalitemstream), core::mem::transmute(&pszfinalfilename), windows_core::from_raw_borrowed(&ppropertystore)).into()
        }
        unsafe extern "system" fn TransferComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquirePlugin_Impl::TransferComplete(this, core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn DisplayConfigureDialog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquirePlugin_Impl::DisplayConfigureDialog(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ProcessItem: ProcessItem::<Identity, Impl, OFFSET>,
            TransferComplete: TransferComplete::<Identity, Impl, OFFSET>,
            DisplayConfigureDialog: DisplayConfigureDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquirePlugin as windows_core::Interface>::IID
    }
}
pub trait IPhotoAcquireProgressCB_Impl: Sized {
    fn Cancelled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn StartEnumeration(&self, pphotoacquiresource: Option<&IPhotoAcquireSource>) -> windows_core::Result<()>;
    fn FoundItem(&self, pphotoacquireitem: Option<&IPhotoAcquireItem>) -> windows_core::Result<()>;
    fn EndEnumeration(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn StartTransfer(&self, pphotoacquiresource: Option<&IPhotoAcquireSource>) -> windows_core::Result<()>;
    fn StartItemTransfer(&self, nitemindex: u32, pphotoacquireitem: Option<&IPhotoAcquireItem>) -> windows_core::Result<()>;
    fn DirectoryCreated(&self, pszdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UpdateTransferPercent(&self, foverall: super::super::Foundation::BOOL, npercent: u32) -> windows_core::Result<()>;
    fn EndItemTransfer(&self, nitemindex: u32, pphotoacquireitem: Option<&IPhotoAcquireItem>, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn EndTransfer(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn StartDelete(&self, pphotoacquiresource: Option<&IPhotoAcquireSource>) -> windows_core::Result<()>;
    fn StartItemDelete(&self, nitemindex: u32, pphotoacquireitem: Option<&IPhotoAcquireItem>) -> windows_core::Result<()>;
    fn UpdateDeletePercent(&self, npercent: u32) -> windows_core::Result<()>;
    fn EndItemDelete(&self, nitemindex: u32, pphotoacquireitem: Option<&IPhotoAcquireItem>, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn EndDelete(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn EndSession(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetDeleteAfterAcquire(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ErrorAdvise(&self, hr: windows_core::HRESULT, pszerrormessage: &windows_core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> windows_core::Result<ERROR_ADVISE_RESULT>;
    fn GetUserInput(&self, riidtype: *const windows_core::GUID, punknown: Option<&windows_core::IUnknown>, ppropvarresult: *mut windows_core::PROPVARIANT, ppropvardefault: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPhotoAcquireProgressCB {}
impl IPhotoAcquireProgressCB_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>() -> IPhotoAcquireProgressCB_Vtbl {
        unsafe extern "system" fn Cancelled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireProgressCB_Impl::Cancelled(this) {
                Ok(ok__) => {
                    core::ptr::write(pfcancelled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::StartEnumeration(this, windows_core::from_raw_borrowed(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn FoundItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquireitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::FoundItem(this, windows_core::from_raw_borrowed(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::EndEnumeration(this, core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn StartTransfer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::StartTransfer(this, windows_core::from_raw_borrowed(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn StartItemTransfer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::StartItemTransfer(this, core::mem::transmute_copy(&nitemindex), windows_core::from_raw_borrowed(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn DirectoryCreated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdirectory: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::DirectoryCreated(this, core::mem::transmute(&pszdirectory)).into()
        }
        unsafe extern "system" fn UpdateTransferPercent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::UpdateTransferPercent(this, core::mem::transmute_copy(&foverall), core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn EndItemTransfer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::EndItemTransfer(this, core::mem::transmute_copy(&nitemindex), windows_core::from_raw_borrowed(&pphotoacquireitem), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndTransfer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::EndTransfer(this, core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn StartDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoacquiresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::StartDelete(this, windows_core::from_raw_borrowed(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn StartItemDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::StartItemDelete(this, core::mem::transmute_copy(&nitemindex), windows_core::from_raw_borrowed(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn UpdateDeletePercent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, npercent: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::UpdateDeletePercent(this, core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn EndItemDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::EndItemDelete(this, core::mem::transmute_copy(&nitemindex), windows_core::from_raw_borrowed(&pphotoacquireitem), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::EndDelete(this, core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::EndSession(this, core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn GetDeleteAfterAcquire<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireProgressCB_Impl::GetDeleteAfterAcquire(this) {
                Ok(ok__) => {
                    core::ptr::write(pfdeleteafteracquire, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorAdvise<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pszerrormessage: windows_core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireProgressCB_Impl::ErrorAdvise(this, core::mem::transmute_copy(&hr), core::mem::transmute(&pszerrormessage), core::mem::transmute_copy(&nmessagetype)) {
                Ok(ok__) => {
                    core::ptr::write(pnerroradviseresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riidtype: *const windows_core::GUID, punknown: *mut core::ffi::c_void, ppropvarresult: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, ppropvardefault: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireProgressCB_Impl::GetUserInput(this, core::mem::transmute_copy(&riidtype), windows_core::from_raw_borrowed(&punknown), core::mem::transmute_copy(&ppropvarresult), core::mem::transmute_copy(&ppropvardefault)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancelled: Cancelled::<Identity, Impl, OFFSET>,
            StartEnumeration: StartEnumeration::<Identity, Impl, OFFSET>,
            FoundItem: FoundItem::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            StartTransfer: StartTransfer::<Identity, Impl, OFFSET>,
            StartItemTransfer: StartItemTransfer::<Identity, Impl, OFFSET>,
            DirectoryCreated: DirectoryCreated::<Identity, Impl, OFFSET>,
            UpdateTransferPercent: UpdateTransferPercent::<Identity, Impl, OFFSET>,
            EndItemTransfer: EndItemTransfer::<Identity, Impl, OFFSET>,
            EndTransfer: EndTransfer::<Identity, Impl, OFFSET>,
            StartDelete: StartDelete::<Identity, Impl, OFFSET>,
            StartItemDelete: StartItemDelete::<Identity, Impl, OFFSET>,
            UpdateDeletePercent: UpdateDeletePercent::<Identity, Impl, OFFSET>,
            EndItemDelete: EndItemDelete::<Identity, Impl, OFFSET>,
            EndDelete: EndDelete::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
            GetDeleteAfterAcquire: GetDeleteAfterAcquire::<Identity, Impl, OFFSET>,
            ErrorAdvise: ErrorAdvise::<Identity, Impl, OFFSET>,
            GetUserInput: GetUserInput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireProgressCB as windows_core::Interface>::IID
    }
}
pub trait IPhotoAcquireSettings_Impl: Sized {
    fn InitializeFromRegistry(&self, pszregistrykey: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFlags(&self, dwphotoacquireflags: u32) -> windows_core::Result<()>;
    fn SetOutputFilenameTemplate(&self, psztemplate: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSequencePaddingWidth(&self, dwwidth: u32) -> windows_core::Result<()>;
    fn SetSequenceZeroPadding(&self, fzeropad: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetGroupTag(&self, pszgrouptag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn GetOutputFilenameTemplate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSequencePaddingWidth(&self) -> windows_core::Result<u32>;
    fn GetSequenceZeroPadding(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetGroupTag(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAcquisitionTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
}
impl windows_core::RuntimeName for IPhotoAcquireSettings {}
impl IPhotoAcquireSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>() -> IPhotoAcquireSettings_Vtbl {
        unsafe extern "system" fn InitializeFromRegistry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszregistrykey: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSettings_Impl::InitializeFromRegistry(this, core::mem::transmute(&pszregistrykey)).into()
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwphotoacquireflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSettings_Impl::SetFlags(this, core::mem::transmute_copy(&dwphotoacquireflags)).into()
        }
        unsafe extern "system" fn SetOutputFilenameTemplate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztemplate: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSettings_Impl::SetOutputFilenameTemplate(this, core::mem::transmute(&psztemplate)).into()
        }
        unsafe extern "system" fn SetSequencePaddingWidth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwidth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSettings_Impl::SetSequencePaddingWidth(this, core::mem::transmute_copy(&dwwidth)).into()
        }
        unsafe extern "system" fn SetSequenceZeroPadding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSettings_Impl::SetSequenceZeroPadding(this, core::mem::transmute_copy(&fzeropad)).into()
        }
        unsafe extern "system" fn SetGroupTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgrouptag: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSettings_Impl::SetGroupTag(this, core::mem::transmute(&pszgrouptag)).into()
        }
        unsafe extern "system" fn SetAcquisitionTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSettings_Impl::SetAcquisitionTime(this, core::mem::transmute_copy(&pftacquisitiontime)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSettings_Impl::GetFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwphotoacquireflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFilenameTemplate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtemplate: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSettings_Impl::GetOutputFilenameTemplate(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtemplate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequencePaddingWidth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSettings_Impl::GetSequencePaddingWidth(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwwidth, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequenceZeroPadding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSettings_Impl::GetSequenceZeroPadding(this) {
                Ok(ok__) => {
                    core::ptr::write(pfzeropad, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrgrouptag: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSettings_Impl::GetGroupTag(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrgrouptag, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcquisitionTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSettings_Impl::GetAcquisitionTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pftacquisitiontime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromRegistry: InitializeFromRegistry::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            SetOutputFilenameTemplate: SetOutputFilenameTemplate::<Identity, Impl, OFFSET>,
            SetSequencePaddingWidth: SetSequencePaddingWidth::<Identity, Impl, OFFSET>,
            SetSequenceZeroPadding: SetSequenceZeroPadding::<Identity, Impl, OFFSET>,
            SetGroupTag: SetGroupTag::<Identity, Impl, OFFSET>,
            SetAcquisitionTime: SetAcquisitionTime::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetOutputFilenameTemplate: GetOutputFilenameTemplate::<Identity, Impl, OFFSET>,
            GetSequencePaddingWidth: GetSequencePaddingWidth::<Identity, Impl, OFFSET>,
            GetSequenceZeroPadding: GetSequenceZeroPadding::<Identity, Impl, OFFSET>,
            GetGroupTag: GetGroupTag::<Identity, Impl, OFFSET>,
            GetAcquisitionTime: GetAcquisitionTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireSettings as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IPhotoAcquireSource_Impl: Sized {
    fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDeviceIcons(&self, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<()>;
    fn InitializeItemList(&self, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: Option<&IPhotoAcquireProgressCB>, pnitemcount: *mut u32) -> windows_core::Result<()>;
    fn GetItemCount(&self) -> windows_core::Result<u32>;
    fn GetItemAt(&self, nindex: u32) -> windows_core::Result<IPhotoAcquireItem>;
    fn GetPhotoAcquireSettings(&self) -> windows_core::Result<IPhotoAcquireSettings>;
    fn GetDeviceId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BindToObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IPhotoAcquireSource {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IPhotoAcquireSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>() -> IPhotoAcquireSource_Vtbl {
        unsafe extern "system" fn GetFriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfriendlyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSource_Impl::GetFriendlyName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfriendlyname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcons<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSource_Impl::GetDeviceIcons(this, core::mem::transmute_copy(&nsize), core::mem::transmute_copy(&phlargeicon), core::mem::transmute_copy(&phsmallicon)).into()
        }
        unsafe extern "system" fn InitializeItemList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: *mut core::ffi::c_void, pnitemcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSource_Impl::InitializeItemList(this, core::mem::transmute_copy(&fforceenumeration), windows_core::from_raw_borrowed(&pphotoacquireprogresscb), core::mem::transmute_copy(&pnitemcount)).into()
        }
        unsafe extern "system" fn GetItemCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnitemcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSource_Impl::GetItemCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pnitemcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSource_Impl::GetItemAt(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppphotoacquireitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhotoAcquireSettings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppphotoacquiresettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSource_Impl::GetPhotoAcquireSettings(this) {
                Ok(ok__) => {
                    core::ptr::write(ppphotoacquiresettings, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdeviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoAcquireSource_Impl::GetDeviceId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdeviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoAcquireSource_Impl::BindToObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            GetDeviceIcons: GetDeviceIcons::<Identity, Impl, OFFSET>,
            InitializeItemList: InitializeItemList::<Identity, Impl, OFFSET>,
            GetItemCount: GetItemCount::<Identity, Impl, OFFSET>,
            GetItemAt: GetItemAt::<Identity, Impl, OFFSET>,
            GetPhotoAcquireSettings: GetPhotoAcquireSettings::<Identity, Impl, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            BindToObject: BindToObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoAcquireSource as windows_core::Interface>::IID
    }
}
pub trait IPhotoProgressActionCB_Impl: Sized {
    fn DoAction(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPhotoProgressActionCB {}
impl IPhotoProgressActionCB_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressActionCB_Impl, const OFFSET: isize>() -> IPhotoProgressActionCB_Vtbl {
        unsafe extern "system" fn DoAction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressActionCB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressActionCB_Impl::DoAction(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoAction: DoAction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoProgressActionCB as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoProgressDialog_Impl: Sized {
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn SetTitle(&self, psztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShowCheckbox(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetCheckboxText(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetCheckboxCheck(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetCheckboxTooltip(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCaption(&self, psztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetImage(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
    fn SetPercentComplete(&self, npercent: i32) -> windows_core::Result<()>;
    fn SetProgressText(&self, pszprogresstext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetActionLinkCallback(&self, pphotoprogressactioncb: Option<&IPhotoProgressActionCB>) -> windows_core::Result<()>;
    fn SetActionLinkText(&self, pszcaption: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShowActionLink(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsCancelled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetUserInput(&self, riidtype: *const windows_core::GUID, punknown: Option<&windows_core::IUnknown>, ppropvarresult: *mut windows_core::PROPVARIANT, ppropvardefault: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IPhotoProgressDialog {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoProgressDialog_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>() -> IPhotoProgressDialog_Vtbl {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::Create(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoProgressDialog_Impl::GetWindow(this) {
                Ok(ok__) => {
                    core::ptr::write(phwndprogressdialog, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::Destroy(this).into()
        }
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetTitle(this, core::mem::transmute(&psztitle)).into()
        }
        unsafe extern "system" fn ShowCheckbox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::ShowCheckbox(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn SetCheckboxText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetCheckboxText(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute(&pszcheckboxtext)).into()
        }
        unsafe extern "system" fn SetCheckboxCheck<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetCheckboxCheck(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute_copy(&fchecked)).into()
        }
        unsafe extern "system" fn SetCheckboxTooltip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetCheckboxTooltip(this, core::mem::transmute_copy(&ncheckboxid), core::mem::transmute(&pszcheckboxtooltiptext)).into()
        }
        unsafe extern "system" fn IsCheckboxChecked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoProgressDialog_Impl::IsCheckboxChecked(this, core::mem::transmute_copy(&ncheckboxid)) {
                Ok(ok__) => {
                    core::ptr::write(pfchecked, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetCaption(this, core::mem::transmute(&psztitle)).into()
        }
        unsafe extern "system" fn SetImage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetImage(this, core::mem::transmute_copy(&nimagetype), core::mem::transmute_copy(&hicon), core::mem::transmute_copy(&hbitmap)).into()
        }
        unsafe extern "system" fn SetPercentComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, npercent: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetPercentComplete(this, core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn SetProgressText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprogresstext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetProgressText(this, core::mem::transmute(&pszprogresstext)).into()
        }
        unsafe extern "system" fn SetActionLinkCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphotoprogressactioncb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetActionLinkCallback(this, windows_core::from_raw_borrowed(&pphotoprogressactioncb)).into()
        }
        unsafe extern "system" fn SetActionLinkText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcaption: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::SetActionLinkText(this, core::mem::transmute(&pszcaption)).into()
        }
        unsafe extern "system" fn ShowActionLink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::ShowActionLink(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn IsCancelled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPhotoProgressDialog_Impl::IsCancelled(this) {
                Ok(ok__) => {
                    core::ptr::write(pfcancelled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riidtype: *const windows_core::GUID, punknown: *mut core::ffi::c_void, ppropvarresult: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, ppropvardefault: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPhotoProgressDialog_Impl::GetUserInput(this, core::mem::transmute_copy(&riidtype), windows_core::from_raw_borrowed(&punknown), core::mem::transmute_copy(&ppropvarresult), core::mem::transmute_copy(&ppropvardefault)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            ShowCheckbox: ShowCheckbox::<Identity, Impl, OFFSET>,
            SetCheckboxText: SetCheckboxText::<Identity, Impl, OFFSET>,
            SetCheckboxCheck: SetCheckboxCheck::<Identity, Impl, OFFSET>,
            SetCheckboxTooltip: SetCheckboxTooltip::<Identity, Impl, OFFSET>,
            IsCheckboxChecked: IsCheckboxChecked::<Identity, Impl, OFFSET>,
            SetCaption: SetCaption::<Identity, Impl, OFFSET>,
            SetImage: SetImage::<Identity, Impl, OFFSET>,
            SetPercentComplete: SetPercentComplete::<Identity, Impl, OFFSET>,
            SetProgressText: SetProgressText::<Identity, Impl, OFFSET>,
            SetActionLinkCallback: SetActionLinkCallback::<Identity, Impl, OFFSET>,
            SetActionLinkText: SetActionLinkText::<Identity, Impl, OFFSET>,
            ShowActionLink: ShowActionLink::<Identity, Impl, OFFSET>,
            IsCancelled: IsCancelled::<Identity, Impl, OFFSET>,
            GetUserInput: GetUserInput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhotoProgressDialog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IUserInputString_Impl: Sized {
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
impl windows_core::RuntimeName for IUserInputString {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IUserInputString_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>() -> IUserInputString_Vtbl {
        unsafe extern "system" fn GetSubmitButtonText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubmitbuttontext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetSubmitButtonText(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubmitbuttontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrompt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprompttitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetPrompt(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrprompttitle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstringid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetStringId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrstringid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetStringType(this) {
                Ok(ok__) => {
                    core::ptr::write(pnstringtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTooltipText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtooltiptext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetTooltipText(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtooltiptext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchmaxlength: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetMaxLength(this) {
                Ok(ok__) => {
                    core::ptr::write(pcchmaxlength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdefault: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetDefault(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdefault, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnmrucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetMruCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pnmrucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruEntryAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pbstrmruentry: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserInputString_Impl::GetMruEntryAt(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrmruentry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUserInputString_Impl::GetImage(this, core::mem::transmute_copy(&nsize), core::mem::transmute_copy(&phbitmap), core::mem::transmute_copy(&phicon)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSubmitButtonText: GetSubmitButtonText::<Identity, Impl, OFFSET>,
            GetPrompt: GetPrompt::<Identity, Impl, OFFSET>,
            GetStringId: GetStringId::<Identity, Impl, OFFSET>,
            GetStringType: GetStringType::<Identity, Impl, OFFSET>,
            GetTooltipText: GetTooltipText::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetDefault: GetDefault::<Identity, Impl, OFFSET>,
            GetMruCount: GetMruCount::<Identity, Impl, OFFSET>,
            GetMruEntryAt: GetMruEntryAt::<Identity, Impl, OFFSET>,
            GetImage: GetImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserInputString as windows_core::Interface>::IID
    }
}
