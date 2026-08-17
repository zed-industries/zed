#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowBulkCapabilities_Impl: Sized + ISideShowCapabilities_Impl {
    fn GetCapabilities(&self, in_keycollection: Option<&ISideShowKeyCollection>, inout_pvalues: *mut Option<ISideShowPropVariantCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISideShowBulkCapabilities {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowBulkCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowBulkCapabilities_Impl, const OFFSET: isize>() -> ISideShowBulkCapabilities_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowBulkCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_keycollection: *mut core::ffi::c_void, inout_pvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowBulkCapabilities_Impl::GetCapabilities(this, windows_core::from_raw_borrowed(&in_keycollection), core::mem::transmute_copy(&inout_pvalues)).into()
        }
        Self { base__: ISideShowCapabilities_Vtbl::new::<Identity, Impl, OFFSET>(), GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowBulkCapabilities as windows_core::Interface>::IID || iid == &<ISideShowCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowCapabilities_Impl: Sized {
    fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISideShowCapabilities {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowCapabilities_Impl, const OFFSET: isize>() -> ISideShowCapabilities_Vtbl {
        unsafe extern "system" fn GetCapability<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowCapabilities_Impl::GetCapability(this, core::mem::transmute_copy(&in_keycapability), core::mem::transmute_copy(&inout_pvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCapability: GetCapability::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowCapabilities as windows_core::Interface>::IID
    }
}
pub trait ISideShowCapabilitiesCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, in_dwindex: u32) -> windows_core::Result<ISideShowCapabilities>;
}
impl windows_core::RuntimeName for ISideShowCapabilitiesCollection {}
impl ISideShowCapabilitiesCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>() -> ISideShowCapabilitiesCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pdwcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowCapabilitiesCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(out_pdwcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowCapabilitiesCollection_Impl::GetAt(this, core::mem::transmute_copy(&in_dwindex)) {
                Ok(ok__) => {
                    core::ptr::write(out_ppcapabilities, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowCapabilitiesCollection as windows_core::Interface>::IID
    }
}
pub trait ISideShowContent_Impl: Sized {
    fn GetContent(&self, in_picapabilities: Option<&ISideShowCapabilities>, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::Result<()>;
    fn ContentId(&self) -> windows_core::Result<u32>;
    fn DifferentiateContent(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ISideShowContent {}
impl ISideShowContent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: isize>() -> ISideShowContent_Vtbl {
        unsafe extern "system" fn GetContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picapabilities: *mut core::ffi::c_void, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowContent_Impl::GetContent(this, windows_core::from_raw_borrowed(&in_picapabilities), core::mem::transmute_copy(&out_pdwsize), core::mem::transmute_copy(&out_ppbdata)).into()
        }
        unsafe extern "system" fn ContentId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pcontentid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowContent_Impl::ContentId(this) {
                Ok(ok__) => {
                    core::ptr::write(out_pcontentid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DifferentiateContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowContent_Impl::DifferentiateContent(this) {
                Ok(ok__) => {
                    core::ptr::write(out_pfdifferentiatecontent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContent: GetContent::<Identity, Impl, OFFSET>,
            ContentId: ContentId::<Identity, Impl, OFFSET>,
            DifferentiateContent: DifferentiateContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowContent as windows_core::Interface>::IID
    }
}
pub trait ISideShowContentManager_Impl: Sized {
    fn Add(&self, in_picontent: Option<&ISideShowContent>) -> windows_core::Result<()>;
    fn Remove(&self, in_contentid: u32) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn SetEventSink(&self, in_pievents: Option<&ISideShowEvents>) -> windows_core::Result<()>;
    fn GetDeviceCapabilities(&self) -> windows_core::Result<ISideShowCapabilitiesCollection>;
}
impl windows_core::RuntimeName for ISideShowContentManager {}
impl ISideShowContentManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: isize>() -> ISideShowContentManager_Vtbl {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picontent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowContentManager_Impl::Add(this, windows_core::from_raw_borrowed(&in_picontent)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_contentid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowContentManager_Impl::Remove(this, core::mem::transmute_copy(&in_contentid)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowContentManager_Impl::RemoveAll(this).into()
        }
        unsafe extern "system" fn SetEventSink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pievents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowContentManager_Impl::SetEventSink(this, windows_core::from_raw_borrowed(&in_pievents)).into()
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowContentManager_Impl::GetDeviceCapabilities(this) {
                Ok(ok__) => {
                    core::ptr::write(out_ppcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowContentManager as windows_core::Interface>::IID
    }
}
pub trait ISideShowEvents_Impl: Sized {
    fn ContentMissing(&self, in_contentid: u32) -> windows_core::Result<ISideShowContent>;
    fn ApplicationEvent(&self, in_picapabilities: Option<&ISideShowCapabilities>, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> windows_core::Result<()>;
    fn DeviceAdded(&self, in_pidevice: Option<&ISideShowCapabilities>) -> windows_core::Result<()>;
    fn DeviceRemoved(&self, in_pidevice: Option<&ISideShowCapabilities>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISideShowEvents {}
impl ISideShowEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: isize>() -> ISideShowEvents_Vtbl {
        unsafe extern "system" fn ContentMissing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowEvents_Impl::ContentMissing(this, core::mem::transmute_copy(&in_contentid)) {
                Ok(ok__) => {
                    core::ptr::write(out_ppicontent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picapabilities: *mut core::ffi::c_void, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowEvents_Impl::ApplicationEvent(this, windows_core::from_raw_borrowed(&in_picapabilities), core::mem::transmute_copy(&in_dweventid), core::mem::transmute_copy(&in_dweventsize), core::mem::transmute_copy(&in_pbeventdata)).into()
        }
        unsafe extern "system" fn DeviceAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pidevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowEvents_Impl::DeviceAdded(this, windows_core::from_raw_borrowed(&in_pidevice)).into()
        }
        unsafe extern "system" fn DeviceRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pidevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowEvents_Impl::DeviceRemoved(this, windows_core::from_raw_borrowed(&in_pidevice)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ContentMissing: ContentMissing::<Identity, Impl, OFFSET>,
            ApplicationEvent: ApplicationEvent::<Identity, Impl, OFFSET>,
            DeviceAdded: DeviceAdded::<Identity, Impl, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowKeyCollection_Impl: Sized {
    fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISideShowKeyCollection {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowKeyCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>() -> ISideShowKeyCollection_Vtbl {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowKeyCollection_Impl::Add(this, core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowKeyCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowKeyCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowKeyCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowKeyCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowKeyCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ISideShowNotification_Impl: Sized {
    fn NotificationId(&self) -> windows_core::Result<u32>;
    fn SetNotificationId(&self, in_notificationid: u32) -> windows_core::Result<()>;
    fn Title(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTitle(&self, in_pwsztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Message(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetMessage(&self, in_pwszmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Image(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn SetImage(&self, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<()>;
    fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetExpirationTime(&self, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ISideShowNotification {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ISideShowNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>() -> ISideShowNotification_Vtbl {
        unsafe extern "system" fn NotificationId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pnotificationid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowNotification_Impl::NotificationId(this) {
                Ok(ok__) => {
                    core::ptr::write(out_pnotificationid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_notificationid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotification_Impl::SetNotificationId(this, core::mem::transmute_copy(&in_notificationid)).into()
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppwsztitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowNotification_Impl::Title(this) {
                Ok(ok__) => {
                    core::ptr::write(out_ppwsztitle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pwsztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotification_Impl::SetTitle(this, core::mem::transmute(&in_pwsztitle)).into()
        }
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppwszmessage: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowNotification_Impl::Message(this) {
                Ok(ok__) => {
                    core::ptr::write(out_ppwszmessage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pwszmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotification_Impl::SetMessage(this, core::mem::transmute(&in_pwszmessage)).into()
        }
        unsafe extern "system" fn Image<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowNotification_Impl::Image(this) {
                Ok(ok__) => {
                    core::ptr::write(out_phicon, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotification_Impl::SetImage(this, core::mem::transmute_copy(&in_hicon)).into()
        }
        unsafe extern "system" fn ExpirationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowNotification_Impl::ExpirationTime(this) {
                Ok(ok__) => {
                    core::ptr::write(out_ptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotification_Impl::SetExpirationTime(this, core::mem::transmute_copy(&in_ptime)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotificationId: NotificationId::<Identity, Impl, OFFSET>,
            SetNotificationId: SetNotificationId::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
            SetMessage: SetMessage::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            SetImage: SetImage::<Identity, Impl, OFFSET>,
            ExpirationTime: ExpirationTime::<Identity, Impl, OFFSET>,
            SetExpirationTime: SetExpirationTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowNotification as windows_core::Interface>::IID
    }
}
pub trait ISideShowNotificationManager_Impl: Sized {
    fn Show(&self, in_pinotification: Option<&ISideShowNotification>) -> windows_core::Result<()>;
    fn Revoke(&self, in_notificationid: u32) -> windows_core::Result<()>;
    fn RevokeAll(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISideShowNotificationManager {}
impl ISideShowNotificationManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: isize>() -> ISideShowNotificationManager_Vtbl {
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pinotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotificationManager_Impl::Show(this, windows_core::from_raw_borrowed(&in_pinotification)).into()
        }
        unsafe extern "system" fn Revoke<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_notificationid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotificationManager_Impl::Revoke(this, core::mem::transmute_copy(&in_notificationid)).into()
        }
        unsafe extern "system" fn RevokeAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowNotificationManager_Impl::RevokeAll(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, Impl, OFFSET>,
            Revoke: Revoke::<Identity, Impl, OFFSET>,
            RevokeAll: RevokeAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowNotificationManager as windows_core::Interface>::IID
    }
}
pub trait ISideShowPropVariantCollection_Impl: Sized {
    fn Add(&self, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISideShowPropVariantCollection {}
impl ISideShowPropVariantCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>() -> ISideShowPropVariantCollection_Vtbl {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowPropVariantCollection_Impl::Add(this, core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowPropVariantCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowPropVariantCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowPropVariantCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISideShowPropVariantCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowPropVariantCollection as windows_core::Interface>::IID
    }
}
pub trait ISideShowSession_Impl: Sized {
    fn RegisterContent(&self, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID) -> windows_core::Result<ISideShowContentManager>;
    fn RegisterNotifications(&self, in_applicationid: *const windows_core::GUID) -> windows_core::Result<ISideShowNotificationManager>;
}
impl windows_core::RuntimeName for ISideShowSession {}
impl ISideShowSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowSession_Impl, const OFFSET: isize>() -> ISideShowSession_Vtbl {
        unsafe extern "system" fn RegisterContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID, out_ppicontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowSession_Impl::RegisterContent(this, core::mem::transmute_copy(&in_applicationid), core::mem::transmute_copy(&in_endpointid)) {
                Ok(ok__) => {
                    core::ptr::write(out_ppicontent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterNotifications<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISideShowSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_applicationid: *const windows_core::GUID, out_ppinotification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISideShowSession_Impl::RegisterNotifications(this, core::mem::transmute_copy(&in_applicationid)) {
                Ok(ok__) => {
                    core::ptr::write(out_ppinotification, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterContent: RegisterContent::<Identity, Impl, OFFSET>,
            RegisterNotifications: RegisterNotifications::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowSession as windows_core::Interface>::IID
    }
}
