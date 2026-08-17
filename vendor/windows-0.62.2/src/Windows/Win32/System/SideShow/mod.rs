#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct APPLICATION_EVENT_DATA {
    pub cbApplicationEventData: u32,
    pub ApplicationId: windows_core::GUID,
    pub EndpointId: windows_core::GUID,
    pub dwEventId: u32,
    pub cbEventData: u32,
    pub bEventData: [u8; 1],
}
impl Default for APPLICATION_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONTENT_ID_GLANCE: u32 = 0u32;
pub const CONTENT_ID_HOME: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CONTENT_MISSING_EVENT_DATA {
    pub cbContentMissingEventData: u32,
    pub ApplicationId: windows_core::GUID,
    pub EndpointId: windows_core::GUID,
    pub ContentId: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVICE_USER_CHANGE_EVENT_DATA {
    pub cbDeviceUserChangeEventData: u32,
    pub wszUser: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct EVENT_DATA_HEADER {
    pub cbEventDataHeader: u32,
    pub guidEventType: windows_core::GUID,
    pub dwVersion: u32,
    pub cbEventDataSid: u32,
}
pub const GUID_DEVINTERFACE_SIDESHOW: windows_core::GUID = windows_core::GUID::from_u128(0x152e5811_feb9_4b00_90f4_d32947ae1681);
windows_core::imp::define_interface!(ISideShowBulkCapabilities, ISideShowBulkCapabilities_Vtbl, 0x3a2b7fbc_3ad5_48bd_bbf1_0e6cfbd10807);
impl core::ops::Deref for ISideShowBulkCapabilities {
    type Target = ISideShowCapabilities;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowBulkCapabilities, windows_core::IUnknown, ISideShowCapabilities);
impl ISideShowBulkCapabilities {
    pub unsafe fn GetCapabilities<P0>(&self, in_keycollection: P0, inout_pvalues: *mut Option<ISideShowPropVariantCollection>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowKeyCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), in_keycollection.param().abi(), core::mem::transmute(inout_pvalues)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowBulkCapabilities_Vtbl {
    pub base__: ISideShowCapabilities_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISideShowBulkCapabilities_Impl: ISideShowCapabilities_Impl {
    fn GetCapabilities(&self, in_keycollection: windows_core::Ref<ISideShowKeyCollection>, inout_pvalues: windows_core::OutRef<ISideShowPropVariantCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ISideShowBulkCapabilities_Vtbl {
    pub const fn new<Identity: ISideShowBulkCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: ISideShowBulkCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_keycollection: *mut core::ffi::c_void, inout_pvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowBulkCapabilities_Impl::GetCapabilities(this, core::mem::transmute_copy(&in_keycollection), core::mem::transmute_copy(&inout_pvalues)).into()
            }
        }
        Self { base__: ISideShowCapabilities_Vtbl::new::<Identity, OFFSET>(), GetCapabilities: GetCapabilities::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowBulkCapabilities as windows_core::Interface>::IID || iid == &<ISideShowCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISideShowBulkCapabilities {}
windows_core::imp::define_interface!(ISideShowCapabilities, ISideShowCapabilities_Vtbl, 0x535e1379_c09e_4a54_a511_597bab3a72b8);
windows_core::imp::interface_hierarchy!(ISideShowCapabilities, windows_core::IUnknown);
impl ISideShowCapabilities {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCapability(&self, in_keycapability: *const super::super::Foundation::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCapability)(windows_core::Interface::as_raw(self), in_keycapability, core::mem::transmute(inout_pvalue)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetCapability: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetCapability: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISideShowCapabilities_Impl: windows_core::IUnknownImpl {
    fn GetCapability(&self, in_keycapability: *const super::super::Foundation::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ISideShowCapabilities_Vtbl {
    pub const fn new<Identity: ISideShowCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapability<Identity: ISideShowCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_keycapability: *const super::super::Foundation::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowCapabilities_Impl::GetCapability(this, core::mem::transmute_copy(&in_keycapability), core::mem::transmute_copy(&inout_pvalue)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCapability: GetCapability::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISideShowCapabilities {}
windows_core::imp::define_interface!(ISideShowCapabilitiesCollection, ISideShowCapabilitiesCollection_Vtbl, 0x50305597_5e0d_4ff7_b3af_33d0d9bd52dd);
windows_core::imp::interface_hierarchy!(ISideShowCapabilitiesCollection, windows_core::IUnknown);
impl ISideShowCapabilitiesCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, in_dwindex: u32) -> windows_core::Result<ISideShowCapabilities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), in_dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilitiesCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISideShowCapabilitiesCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, in_dwindex: u32) -> windows_core::Result<ISideShowCapabilities>;
}
impl ISideShowCapabilitiesCollection_Vtbl {
    pub const fn new<Identity: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowCapabilitiesCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        out_pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowCapabilitiesCollection_Impl::GetAt(this, core::mem::transmute_copy(&in_dwindex)) {
                    Ok(ok__) => {
                        out_ppcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowCapabilitiesCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISideShowCapabilitiesCollection {}
windows_core::imp::define_interface!(ISideShowContent, ISideShowContent_Vtbl, 0xc18552ed_74ff_4fec_be07_4cfed29d4887);
windows_core::imp::interface_hierarchy!(ISideShowContent, windows_core::IUnknown);
impl ISideShowContent {
    pub unsafe fn GetContent<P0>(&self, in_picapabilities: P0, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetContent)(windows_core::Interface::as_raw(self), in_picapabilities.param().abi(), out_pdwsize as _, out_ppbdata as _).ok() }
    }
    pub unsafe fn ContentId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DifferentiateContent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DifferentiateContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DifferentiateContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ISideShowContent_Impl: windows_core::IUnknownImpl {
    fn GetContent(&self, in_picapabilities: windows_core::Ref<ISideShowCapabilities>, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::Result<()>;
    fn ContentId(&self) -> windows_core::Result<u32>;
    fn DifferentiateContent(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ISideShowContent_Vtbl {
    pub const fn new<Identity: ISideShowContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContent<Identity: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picapabilities: *mut core::ffi::c_void, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowContent_Impl::GetContent(this, core::mem::transmute_copy(&in_picapabilities), core::mem::transmute_copy(&out_pdwsize), core::mem::transmute_copy(&out_ppbdata)).into()
            }
        }
        unsafe extern "system" fn ContentId<Identity: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pcontentid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowContent_Impl::ContentId(this) {
                    Ok(ok__) => {
                        out_pcontentid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DifferentiateContent<Identity: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pfdifferentiatecontent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowContent_Impl::DifferentiateContent(this) {
                    Ok(ok__) => {
                        out_pfdifferentiatecontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContent: GetContent::<Identity, OFFSET>,
            ContentId: ContentId::<Identity, OFFSET>,
            DifferentiateContent: DifferentiateContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowContent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISideShowContent {}
windows_core::imp::define_interface!(ISideShowContentManager, ISideShowContentManager_Vtbl, 0xa5d5b66b_eef9_41db_8d7e_e17c33ab10b0);
windows_core::imp::interface_hierarchy!(ISideShowContentManager, windows_core::IUnknown);
impl ISideShowContentManager {
    pub unsafe fn Add<P0>(&self, in_picontent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowContent>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), in_picontent.param().abi()).ok() }
    }
    pub unsafe fn Remove(&self, in_contentid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), in_contentid).ok() }
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetEventSink<P0>(&self, in_pievents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), in_pievents.param().abi()).ok() }
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> windows_core::Result<ISideShowCapabilitiesCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceCapabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContentManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISideShowContentManager_Impl: windows_core::IUnknownImpl {
    fn Add(&self, in_picontent: windows_core::Ref<ISideShowContent>) -> windows_core::Result<()>;
    fn Remove(&self, in_contentid: u32) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn SetEventSink(&self, in_pievents: windows_core::Ref<ISideShowEvents>) -> windows_core::Result<()>;
    fn GetDeviceCapabilities(&self) -> windows_core::Result<ISideShowCapabilitiesCollection>;
}
impl ISideShowContentManager_Vtbl {
    pub const fn new<Identity: ISideShowContentManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picontent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowContentManager_Impl::Add(this, core::mem::transmute_copy(&in_picontent)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_contentid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowContentManager_Impl::Remove(this, core::mem::transmute_copy(&in_contentid)).into()
            }
        }
        unsafe extern "system" fn RemoveAll<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowContentManager_Impl::RemoveAll(this).into()
            }
        }
        unsafe extern "system" fn SetEventSink<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pievents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowContentManager_Impl::SetEventSink(this, core::mem::transmute_copy(&in_pievents)).into()
            }
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowContentManager_Impl::GetDeviceCapabilities(this) {
                    Ok(ok__) => {
                        out_ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowContentManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISideShowContentManager {}
windows_core::imp::define_interface!(ISideShowEvents, ISideShowEvents_Vtbl, 0x61feca4c_deb4_4a7e_8d75_51f1132d615b);
windows_core::imp::interface_hierarchy!(ISideShowEvents, windows_core::IUnknown);
impl ISideShowEvents {
    pub unsafe fn ContentMissing(&self, in_contentid: u32) -> windows_core::Result<ISideShowContent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentMissing)(windows_core::Interface::as_raw(self), in_contentid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ApplicationEvent<P0>(&self, in_picapabilities: P0, in_dweventid: u32, in_pbeventdata: Option<&[u8]>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        unsafe { (windows_core::Interface::vtable(self).ApplicationEvent)(windows_core::Interface::as_raw(self), in_picapabilities.param().abi(), in_dweventid, in_pbeventdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(in_pbeventdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
    }
    pub unsafe fn DeviceAdded<P0>(&self, in_pidevice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeviceAdded)(windows_core::Interface::as_raw(self), in_pidevice.param().abi()).ok() }
    }
    pub unsafe fn DeviceRemoved<P0>(&self, in_pidevice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeviceRemoved)(windows_core::Interface::as_raw(self), in_pidevice.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ContentMissing: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const u8) -> windows_core::HRESULT,
    pub DeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISideShowEvents_Impl: windows_core::IUnknownImpl {
    fn ContentMissing(&self, in_contentid: u32) -> windows_core::Result<ISideShowContent>;
    fn ApplicationEvent(&self, in_picapabilities: windows_core::Ref<ISideShowCapabilities>, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> windows_core::Result<()>;
    fn DeviceAdded(&self, in_pidevice: windows_core::Ref<ISideShowCapabilities>) -> windows_core::Result<()>;
    fn DeviceRemoved(&self, in_pidevice: windows_core::Ref<ISideShowCapabilities>) -> windows_core::Result<()>;
}
impl ISideShowEvents_Vtbl {
    pub const fn new<Identity: ISideShowEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContentMissing<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowEvents_Impl::ContentMissing(this, core::mem::transmute_copy(&in_contentid)) {
                    Ok(ok__) => {
                        out_ppicontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationEvent<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picapabilities: *mut core::ffi::c_void, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowEvents_Impl::ApplicationEvent(this, core::mem::transmute_copy(&in_picapabilities), core::mem::transmute_copy(&in_dweventid), core::mem::transmute_copy(&in_dweventsize), core::mem::transmute_copy(&in_pbeventdata)).into()
            }
        }
        unsafe extern "system" fn DeviceAdded<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pidevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowEvents_Impl::DeviceAdded(this, core::mem::transmute_copy(&in_pidevice)).into()
            }
        }
        unsafe extern "system" fn DeviceRemoved<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pidevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowEvents_Impl::DeviceRemoved(this, core::mem::transmute_copy(&in_pidevice)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ContentMissing: ContentMissing::<Identity, OFFSET>,
            ApplicationEvent: ApplicationEvent::<Identity, OFFSET>,
            DeviceAdded: DeviceAdded::<Identity, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISideShowEvents {}
windows_core::imp::define_interface!(ISideShowKeyCollection, ISideShowKeyCollection_Vtbl, 0x045473bc_a37b_4957_b144_68105411ed8e);
windows_core::imp::interface_hierarchy!(ISideShowKeyCollection, windows_core::IUnknown);
impl ISideShowKeyCollection {
    pub unsafe fn Add(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), key).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *mut super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, pkey as _).ok() }
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems).ok() }
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowKeyCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ISideShowKeyCollection_Impl: windows_core::IUnknownImpl {
    fn Add(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pkey: *mut super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
impl ISideShowKeyCollection_Vtbl {
    pub const fn new<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowKeyCollection_Impl::Add(this, core::mem::transmute_copy(&key)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowKeyCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pkey: *mut super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowKeyCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowKeyCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowKeyCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowKeyCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISideShowKeyCollection {}
windows_core::imp::define_interface!(ISideShowNotification, ISideShowNotification_Vtbl, 0x03c93300_8ab2_41c5_9b79_46127a30e148);
windows_core::imp::interface_hierarchy!(ISideShowNotification, windows_core::IUnknown);
impl ISideShowNotification {
    pub unsafe fn NotificationId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotificationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationId(&self, in_notificationid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationId)(windows_core::Interface::as_raw(self), in_notificationid).ok() }
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTitle<P0>(&self, in_pwsztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), in_pwsztitle.param().abi()).ok() }
    }
    pub unsafe fn Message(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMessage<P0>(&self, in_pwszmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMessage)(windows_core::Interface::as_raw(self), in_pwszmessage.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn Image(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Image)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetImage(&self, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetImage)(windows_core::Interface::as_raw(self), in_hicon).ok() }
    }
    pub unsafe fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpirationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExpirationTime(&self, in_ptime: Option<*const super::super::Foundation::SYSTEMTIME>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExpirationTime)(windows_core::Interface::as_raw(self), in_ptime.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotificationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNotificationId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    Image: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetImage: usize,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ISideShowNotification_Impl: windows_core::IUnknownImpl {
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
impl ISideShowNotification_Vtbl {
    pub const fn new<Identity: ISideShowNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotificationId<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pnotificationid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowNotification_Impl::NotificationId(this) {
                    Ok(ok__) => {
                        out_pnotificationid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationId<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_notificationid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotification_Impl::SetNotificationId(this, core::mem::transmute_copy(&in_notificationid)).into()
            }
        }
        unsafe extern "system" fn Title<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppwsztitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowNotification_Impl::Title(this) {
                    Ok(ok__) => {
                        out_ppwsztitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTitle<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pwsztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotification_Impl::SetTitle(this, core::mem::transmute(&in_pwsztitle)).into()
            }
        }
        unsafe extern "system" fn Message<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppwszmessage: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowNotification_Impl::Message(this) {
                    Ok(ok__) => {
                        out_ppwszmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessage<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pwszmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotification_Impl::SetMessage(this, core::mem::transmute(&in_pwszmessage)).into()
            }
        }
        unsafe extern "system" fn Image<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowNotification_Impl::Image(this) {
                    Ok(ok__) => {
                        out_phicon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetImage<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotification_Impl::SetImage(this, core::mem::transmute_copy(&in_hicon)).into()
            }
        }
        unsafe extern "system" fn ExpirationTime<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowNotification_Impl::ExpirationTime(this) {
                    Ok(ok__) => {
                        out_ptime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExpirationTime<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotification_Impl::SetExpirationTime(this, core::mem::transmute_copy(&in_ptime)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotificationId: NotificationId::<Identity, OFFSET>,
            SetNotificationId: SetNotificationId::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
            SetMessage: SetMessage::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            SetImage: SetImage::<Identity, OFFSET>,
            ExpirationTime: ExpirationTime::<Identity, OFFSET>,
            SetExpirationTime: SetExpirationTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowNotification as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ISideShowNotification {}
windows_core::imp::define_interface!(ISideShowNotificationManager, ISideShowNotificationManager_Vtbl, 0x63cea909_f2b9_4302_b5e1_c68e6d9ab833);
windows_core::imp::interface_hierarchy!(ISideShowNotificationManager, windows_core::IUnknown);
impl ISideShowNotificationManager {
    pub unsafe fn Show<P0>(&self, in_pinotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), in_pinotification.param().abi()).ok() }
    }
    pub unsafe fn Revoke(&self, in_notificationid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revoke)(windows_core::Interface::as_raw(self), in_notificationid).ok() }
    }
    pub unsafe fn RevokeAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevokeAll)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotificationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Revoke: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RevokeAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISideShowNotificationManager_Impl: windows_core::IUnknownImpl {
    fn Show(&self, in_pinotification: windows_core::Ref<ISideShowNotification>) -> windows_core::Result<()>;
    fn Revoke(&self, in_notificationid: u32) -> windows_core::Result<()>;
    fn RevokeAll(&self) -> windows_core::Result<()>;
}
impl ISideShowNotificationManager_Vtbl {
    pub const fn new<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pinotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotificationManager_Impl::Show(this, core::mem::transmute_copy(&in_pinotification)).into()
            }
        }
        unsafe extern "system" fn Revoke<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_notificationid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotificationManager_Impl::Revoke(this, core::mem::transmute_copy(&in_notificationid)).into()
            }
        }
        unsafe extern "system" fn RevokeAll<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowNotificationManager_Impl::RevokeAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, OFFSET>,
            Revoke: Revoke::<Identity, OFFSET>,
            RevokeAll: RevokeAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowNotificationManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISideShowNotificationManager {}
windows_core::imp::define_interface!(ISideShowPropVariantCollection, ISideShowPropVariantCollection_Vtbl, 0x2ea7a549_7bff_4aae_bab0_22d43111de49);
windows_core::imp::interface_hierarchy!(ISideShowPropVariantCollection, windows_core::IUnknown);
impl ISideShowPropVariantCollection {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems).ok() }
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowPropVariantCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetAt: usize,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISideShowPropVariantCollection_Impl: windows_core::IUnknownImpl {
    fn Add(&self, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ISideShowPropVariantCollection_Vtbl {
    pub const fn new<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowPropVariantCollection_Impl::Add(this, core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowPropVariantCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowPropVariantCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowPropVariantCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISideShowPropVariantCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowPropVariantCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISideShowPropVariantCollection {}
windows_core::imp::define_interface!(ISideShowSession, ISideShowSession_Vtbl, 0xe22331ee_9e7d_4922_9fc2_ab7aa41ce491);
windows_core::imp::interface_hierarchy!(ISideShowSession, windows_core::IUnknown);
impl ISideShowSession {
    pub unsafe fn RegisterContent(&self, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID) -> windows_core::Result<ISideShowContentManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterContent)(windows_core::Interface::as_raw(self), in_applicationid, in_endpointid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterNotifications(&self, in_applicationid: *const windows_core::GUID) -> windows_core::Result<ISideShowNotificationManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterNotifications)(windows_core::Interface::as_raw(self), in_applicationid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterContent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISideShowSession_Impl: windows_core::IUnknownImpl {
    fn RegisterContent(&self, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID) -> windows_core::Result<ISideShowContentManager>;
    fn RegisterNotifications(&self, in_applicationid: *const windows_core::GUID) -> windows_core::Result<ISideShowNotificationManager>;
}
impl ISideShowSession_Vtbl {
    pub const fn new<Identity: ISideShowSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterContent<Identity: ISideShowSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID, out_ppicontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowSession_Impl::RegisterContent(this, core::mem::transmute_copy(&in_applicationid), core::mem::transmute_copy(&in_endpointid)) {
                    Ok(ok__) => {
                        out_ppicontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterNotifications<Identity: ISideShowSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_applicationid: *const windows_core::GUID, out_ppinotification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISideShowSession_Impl::RegisterNotifications(this, core::mem::transmute_copy(&in_applicationid)) {
                    Ok(ok__) => {
                        out_ppinotification.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterContent: RegisterContent::<Identity, OFFSET>,
            RegisterNotifications: RegisterNotifications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISideShowSession {}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NEW_EVENT_DATA_AVAILABLE {
    pub cbNewEventDataAvailable: u32,
    pub dwVersion: u32,
}
pub const SCF_BUTTON_BACK: SCF_BUTTON_IDS = SCF_BUTTON_IDS(65280i32);
pub const SCF_BUTTON_DOWN: SCF_BUTTON_IDS = SCF_BUTTON_IDS(4i32);
pub const SCF_BUTTON_FASTFORWARD: SCF_BUTTON_IDS = SCF_BUTTON_IDS(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCF_BUTTON_IDS(pub i32);
pub const SCF_BUTTON_LEFT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(5i32);
pub const SCF_BUTTON_MENU: SCF_BUTTON_IDS = SCF_BUTTON_IDS(1i32);
pub const SCF_BUTTON_PAUSE: SCF_BUTTON_IDS = SCF_BUTTON_IDS(8i32);
pub const SCF_BUTTON_PLAY: SCF_BUTTON_IDS = SCF_BUTTON_IDS(7i32);
pub const SCF_BUTTON_REWIND: SCF_BUTTON_IDS = SCF_BUTTON_IDS(10i32);
pub const SCF_BUTTON_RIGHT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(6i32);
pub const SCF_BUTTON_SELECT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(2i32);
pub const SCF_BUTTON_STOP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(11i32);
pub const SCF_BUTTON_UP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCF_CONTEXTMENU_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub PreviousItemId: u32,
    pub MenuPage: u32,
    pub MenuItemId: u32,
}
pub const SCF_EVENT_CONTEXTMENU: SCF_EVENT_IDS = SCF_EVENT_IDS(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCF_EVENT_HEADER {
    pub PreviousPage: u32,
    pub TargetPage: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCF_EVENT_IDS(pub i32);
pub const SCF_EVENT_MENUACTION: SCF_EVENT_IDS = SCF_EVENT_IDS(2i32);
pub const SCF_EVENT_NAVIGATION: SCF_EVENT_IDS = SCF_EVENT_IDS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCF_MENUACTION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
    pub ItemId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCF_NAVIGATION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
}
pub const SIDESHOW_APPLICATION_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x4cb572fa_1d3b_49b3_a17a_2e6bff052854);
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 16 };
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 15 };
pub const SIDESHOW_CAPABILITY_COLOR_DEPTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 5 };
pub const SIDESHOW_CAPABILITY_COLOR_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 6 };
pub const SIDESHOW_CAPABILITY_CURRENT_LANGUAGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 9 };
pub const SIDESHOW_CAPABILITY_DATA_CACHE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 7 };
pub const SIDESHOW_CAPABILITY_DEVICE_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 1 };
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99);
pub const SIDESHOW_CAPABILITY_SCREEN_HEIGHT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 4 };
pub const SIDESHOW_CAPABILITY_SCREEN_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 2 };
pub const SIDESHOW_CAPABILITY_SCREEN_WIDTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 3 };
pub const SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 14 };
pub const SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 8 };
pub const SIDESHOW_CAPABILITY_SUPPORTED_THEMES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 10 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIDESHOW_COLOR_TYPE(pub i32);
pub const SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(2i32);
pub const SIDESHOW_COLOR_TYPE_COLOR: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(0i32);
pub const SIDESHOW_COLOR_TYPE_GREYSCALE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(1i32);
pub const SIDESHOW_CONTENT_MISSING_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x5007fba8_d313_439f_bea2_a50201d3e9a8);
pub const SIDESHOW_ENDPOINT_ICAL: windows_core::GUID = windows_core::GUID::from_u128(0x4dff36b5_9dde_4f76_9a2a_96435047063d);
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: windows_core::GUID = windows_core::GUID::from_u128(0xa9a5353f_2d4b_47ce_93ee_759f3a7dda4f);
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: windows_core::GUID = windows_core::GUID::from_u128(0x57813854_2fc1_411c_a59f_f24927608804);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIDESHOW_SCREEN_TYPE(pub i32);
pub const SIDESHOW_SCREEN_TYPE_BITMAP: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(0i32);
pub const SIDESHOW_SCREEN_TYPE_TEXT: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(1i32);
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x5009673c_3f7d_4c7e_9971_eaa2e91f1575);
pub const SideShowKeyCollection: windows_core::GUID = windows_core::GUID::from_u128(0xdfbbdbf8_18de_49b8_83dc_ebc727c62d94);
pub const SideShowNotification: windows_core::GUID = windows_core::GUID::from_u128(0x0ce3e86f_d5cd_4525_a766_1abab1a752f5);
pub const SideShowPropVariantCollection: windows_core::GUID = windows_core::GUID::from_u128(0xe640f415_539e_4923_96cd_5f093bc250cd);
pub const SideShowSession: windows_core::GUID = windows_core::GUID::from_u128(0xe20543b9_f785_4ea2_981e_c4ffa76bbc7c);
pub const VERSION_1_WINDOWS_7: u32 = 0u32;
