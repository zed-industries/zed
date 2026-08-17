#[inline]
pub unsafe fn CloseIMsgSession(lpmsgsess: LPMSGSESS) {
    windows_core::link!("mapi32.dll" "system" fn CloseIMsgSession(lpmsgsess : LPMSGSESS));
    unsafe { CloseIMsgSession(lpmsgsess) }
}
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn GetAttribIMsgOnIStg(lpobject: *mut core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> windows_core::Result<()> {
    windows_core::link!("mapi32.dll" "system" fn GetAttribIMsgOnIStg(lpobject : *mut core::ffi::c_void, lpproptagarray : *mut super::super::System::AddressBook:: SPropTagArray, lpppropattrarray : *mut *mut SPropAttrArray) -> windows_core::HRESULT);
    unsafe { GetAttribIMsgOnIStg(lpobject as _, lpproptagarray as _, lpppropattrarray as _).ok() }
}
#[inline]
pub unsafe fn MapStorageSCode(stgscode: i32) -> i32 {
    windows_core::link!("mapi32.dll" "system" fn MapStorageSCode(stgscode : i32) -> i32);
    unsafe { MapStorageSCode(stgscode) }
}
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OpenIMsgOnIStg<P4, P6>(lpmsgsess: LPMSGSESS, lpallocatebuffer: super::super::System::AddressBook::LPALLOCATEBUFFER, lpallocatemore: super::super::System::AddressBook::LPALLOCATEMORE, lpfreebuffer: super::super::System::AddressBook::LPFREEBUFFER, lpmalloc: P4, lpmapisup: *mut core::ffi::c_void, lpstg: P6, lpfmsgcallrelease: *mut MSGCALLRELEASE, ulcallerdata: u32, ulflags: u32, lppmsg: *mut Option<super::super::System::AddressBook::IMessage>) -> i32
where
    P4: windows_core::Param<super::super::System::Com::IMalloc>,
    P6: windows_core::Param<super::super::System::Com::StructuredStorage::IStorage>,
{
    windows_core::link!("mapi32.dll" "system" fn OpenIMsgOnIStg(lpmsgsess : LPMSGSESS, lpallocatebuffer : super::super::System::AddressBook:: LPALLOCATEBUFFER, lpallocatemore : super::super::System::AddressBook:: LPALLOCATEMORE, lpfreebuffer : super::super::System::AddressBook:: LPFREEBUFFER, lpmalloc : * mut core::ffi::c_void, lpmapisup : *mut core::ffi::c_void, lpstg : * mut core::ffi::c_void, lpfmsgcallrelease : *mut MSGCALLRELEASE, ulcallerdata : u32, ulflags : u32, lppmsg : *mut * mut core::ffi::c_void) -> i32);
    unsafe { OpenIMsgOnIStg(lpmsgsess, lpallocatebuffer, lpallocatemore, lpfreebuffer, lpmalloc.param().abi(), lpmapisup as _, lpstg.param().abi(), lpfmsgcallrelease as _, ulcallerdata, ulflags, core::mem::transmute(lppmsg)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenIMsgSession<P0>(lpmalloc: P0, ulflags: u32, lppmsgsess: *mut LPMSGSESS) -> i32
where
    P0: windows_core::Param<super::super::System::Com::IMalloc>,
{
    windows_core::link!("mapi32.dll" "system" fn OpenIMsgSession(lpmalloc : * mut core::ffi::c_void, ulflags : u32, lppmsgsess : *mut LPMSGSESS) -> i32);
    unsafe { OpenIMsgSession(lpmalloc.param().abi(), ulflags, lppmsgsess as _) }
}
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn SetAttribIMsgOnIStg(lpobject: *mut core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> windows_core::Result<()> {
    windows_core::link!("mapi32.dll" "system" fn SetAttribIMsgOnIStg(lpobject : *mut core::ffi::c_void, lpproptags : *mut super::super::System::AddressBook:: SPropTagArray, lppropattrs : *mut SPropAttrArray, lpppropproblems : *mut *mut super::super::System::AddressBook:: SPropProblemArray) -> windows_core::HRESULT);
    unsafe { SetAttribIMsgOnIStg(lpobject as _, lpproptags as _, lppropattrs as _, lpppropproblems as _).ok() }
}
pub const BlockRange: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca27_2204_11dd_966a_001aa01bbc58);
pub const BlockRangeList: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca28_2204_11dd_966a_001aa01bbc58);
pub const BootOptions: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fce_975b_59be_a960_9a2a262853a5);
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: windows_core::GUID = windows_core::GUID::from_u128(0xbd0b4366_8e03_11d2_94f6_00c04f79f1d6);
pub const CATID_SMTP_DSN: windows_core::GUID = windows_core::GUID::from_u128(0x22b55731_f5f8_4d23_bd8f_87b52371a73a);
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: windows_core::GUID = windows_core::GUID::from_u128(0x84ff368a_fab3_43d7_bcdf_692c5b46e6b1);
pub const CATID_SMTP_LOG: windows_core::GUID = windows_core::GUID::from_u128(0x93d0a538_2c1e_4b68_a7c9_d73a8aa6ee97);
pub const CATID_SMTP_MAXMSGSIZE: windows_core::GUID = windows_core::GUID::from_u128(0xebf159de_a67e_11d2_94f7_00c04f79f1d6);
pub const CATID_SMTP_MSGTRACKLOG: windows_core::GUID = windows_core::GUID::from_u128(0xc6df52aa_7db0_11d2_94f4_00c04f79f1d6);
pub const CATID_SMTP_ON_BEFORE_DATA: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c92_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_INBOUND_COMMAND: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c8d_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_MESSAGE_START: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c90_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_PER_RECIPIENT: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c91_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SERVER_RESPONSE: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c8e_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_END: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c93_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_START: windows_core::GUID = windows_core::GUID::from_u128(0xf6628c8f_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_STORE_DRIVER: windows_core::GUID = windows_core::GUID::from_u128(0x59175850_e533_11d1_aa67_00c04fa345f6);
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: windows_core::GUID = windows_core::GUID::from_u128(0x960252a3_0a3a_11d2_9e00_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: windows_core::GUID = windows_core::GUID::from_u128(0x76719654_05a6_11d2_9dfd_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: windows_core::GUID = windows_core::GUID::from_u128(0xa3acfb0d_83ff_11d2_9e14_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_ROUTER: windows_core::GUID = windows_core::GUID::from_u128(0x283430c9_1850_11d2_9e03_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_SUBMISSION: windows_core::GUID = windows_core::GUID::from_u128(0xff3caa23_00b9_11d2_9dfb_00c04fa322ba);
pub const CLSID_SmtpCat: windows_core::GUID = windows_core::GUID::from_u128(0xb23c35b7_9219_11d2_9e17_00c04fa322ba);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2DataEvents, DDiscFormat2DataEvents_Vtbl, 0x2735413c_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2DataEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2DataEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2DataEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2DataEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2DataEvents_Impl: super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, progress: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2DataEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2DataEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2DataEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2EraseEvents, DDiscFormat2EraseEvents_Vtbl, 0x2735413a_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2EraseEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2EraseEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2EraseEvents {
    pub unsafe fn Update<P0>(&self, object: P0, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), elapsedseconds, estimatedtotalseconds).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2EraseEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2EraseEvents_Impl: super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2EraseEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2EraseEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&elapsedseconds), core::mem::transmute_copy(&estimatedtotalseconds)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2EraseEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2RawCDEvents, DDiscFormat2RawCDEvents_Vtbl, 0x27354142_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2RawCDEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2RawCDEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2RawCDEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2RawCDEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2RawCDEvents_Impl: super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, progress: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2RawCDEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2RawCDEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2RawCDEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscFormat2TrackAtOnceEvents, DDiscFormat2TrackAtOnceEvents_Vtbl, 0x2735413f_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscFormat2TrackAtOnceEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscFormat2TrackAtOnceEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2TrackAtOnceEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, progress: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2TrackAtOnceEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2TrackAtOnceEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DDiscMaster2Events, DDiscMaster2Events_Vtbl, 0x27354131_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DDiscMaster2Events {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DDiscMaster2Events, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DDiscMaster2Events {
    pub unsafe fn NotifyDeviceAdded<P0>(&self, object: P0, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyDeviceAdded)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(uniqueid)).ok() }
    }
    pub unsafe fn NotifyDeviceRemoved<P0>(&self, object: P0, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyDeviceRemoved)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(uniqueid)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscMaster2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub NotifyDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscMaster2Events_Impl: super::super::System::Com::IDispatch_Impl {
    fn NotifyDeviceAdded(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NotifyDeviceRemoved(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscMaster2Events_Vtbl {
    pub const fn new<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyDeviceAdded<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, uniqueid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscMaster2Events_Impl::NotifyDeviceAdded(this, core::mem::transmute_copy(&object), core::mem::transmute(&uniqueid)).into()
            }
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, uniqueid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscMaster2Events_Impl::NotifyDeviceRemoved(this, core::mem::transmute_copy(&object), core::mem::transmute(&uniqueid)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Identity, OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscMaster2Events as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscMaster2Events {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DFileSystemImageEvents, DFileSystemImageEvents_Vtbl, 0x2c941fdf_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DFileSystemImageEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DFileSystemImageEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageEvents {
    pub unsafe fn Update<P0>(&self, object: P0, currentfile: &windows_core::BSTR, copiedsectors: i32, totalsectors: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(currentfile), copiedsectors, totalsectors).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageEvents_Impl: super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, currentfile: &windows_core::BSTR, copiedsectors: i32, totalsectors: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DFileSystemImageEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, currentfile: *mut core::ffi::c_void, copiedsectors: i32, totalsectors: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DFileSystemImageEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute(&currentfile), core::mem::transmute_copy(&copiedsectors), core::mem::transmute_copy(&totalsectors)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DFileSystemImageEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DFileSystemImageImportEvents, DFileSystemImageImportEvents_Vtbl, 0xd25c30f9_4087_4366_9e24_e55be286424b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DFileSystemImageImportEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DFileSystemImageImportEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageImportEvents {
    pub unsafe fn UpdateImport<P0>(&self, object: P0, filesystem: FsiFileSystems, currentitem: &windows_core::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateImport)(windows_core::Interface::as_raw(self), object.param().abi(), filesystem, core::mem::transmute_copy(currentitem), importeddirectoryitems, totaldirectoryitems, importedfileitems, totalfileitems).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageImportEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UpdateImport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsiFileSystems, *mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageImportEvents_Impl: super::super::System::Com::IDispatch_Impl {
    fn UpdateImport(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, filesystem: FsiFileSystems, currentitem: &windows_core::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DFileSystemImageImportEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateImport<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, filesystem: FsiFileSystems, currentitem: *mut core::ffi::c_void, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DFileSystemImageImportEvents_Impl::UpdateImport(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&filesystem), core::mem::transmute(&currentitem), core::mem::transmute_copy(&importeddirectoryitems), core::mem::transmute_copy(&totaldirectoryitems), core::mem::transmute_copy(&importedfileitems), core::mem::transmute_copy(&totalfileitems)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), UpdateImport: UpdateImport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DFileSystemImageImportEvents {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISC_RECORDER_STATE_FLAGS(pub u32);
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DWriteEngine2Events, DWriteEngine2Events_Vtbl, 0x27354137_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DWriteEngine2Events {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DWriteEngine2Events, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DWriteEngine2Events {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DWriteEngine2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DWriteEngine2Events_Impl: super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::super::System::Com::IDispatch>, progress: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DWriteEngine2Events_Vtbl {
    pub const fn new<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DWriteEngine2Events_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DWriteEngine2Events as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DWriteEngine2Events {}
pub const Emulation12MFloppy: EmulationType = EmulationType(1i32);
pub const Emulation144MFloppy: EmulationType = EmulationType(2i32);
pub const Emulation288MFloppy: EmulationType = EmulationType(3i32);
pub const EmulationHardDisk: EmulationType = EmulationType(4i32);
pub const EmulationNone: EmulationType = EmulationType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmulationType(pub i32);
pub const EnumFsiItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc6_975b_59be_a960_9a2a262853a5);
pub const EnumProgressItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fca_975b_59be_a960_9a2a262853a5);
pub const FileSystemImageResult: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcc_975b_59be_a960_9a2a262853a5);
pub const FsiDirectoryItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc8_975b_59be_a960_9a2a262853a5);
pub const FsiFileItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc7_975b_59be_a960_9a2a262853a5);
pub const FsiFileSystemISO9660: FsiFileSystems = FsiFileSystems(1i32);
pub const FsiFileSystemJoliet: FsiFileSystems = FsiFileSystems(2i32);
pub const FsiFileSystemNone: FsiFileSystems = FsiFileSystems(0i32);
pub const FsiFileSystemUDF: FsiFileSystems = FsiFileSystems(4i32);
pub const FsiFileSystemUnknown: FsiFileSystems = FsiFileSystems(1073741824i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsiFileSystems(pub i32);
pub const FsiItemDirectory: FsiItemType = FsiItemType(1i32);
pub const FsiItemFile: FsiItemType = FsiItemType(2i32);
pub const FsiItemNotFound: FsiItemType = FsiItemType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsiItemType(pub i32);
pub const FsiNamedStreams: windows_core::GUID = windows_core::GUID::from_u128(0xc6b6f8ed_6d19_44b4_b539_b159b793a32d);
pub const FsiStream: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcd_975b_59be_a960_9a2a262853a5);
pub const GUID_SMTPSVC_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x1b3c0666_e470_11d1_aa67_00c04fa345f6);
pub const GUID_SMTP_SOURCE_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xfb65c4dc_e468_11d1_aa67_00c04fa345f6);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBlockRange, IBlockRange_Vtbl, 0xb507ca25_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBlockRange {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBlockRange, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRange {
    pub unsafe fn StartLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRange_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EndLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRange_Impl: super::super::System::Com::IDispatch_Impl {
    fn StartLba(&self) -> windows_core::Result<i32>;
    fn EndLba(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBlockRange_Vtbl {
    pub const fn new<Identity: IBlockRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRange_Impl::StartLba(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRange_Impl::EndLba(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartLba: StartLba::<Identity, OFFSET>,
            EndLba: EndLba::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRange as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBlockRange {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBlockRangeList, IBlockRangeList_Vtbl, 0xb507ca26_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBlockRangeList {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBlockRangeList, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRangeList {
    pub unsafe fn BlockRanges(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockRanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRangeList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub BlockRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRangeList_Impl: super::super::System::Com::IDispatch_Impl {
    fn BlockRanges(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBlockRangeList_Vtbl {
    pub const fn new<Identity: IBlockRangeList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BlockRanges<Identity: IBlockRangeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRangeList_Impl::BlockRanges(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), BlockRanges: BlockRanges::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRangeList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBlockRangeList {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBootOptions, IBootOptions_Vtbl, 0x2c941fd4_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBootOptions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBootOptions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBootOptions {
    pub unsafe fn BootImage(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BootImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Manufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Manufacturer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetManufacturer(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetManufacturer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn PlatformId(&self) -> windows_core::Result<PlatformId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlatformId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlatformId(&self, newval: PlatformId) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlatformId)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn Emulation(&self) -> windows_core::Result<EmulationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Emulation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEmulation(&self, newval: EmulationType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEmulation)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn ImageSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AssignBootImage<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AssignBootImage)(windows_core::Interface::as_raw(self), newval.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub BootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformId) -> windows_core::HRESULT,
    pub SetPlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformId) -> windows_core::HRESULT,
    pub Emulation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EmulationType) -> windows_core::HRESULT,
    pub SetEmulation: unsafe extern "system" fn(*mut core::ffi::c_void, EmulationType) -> windows_core::HRESULT,
    pub ImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AssignBootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBootOptions_Impl: super::super::System::Com::IDispatch_Impl {
    fn BootImage(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn Manufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetManufacturer(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlatformId(&self) -> windows_core::Result<PlatformId>;
    fn SetPlatformId(&self, newval: PlatformId) -> windows_core::Result<()>;
    fn Emulation(&self) -> windows_core::Result<EmulationType>;
    fn SetEmulation(&self, newval: EmulationType) -> windows_core::Result<()>;
    fn ImageSize(&self) -> windows_core::Result<u32>;
    fn AssignBootImage(&self, newval: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBootOptions_Vtbl {
    pub const fn new<Identity: IBootOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BootImage<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::BootImage(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Manufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::Manufacturer(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::SetManufacturer(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn PlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut PlatformId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::PlatformId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: PlatformId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::SetPlatformId(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Emulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut EmulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::Emulation(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEmulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: EmulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::SetEmulation(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ImageSize<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::ImageSize(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssignBootImage<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::AssignBootImage(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BootImage: BootImage::<Identity, OFFSET>,
            Manufacturer: Manufacturer::<Identity, OFFSET>,
            SetManufacturer: SetManufacturer::<Identity, OFFSET>,
            PlatformId: PlatformId::<Identity, OFFSET>,
            SetPlatformId: SetPlatformId::<Identity, OFFSET>,
            Emulation: Emulation::<Identity, OFFSET>,
            SetEmulation: SetEmulation::<Identity, OFFSET>,
            ImageSize: ImageSize::<Identity, OFFSET>,
            AssignBootImage: AssignBootImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBootOptions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBootOptions {}
windows_core::imp::define_interface!(IBurnVerification, IBurnVerification_Vtbl, 0xd2ffd834_958b_426d_8470_2a13879c6a91);
windows_core::imp::interface_hierarchy!(IBurnVerification, windows_core::IUnknown);
impl IBurnVerification {
    pub unsafe fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBurnVerificationLevel)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BurnVerificationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBurnVerification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
    pub BurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
}
pub trait IBurnVerification_Impl: windows_core::IUnknownImpl {
    fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::Result<()>;
    fn BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl IBurnVerification_Vtbl {
    pub const fn new<Identity: IBurnVerification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBurnVerification_Impl::SetBurnVerificationLevel(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBurnVerification_Impl::BurnVerificationLevel(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBurnVerificationLevel: SetBurnVerificationLevel::<Identity, OFFSET>,
            BurnVerificationLevel: BurnVerificationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBurnVerification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBurnVerification {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2, IDiscFormat2_Vtbl, 0x27354152_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2 {
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRecorderSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCurrentMediaSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MediaPhysicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaPhysicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MediaHeuristicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaHeuristicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedMediaTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedMediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IsRecorderSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsCurrentMediaSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MediaPhysicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MediaHeuristicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SupportedMediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2_Impl: super::super::System::Com::IDispatch_Impl {
    fn IsRecorderSupported(&self, recorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCurrentMediaSupported(&self, recorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaPhysicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaHeuristicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedMediaTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2_Vtbl {
    pub const fn new<Identity: IDiscFormat2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsRecorderSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::IsRecorderSupported(this, core::mem::transmute_copy(&recorder)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::IsCurrentMediaSupported(this, core::mem::transmute_copy(&recorder)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::MediaPhysicallyBlank(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::MediaHeuristicallyBlank(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedMediaTypes<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::SupportedMediaTypes(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsRecorderSupported: IsRecorderSupported::<Identity, OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Identity, OFFSET>,
            MediaPhysicallyBlank: MediaPhysicallyBlank::<Identity, OFFSET>,
            MediaHeuristicallyBlank: MediaHeuristicallyBlank::<Identity, OFFSET>,
            SupportedMediaTypes: SupportedMediaTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2Data, IDiscFormat2Data_Vtbl, 0x27354153_9f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2Data {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2Data, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Data {
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPostgapAlreadyInImage(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPostgapAlreadyInImage)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn PostgapAlreadyInImage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostgapAlreadyInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentMediaStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteProtectStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NextWritableAddress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetForceMediaToBeClosed(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetForceMediaToBeClosed)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn ForceMediaToBeClosed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForceMediaToBeClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisableConsumerDvdCompatibilityMode(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetForceOverwrite(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetForceOverwrite)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn ForceOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForceOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Write<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), data.param().abi()).ok() }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Data_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentMediaStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT,
    pub WriteProtectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Data_Impl: IDiscFormat2_Impl {
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPostgapAlreadyInImage(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PostgapAlreadyInImage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> windows_core::Result<i32>;
    fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn SetForceMediaToBeClosed(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ForceMediaToBeClosed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableConsumerDvdCompatibilityMode(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetForceOverwrite(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ForceOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Write(&self, data: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2Data_Vtbl {
    pub const fn new<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetPostgapAlreadyInImage(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::PostgapAlreadyInImage(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentMediaStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentMediaStatus(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteProtectStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::WriteProtectStatus(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::NextWritableAddress(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::StartAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::LastWrittenAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetForceMediaToBeClosed(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::ForceMediaToBeClosed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetDisableConsumerDvdCompatibilityMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::DisableConsumerDvdCompatibilityMode(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::SupportedWriteSpeedDescriptors(this) {
                    Ok(ok__) => {
                        supportedspeeddescriptors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetForceOverwrite(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::ForceOverwrite(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::MultisessionInterfaces(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::Write(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn CancelWrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::CancelWrite(this).into()
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            SetPostgapAlreadyInImage: SetPostgapAlreadyInImage::<Identity, OFFSET>,
            PostgapAlreadyInImage: PostgapAlreadyInImage::<Identity, OFFSET>,
            CurrentMediaStatus: CurrentMediaStatus::<Identity, OFFSET>,
            WriteProtectStatus: WriteProtectStatus::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            SetForceMediaToBeClosed: SetForceMediaToBeClosed::<Identity, OFFSET>,
            ForceMediaToBeClosed: ForceMediaToBeClosed::<Identity, OFFSET>,
            SetDisableConsumerDvdCompatibilityMode: SetDisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            DisableConsumerDvdCompatibilityMode: DisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
            SetForceOverwrite: SetForceOverwrite::<Identity, OFFSET>,
            ForceOverwrite: ForceOverwrite::<Identity, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Data as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2Data {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2DataEventArgs, IDiscFormat2DataEventArgs_Vtbl, 0x2735413d_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2DataEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2DataEventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2DataEventArgs {
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2DataEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2DataEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
    fn TotalTime(&self) -> windows_core::Result<i32>;
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2DataEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::RemainingTime(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::TotalTime(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
            TotalTime: TotalTime::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2DataEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2DataEventArgs {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2Erase, IDiscFormat2Erase_Vtbl, 0x27354156_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2Erase {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2Erase, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Erase {
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFullErase(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFullErase)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn FullErase(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullErase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EraseMedia(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EraseMedia)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Erase_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFullErase: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FullErase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EraseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Erase_Impl: IDiscFormat2_Impl {
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetFullErase(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FullErase(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EraseMedia(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2Erase_Vtbl {
    pub const fn new<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::SetFullErase(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn FullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::FullErase(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EraseMedia<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::EraseMedia(this).into()
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetFullErase: SetFullErase::<Identity, OFFSET>,
            FullErase: FullErase::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            EraseMedia: EraseMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Erase as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2Erase {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2RawCD, IDiscFormat2RawCD_Vtbl, 0x27354155_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2RawCD {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCD, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCD {
    pub unsafe fn PrepareMedia(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WriteMedia<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteMedia)(windows_core::Interface::as_raw(self), data.param().abi()).ok() }
    }
    pub unsafe fn WriteMedia2<P0>(&self, data: P0, streamleadinsectors: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteMedia2)(windows_core::Interface::as_raw(self), data.param().abi(), streamleadinsectors).ok() }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav).ok() }
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartOfNextSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOfNextSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastPossibleStartOfLeadout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastPossibleStartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedSectorTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedSectorTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRequestedSectorType)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedSectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCD_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteMedia2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub StartOfNextSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastPossibleStartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SupportedSectorTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetRequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub RequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCD_Impl: IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn WriteMedia(&self, data: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn WriteMedia2(&self, data: windows_core::Ref<super::super::System::Com::IStream>, streamleadinsectors: i32) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartOfNextSession(&self) -> windows_core::Result<i32>;
    fn LastPossibleStartOfLeadout(&self) -> windows_core::Result<i32>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SupportedSectorTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2RawCD_Vtbl {
    pub const fn new<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::PrepareMedia(this).into()
            }
        }
        unsafe extern "system" fn WriteMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::WriteMedia(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn WriteMedia2<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, streamleadinsectors: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::WriteMedia2(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&streamleadinsectors)).into()
            }
        }
        unsafe extern "system" fn CancelWrite<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::CancelWrite(this).into()
            }
        }
        unsafe extern "system" fn ReleaseMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::ReleaseMedia(this).into()
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartOfNextSession<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::StartOfNextSession(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::LastPossibleStartOfLeadout(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedSectorTypes<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::SupportedSectorTypes(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetRequestedSectorType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::RequestedSectorType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::SupportedWriteSpeedDescriptors(this) {
                    Ok(ok__) => {
                        supportedspeeddescriptors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, OFFSET>,
            WriteMedia: WriteMedia::<Identity, OFFSET>,
            WriteMedia2: WriteMedia2::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            StartOfNextSession: StartOfNextSession::<Identity, OFFSET>,
            LastPossibleStartOfLeadout: LastPossibleStartOfLeadout::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SupportedSectorTypes: SupportedSectorTypes::<Identity, OFFSET>,
            SetRequestedSectorType: SetRequestedSectorType::<Identity, OFFSET>,
            RequestedSectorType: RequestedSectorType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCD as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2RawCD {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2RawCDEventArgs, IDiscFormat2RawCDEventArgs_Vtbl, 0x27354143_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2RawCDEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCDEventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCDEventArgs {
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCDEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2RawCDEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::RemainingTime(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCDEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2RawCDEventArgs {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2TrackAtOnce, IDiscFormat2TrackAtOnce_Vtbl, 0x27354154_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2TrackAtOnce {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnce, windows_core::IUnknown, super::super::System::Com::IDispatch, IDiscFormat2);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnce {
    pub unsafe fn PrepareMedia(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddAudioTrack<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAudioTrack)(windows_core::Interface::as_raw(self), data.param().abi()).ok() }
    }
    pub unsafe fn CancelAddTrack(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelAddTrack)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav).ok() }
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UsedSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsedSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDoNotFinalizeMedia(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDoNotFinalizeMedia)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn DoNotFinalizeMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoNotFinalizeMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnce_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAudioTrack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAddTrack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UsedSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnce_Impl: IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn AddAudioTrack(&self, data: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CancelAddTrack(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn NumberOfExistingTracks(&self) -> windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn UsedSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn SetDoNotFinalizeMedia(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DoNotFinalizeMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2TrackAtOnce_Vtbl {
    pub const fn new<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::PrepareMedia(this).into()
            }
        }
        unsafe extern "system" fn AddAudioTrack<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::AddAudioTrack(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn CancelAddTrack<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::CancelAddTrack(this).into()
            }
        }
        unsafe extern "system" fn ReleaseMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::ReleaseMedia(this).into()
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::NumberOfExistingTracks(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::UsedSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetDoNotFinalizeMedia(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::DoNotFinalizeMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::ExpectedTableOfContents(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::SupportedWriteSpeedDescriptors(this) {
                    Ok(ok__) => {
                        supportedspeeddescriptors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, OFFSET>,
            AddAudioTrack: AddAudioTrack::<Identity, OFFSET>,
            CancelAddTrack: CancelAddTrack::<Identity, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
            UsedSectorsOnMedia: UsedSectorsOnMedia::<Identity, OFFSET>,
            SetDoNotFinalizeMedia: SetDoNotFinalizeMedia::<Identity, OFFSET>,
            DoNotFinalizeMedia: DoNotFinalizeMedia::<Identity, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnce as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2TrackAtOnce {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscFormat2TrackAtOnceEventArgs, IDiscFormat2TrackAtOnceEventArgs_Vtbl, 0x27354140_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscFormat2TrackAtOnceEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnceEventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnceEventArgs {
    pub unsafe fn CurrentTrackNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn CurrentTrackNumber(&self) -> windows_core::Result<i32>;
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentTrackNumber<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::CurrentTrackNumber(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::RemainingTime(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            CurrentTrackNumber: CurrentTrackNumber::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnceEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2TrackAtOnceEventArgs {}
windows_core::imp::define_interface!(IDiscMaster, IDiscMaster_Vtbl, 0x520cca62_51a5_11d3_9144_00104ba11c5e);
windows_core::imp::interface_hierarchy!(IDiscMaster, windows_core::IUnknown);
impl IDiscMaster {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumDiscMasterFormats(&self) -> windows_core::Result<IEnumDiscMasterFormats> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDiscMasterFormats)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActiveDiscMasterFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveDiscMasterFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetActiveDiscMasterFormat(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActiveDiscMasterFormat)(windows_core::Interface::as_raw(self), riid, ppunk as _).ok() }
    }
    pub unsafe fn EnumDiscRecorders(&self) -> windows_core::Result<IEnumDiscRecorders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDiscRecorders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActiveDiscRecorder(&self) -> windows_core::Result<IDiscRecorder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveDiscRecorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetActiveDiscRecorder<P0>(&self, precorder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActiveDiscRecorder)(windows_core::Interface::as_raw(self), precorder.param().abi()).ok() }
    }
    pub unsafe fn ClearFormatContent(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearFormatContent)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ProgressAdvise<P0>(&self, pevents: P0) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<IDiscMasterProgressEvents>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressAdvise)(windows_core::Interface::as_raw(self), pevents.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProgressUnadvise(&self, vcookie: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProgressUnadvise)(windows_core::Interface::as_raw(self), vcookie).ok() }
    }
    pub unsafe fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecordDisc)(windows_core::Interface::as_raw(self), bsimulate, bejectafterburn).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDiscMasterFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveDiscMasterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetActiveDiscMasterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDiscRecorders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearFormatContent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProgressAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub ProgressUnadvise: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub RecordDisc: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDiscMaster_Impl: windows_core::IUnknownImpl {
    fn Open(&self) -> windows_core::Result<()>;
    fn EnumDiscMasterFormats(&self) -> windows_core::Result<IEnumDiscMasterFormats>;
    fn GetActiveDiscMasterFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetActiveDiscMasterFormat(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumDiscRecorders(&self) -> windows_core::Result<IEnumDiscRecorders>;
    fn GetActiveDiscRecorder(&self) -> windows_core::Result<IDiscRecorder>;
    fn SetActiveDiscRecorder(&self, precorder: windows_core::Ref<IDiscRecorder>) -> windows_core::Result<()>;
    fn ClearFormatContent(&self) -> windows_core::Result<()>;
    fn ProgressAdvise(&self, pevents: windows_core::Ref<IDiscMasterProgressEvents>) -> windows_core::Result<usize>;
    fn ProgressUnadvise(&self, vcookie: usize) -> windows_core::Result<()>;
    fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IDiscMaster_Vtbl {
    pub const fn new<Identity: IDiscMaster_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMaster_Impl::Open(this).into()
            }
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster_Impl::EnumDiscMasterFormats(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster_Impl::GetActiveDiscMasterFormat(this) {
                    Ok(ok__) => {
                        lpiid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMaster_Impl::SetActiveDiscMasterFormat(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn EnumDiscRecorders<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster_Impl::EnumDiscRecorders(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprecorder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster_Impl::GetActiveDiscRecorder(this) {
                    Ok(ok__) => {
                        pprecorder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMaster_Impl::SetActiveDiscRecorder(this, core::mem::transmute_copy(&precorder)).into()
            }
        }
        unsafe extern "system" fn ClearFormatContent<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMaster_Impl::ClearFormatContent(this).into()
            }
        }
        unsafe extern "system" fn ProgressAdvise<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void, pvcookie: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster_Impl::ProgressAdvise(this, core::mem::transmute_copy(&pevents)) {
                    Ok(ok__) => {
                        pvcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressUnadvise<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcookie: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMaster_Impl::ProgressUnadvise(this, core::mem::transmute_copy(&vcookie)).into()
            }
        }
        unsafe extern "system" fn RecordDisc<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMaster_Impl::RecordDisc(this, core::mem::transmute_copy(&bsimulate), core::mem::transmute_copy(&bejectafterburn)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMaster_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            EnumDiscMasterFormats: EnumDiscMasterFormats::<Identity, OFFSET>,
            GetActiveDiscMasterFormat: GetActiveDiscMasterFormat::<Identity, OFFSET>,
            SetActiveDiscMasterFormat: SetActiveDiscMasterFormat::<Identity, OFFSET>,
            EnumDiscRecorders: EnumDiscRecorders::<Identity, OFFSET>,
            GetActiveDiscRecorder: GetActiveDiscRecorder::<Identity, OFFSET>,
            SetActiveDiscRecorder: SetActiveDiscRecorder::<Identity, OFFSET>,
            ClearFormatContent: ClearFormatContent::<Identity, OFFSET>,
            ProgressAdvise: ProgressAdvise::<Identity, OFFSET>,
            ProgressUnadvise: ProgressUnadvise::<Identity, OFFSET>,
            RecordDisc: RecordDisc::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMaster as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiscMaster {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscMaster2, IDiscMaster2_Vtbl, 0x27354130_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscMaster2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscMaster2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDiscMaster2 {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSupportedEnvironment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupportedEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsSupportedEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscMaster2_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn IsSupportedEnvironment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscMaster2_Vtbl {
    pub const fn new<Identity: IDiscMaster2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::Count(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSupportedEnvironment<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::IsSupportedEnvironment(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            IsSupportedEnvironment: IsSupportedEnvironment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMaster2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscMaster2 {}
windows_core::imp::define_interface!(IDiscMasterProgressEvents, IDiscMasterProgressEvents_Vtbl, 0xec9e51c1_4e5d_11d3_9144_00104ba11c5e);
windows_core::imp::interface_hierarchy!(IDiscMasterProgressEvents, windows_core::IUnknown);
impl IDiscMasterProgressEvents {
    pub unsafe fn QueryCancel(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryCancel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NotifyPnPActivity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyPnPActivity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyAddProgress)(windows_core::Interface::as_raw(self), ncompletedsteps, ntotalsteps).ok() }
    }
    pub unsafe fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyBlockProgress)(windows_core::Interface::as_raw(self), ncompleted, ntotal).ok() }
    }
    pub unsafe fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyTrackProgress)(windows_core::Interface::as_raw(self), ncurrenttrack, ntotaltracks).ok() }
    }
    pub unsafe fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyPreparingBurn)(windows_core::Interface::as_raw(self), nestimatedseconds).ok() }
    }
    pub unsafe fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyClosingDisc)(windows_core::Interface::as_raw(self), nestimatedseconds).ok() }
    }
    pub unsafe fn NotifyBurnComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyBurnComplete)(windows_core::Interface::as_raw(self), status).ok() }
    }
    pub unsafe fn NotifyEraseComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyEraseComplete)(windows_core::Interface::as_raw(self), status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMasterProgressEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub NotifyPnPActivity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyAddProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub NotifyBlockProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub NotifyTrackProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub NotifyPreparingBurn: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NotifyClosingDisc: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NotifyBurnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub NotifyEraseComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IDiscMasterProgressEvents_Impl: windows_core::IUnknownImpl {
    fn QueryCancel(&self) -> windows_core::Result<u8>;
    fn NotifyPnPActivity(&self) -> windows_core::Result<()>;
    fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> windows_core::Result<()>;
    fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> windows_core::Result<()>;
    fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> windows_core::Result<()>;
    fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> windows_core::Result<()>;
    fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> windows_core::Result<()>;
    fn NotifyBurnComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn NotifyEraseComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IDiscMasterProgressEvents_Vtbl {
    pub const fn new<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryCancel<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcancel: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMasterProgressEvents_Impl::QueryCancel(this) {
                    Ok(ok__) => {
                        pbcancel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyPnPActivity<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyPnPActivity(this).into()
            }
        }
        unsafe extern "system" fn NotifyAddProgress<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyAddProgress(this, core::mem::transmute_copy(&ncompletedsteps), core::mem::transmute_copy(&ntotalsteps)).into()
            }
        }
        unsafe extern "system" fn NotifyBlockProgress<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncompleted: i32, ntotal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyBlockProgress(this, core::mem::transmute_copy(&ncompleted), core::mem::transmute_copy(&ntotal)).into()
            }
        }
        unsafe extern "system" fn NotifyTrackProgress<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyTrackProgress(this, core::mem::transmute_copy(&ncurrenttrack), core::mem::transmute_copy(&ntotaltracks)).into()
            }
        }
        unsafe extern "system" fn NotifyPreparingBurn<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nestimatedseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyPreparingBurn(this, core::mem::transmute_copy(&nestimatedseconds)).into()
            }
        }
        unsafe extern "system" fn NotifyClosingDisc<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nestimatedseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyClosingDisc(this, core::mem::transmute_copy(&nestimatedseconds)).into()
            }
        }
        unsafe extern "system" fn NotifyBurnComplete<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyBurnComplete(this, core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn NotifyEraseComplete<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscMasterProgressEvents_Impl::NotifyEraseComplete(this, core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCancel: QueryCancel::<Identity, OFFSET>,
            NotifyPnPActivity: NotifyPnPActivity::<Identity, OFFSET>,
            NotifyAddProgress: NotifyAddProgress::<Identity, OFFSET>,
            NotifyBlockProgress: NotifyBlockProgress::<Identity, OFFSET>,
            NotifyTrackProgress: NotifyTrackProgress::<Identity, OFFSET>,
            NotifyPreparingBurn: NotifyPreparingBurn::<Identity, OFFSET>,
            NotifyClosingDisc: NotifyClosingDisc::<Identity, OFFSET>,
            NotifyBurnComplete: NotifyBurnComplete::<Identity, OFFSET>,
            NotifyEraseComplete: NotifyEraseComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMasterProgressEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiscMasterProgressEvents {}
windows_core::imp::define_interface!(IDiscRecorder, IDiscRecorder_Vtbl, 0x85ac9776_ca88_4cf2_894e_09598c078a41);
windows_core::imp::interface_hierarchy!(IDiscRecorder, windows_core::IUnknown);
impl IDiscRecorder {
    pub unsafe fn Init(&self, pbyuniqueid: &[u8], nuldrivenumber: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), core::mem::transmute(pbyuniqueid.as_ptr()), pbyuniqueid.len().try_into().unwrap(), nuldrivenumber).ok() }
    }
    pub unsafe fn GetRecorderGUID(&self, pbyuniqueid: Option<&mut [u8]>, pulreturnsizerequired: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRecorderGUID)(windows_core::Interface::as_raw(self), core::mem::transmute(pbyuniqueid.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbyuniqueid.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pulreturnsizerequired as _).ok() }
    }
    pub unsafe fn GetRecorderType(&self) -> windows_core::Result<RECORDER_TYPES> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecorderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDisplayNames(&self, pbstrvendorid: Option<*mut windows_core::BSTR>, pbstrproductid: Option<*mut windows_core::BSTR>, pbstrrevision: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayNames)(windows_core::Interface::as_raw(self), pbstrvendorid.unwrap_or(core::mem::zeroed()) as _, pbstrproductid.unwrap_or(core::mem::zeroed()) as _, pbstrrevision.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetBasePnPID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBasePnPID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetRecorderProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecorderProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetRecorderProperties<P0>(&self, ppropstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IPropertyStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorderProperties)(windows_core::Interface::as_raw(self), ppropstg.param().abi()).ok() }
    }
    pub unsafe fn GetRecorderState(&self) -> windows_core::Result<DISC_RECORDER_STATE_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecorderState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OpenExclusive(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenExclusive)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryMediaType)(windows_core::Interface::as_raw(self), fmediatype as _, fmediaflags as _).ok() }
    }
    pub unsafe fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryMediaInfo)(windows_core::Interface::as_raw(self), pbsessions as _, pblasttrack as _, ulstartaddress as _, ulnextwritable as _, ulfreeblocks as _).ok() }
    }
    pub unsafe fn Eject(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Eject)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Erase(&self, bfullerase: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Erase)(windows_core::Interface::as_raw(self), bfullerase).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32) -> windows_core::HRESULT,
    pub GetRecorderGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRecorderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RECORDER_TYPES) -> windows_core::HRESULT,
    pub GetDisplayNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBasePnPID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetRecorderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetRecorderProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetRecorderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetRecorderProperties: usize,
    pub GetRecorderState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DISC_RECORDER_STATE_FLAGS) -> windows_core::HRESULT,
    pub OpenExclusive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MEDIA_TYPES, *mut MEDIA_FLAGS) -> windows_core::HRESULT,
    pub QueryMediaInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u8, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Eject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Erase: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IDiscRecorder_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> windows_core::Result<()>;
    fn GetRecorderGUID(&self, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetRecorderType(&self) -> windows_core::Result<RECORDER_TYPES>;
    fn GetDisplayNames(&self, pbstrvendorid: *mut windows_core::BSTR, pbstrproductid: *mut windows_core::BSTR, pbstrrevision: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBasePnPID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRecorderProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetRecorderProperties(&self, ppropstg: windows_core::Ref<super::super::System::Com::StructuredStorage::IPropertyStorage>) -> windows_core::Result<()>;
    fn GetRecorderState(&self) -> windows_core::Result<DISC_RECORDER_STATE_FLAGS>;
    fn OpenExclusive(&self) -> windows_core::Result<()>;
    fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> windows_core::Result<()>;
    fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> windows_core::Result<()>;
    fn Eject(&self) -> windows_core::Result<()>;
    fn Erase(&self, bfullerase: u8) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IDiscRecorder_Vtbl {
    pub const fn new<Identity: IDiscRecorder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::Init(this, core::mem::transmute_copy(&pbyuniqueid), core::mem::transmute_copy(&nulidsize), core::mem::transmute_copy(&nuldrivenumber)).into()
            }
        }
        unsafe extern "system" fn GetRecorderGUID<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::GetRecorderGUID(this, core::mem::transmute_copy(&pbyuniqueid), core::mem::transmute_copy(&ulbuffersize), core::mem::transmute_copy(&pulreturnsizerequired)).into()
            }
        }
        unsafe extern "system" fn GetRecorderType<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder_Impl::GetRecorderType(this) {
                    Ok(ok__) => {
                        ftypecode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayNames<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvendorid: *mut *mut core::ffi::c_void, pbstrproductid: *mut *mut core::ffi::c_void, pbstrrevision: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::GetDisplayNames(this, core::mem::transmute_copy(&pbstrvendorid), core::mem::transmute_copy(&pbstrproductid), core::mem::transmute_copy(&pbstrrevision)).into()
            }
        }
        unsafe extern "system" fn GetBasePnPID<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbasepnpid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder_Impl::GetBasePnPID(this) {
                    Ok(ok__) => {
                        pbstrbasepnpid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPath<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder_Impl::GetPath(this) {
                    Ok(ok__) => {
                        pbstrpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRecorderProperties<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder_Impl::GetRecorderProperties(this) {
                    Ok(ok__) => {
                        pppropstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRecorderProperties<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::SetRecorderProperties(this, core::mem::transmute_copy(&ppropstg)).into()
            }
        }
        unsafe extern "system" fn GetRecorderState<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder_Impl::GetRecorderState(this) {
                    Ok(ok__) => {
                        puldevstateflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenExclusive<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::OpenExclusive(this).into()
            }
        }
        unsafe extern "system" fn QueryMediaType<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::QueryMediaType(this, core::mem::transmute_copy(&fmediatype), core::mem::transmute_copy(&fmediaflags)).into()
            }
        }
        unsafe extern "system" fn QueryMediaInfo<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::QueryMediaInfo(this, core::mem::transmute_copy(&pbsessions), core::mem::transmute_copy(&pblasttrack), core::mem::transmute_copy(&ulstartaddress), core::mem::transmute_copy(&ulnextwritable), core::mem::transmute_copy(&ulfreeblocks)).into()
            }
        }
        unsafe extern "system" fn Eject<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::Eject(this).into()
            }
        }
        unsafe extern "system" fn Erase<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullerase: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::Erase(this, core::mem::transmute_copy(&bfullerase)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetRecorderGUID: GetRecorderGUID::<Identity, OFFSET>,
            GetRecorderType: GetRecorderType::<Identity, OFFSET>,
            GetDisplayNames: GetDisplayNames::<Identity, OFFSET>,
            GetBasePnPID: GetBasePnPID::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetRecorderProperties: GetRecorderProperties::<Identity, OFFSET>,
            SetRecorderProperties: SetRecorderProperties::<Identity, OFFSET>,
            GetRecorderState: GetRecorderState::<Identity, OFFSET>,
            OpenExclusive: OpenExclusive::<Identity, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, OFFSET>,
            QueryMediaInfo: QueryMediaInfo::<Identity, OFFSET>,
            Eject: Eject::<Identity, OFFSET>,
            Erase: Erase::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IDiscRecorder {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDiscRecorder2, IDiscRecorder2_Vtbl, 0x27354133_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDiscRecorder2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDiscRecorder2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDiscRecorder2 {
    pub unsafe fn EjectMedia(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EjectMedia)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CloseTray(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseTray)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AcquireExclusiveAccess(&self, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcquireExclusiveAccess)(windows_core::Interface::as_raw(self), force, core::mem::transmute_copy(__midl__idiscrecorder20000)).ok() }
    }
    pub unsafe fn ReleaseExclusiveAccess(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseExclusiveAccess)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisableMcn(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableMcn)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnableMcn(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableMcn)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InitializeDiscRecorder(&self, recorderuniqueid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeDiscRecorder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(recorderuniqueid)).ok() }
    }
    pub unsafe fn ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActiveDiscRecorder)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VendorId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VendorId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductRevision(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumePathNames(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumePathNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceCanLoadMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceCanLoadMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LegacyDeviceNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LegacyDeviceNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedModePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedModePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExclusiveAccessOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EjectMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseTray: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcquireExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumePathNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub DeviceCanLoadMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LegacyDeviceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub CurrentFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub CurrentProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ExclusiveAccessOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscRecorder2_Impl: super::super::System::Com::IDispatch_Impl {
    fn EjectMedia(&self) -> windows_core::Result<()>;
    fn CloseTray(&self) -> windows_core::Result<()>;
    fn AcquireExclusiveAccess(&self, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReleaseExclusiveAccess(&self) -> windows_core::Result<()>;
    fn DisableMcn(&self) -> windows_core::Result<()>;
    fn EnableMcn(&self) -> windows_core::Result<()>;
    fn InitializeDiscRecorder(&self, recorderuniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VendorId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductRevision(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumePathNames(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DeviceCanLoadMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LegacyDeviceNumber(&self) -> windows_core::Result<i32>;
    fn SupportedFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedModePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscRecorder2_Vtbl {
    pub const fn new<Identity: IDiscRecorder2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EjectMedia<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::EjectMedia(this).into()
            }
        }
        unsafe extern "system" fn CloseTray<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::CloseTray(this).into()
            }
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::AcquireExclusiveAccess(this, core::mem::transmute_copy(&force), core::mem::transmute(&__midl__idiscrecorder20000)).into()
            }
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::ReleaseExclusiveAccess(this).into()
            }
        }
        unsafe extern "system" fn DisableMcn<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::DisableMcn(this).into()
            }
        }
        unsafe extern "system" fn EnableMcn<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::EnableMcn(this).into()
            }
        }
        unsafe extern "system" fn InitializeDiscRecorder<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorderuniqueid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::InitializeDiscRecorder(this, core::mem::transmute(&recorderuniqueid)).into()
            }
        }
        unsafe extern "system" fn ActiveDiscRecorder<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ActiveDiscRecorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VendorId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::VendorId(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ProductId(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductRevision<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ProductRevision(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::VolumeName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumePathNames<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::VolumePathNames(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::DeviceCanLoadMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LegacyDeviceNumber<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, legacydevicenumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::LegacyDeviceNumber(this) {
                    Ok(ok__) => {
                        legacydevicenumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::SupportedFeaturePages(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::CurrentFeaturePages(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::SupportedProfiles(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::CurrentProfiles(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedModePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::SupportedModePages(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ExclusiveAccessOwner(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EjectMedia: EjectMedia::<Identity, OFFSET>,
            CloseTray: CloseTray::<Identity, OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Identity, OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Identity, OFFSET>,
            DisableMcn: DisableMcn::<Identity, OFFSET>,
            EnableMcn: EnableMcn::<Identity, OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Identity, OFFSET>,
            ActiveDiscRecorder: ActiveDiscRecorder::<Identity, OFFSET>,
            VendorId: VendorId::<Identity, OFFSET>,
            ProductId: ProductId::<Identity, OFFSET>,
            ProductRevision: ProductRevision::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            VolumePathNames: VolumePathNames::<Identity, OFFSET>,
            DeviceCanLoadMedia: DeviceCanLoadMedia::<Identity, OFFSET>,
            LegacyDeviceNumber: LegacyDeviceNumber::<Identity, OFFSET>,
            SupportedFeaturePages: SupportedFeaturePages::<Identity, OFFSET>,
            CurrentFeaturePages: CurrentFeaturePages::<Identity, OFFSET>,
            SupportedProfiles: SupportedProfiles::<Identity, OFFSET>,
            CurrentProfiles: CurrentProfiles::<Identity, OFFSET>,
            SupportedModePages: SupportedModePages::<Identity, OFFSET>,
            ExclusiveAccessOwner: ExclusiveAccessOwner::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscRecorder2 {}
windows_core::imp::define_interface!(IDiscRecorder2Ex, IDiscRecorder2Ex_Vtbl, 0x27354132_7f64_5b0f_8f00_5d77afbe261e);
windows_core::imp::interface_hierarchy!(IDiscRecorder2Ex, windows_core::IUnknown);
impl IDiscRecorder2Ex {
    pub unsafe fn SendCommandNoData(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendCommandNoData)(windows_core::Interface::as_raw(self), core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), core::mem::transmute(sensebuffer.as_ptr()), timeout).ok() }
    }
    pub unsafe fn SendCommandSendDataToDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendCommandSendDataToDevice)(windows_core::Interface::as_raw(self), core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), core::mem::transmute(sensebuffer.as_ptr()), timeout, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SendCommandGetDataFromDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: &mut [u8], bufferfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendCommandGetDataFromDevice)(windows_core::Interface::as_raw(self), core::mem::transmute(cdb.as_ptr()), cdb.len().try_into().unwrap(), core::mem::transmute(sensebuffer.as_ptr()), timeout, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), bufferfetched as _).ok() }
    }
    pub unsafe fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadDvdStructure)(windows_core::Interface::as_raw(self), format, address, layer, agid, data as _, count as _).ok() }
    }
    pub unsafe fn SendDvdStructure(&self, format: u32, data: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendDvdStructure)(windows_core::Interface::as_raw(self), format, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDescriptor)(windows_core::Interface::as_raw(self), data as _, bytesize as _).ok() }
    }
    pub unsafe fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceDescriptor)(windows_core::Interface::as_raw(self), data as _, bytesize as _).ok() }
    }
    pub unsafe fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDiscInformation)(windows_core::Interface::as_raw(self), discinformation as _, bytesize as _).ok() }
    }
    pub unsafe fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTrackInformation)(windows_core::Interface::as_raw(self), address, addresstype, trackinformation as _, bytesize as _).ok() }
    }
    pub unsafe fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: bool, featuredata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFeaturePage)(windows_core::Interface::as_raw(self), requestedfeature, currentfeatureonly, featuredata as _, bytesize as _).ok() }
    }
    pub unsafe fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetModePage)(windows_core::Interface::as_raw(self), requestedmodepage, requesttype, modepagedata as _, bytesize as _).ok() }
    }
    pub unsafe fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetModePage)(windows_core::Interface::as_raw(self), requesttype, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetSupportedFeaturePages(&self, currentfeatureonly: bool, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedFeaturePages)(windows_core::Interface::as_raw(self), currentfeatureonly, featuredata as _, bytesize as _).ok() }
    }
    pub unsafe fn GetSupportedProfiles(&self, currentonly: bool, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedProfiles)(windows_core::Interface::as_raw(self), currentonly, profiletypes as _, validprofiles as _).ok() }
    }
    pub unsafe fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedModePages)(windows_core::Interface::as_raw(self), requesttype, modepagetypes as _, validpages as _).ok() }
    }
    pub unsafe fn GetByteAlignmentMask(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByteAlignmentMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumNonPageAlignedTransferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumPageAlignedTransferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2Ex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendCommandNoData: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32) -> windows_core::HRESULT,
    pub SendCommandSendDataToDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32, *const u8, u32) -> windows_core::HRESULT,
    pub SendCommandGetDataFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub ReadDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SendDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32) -> windows_core::HRESULT,
    pub GetAdapterDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetDiscInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetTrackInformation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, IMAPI_READ_TRACK_ADDRESS_TYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetFeaturePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FEATURE_PAGE_TYPE, bool, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_TYPE, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub GetSupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut IMAPI_FEATURE_PAGE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut IMAPI_PROFILE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut IMAPI_MODE_PAGE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetByteAlignmentMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaximumNonPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaximumPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDiscRecorder2Ex_Impl: windows_core::IUnknownImpl {
    fn SendCommandNoData(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> windows_core::Result<()>;
    fn SendCommandSendDataToDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> windows_core::Result<()>;
    fn SendCommandGetDataFromDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> windows_core::Result<()>;
    fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn SendDvdStructure(&self, format: u32, data: *const u8, count: u32) -> windows_core::Result<()>;
    fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: bool, featuredata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> windows_core::Result<()>;
    fn GetSupportedFeaturePages(&self, currentfeatureonly: bool, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetSupportedProfiles(&self, currentonly: bool, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> windows_core::Result<()>;
    fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> windows_core::Result<()>;
    fn GetByteAlignmentMask(&self) -> windows_core::Result<u32>;
    fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<u32>;
    fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<u32>;
}
impl IDiscRecorder2Ex_Vtbl {
    pub const fn new<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendCommandNoData<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandNoData(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandSendDataToDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize)).into()
            }
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandGetDataFromDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&bufferfetched)).into()
            }
        }
        unsafe extern "system" fn ReadDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::ReadDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&address), core::mem::transmute_copy(&layer), core::mem::transmute_copy(&agid), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SendDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: u32, data: *const u8, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetAdapterDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetDeviceDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetDeviceDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetDiscInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetDiscInformation(this, core::mem::transmute_copy(&discinformation), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetTrackInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetTrackInformation(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&addresstype), core::mem::transmute_copy(&trackinformation), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetFeaturePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: bool, featuredata: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetFeaturePage(this, core::mem::transmute_copy(&requestedfeature), core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetModePage(this, core::mem::transmute_copy(&requestedmodepage), core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&modepagedata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn SetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SetModePage(this, core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentfeatureonly: bool, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetSupportedFeaturePages(this, core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetSupportedProfiles<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentonly: bool, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetSupportedProfiles(this, core::mem::transmute_copy(&currentonly), core::mem::transmute_copy(&profiletypes), core::mem::transmute_copy(&validprofiles)).into()
            }
        }
        unsafe extern "system" fn GetSupportedModePages<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetSupportedModePages(this, core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&modepagetypes), core::mem::transmute_copy(&validpages)).into()
            }
        }
        unsafe extern "system" fn GetByteAlignmentMask<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2Ex_Impl::GetByteAlignmentMask(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2Ex_Impl::GetMaximumNonPageAlignedTransferSize(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2Ex_Impl::GetMaximumPageAlignedTransferSize(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendCommandNoData: SendCommandNoData::<Identity, OFFSET>,
            SendCommandSendDataToDevice: SendCommandSendDataToDevice::<Identity, OFFSET>,
            SendCommandGetDataFromDevice: SendCommandGetDataFromDevice::<Identity, OFFSET>,
            ReadDvdStructure: ReadDvdStructure::<Identity, OFFSET>,
            SendDvdStructure: SendDvdStructure::<Identity, OFFSET>,
            GetAdapterDescriptor: GetAdapterDescriptor::<Identity, OFFSET>,
            GetDeviceDescriptor: GetDeviceDescriptor::<Identity, OFFSET>,
            GetDiscInformation: GetDiscInformation::<Identity, OFFSET>,
            GetTrackInformation: GetTrackInformation::<Identity, OFFSET>,
            GetFeaturePage: GetFeaturePage::<Identity, OFFSET>,
            GetModePage: GetModePage::<Identity, OFFSET>,
            SetModePage: SetModePage::<Identity, OFFSET>,
            GetSupportedFeaturePages: GetSupportedFeaturePages::<Identity, OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Identity, OFFSET>,
            GetSupportedModePages: GetSupportedModePages::<Identity, OFFSET>,
            GetByteAlignmentMask: GetByteAlignmentMask::<Identity, OFFSET>,
            GetMaximumNonPageAlignedTransferSize: GetMaximumNonPageAlignedTransferSize::<Identity, OFFSET>,
            GetMaximumPageAlignedTransferSize: GetMaximumPageAlignedTransferSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2Ex as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiscRecorder2Ex {}
windows_core::imp::define_interface!(IEnumDiscMasterFormats, IEnumDiscMasterFormats_Vtbl, 0xddf445e1_54ba_11d3_9144_00104ba11c5e);
windows_core::imp::interface_hierarchy!(IEnumDiscMasterFormats, windows_core::IUnknown);
impl IEnumDiscMasterFormats {
    pub unsafe fn Next(&self, lpiidformatid: &mut [windows_core::GUID], pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lpiidformatid.len().try_into().unwrap(), core::mem::transmute(lpiidformatid.as_ptr()), pcfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, cformats: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cformats).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDiscMasterFormats> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscMasterFormats_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDiscMasterFormats_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cformats: u32, lpiidformatid: *mut windows_core::GUID, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cformats: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDiscMasterFormats>;
}
impl IEnumDiscMasterFormats_Vtbl {
    pub const fn new<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32, lpiidformatid: *mut windows_core::GUID, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDiscMasterFormats_Impl::Next(this, core::mem::transmute_copy(&cformats), core::mem::transmute_copy(&lpiidformatid), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDiscMasterFormats_Impl::Skip(this, core::mem::transmute_copy(&cformats)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDiscMasterFormats_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDiscMasterFormats_Impl::Clone(this) {
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
        iid == &<IEnumDiscMasterFormats as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDiscMasterFormats {}
windows_core::imp::define_interface!(IEnumDiscRecorders, IEnumDiscRecorders_Vtbl, 0x9b1921e1_54ac_11d3_9144_00104ba11c5e);
windows_core::imp::interface_hierarchy!(IEnumDiscRecorders, windows_core::IUnknown);
impl IEnumDiscRecorders {
    pub unsafe fn Next(&self, pprecorder: &mut [Option<IDiscRecorder>], pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pprecorder.len().try_into().unwrap(), core::mem::transmute(pprecorder.as_ptr()), pcfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, crecorders: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), crecorders).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDiscRecorders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscRecorders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDiscRecorders_Impl: windows_core::IUnknownImpl {
    fn Next(&self, crecorders: u32, pprecorder: *mut Option<IDiscRecorder>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, crecorders: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDiscRecorders>;
}
impl IEnumDiscRecorders_Vtbl {
    pub const fn new<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crecorders: u32, pprecorder: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDiscRecorders_Impl::Next(this, core::mem::transmute_copy(&crecorders), core::mem::transmute_copy(&pprecorder), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crecorders: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDiscRecorders_Impl::Skip(this, core::mem::transmute_copy(&crecorders)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDiscRecorders_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDiscRecorders_Impl::Clone(this) {
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
        iid == &<IEnumDiscRecorders as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDiscRecorders {}
windows_core::imp::define_interface!(IEnumFsiItems, IEnumFsiItems_Vtbl, 0x2c941fda_975b_59be_a960_9a2a262853a5);
windows_core::imp::interface_hierarchy!(IEnumFsiItems, windows_core::IUnknown);
impl IEnumFsiItems {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [Option<IFsiItem>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumFsiItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFsiItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumFsiItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IFsiItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumFsiItems_Vtbl {
    pub const fn new<Identity: IEnumFsiItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFsiItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFsiItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFsiItems_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumFsiItems_Impl::Clone(this) {
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
        iid == &<IEnumFsiItems as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumFsiItems {}
windows_core::imp::define_interface!(IEnumProgressItems, IEnumProgressItems_Vtbl, 0x2c941fd6_975b_59be_a960_9a2a262853a5);
windows_core::imp::interface_hierarchy!(IEnumProgressItems, windows_core::IUnknown);
impl IEnumProgressItems {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [Option<IProgressItem>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumProgressItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumProgressItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumProgressItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IProgressItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumProgressItems_Vtbl {
    pub const fn new<Identity: IEnumProgressItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumProgressItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumProgressItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumProgressItems_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumProgressItems_Impl::Clone(this) {
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
        iid == &<IEnumProgressItems as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumProgressItems {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImage, IFileSystemImage_Vtbl, 0x2c941fe1_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImage {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImage, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage {
    pub unsafe fn Root(&self) -> windows_core::Result<IFsiDirectoryItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Root)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SessionStartBlock(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionStartBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSessionStartBlock)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn FreeMediaBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeMediaBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFreeMediaBlocks)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn SetMaxMediaBlocksFromDevice<P0>(&self, discrecorder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMaxMediaBlocksFromDevice)(windows_core::Interface::as_raw(self), discrecorder.param().abi()).ok() }
    }
    pub unsafe fn UsedBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsedBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetVolumeName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVolumeName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportedVolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BootImageOptions(&self) -> windows_core::Result<IBootOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BootImageOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBootImageOptions<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBootOptions>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBootImageOptions)(windows_core::Interface::as_raw(self), newval.param().abi()).ok() }
    }
    pub unsafe fn FileCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DirectoryCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DirectoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetWorkingDirectory(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn ChangePoint(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ChangePoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StrictFileSystemCompliance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrictFileSystemCompliance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrictFileSystemCompliance)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn UseRestrictedCharacterSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemsToCreate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileSystemsToCreate)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUDFRevision)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn UDFRevision(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UDFRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UDFRevisionsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ChooseImageDefaults<P0>(&self, discrecorder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ChooseImageDefaults)(windows_core::Interface::as_raw(self), discrecorder.param().abi()).ok() }
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChooseImageDefaultsForMediaType)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ISO9660InterchangeLevelsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Exists(&self, fullpath: &windows_core::BSTR) -> windows_core::Result<FsiItemType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Exists)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(fullpath), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CalculateDiscIdentifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CalculateDiscIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IdentifyFileSystemsOnDisc<P0>(&self, discrecorder: P0) -> windows_core::Result<FsiFileSystems>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IdentifyFileSystemsOnDisc)(windows_core::Interface::as_raw(self), discrecorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultFileSystemForImport)(windows_core::Interface::as_raw(self), filesystems, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ImportFileSystem(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImportSpecificFileSystem)(windows_core::Interface::as_raw(self), filesystemtouse).ok() }
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RollbackToChangePoint)(windows_core::Interface::as_raw(self), changepoint).ok() }
    }
    pub unsafe fn LockInChangePoint(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockInChangePoint)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CreateDirectoryItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiDirectoryItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDirectoryItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFileItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiFileItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeNameUDF)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeNameJoliet)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeNameISO9660)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn StageFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StageFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStageFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStageFiles)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMultisessionInterfaces)(windows_core::Interface::as_raw(self), newval).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetMaxMediaBlocksFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UsedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportedVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DirectoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub SetFileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    pub FileSystemsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub SetUDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UDFRevisionsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ChooseImageDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChooseImageDefaultsForMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ISO9660InterchangeLevelsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Exists: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FsiItemType) -> windows_core::HRESULT,
    pub CalculateDiscIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IdentifyFileSystemsOnDisc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub GetDefaultFileSystemForImport: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    pub RollbackToChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LockInChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDirectoryItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeNameUDF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeNameJoliet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeNameISO9660: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetMultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage_Impl: super::super::System::Com::IDispatch_Impl {
    fn Root(&self) -> windows_core::Result<IFsiDirectoryItem>;
    fn SessionStartBlock(&self) -> windows_core::Result<i32>;
    fn SetSessionStartBlock(&self, newval: i32) -> windows_core::Result<()>;
    fn FreeMediaBlocks(&self) -> windows_core::Result<i32>;
    fn SetFreeMediaBlocks(&self, newval: i32) -> windows_core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(&self, discrecorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn UsedBlocks(&self) -> windows_core::Result<i32>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVolumeName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BootImageOptions(&self) -> windows_core::Result<IBootOptions>;
    fn SetBootImageOptions(&self, newval: windows_core::Ref<IBootOptions>) -> windows_core::Result<()>;
    fn FileCount(&self) -> windows_core::Result<i32>;
    fn DirectoryCount(&self) -> windows_core::Result<i32>;
    fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetWorkingDirectory(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangePoint(&self) -> windows_core::Result<i32>;
    fn StrictFileSystemCompliance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStrictFileSystemCompliance(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseRestrictedCharacterSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseRestrictedCharacterSet(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems>;
    fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::Result<()>;
    fn FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems>;
    fn SetUDFRevision(&self, newval: i32) -> windows_core::Result<()>;
    fn UDFRevision(&self) -> windows_core::Result<i32>;
    fn UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ChooseImageDefaults(&self, discrecorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::Result<()>;
    fn SetISO9660InterchangeLevel(&self, newval: i32) -> windows_core::Result<()>;
    fn ISO9660InterchangeLevel(&self) -> windows_core::Result<i32>;
    fn ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult>;
    fn Exists(&self, fullpath: &windows_core::BSTR) -> windows_core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IdentifyFileSystemsOnDisc(&self, discrecorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> windows_core::Result<FsiFileSystems>;
    fn ImportFileSystem(&self) -> windows_core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> windows_core::Result<()>;
    fn RollbackToChangePoint(&self, changepoint: i32) -> windows_core::Result<()>;
    fn LockInChangePoint(&self) -> windows_core::Result<()>;
    fn CreateDirectoryItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiFileItem>;
    fn VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StageFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStageFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage_Vtbl {
    pub const fn new<Identity: IFileSystemImage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Root<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::Root(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::SessionStartBlock(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetSessionStartBlock(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FreeMediaBlocks(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetFreeMediaBlocks(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetMaxMediaBlocksFromDevice(this, core::mem::transmute_copy(&discrecorder)).into()
            }
        }
        unsafe extern "system" fn UsedBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UsedBlocks(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetVolumeName(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn ImportedVolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ImportedVolumeName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::BootImageOptions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetBootImageOptions(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FileCount(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DirectoryCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::DirectoryCount(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::WorkingDirectory(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetWorkingDirectory(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn ChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ChangePoint(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::StrictFileSystemCompliance(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetStrictFileSystemCompliance(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UseRestrictedCharacterSet(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetUseRestrictedCharacterSet(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FileSystemsToCreate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetFileSystemsToCreate(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileSystemsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FileSystemsSupported(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetUDFRevision(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn UDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UDFRevision(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UDFRevisionsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UDFRevisionsSupported(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChooseImageDefaults<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::ChooseImageDefaults(this, core::mem::transmute_copy(&discrecorder)).into()
            }
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::ChooseImageDefaultsForMediaType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetISO9660InterchangeLevel(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ISO9660InterchangeLevel(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ISO9660InterchangeLevelsSupported(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateResultImage<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CreateResultImage(this) {
                    Ok(ok__) => {
                        resultstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Exists<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullpath: *mut core::ffi::c_void, itemtype: *mut FsiItemType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::Exists(this, core::mem::transmute(&fullpath)) {
                    Ok(ok__) => {
                        itemtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discidentifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CalculateDiscIdentifier(this) {
                    Ok(ok__) => {
                        discidentifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void, filesystems: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::IdentifyFileSystemsOnDisc(this, core::mem::transmute_copy(&discrecorder)) {
                    Ok(ok__) => {
                        filesystems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::GetDefaultFileSystemForImport(this, core::mem::transmute_copy(&filesystems)) {
                    Ok(ok__) => {
                        importdefault.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportFileSystem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ImportFileSystem(this) {
                    Ok(ok__) => {
                        importedfilesystem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystemtouse: FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::ImportSpecificFileSystem(this, core::mem::transmute_copy(&filesystemtouse)).into()
            }
        }
        unsafe extern "system" fn RollbackToChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changepoint: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::RollbackToChangePoint(this, core::mem::transmute_copy(&changepoint)).into()
            }
        }
        unsafe extern "system" fn LockInChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::LockInChangePoint(this).into()
            }
        }
        unsafe extern "system" fn CreateDirectoryItem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, newitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CreateDirectoryItem(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        newitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFileItem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, newitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CreateFileItem(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        newitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeNameUDF<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeNameUDF(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeNameJoliet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeNameJoliet(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeNameISO9660<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeNameISO9660(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::StageFiles(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetStageFiles(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::MultisessionInterfaces(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetMultisessionInterfaces(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Root: Root::<Identity, OFFSET>,
            SessionStartBlock: SessionStartBlock::<Identity, OFFSET>,
            SetSessionStartBlock: SetSessionStartBlock::<Identity, OFFSET>,
            FreeMediaBlocks: FreeMediaBlocks::<Identity, OFFSET>,
            SetFreeMediaBlocks: SetFreeMediaBlocks::<Identity, OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Identity, OFFSET>,
            UsedBlocks: UsedBlocks::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, OFFSET>,
            ImportedVolumeName: ImportedVolumeName::<Identity, OFFSET>,
            BootImageOptions: BootImageOptions::<Identity, OFFSET>,
            SetBootImageOptions: SetBootImageOptions::<Identity, OFFSET>,
            FileCount: FileCount::<Identity, OFFSET>,
            DirectoryCount: DirectoryCount::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            ChangePoint: ChangePoint::<Identity, OFFSET>,
            StrictFileSystemCompliance: StrictFileSystemCompliance::<Identity, OFFSET>,
            SetStrictFileSystemCompliance: SetStrictFileSystemCompliance::<Identity, OFFSET>,
            UseRestrictedCharacterSet: UseRestrictedCharacterSet::<Identity, OFFSET>,
            SetUseRestrictedCharacterSet: SetUseRestrictedCharacterSet::<Identity, OFFSET>,
            FileSystemsToCreate: FileSystemsToCreate::<Identity, OFFSET>,
            SetFileSystemsToCreate: SetFileSystemsToCreate::<Identity, OFFSET>,
            FileSystemsSupported: FileSystemsSupported::<Identity, OFFSET>,
            SetUDFRevision: SetUDFRevision::<Identity, OFFSET>,
            UDFRevision: UDFRevision::<Identity, OFFSET>,
            UDFRevisionsSupported: UDFRevisionsSupported::<Identity, OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Identity, OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Identity, OFFSET>,
            SetISO9660InterchangeLevel: SetISO9660InterchangeLevel::<Identity, OFFSET>,
            ISO9660InterchangeLevel: ISO9660InterchangeLevel::<Identity, OFFSET>,
            ISO9660InterchangeLevelsSupported: ISO9660InterchangeLevelsSupported::<Identity, OFFSET>,
            CreateResultImage: CreateResultImage::<Identity, OFFSET>,
            Exists: Exists::<Identity, OFFSET>,
            CalculateDiscIdentifier: CalculateDiscIdentifier::<Identity, OFFSET>,
            IdentifyFileSystemsOnDisc: IdentifyFileSystemsOnDisc::<Identity, OFFSET>,
            GetDefaultFileSystemForImport: GetDefaultFileSystemForImport::<Identity, OFFSET>,
            ImportFileSystem: ImportFileSystem::<Identity, OFFSET>,
            ImportSpecificFileSystem: ImportSpecificFileSystem::<Identity, OFFSET>,
            RollbackToChangePoint: RollbackToChangePoint::<Identity, OFFSET>,
            LockInChangePoint: LockInChangePoint::<Identity, OFFSET>,
            CreateDirectoryItem: CreateDirectoryItem::<Identity, OFFSET>,
            CreateFileItem: CreateFileItem::<Identity, OFFSET>,
            VolumeNameUDF: VolumeNameUDF::<Identity, OFFSET>,
            VolumeNameJoliet: VolumeNameJoliet::<Identity, OFFSET>,
            VolumeNameISO9660: VolumeNameISO9660::<Identity, OFFSET>,
            StageFiles: StageFiles::<Identity, OFFSET>,
            SetStageFiles: SetStageFiles::<Identity, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, OFFSET>,
            SetMultisessionInterfaces: SetMultisessionInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImage {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImage2, IFileSystemImage2_Vtbl, 0xd7644b2c_1537_4767_b62f_f1387b02ddfd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImage2 {
    type Target = IFileSystemImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImage2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImage);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage2 {
    pub unsafe fn BootImageOptionsArray(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BootImageOptionsArray)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBootImageOptionsArray)(windows_core::Interface::as_raw(self), newval).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage2_Vtbl {
    pub base__: IFileSystemImage_Vtbl,
    pub BootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetBootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage2_Impl: IFileSystemImage_Impl {
    fn BootImageOptionsArray(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage2_Vtbl {
    pub const fn new<Identity: IFileSystemImage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage2_Impl::BootImageOptionsArray(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage2_Impl::SetBootImageOptionsArray(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFileSystemImage_Vtbl::new::<Identity, OFFSET>(),
            BootImageOptionsArray: BootImageOptionsArray::<Identity, OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImage2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImage3, IFileSystemImage3_Vtbl, 0x7cff842c_7e97_4807_8304_910dd8f7c051);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImage3 {
    type Target = IFileSystemImage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImage3, windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImage, IFileSystemImage2);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage3 {
    pub unsafe fn CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreateRedundantUdfMetadataFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProbeSpecificFileSystem)(windows_core::Interface::as_raw(self), filesystemtoprobe, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage3_Vtbl {
    pub base__: IFileSystemImage2_Vtbl,
    pub CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetCreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ProbeSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage3_Impl: IFileSystemImage2_Impl {
    fn CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCreateRedundantUdfMetadataFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage3_Vtbl {
    pub const fn new<Identity: IFileSystemImage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage3_Impl::CreateRedundantUdfMetadataFiles(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage3_Impl::SetCreateRedundantUdfMetadataFiles(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage3_Impl::ProbeSpecificFileSystem(this, core::mem::transmute_copy(&filesystemtoprobe)) {
                    Ok(ok__) => {
                        isappendable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFileSystemImage2_Vtbl::new::<Identity, OFFSET>(),
            CreateRedundantUdfMetadataFiles: CreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            SetCreateRedundantUdfMetadataFiles: SetCreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<IFileSystemImage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImage3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImageResult, IFileSystemImageResult_Vtbl, 0x2c941fd8_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImageResult {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult {
    pub unsafe fn ImageStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProgressItems(&self) -> windows_core::Result<IProgressItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TotalBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BlockSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DiscId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiscId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ImageStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TotalBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DiscId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult_Impl: super::super::System::Com::IDispatch_Impl {
    fn ImageStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn ProgressItems(&self) -> windows_core::Result<IProgressItems>;
    fn TotalBlocks(&self) -> windows_core::Result<i32>;
    fn BlockSize(&self) -> windows_core::Result<i32>;
    fn DiscId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImageResult_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImageStream<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::ImageStream(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressItems<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::ProgressItems(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalBlocks<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::TotalBlocks(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BlockSize<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::BlockSize(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DiscId<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::DiscId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ImageStream: ImageStream::<Identity, OFFSET>,
            ProgressItems: ProgressItems::<Identity, OFFSET>,
            TotalBlocks: TotalBlocks::<Identity, OFFSET>,
            BlockSize: BlockSize::<Identity, OFFSET>,
            DiscId: DiscId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImageResult {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFileSystemImageResult2, IFileSystemImageResult2_Vtbl, 0xb507ca29_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFileSystemImageResult2 {
    type Target = IFileSystemImageResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFileSystemImageResult);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult2 {
    pub unsafe fn ModifiedBlocks(&self) -> windows_core::Result<IBlockRangeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifiedBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult2_Vtbl {
    pub base__: IFileSystemImageResult_Vtbl,
    pub ModifiedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult2_Impl: IFileSystemImageResult_Impl {
    fn ModifiedBlocks(&self) -> windows_core::Result<IBlockRangeList>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImageResult2_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModifiedBlocks<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult2_Impl::ModifiedBlocks(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IFileSystemImageResult_Vtbl::new::<Identity, OFFSET>(), ModifiedBlocks: ModifiedBlocks::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImageResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImageResult2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiDirectoryItem, IFsiDirectoryItem_Vtbl, 0x2c941fdc_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiDirectoryItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsiItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFsiItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddDirectory(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn AddFile<P1>(&self, path: &windows_core::BSTR, filedata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), filedata.param().abi()).ok() }
    }
    pub unsafe fn AddTree(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTree)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcedirectory), includebasedirectory).ok() }
    }
    pub unsafe fn Add<P0>(&self, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsiItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), item.param().abi()).ok() }
    }
    pub unsafe fn Remove(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn RemoveTree(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveTree)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumFsiItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem_Impl: IFsiItem_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsiItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems>;
    fn AddDirectory(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddFile(&self, path: &windows_core::BSTR, filedata: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn AddTree(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Add(&self, item: windows_core::Ref<IFsiItem>) -> windows_core::Result<()>;
    fn Remove(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveTree(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiDirectoryItem_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::get_Item(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFsiItems<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::EnumFsiItems(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDirectory<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::AddDirectory(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn AddFile<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, filedata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::AddFile(this, core::mem::transmute(&path), core::mem::transmute_copy(&filedata)).into()
            }
        }
        unsafe extern "system" fn AddTree<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: *mut core::ffi::c_void, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::AddTree(this, core::mem::transmute(&sourcedirectory), core::mem::transmute_copy(&includebasedirectory)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::Add(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::Remove(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn RemoveTree<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::RemoveTree(this, core::mem::transmute(&path)).into()
            }
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            EnumFsiItems: EnumFsiItems::<Identity, OFFSET>,
            AddDirectory: AddDirectory::<Identity, OFFSET>,
            AddFile: AddFile::<Identity, OFFSET>,
            AddTree: AddTree::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveTree: RemoveTree::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiDirectoryItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiDirectoryItem2, IFsiDirectoryItem2_Vtbl, 0xf7fb4b9b_6d96_4d7b_9115_201b144811ef);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiDirectoryItem2 {
    type Target = IFsiDirectoryItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem, IFsiDirectoryItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem2 {
    pub unsafe fn AddTreeWithNamedStreams(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTreeWithNamedStreams)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcedirectory), includebasedirectory).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem2_Vtbl {
    pub base__: IFsiDirectoryItem_Vtbl,
    pub AddTreeWithNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem2_Impl: IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiDirectoryItem2_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTreeWithNamedStreams<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: *mut core::ffi::c_void, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem2_Impl::AddTreeWithNamedStreams(this, core::mem::transmute(&sourcedirectory), core::mem::transmute_copy(&includebasedirectory)).into()
            }
        }
        Self { base__: IFsiDirectoryItem_Vtbl::new::<Identity, OFFSET>(), AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiDirectoryItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiDirectoryItem2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiFileItem, IFsiFileItem_Vtbl, 0x2c941fdb_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiFileItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiFileItem, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem {
    pub unsafe fn DataSize(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DataSize32BitLow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSize32BitLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DataSize32BitHigh(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSize32BitHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Data(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetData<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), newval.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    pub DataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub DataSize32BitLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataSize32BitHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem_Impl: IFsiItem_Impl {
    fn DataSize(&self) -> windows_core::Result<i64>;
    fn DataSize32BitLow(&self) -> windows_core::Result<i32>;
    fn DataSize32BitHigh(&self) -> windows_core::Result<i32>;
    fn Data(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetData(&self, newval: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiFileItem_Vtbl {
    pub const fn new<Identity: IFsiFileItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataSize<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::DataSize(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataSize32BitLow<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::DataSize32BitLow(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataSize32BitHigh<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::DataSize32BitHigh(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Data<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::Data(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem_Impl::SetData(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, OFFSET>(),
            DataSize: DataSize::<Identity, OFFSET>,
            DataSize32BitLow: DataSize32BitLow::<Identity, OFFSET>,
            DataSize32BitHigh: DataSize32BitHigh::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiFileItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiFileItem2, IFsiFileItem2_Vtbl, 0x199d0c19_11e1_40eb_8ec2_c8c822a07792);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiFileItem2 {
    type Target = IFsiFileItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiFileItem2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsiItem, IFsiFileItem);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem2 {
    pub unsafe fn FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FsiNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsNamedStream(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNamedStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddStream<P1>(&self, name: &windows_core::BSTR, streamdata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), streamdata.param().abi()).ok() }
    }
    pub unsafe fn RemoveStream(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsRealTime(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsRealTime)(windows_core::Interface::as_raw(self), newval).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem2_Vtbl {
    pub base__: IFsiFileItem_Vtbl,
    pub FsiNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsNamedStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem2_Impl: IFsiFileItem_Impl {
    fn FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams>;
    fn IsNamedStream(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AddStream(&self, name: &windows_core::BSTR, streamdata: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn RemoveStream(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsRealTime(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiFileItem2_Vtbl {
    pub const fn new<Identity: IFsiFileItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FsiNamedStreams<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::FsiNamedStreams(this) {
                    Ok(ok__) => {
                        streams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsNamedStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::IsNamedStream(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, streamdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem2_Impl::AddStream(this, core::mem::transmute(&name), core::mem::transmute_copy(&streamdata)).into()
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem2_Impl::RemoveStream(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn IsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::IsRealTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem2_Impl::SetIsRealTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFsiFileItem_Vtbl::new::<Identity, OFFSET>(),
            FsiNamedStreams: FsiNamedStreams::<Identity, OFFSET>,
            IsNamedStream: IsNamedStream::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            IsRealTime: IsRealTime::<Identity, OFFSET>,
            SetIsRealTime: SetIsRealTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiFileItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiFileItem2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiItem, IFsiItem_Vtbl, 0x2c941fd9_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiItem {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiItem, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsiItem {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FullPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreationTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCreationTime)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn LastAccessedTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastAccessedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLastAccessedTime)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn LastModifiedTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModifiedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLastModifiedTime)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn IsHidden(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsHidden(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsHidden)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemName)(windows_core::Interface::as_raw(self), filesystem, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemPath)(windows_core::Interface::as_raw(self), filesystem, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetCreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FileSystemName: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileSystemPath: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiItem_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FullPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreationTime(&self) -> windows_core::Result<f64>;
    fn SetCreationTime(&self, newval: f64) -> windows_core::Result<()>;
    fn LastAccessedTime(&self) -> windows_core::Result<f64>;
    fn SetLastAccessedTime(&self, newval: f64) -> windows_core::Result<()>;
    fn LastModifiedTime(&self) -> windows_core::Result<f64>;
    fn SetLastModifiedTime(&self, newval: f64) -> windows_core::Result<()>;
    fn IsHidden(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsHidden(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FileSystemName(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
    fn FileSystemPath(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiItem_Vtbl {
    pub const fn new<Identity: IFsiItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FullPath<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::FullPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::CreationTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetCreationTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn LastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::LastAccessedTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetLastAccessedTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn LastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::LastModifiedTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetLastModifiedTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn IsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::IsHidden(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetIsHidden(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileSystemName<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::FileSystemName(this, core::mem::transmute_copy(&filesystem)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileSystemPath<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::FileSystemPath(this, core::mem::transmute_copy(&filesystem)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            FullPath: FullPath::<Identity, OFFSET>,
            CreationTime: CreationTime::<Identity, OFFSET>,
            SetCreationTime: SetCreationTime::<Identity, OFFSET>,
            LastAccessedTime: LastAccessedTime::<Identity, OFFSET>,
            SetLastAccessedTime: SetLastAccessedTime::<Identity, OFFSET>,
            LastModifiedTime: LastModifiedTime::<Identity, OFFSET>,
            SetLastModifiedTime: SetLastModifiedTime::<Identity, OFFSET>,
            IsHidden: IsHidden::<Identity, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, OFFSET>,
            FileSystemName: FileSystemName::<Identity, OFFSET>,
            FileSystemPath: FileSystemPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsiNamedStreams, IFsiNamedStreams_Vtbl, 0xed79ba56_5294_4250_8d46_f9aecee23459);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsiNamedStreams {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsiNamedStreams, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsiNamedStreams {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IFsiFileItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiNamedStreams_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiNamedStreams_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IFsiFileItem2>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiNamedStreams_Vtbl {
    pub const fn new<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumNamedStreams<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::EnumNamedStreams(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            EnumNamedStreams: EnumNamedStreams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiNamedStreams as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiNamedStreams {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IIsoImageManager, IIsoImageManager_Vtbl, 0x6ca38be5_fbbb_4800_95a1_a438865eb0d4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IIsoImageManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IIsoImageManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IIsoImageManager {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Stream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPath(&self, val: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)).ok() }
    }
    pub unsafe fn SetStream<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStream)(windows_core::Interface::as_raw(self), data.param().abi()).ok() }
    }
    pub unsafe fn Validate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIsoImageManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIsoImageManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Stream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetPath(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetStream(&self, data: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Validate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIsoImageManager_Vtbl {
    pub const fn new<Identity: IIsoImageManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsoImageManager_Impl::Path(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stream<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsoImageManager_Impl::Stream(this) {
                    Ok(ok__) => {
                        data.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsoImageManager_Impl::SetPath(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn SetStream<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsoImageManager_Impl::SetStream(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn Validate<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsoImageManager_Impl::Validate(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            SetStream: SetStream::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsoImageManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIsoImageManager {}
windows_core::imp::define_interface!(IJolietDiscMaster, IJolietDiscMaster_Vtbl, 0xe3bc42ce_4e5c_11d3_9144_00104ba11c5e);
windows_core::imp::interface_hierarchy!(IJolietDiscMaster, windows_core::IUnknown);
impl IJolietDiscMaster {
    pub unsafe fn GetTotalDataBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalDataBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUsedDataBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUsedDataBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataBlockSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataBlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn AddData<P0>(&self, pstorage: P0, lfileoverwrite: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddData)(windows_core::Interface::as_raw(self), pstorage.param().abi(), lfileoverwrite).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetJolietProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJolietProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetJolietProperties<P0>(&self, ppropstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IPropertyStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetJolietProperties)(windows_core::Interface::as_raw(self), ppropstg.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJolietDiscMaster_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTotalDataBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetUsedDataBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDataBlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub AddData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    AddData: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetJolietProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetJolietProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetJolietProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetJolietProperties: usize,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IJolietDiscMaster_Impl: windows_core::IUnknownImpl {
    fn GetTotalDataBlocks(&self) -> windows_core::Result<i32>;
    fn GetUsedDataBlocks(&self) -> windows_core::Result<i32>;
    fn GetDataBlockSize(&self) -> windows_core::Result<i32>;
    fn AddData(&self, pstorage: windows_core::Ref<super::super::System::Com::StructuredStorage::IStorage>, lfileoverwrite: i32) -> windows_core::Result<()>;
    fn GetJolietProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetJolietProperties(&self, ppropstg: windows_core::Ref<super::super::System::Com::StructuredStorage::IPropertyStorage>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IJolietDiscMaster_Vtbl {
    pub const fn new<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTotalDataBlocks<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJolietDiscMaster_Impl::GetTotalDataBlocks(this) {
                    Ok(ok__) => {
                        pnblocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUsedDataBlocks<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJolietDiscMaster_Impl::GetUsedDataBlocks(this) {
                    Ok(ok__) => {
                        pnblocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataBlockSize<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblockbytes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJolietDiscMaster_Impl::GetDataBlockSize(this) {
                    Ok(ok__) => {
                        pnblockbytes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddData<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorage: *mut core::ffi::c_void, lfileoverwrite: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJolietDiscMaster_Impl::AddData(this, core::mem::transmute_copy(&pstorage), core::mem::transmute_copy(&lfileoverwrite)).into()
            }
        }
        unsafe extern "system" fn GetJolietProperties<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJolietDiscMaster_Impl::GetJolietProperties(this) {
                    Ok(ok__) => {
                        pppropstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJolietProperties<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJolietDiscMaster_Impl::SetJolietProperties(this, core::mem::transmute_copy(&ppropstg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTotalDataBlocks: GetTotalDataBlocks::<Identity, OFFSET>,
            GetUsedDataBlocks: GetUsedDataBlocks::<Identity, OFFSET>,
            GetDataBlockSize: GetDataBlockSize::<Identity, OFFSET>,
            AddData: AddData::<Identity, OFFSET>,
            GetJolietProperties: GetJolietProperties::<Identity, OFFSET>,
            SetJolietProperties: SetJolietProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJolietDiscMaster as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IJolietDiscMaster {}
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
pub const IMAPI2FS_FullVersion_STR: windows_core::PCSTR = windows_core::s!("1.0");
pub const IMAPI2FS_FullVersion_WSTR: windows_core::PCWSTR = windows_core::w!("1.0");
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
pub const IMAPILib2_MajorVersion: u32 = 1u32;
pub const IMAPILib2_MinorVersion: u32 = 0u32;
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_BURN_VERIFICATION_LEVEL(pub i32);
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(0i32);
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(1i32);
pub const IMAPI_CD_SECTOR_AUDIO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(0i32);
pub const IMAPI_CD_SECTOR_MODE1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(2i32);
pub const IMAPI_CD_SECTOR_MODE1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(6i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(3i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(7i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(4i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(8i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(5i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(9i32);
pub const IMAPI_CD_SECTOR_MODE_ZERO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_CD_SECTOR_TYPE(pub i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(0i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(1i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(pub i32);
pub const IMAPI_E_ALREADYOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80040222_u32 as _);
pub const IMAPI_E_BADJOLIETNAME: windows_core::HRESULT = windows_core::HRESULT(0x8004021D_u32 as _);
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: windows_core::HRESULT = windows_core::HRESULT(0x8004022E_u32 as _);
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x8004022C_u32 as _);
pub const IMAPI_E_COMPRESSEDSTASH: windows_core::HRESULT = windows_core::HRESULT(0x80040228_u32 as _);
pub const IMAPI_E_DEVICE_INVALIDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040214_u32 as _);
pub const IMAPI_E_DEVICE_NOPROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0x80040211_u32 as _);
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: windows_core::HRESULT = windows_core::HRESULT(0x80040212_u32 as _);
pub const IMAPI_E_DEVICE_NOTPRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80040213_u32 as _);
pub const IMAPI_E_DEVICE_STILL_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80040226_u32 as _);
pub const IMAPI_E_DISCFULL: windows_core::HRESULT = windows_core::HRESULT(0x8004021C_u32 as _);
pub const IMAPI_E_DISCINFO: windows_core::HRESULT = windows_core::HRESULT(0x80040219_u32 as _);
pub const IMAPI_E_ENCRYPTEDSTASH: windows_core::HRESULT = windows_core::HRESULT(0x80040229_u32 as _);
pub const IMAPI_E_FILEACCESS: windows_core::HRESULT = windows_core::HRESULT(0x80040218_u32 as _);
pub const IMAPI_E_FILEEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80040224_u32 as _);
pub const IMAPI_E_FILESYSTEM: windows_core::HRESULT = windows_core::HRESULT(0x80040217_u32 as _);
pub const IMAPI_E_GENERIC: windows_core::HRESULT = windows_core::HRESULT(0x8004020E_u32 as _);
pub const IMAPI_E_INITIALIZE_ENDWRITE: windows_core::HRESULT = windows_core::HRESULT(0x80040216_u32 as _);
pub const IMAPI_E_INITIALIZE_WRITE: windows_core::HRESULT = windows_core::HRESULT(0x80040215_u32 as _);
pub const IMAPI_E_INVALIDIMAGE: windows_core::HRESULT = windows_core::HRESULT(0x8004021E_u32 as _);
pub const IMAPI_E_LOSS_OF_STREAMING: windows_core::HRESULT = windows_core::HRESULT(0x80040227_u32 as _);
pub const IMAPI_E_MEDIUM_INVALIDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040210_u32 as _);
pub const IMAPI_E_MEDIUM_NOTPRESENT: windows_core::HRESULT = windows_core::HRESULT(0x8004020F_u32 as _);
pub const IMAPI_E_NOACTIVEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004021F_u32 as _);
pub const IMAPI_E_NOACTIVERECORDER: windows_core::HRESULT = windows_core::HRESULT(0x80040220_u32 as _);
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: windows_core::HRESULT = windows_core::HRESULT(0x8004022A_u32 as _);
pub const IMAPI_E_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8004020C_u32 as _);
pub const IMAPI_E_NOTOPENED: windows_core::HRESULT = windows_core::HRESULT(0x8004020B_u32 as _);
pub const IMAPI_E_REMOVABLESTASH: windows_core::HRESULT = windows_core::HRESULT(0x8004022B_u32 as _);
pub const IMAPI_E_STASHINUSE: windows_core::HRESULT = windows_core::HRESULT(0x80040225_u32 as _);
pub const IMAPI_E_TRACKNOTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x8004021A_u32 as _);
pub const IMAPI_E_TRACKOPEN: windows_core::HRESULT = windows_core::HRESULT(0x8004021B_u32 as _);
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: windows_core::HRESULT = windows_core::HRESULT(0x8004022D_u32 as _);
pub const IMAPI_E_USERABORT: windows_core::HRESULT = windows_core::HRESULT(0x8004020D_u32 as _);
pub const IMAPI_E_WRONGDISC: windows_core::HRESULT = windows_core::HRESULT(0x80040223_u32 as _);
pub const IMAPI_E_WRONGFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80040221_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_FEATURE_PAGE_TYPE(pub i32);
pub const IMAPI_FEATURE_PAGE_TYPE_AACS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(269i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(56i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(64i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(65i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(39i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(259i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(46i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(29i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(30i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(55i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(45i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CORE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(1i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(266i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(48i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(50i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(49i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(267i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CSS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(262i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(47i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(43i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(42i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(59i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(31i32);
pub const IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(258i32);
pub const IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(41i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(268i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(35i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(36i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(80i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(81i32);
pub const IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(33i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(51i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(264i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(265i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(260i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MORPHING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(2i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MRW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(40i32);
pub const IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(256i32);
pub const IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(0i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(16i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(32i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(263i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(3i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(38i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(44i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(34i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SMART: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(257i32);
pub const IMAPI_FEATURE_PAGE_TYPE_TIMEOUT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(261i32);
pub const IMAPI_FEATURE_PAGE_TYPE_VCPS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(272i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(37i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_FORMAT2_DATA_MEDIA_STATE(pub i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1024i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2048i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(16384i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(15i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4096i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(0i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(64512i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(32768i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8192i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_FORMAT2_DATA_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(6i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(5i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(7i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(pub i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(2i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(3i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_FORMAT2_TAO_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(4i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_MEDIA_PHYSICAL_TYPE(pub i32);
pub const IMAPI_MEDIA_TYPE_BDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(18i32);
pub const IMAPI_MEDIA_TYPE_BDRE: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
pub const IMAPI_MEDIA_TYPE_BDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(17i32);
pub const IMAPI_MEDIA_TYPE_CDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(2i32);
pub const IMAPI_MEDIA_TYPE_CDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(1i32);
pub const IMAPI_MEDIA_TYPE_CDRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(3i32);
pub const IMAPI_MEDIA_TYPE_DISK: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(12i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(9i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(10i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(11i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(6i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(7i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(13i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(8i32);
pub const IMAPI_MEDIA_TYPE_DVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(5i32);
pub const IMAPI_MEDIA_TYPE_DVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(4i32);
pub const IMAPI_MEDIA_TYPE_HDDVDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(15i32);
pub const IMAPI_MEDIA_TYPE_HDDVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(16i32);
pub const IMAPI_MEDIA_TYPE_HDDVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(14i32);
pub const IMAPI_MEDIA_TYPE_MAX: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
pub const IMAPI_MEDIA_TYPE_UNKNOWN: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_MEDIA_WRITE_PROTECT_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_MODE_PAGE_REQUEST_TYPE(pub i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(1i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(0i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(2i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_MODE_PAGE_TYPE(pub i32);
pub const IMAPI_MODE_PAGE_TYPE_CACHING: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(8i32);
pub const IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(28i32);
pub const IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(42i32);
pub const IMAPI_MODE_PAGE_TYPE_MRW: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(3i32);
pub const IMAPI_MODE_PAGE_TYPE_POWER_CONDITION: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(26i32);
pub const IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(1i32);
pub const IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(29i32);
pub const IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_PROFILE_TYPE(pub i32);
pub const IMAPI_PROFILE_TYPE_AS_MO: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(5i32);
pub const IMAPI_PROFILE_TYPE_BD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(67i32);
pub const IMAPI_PROFILE_TYPE_BD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(64i32);
pub const IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(66i32);
pub const IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65i32);
pub const IMAPI_PROFILE_TYPE_CDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(8i32);
pub const IMAPI_PROFILE_TYPE_CD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(9i32);
pub const IMAPI_PROFILE_TYPE_CD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(10i32);
pub const IMAPI_PROFILE_TYPE_DDCDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(32i32);
pub const IMAPI_PROFILE_TYPE_DDCD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(33i32);
pub const IMAPI_PROFILE_TYPE_DDCD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(34i32);
pub const IMAPI_PROFILE_TYPE_DVDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(16i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(17i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(19i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(20i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(22i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(21i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(27i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(26i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(42i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(43i32);
pub const IMAPI_PROFILE_TYPE_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(18i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(82i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(81i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(80i32);
pub const IMAPI_PROFILE_TYPE_INVALID: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(0i32);
pub const IMAPI_PROFILE_TYPE_MO_ERASABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(3i32);
pub const IMAPI_PROFILE_TYPE_MO_WRITE_ONCE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(4i32);
pub const IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(1i32);
pub const IMAPI_PROFILE_TYPE_NON_STANDARD: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65535i32);
pub const IMAPI_PROFILE_TYPE_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAPI_READ_TRACK_ADDRESS_TYPE(pub i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(0i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(2i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(1i32);
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
pub const IMAPI_S_BUFFER_TO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x40201_u32 as _);
pub const IMAPI_S_PROPERTIESIGNORED: windows_core::HRESULT = windows_core::HRESULT(0x40200_u32 as _);
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(2i32);
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16i32);
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(4i32);
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(8i32);
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16384i32);
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(1i32);
pub const IMMPID_CPV_AFTER__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32769i32);
pub const IMMPID_CPV_BEFORE__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32767i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMMPID_CPV_ENUM(pub i32);
pub const IMMPID_CP_START: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32768i32);
pub const IMMPID_MPV_AFTER__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12294i32);
pub const IMMPID_MPV_BEFORE__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12287i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMMPID_MPV_ENUM(pub i32);
pub const IMMPID_MPV_MESSAGE_CREATION_FLAGS: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12289i32);
pub const IMMPID_MPV_MESSAGE_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12290i32);
pub const IMMPID_MPV_STORE_DRIVER_HANDLE: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12288i32);
pub const IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12293i32);
pub const IMMPID_MPV_TOTAL_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12291i32);
pub const IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12292i32);
pub const IMMPID_MP_AFTER__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4153i32);
pub const IMMPID_MP_ARRIVAL_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4121i32);
pub const IMMPID_MP_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4115i32);
pub const IMMPID_MP_AUTHENTICATED_USER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4104i32);
pub const IMMPID_MP_BEFORE__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4095i32);
pub const IMMPID_MP_BINARYMIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4109i32);
pub const IMMPID_MP_CHUNKING_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4108i32);
pub const IMMPID_MP_CLIENT_AUTH_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4149i32);
pub const IMMPID_MP_CLIENT_AUTH_USER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4148i32);
pub const IMMPID_MP_CONNECTION_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4105i32);
pub const IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4134i32);
pub const IMMPID_MP_CONNECTION_SERVER_PORT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4147i32);
pub const IMMPID_MP_CONTENT_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4097i32);
pub const IMMPID_MP_CONTENT_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4145i32);
pub const IMMPID_MP_CRC_GLOBAL: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4150i32);
pub const IMMPID_MP_CRC_RECIPS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4151i32);
pub const IMMPID_MP_DEFERRED_DELIVERY_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4141i32);
pub const IMMPID_MP_DOMAIN_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4102i32);
pub const IMMPID_MP_DSN_ENVID_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4112i32);
pub const IMMPID_MP_DSN_RET_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4113i32);
pub const IMMPID_MP_EIGHTBIT_MIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4107i32);
pub const IMMPID_MP_ENCRYPTION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4146i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMMPID_MP_ENUM(pub i32);
pub const IMMPID_MP_ERROR_CODE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4111i32);
pub const IMMPID_MP_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4117i32);
pub const IMMPID_MP_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4118i32);
pub const IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4126i32);
pub const IMMPID_MP_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4139i32);
pub const IMMPID_MP_HELO_DOMAIN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4106i32);
pub const IMMPID_MP_HR_CAT_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4122i32);
pub const IMMPID_MP_INBOUND_MAIL_FROM_AUTH: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4152i32);
pub const IMMPID_MP_LOCAL_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4119i32);
pub const IMMPID_MP_LOCAL_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4120i32);
pub const IMMPID_MP_MESSAGE_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4116i32);
pub const IMMPID_MP_MSGCLASS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4144i32);
pub const IMMPID_MP_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4123i32);
pub const IMMPID_MP_MSG_SIZE_HINT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4127i32);
pub const IMMPID_MP_NUM_RECIPIENTS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4137i32);
pub const IMMPID_MP_ORIGINAL_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4143i32);
pub const IMMPID_MP_PICKUP_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4103i32);
pub const IMMPID_MP_RECIPIENT_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4096i32);
pub const IMMPID_MP_REMOTE_AUTHENTICATION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4110i32);
pub const IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4114i32);
pub const IMMPID_MP_RFC822_BCC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4133i32);
pub const IMMPID_MP_RFC822_CC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4132i32);
pub const IMMPID_MP_RFC822_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4130i32);
pub const IMMPID_MP_RFC822_MSG_ID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4128i32);
pub const IMMPID_MP_RFC822_MSG_SUBJECT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4129i32);
pub const IMMPID_MP_RFC822_TO_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4131i32);
pub const IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4125i32);
pub const IMMPID_MP_SENDER_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4140i32);
pub const IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4101i32);
pub const IMMPID_MP_SENDER_ADDRESS_OTHER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4142i32);
pub const IMMPID_MP_SENDER_ADDRESS_SMTP: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4098i32);
pub const IMMPID_MP_SENDER_ADDRESS_X400: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4100i32);
pub const IMMPID_MP_SENDER_ADDRESS_X500: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4099i32);
pub const IMMPID_MP_SERVER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4135i32);
pub const IMMPID_MP_SERVER_VERSION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4136i32);
pub const IMMPID_MP_SUPERSEDES_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4124i32);
pub const IMMPID_MP_X_PRIORITY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4138i32);
pub const IMMPID_NMP_AFTER__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24585i32);
pub const IMMPID_NMP_BEFORE__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24575i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMMPID_NMP_ENUM(pub i32);
pub const IMMPID_NMP_HEADERS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24582i32);
pub const IMMPID_NMP_NEWSGROUP_LIST: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24581i32);
pub const IMMPID_NMP_NNTP_APPROVED_HEADER: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24584i32);
pub const IMMPID_NMP_NNTP_PROCESSING: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24583i32);
pub const IMMPID_NMP_POST_TOKEN: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24580i32);
pub const IMMPID_NMP_PRIMARY_ARTID: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24579i32);
pub const IMMPID_NMP_PRIMARY_GROUP: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24578i32);
pub const IMMPID_NMP_SECONDARY_ARTNUM: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24577i32);
pub const IMMPID_NMP_SECONDARY_GROUPS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24576i32);
pub const IMMPID_RPV_AFTER__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16386i32);
pub const IMMPID_RPV_BEFORE__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16383i32);
pub const IMMPID_RPV_DONT_DELIVER: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16384i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMMPID_RPV_ENUM(pub i32);
pub const IMMPID_RPV_NO_NAME_COLLISIONS: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16385i32);
pub const IMMPID_RP_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8195i32);
pub const IMMPID_RP_ADDRESS_OTHER: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8211i32);
pub const IMMPID_RP_ADDRESS_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8201i32);
pub const IMMPID_RP_ADDRESS_TYPE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8194i32);
pub const IMMPID_RP_ADDRESS_TYPE_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8196i32);
pub const IMMPID_RP_ADDRESS_X400: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8202i32);
pub const IMMPID_RP_ADDRESS_X500: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8203i32);
pub const IMMPID_RP_AFTER__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8213i32);
pub const IMMPID_RP_BEFORE__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8191i32);
pub const IMMPID_RP_DISPLAY_NAME: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8212i32);
pub const IMMPID_RP_DOMAIN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8210i32);
pub const IMMPID_RP_DSN_NOTIFY_INVALID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8193i32);
pub const IMMPID_RP_DSN_NOTIFY_SUCCESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8192i32);
pub const IMMPID_RP_DSN_NOTIFY_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8199i32);
pub const IMMPID_RP_DSN_ORCPT_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8200i32);
pub const IMMPID_RP_DSN_PRE_CAT_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8207i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMMPID_RP_ENUM(pub i32);
pub const IMMPID_RP_ERROR_CODE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8197i32);
pub const IMMPID_RP_ERROR_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8198i32);
pub const IMMPID_RP_LEGACY_EX_DN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8204i32);
pub const IMMPID_RP_MDB_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8208i32);
pub const IMMPID_RP_RECIPIENT_FLAGS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8205i32);
pub const IMMPID_RP_SMTP_STATUS_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8206i32);
pub const IMMPID_RP_USER_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8209i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMMP_MPV_STORE_DRIVER_HANDLE {
    pub guidSignature: windows_core::GUID,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisession, IMultisession_Vtbl, 0x27354150_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisession {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisession, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMultisession {
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupportedOnCurrentMediaState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInUse(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInUse)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn InUse(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InUse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportRecorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisession_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IsSupportedOnCurrentMediaState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetInUse: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub InUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ImportRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisession_Impl: super::super::System::Com::IDispatch_Impl {
    fn IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetInUse(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn InUse(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisession_Vtbl {
    pub const fn new<Identity: IMultisession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::IsSupportedOnCurrentMediaState(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultisession_Impl::SetInUse(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn InUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::InUse(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportRecorder<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::ImportRecorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsSupportedOnCurrentMediaState: IsSupportedOnCurrentMediaState::<Identity, OFFSET>,
            SetInUse: SetInUse::<Identity, OFFSET>,
            InUse: InUse::<Identity, OFFSET>,
            ImportRecorder: ImportRecorder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisession as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisession {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisessionRandomWrite, IMultisessionRandomWrite_Vtbl, 0xb507ca23_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisessionRandomWrite {
    type Target = IMultisession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisessionRandomWrite, windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionRandomWrite {
    pub unsafe fn WriteUnitSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenAddress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionRandomWrite_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionRandomWrite_Impl: IMultisession_Impl {
    fn WriteUnitSize(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddress(&self) -> windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionRandomWrite_Vtbl {
    pub const fn new<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteUnitSize<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::WriteUnitSize(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenAddress<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::LastWrittenAddress(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, OFFSET>(),
            WriteUnitSize: WriteUnitSize::<Identity, OFFSET>,
            LastWrittenAddress: LastWrittenAddress::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionRandomWrite as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisessionRandomWrite {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisessionSequential, IMultisessionSequential_Vtbl, 0x27354151_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisessionSequential {
    type Target = IMultisession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisessionSequential, windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential {
    pub unsafe fn IsFirstDataSession(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NextWritableAddress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub IsFirstDataSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential_Impl: IMultisession_Impl {
    fn IsFirstDataSession(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionSequential_Vtbl {
    pub const fn new<Identity: IMultisessionSequential_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsFirstDataSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::IsFirstDataSession(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::StartAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::LastWrittenAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::NextWritableAddress(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, OFFSET>(),
            IsFirstDataSession: IsFirstDataSession::<Identity, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisessionSequential {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMultisessionSequential2, IMultisessionSequential2_Vtbl, 0xb507ca22_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMultisessionSequential2 {
    type Target = IMultisessionSequential;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMultisessionSequential2, windows_core::IUnknown, super::super::System::Com::IDispatch, IMultisession, IMultisessionSequential);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential2 {
    pub unsafe fn WriteUnitSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential2_Vtbl {
    pub base__: IMultisessionSequential_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential2_Impl: IMultisessionSequential_Impl {
    fn WriteUnitSize(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionSequential2_Vtbl {
    pub const fn new<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteUnitSize<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential2_Impl::WriteUnitSize(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMultisessionSequential_Vtbl::new::<Identity, OFFSET>(), WriteUnitSize: WriteUnitSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID || iid == &<IMultisessionSequential as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisessionSequential2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IProgressItem, IProgressItem_Vtbl, 0x2c941fd5_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IProgressItem {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IProgressItem, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItem {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FirstBlock(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirstBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastBlock(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BlockCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FirstBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LastBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BlockCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItem_Impl: super::super::System::Com::IDispatch_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FirstBlock(&self) -> windows_core::Result<u32>;
    fn LastBlock(&self) -> windows_core::Result<u32>;
    fn BlockCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProgressItem_Vtbl {
    pub const fn new<Identity: IProgressItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::Description(this) {
                    Ok(ok__) => {
                        desc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirstBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::FirstBlock(this) {
                    Ok(ok__) => {
                        block.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::LastBlock(this) {
                    Ok(ok__) => {
                        block.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BlockCount<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::BlockCount(this) {
                    Ok(ok__) => {
                        blocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            FirstBlock: FirstBlock::<Identity, OFFSET>,
            LastBlock: LastBlock::<Identity, OFFSET>,
            BlockCount: BlockCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IProgressItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IProgressItems, IProgressItems_Vtbl, 0x2c941fd7_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IProgressItems {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IProgressItems, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItems {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProgressItemFromBlock(&self, block: u32) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItemFromBlock)(windows_core::Interface::as_raw(self), block, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProgressItemFromDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItemFromDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItems_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProgressItemFromBlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProgressItemFromDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItems_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IProgressItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ProgressItemFromBlock(&self, block: u32) -> windows_core::Result<IProgressItem>;
    fn ProgressItemFromDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<IProgressItem>;
    fn EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProgressItems_Vtbl {
    pub const fn new<Identity: IProgressItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: u32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::ProgressItemFromBlock(this, core::mem::transmute_copy(&block)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressItemFromDescription<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::ProgressItemFromDescription(this, core::mem::transmute(&description)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumProgressItems<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::EnumProgressItems(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Identity, OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Identity, OFFSET>,
            EnumProgressItems: EnumProgressItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItems as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IProgressItems {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRawCDImageCreator, IRawCDImageCreator_Vtbl, 0x25983550_9d65_49ce_b335_40630d901227);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRawCDImageCreator {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRawCDImageCreator, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageCreator {
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddTrack<P1>(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: P1) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddTrack)(windows_core::Interface::as_raw(self), datatype, data.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddSpecialPregap<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSpecialPregap)(windows_core::Interface::as_raw(self), data.param().abi()).ok() }
    }
    pub unsafe fn AddSubcodeRWGenerator<P0>(&self, subcode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSubcodeRWGenerator)(windows_core::Interface::as_raw(self), subcode.param().abi()).ok() }
    }
    pub unsafe fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetResultingImageType)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultingImageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartOfLeadout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartOfLeadoutLimit(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartOfLeadoutLimit)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn StartOfLeadoutLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOfLeadoutLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisableGaplessAudio(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisableGaplessAudio)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn DisableGaplessAudio(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisableGaplessAudio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMediaCatalogNumber(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMediaCatalogNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaCatalogNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStartingTrackNumber(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartingTrackNumber)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn StartingTrackNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartingTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_TrackInfo(&self, trackindex: i32) -> windows_core::Result<IRawCDImageTrackInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TrackInfo)(windows_core::Interface::as_raw(self), trackindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastUsedUserSectorInImage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastUsedUserSectorInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageCreator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTrack: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_SECTOR_TYPE, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddSpecialPregap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSubcodeRWGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub ResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub StartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_TrackInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastUsedUserSectorInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageCreator_Impl: super::super::System::Com::IDispatch_Impl {
    fn CreateResultImage(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn AddTrack(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<i32>;
    fn AddSpecialPregap(&self, data: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn AddSubcodeRWGenerator(&self, subcode: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn StartOfLeadout(&self) -> windows_core::Result<i32>;
    fn SetStartOfLeadoutLimit(&self, value: i32) -> windows_core::Result<()>;
    fn StartOfLeadoutLimit(&self) -> windows_core::Result<i32>;
    fn SetDisableGaplessAudio(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisableGaplessAudio(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMediaCatalogNumber(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStartingTrackNumber(&self, value: i32) -> windows_core::Result<()>;
    fn StartingTrackNumber(&self) -> windows_core::Result<i32>;
    fn get_TrackInfo(&self, trackindex: i32) -> windows_core::Result<IRawCDImageTrackInfo>;
    fn NumberOfExistingTracks(&self) -> windows_core::Result<i32>;
    fn LastUsedUserSectorInImage(&self) -> windows_core::Result<i32>;
    fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawCDImageCreator_Vtbl {
    pub const fn new<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateResultImage<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::CreateResultImage(this) {
                    Ok(ok__) => {
                        resultstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTrack<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut core::ffi::c_void, trackindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::AddTrack(this, core::mem::transmute_copy(&datatype), core::mem::transmute_copy(&data)) {
                    Ok(ok__) => {
                        trackindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSpecialPregap<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::AddSpecialPregap(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subcode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::AddSubcodeRWGenerator(this, core::mem::transmute_copy(&subcode)).into()
            }
        }
        unsafe extern "system" fn SetResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetResultingImageType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::ResultingImageType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartOfLeadout<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::StartOfLeadout(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetStartOfLeadoutLimit(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::StartOfLeadoutLimit(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetDisableGaplessAudio(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::DisableGaplessAudio(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetMediaCatalogNumber(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn MediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::MediaCatalogNumber(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetStartingTrackNumber(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::StartingTrackNumber(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TrackInfo<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackindex: i32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_TrackInfo(this, core::mem::transmute_copy(&trackindex)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::NumberOfExistingTracks(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::LastUsedUserSectorInImage(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::ExpectedTableOfContents(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateResultImage: CreateResultImage::<Identity, OFFSET>,
            AddTrack: AddTrack::<Identity, OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Identity, OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Identity, OFFSET>,
            SetResultingImageType: SetResultingImageType::<Identity, OFFSET>,
            ResultingImageType: ResultingImageType::<Identity, OFFSET>,
            StartOfLeadout: StartOfLeadout::<Identity, OFFSET>,
            SetStartOfLeadoutLimit: SetStartOfLeadoutLimit::<Identity, OFFSET>,
            StartOfLeadoutLimit: StartOfLeadoutLimit::<Identity, OFFSET>,
            SetDisableGaplessAudio: SetDisableGaplessAudio::<Identity, OFFSET>,
            DisableGaplessAudio: DisableGaplessAudio::<Identity, OFFSET>,
            SetMediaCatalogNumber: SetMediaCatalogNumber::<Identity, OFFSET>,
            MediaCatalogNumber: MediaCatalogNumber::<Identity, OFFSET>,
            SetStartingTrackNumber: SetStartingTrackNumber::<Identity, OFFSET>,
            StartingTrackNumber: StartingTrackNumber::<Identity, OFFSET>,
            get_TrackInfo: get_TrackInfo::<Identity, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, OFFSET>,
            LastUsedUserSectorInImage: LastUsedUserSectorInImage::<Identity, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageCreator as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRawCDImageCreator {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRawCDImageTrackInfo, IRawCDImageTrackInfo_Vtbl, 0x25983551_9d65_49ce_b335_40630d901227);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRawCDImageTrackInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRawCDImageTrackInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageTrackInfo {
    pub unsafe fn StartingLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartingLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SectorCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TrackNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ISRC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ISRC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetISRC(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetISRC)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DigitalAudioCopySetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDigitalAudioCopySetting)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn AudioHasPreemphasis(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AudioHasPreemphasis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAudioHasPreemphasis(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAudioHasPreemphasis)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn TrackIndexes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrackIndexes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset).ok() }
    }
    pub unsafe fn ClearTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageTrackInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartingLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT,
    pub ISRC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetISRC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    pub SetDigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    pub AudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TrackIndexes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub AddTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClearTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageTrackInfo_Impl: super::super::System::Com::IDispatch_Impl {
    fn StartingLba(&self) -> windows_core::Result<i32>;
    fn SectorCount(&self) -> windows_core::Result<i32>;
    fn TrackNumber(&self) -> windows_core::Result<i32>;
    fn SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn ISRC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetISRC(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::Result<()>;
    fn AudioHasPreemphasis(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAudioHasPreemphasis(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TrackIndexes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()>;
    fn ClearTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawCDImageTrackInfo_Vtbl {
    pub const fn new<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartingLba<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::StartingLba(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SectorCount<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::SectorCount(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrackNumber<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::TrackNumber(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SectorType<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::SectorType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::ISRC(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::SetISRC(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::DigitalAudioCopySetting(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::SetDigitalAudioCopySetting(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn AudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::AudioHasPreemphasis(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::SetAudioHasPreemphasis(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn TrackIndexes<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::TrackIndexes(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::AddTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
            }
        }
        unsafe extern "system" fn ClearTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::ClearTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartingLba: StartingLba::<Identity, OFFSET>,
            SectorCount: SectorCount::<Identity, OFFSET>,
            TrackNumber: TrackNumber::<Identity, OFFSET>,
            SectorType: SectorType::<Identity, OFFSET>,
            ISRC: ISRC::<Identity, OFFSET>,
            SetISRC: SetISRC::<Identity, OFFSET>,
            DigitalAudioCopySetting: DigitalAudioCopySetting::<Identity, OFFSET>,
            SetDigitalAudioCopySetting: SetDigitalAudioCopySetting::<Identity, OFFSET>,
            AudioHasPreemphasis: AudioHasPreemphasis::<Identity, OFFSET>,
            SetAudioHasPreemphasis: SetAudioHasPreemphasis::<Identity, OFFSET>,
            TrackIndexes: TrackIndexes::<Identity, OFFSET>,
            AddTrackIndex: AddTrackIndex::<Identity, OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageTrackInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRawCDImageTrackInfo {}
windows_core::imp::define_interface!(IRedbookDiscMaster, IRedbookDiscMaster_Vtbl, 0xe3bc42cd_4e5c_11d3_9144_00104ba11c5e);
windows_core::imp::interface_hierarchy!(IRedbookDiscMaster, windows_core::IUnknown);
impl IRedbookDiscMaster {
    pub unsafe fn GetTotalAudioTracks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalAudioTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTotalAudioBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalAudioBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUsedAudioBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUsedAudioBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAvailableAudioTrackBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableAudioTrackBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAudioBlockSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioBlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateAudioTrack(&self, nblocks: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateAudioTrack)(windows_core::Interface::as_raw(self), nblocks).ok() }
    }
    pub unsafe fn AddAudioTrackBlocks(&self, pby: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddAudioTrackBlocks)(windows_core::Interface::as_raw(self), core::mem::transmute(pby.as_ptr()), pby.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn CloseAudioTrack(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseAudioTrack)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRedbookDiscMaster_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTotalAudioTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetTotalAudioBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetUsedAudioBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetAvailableAudioTrackBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetAudioBlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CreateAudioTrack: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddAudioTrackBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32) -> windows_core::HRESULT,
    pub CloseAudioTrack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRedbookDiscMaster_Impl: windows_core::IUnknownImpl {
    fn GetTotalAudioTracks(&self) -> windows_core::Result<i32>;
    fn GetTotalAudioBlocks(&self) -> windows_core::Result<i32>;
    fn GetUsedAudioBlocks(&self) -> windows_core::Result<i32>;
    fn GetAvailableAudioTrackBlocks(&self) -> windows_core::Result<i32>;
    fn GetAudioBlockSize(&self) -> windows_core::Result<i32>;
    fn CreateAudioTrack(&self, nblocks: i32) -> windows_core::Result<()>;
    fn AddAudioTrackBlocks(&self, pby: *const u8, cb: i32) -> windows_core::Result<()>;
    fn CloseAudioTrack(&self) -> windows_core::Result<()>;
}
impl IRedbookDiscMaster_Vtbl {
    pub const fn new<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTotalAudioTracks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pntracks: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRedbookDiscMaster_Impl::GetTotalAudioTracks(this) {
                    Ok(ok__) => {
                        pntracks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRedbookDiscMaster_Impl::GetTotalAudioBlocks(this) {
                    Ok(ok__) => {
                        pnblocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRedbookDiscMaster_Impl::GetUsedAudioBlocks(this) {
                    Ok(ok__) => {
                        pnblocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRedbookDiscMaster_Impl::GetAvailableAudioTrackBlocks(this) {
                    Ok(ok__) => {
                        pnblocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAudioBlockSize<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblockbytes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRedbookDiscMaster_Impl::GetAudioBlockSize(this) {
                    Ok(ok__) => {
                        pnblockbytes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAudioTrack<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nblocks: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRedbookDiscMaster_Impl::CreateAudioTrack(this, core::mem::transmute_copy(&nblocks)).into()
            }
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pby: *const u8, cb: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRedbookDiscMaster_Impl::AddAudioTrackBlocks(this, core::mem::transmute_copy(&pby), core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn CloseAudioTrack<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRedbookDiscMaster_Impl::CloseAudioTrack(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTotalAudioTracks: GetTotalAudioTracks::<Identity, OFFSET>,
            GetTotalAudioBlocks: GetTotalAudioBlocks::<Identity, OFFSET>,
            GetUsedAudioBlocks: GetUsedAudioBlocks::<Identity, OFFSET>,
            GetAvailableAudioTrackBlocks: GetAvailableAudioTrackBlocks::<Identity, OFFSET>,
            GetAudioBlockSize: GetAudioBlockSize::<Identity, OFFSET>,
            CreateAudioTrack: CreateAudioTrack::<Identity, OFFSET>,
            AddAudioTrackBlocks: AddAudioTrackBlocks::<Identity, OFFSET>,
            CloseAudioTrack: CloseAudioTrack::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRedbookDiscMaster as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRedbookDiscMaster {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStreamConcatenate, IStreamConcatenate_Vtbl, 0x27354146_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStreamConcatenate {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStreamConcatenate, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IStreamConcatenate {
    pub unsafe fn Initialize<P0, P1>(&self, stream1: P0, stream2: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), stream1.param().abi(), stream2.param().abi()).ok() }
    }
    pub unsafe fn Initialize2(&self, streams: &[Option<super::super::System::Com::IStream>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Append<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), stream.param().abi()).ok() }
    }
    pub unsafe fn Append2(&self, streams: &[Option<super::super::System::Com::IStream>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Append2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamConcatenate_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Initialize2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamConcatenate_Impl: super::super::System::Com::IStream_Impl {
    fn Initialize(&self, stream1: windows_core::Ref<super::super::System::Com::IStream>, stream2: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Initialize2(&self, streams: *const Option<super::super::System::Com::IStream>, streamcount: u32) -> windows_core::Result<()>;
    fn Append(&self, stream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Append2(&self, streams: *const Option<super::super::System::Com::IStream>, streamcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IStreamConcatenate_Vtbl {
    pub const fn new<Identity: IStreamConcatenate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream1: *mut core::ffi::c_void, stream2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Initialize(this, core::mem::transmute_copy(&stream1), core::mem::transmute_copy(&stream2)).into()
            }
        }
        unsafe extern "system" fn Initialize2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Initialize2(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Append(this, core::mem::transmute_copy(&stream)).into()
            }
        }
        unsafe extern "system" fn Append2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Append2(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Initialize2: Initialize2::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Append2: Append2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamConcatenate as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStreamConcatenate {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStreamInterleave, IStreamInterleave_Vtbl, 0x27354147_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStreamInterleave {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStreamInterleave, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IStreamInterleave {
    pub unsafe fn Initialize(&self, streams: *const Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(streams), interleavesizes, streamcount).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamInterleave_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamInterleave_Impl: super::super::System::Com::IStream_Impl {
    fn Initialize(&self, streams: *const Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IStreamInterleave_Vtbl {
    pub const fn new<Identity: IStreamInterleave_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStreamInterleave_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamInterleave_Impl::Initialize(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&interleavesizes), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        Self { base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamInterleave as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStreamInterleave {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStreamPseudoRandomBased, IStreamPseudoRandomBased_Vtbl, 0x27354145_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStreamPseudoRandomBased {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStreamPseudoRandomBased, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IStreamPseudoRandomBased {
    pub unsafe fn SetSeed(&self, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSeed)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn Seed(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Seed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ExtendedSeed(&self, values: &[u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_ExtendedSeed)(windows_core::Interface::as_raw(self), core::mem::transmute(values.as_ptr()), values.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_ExtendedSeed)(windows_core::Interface::as_raw(self), values as _, ecount as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamPseudoRandomBased_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub SetSeed: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Seed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
    pub get_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamPseudoRandomBased_Impl: super::super::System::Com::IStream_Impl {
    fn SetSeed(&self, value: u32) -> windows_core::Result<()>;
    fn Seed(&self) -> windows_core::Result<u32>;
    fn put_ExtendedSeed(&self, values: *const u32, ecount: u32) -> windows_core::Result<()>;
    fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IStreamPseudoRandomBased_Vtbl {
    pub const fn new<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::SetSeed(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Seed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStreamPseudoRandomBased_Impl::Seed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *const u32, ecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::put_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
            }
        }
        unsafe extern "system" fn get_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::get_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(),
            SetSeed: SetSeed::<Identity, OFFSET>,
            Seed: Seed::<Identity, OFFSET>,
            put_ExtendedSeed: put_ExtendedSeed::<Identity, OFFSET>,
            get_ExtendedSeed: get_ExtendedSeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamPseudoRandomBased as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStreamPseudoRandomBased {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWriteEngine2, IWriteEngine2_Vtbl, 0x27354135_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWriteEngine2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWriteEngine2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2 {
    pub unsafe fn WriteSection<P0>(&self, data: P0, startingblockaddress: i32, numberofblocks: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteSection)(windows_core::Interface::as_raw(self), data.param().abi(), startingblockaddress, numberofblocks).ok() }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDiscRecorder2Ex>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetUseStreamingWrite12(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseStreamingWrite12)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn UseStreamingWrite12(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseStreamingWrite12)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartingSectorsPerSecond)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn StartingSectorsPerSecond(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEndingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEndingSectorsPerSecond)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn EndingSectorsPerSecond(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBytesPerSector(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBytesPerSector)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn BytesPerSector(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BytesPerSector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteInProgress(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteInProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WriteSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub WriteInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2_Impl: super::super::System::Com::IDispatch_Impl {
    fn WriteSection(&self, data: windows_core::Ref<super::super::System::Com::IStream>, startingblockaddress: i32, numberofblocks: i32) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2Ex>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex>;
    fn SetUseStreamingWrite12(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseStreamingWrite12(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStartingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()>;
    fn StartingSectorsPerSecond(&self) -> windows_core::Result<i32>;
    fn SetEndingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()>;
    fn EndingSectorsPerSecond(&self) -> windows_core::Result<i32>;
    fn SetBytesPerSector(&self, value: i32) -> windows_core::Result<()>;
    fn BytesPerSector(&self) -> windows_core::Result<i32>;
    fn WriteInProgress(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteEngine2_Vtbl {
    pub const fn new<Identity: IWriteEngine2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteSection<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::WriteSection(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&startingblockaddress), core::mem::transmute_copy(&numberofblocks)).into()
            }
        }
        unsafe extern "system" fn CancelWrite<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::CancelWrite(this).into()
            }
        }
        unsafe extern "system" fn SetRecorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetUseStreamingWrite12(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn UseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::UseStreamingWrite12(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetStartingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::StartingSectorsPerSecond(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetEndingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::EndingSectorsPerSecond(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetBytesPerSector(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::BytesPerSector(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteInProgress<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::WriteInProgress(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WriteSection: WriteSection::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetUseStreamingWrite12: SetUseStreamingWrite12::<Identity, OFFSET>,
            UseStreamingWrite12: UseStreamingWrite12::<Identity, OFFSET>,
            SetStartingSectorsPerSecond: SetStartingSectorsPerSecond::<Identity, OFFSET>,
            StartingSectorsPerSecond: StartingSectorsPerSecond::<Identity, OFFSET>,
            SetEndingSectorsPerSecond: SetEndingSectorsPerSecond::<Identity, OFFSET>,
            EndingSectorsPerSecond: EndingSectorsPerSecond::<Identity, OFFSET>,
            SetBytesPerSector: SetBytesPerSector::<Identity, OFFSET>,
            BytesPerSector: BytesPerSector::<Identity, OFFSET>,
            WriteInProgress: WriteInProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWriteEngine2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWriteEngine2EventArgs, IWriteEngine2EventArgs_Vtbl, 0x27354136_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWriteEngine2EventArgs {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWriteEngine2EventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2EventArgs {
    pub unsafe fn StartLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SectorCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastReadLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastReadLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSystemBuffer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UsedSystemBuffer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsedSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSystemBuffer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2EventArgs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastReadLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UsedSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2EventArgs_Impl: super::super::System::Com::IDispatch_Impl {
    fn StartLba(&self) -> windows_core::Result<i32>;
    fn SectorCount(&self) -> windows_core::Result<i32>;
    fn LastReadLba(&self) -> windows_core::Result<i32>;
    fn LastWrittenLba(&self) -> windows_core::Result<i32>;
    fn TotalSystemBuffer(&self) -> windows_core::Result<i32>;
    fn UsedSystemBuffer(&self) -> windows_core::Result<i32>;
    fn FreeSystemBuffer(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteEngine2EventArgs_Vtbl {
    pub const fn new<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::StartLba(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SectorCount<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::SectorCount(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastReadLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::LastReadLba(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::LastWrittenLba(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::TotalSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UsedSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::UsedSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::FreeSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartLba: StartLba::<Identity, OFFSET>,
            SectorCount: SectorCount::<Identity, OFFSET>,
            LastReadLba: LastReadLba::<Identity, OFFSET>,
            LastWrittenLba: LastWrittenLba::<Identity, OFFSET>,
            TotalSystemBuffer: TotalSystemBuffer::<Identity, OFFSET>,
            UsedSystemBuffer: UsedSystemBuffer::<Identity, OFFSET>,
            FreeSystemBuffer: FreeSystemBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWriteEngine2EventArgs {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWriteSpeedDescriptor, IWriteSpeedDescriptor_Vtbl, 0x27354144_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWriteSpeedDescriptor {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWriteSpeedDescriptor, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWriteSpeedDescriptor {
    pub unsafe fn MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteSpeedDescriptor_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub RotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub WriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteSpeedDescriptor_Impl: super::super::System::Com::IDispatch_Impl {
    fn MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn RotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn WriteSpeed(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteSpeedDescriptor_Vtbl {
    pub const fn new<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MediaType<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::MediaType(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::RotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteSpeed<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::WriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MediaType: MediaType::<Identity, OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Identity, OFFSET>,
            WriteSpeed: WriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWriteSpeedDescriptor {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMSGSESS(pub isize);
pub const MEDIA_BLANK: MEDIA_FLAGS = MEDIA_FLAGS(1i32);
pub const MEDIA_CDDA_CDROM: MEDIA_TYPES = MEDIA_TYPES(1i32);
pub const MEDIA_CD_EXTRA: MEDIA_TYPES = MEDIA_TYPES(4i32);
pub const MEDIA_CD_I: MEDIA_TYPES = MEDIA_TYPES(3i32);
pub const MEDIA_CD_OTHER: MEDIA_TYPES = MEDIA_TYPES(5i32);
pub const MEDIA_CD_ROM_XA: MEDIA_TYPES = MEDIA_TYPES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEDIA_FLAGS(pub i32);
pub const MEDIA_FORMAT_UNUSABLE_BY_IMAPI: MEDIA_FLAGS = MEDIA_FLAGS(8i32);
pub const MEDIA_RW: MEDIA_FLAGS = MEDIA_FLAGS(2i32);
pub const MEDIA_SPECIAL: MEDIA_TYPES = MEDIA_TYPES(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEDIA_TYPES(pub i32);
pub const MEDIA_WRITABLE: MEDIA_FLAGS = MEDIA_FLAGS(4i32);
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
pub const MPV_WRITE_CONTENT: u32 = 2u32;
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
pub const MP_STATUS_RETRY: u32 = 1u32;
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub const MSDiscMasterObj: windows_core::GUID = windows_core::GUID::from_u128(0x520cca63_51a5_11d3_9144_00104ba11c5e);
pub const MSDiscRecorderObj: windows_core::GUID = windows_core::GUID::from_u128(0x520cca61_51a5_11d3_9144_00104ba11c5e);
pub const MSEnumDiscRecordersObj: windows_core::GUID = windows_core::GUID::from_u128(0x8a03567a_63cb_4ba8_baf6_52119816d1ef);
#[cfg(feature = "Win32_System_AddressBook")]
pub type MSGCALLRELEASE = Option<unsafe extern "system" fn(ulcallerdata: u32, lpmessage: windows_core::Ref<super::super::System::AddressBook::IMessage>)>;
pub const MsftDiscFormat2Data: windows_core::GUID = windows_core::GUID::from_u128(0x2735412a_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2Erase: windows_core::GUID = windows_core::GUID::from_u128(0x2735412b_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2RawCD: windows_core::GUID = windows_core::GUID::from_u128(0x27354128_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2TrackAtOnce: windows_core::GUID = windows_core::GUID::from_u128(0x27354129_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscMaster2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412e_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscRecorder2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412d_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftFileSystemImage: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc5_975b_59be_a960_9a2a262853a5);
pub const MsftIsoImageManager: windows_core::GUID = windows_core::GUID::from_u128(0xceee3b62_8f56_4056_869b_ef16917e3efc);
pub const MsftMultisessionRandomWrite: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca24_2204_11dd_966a_001aa01bbc58);
pub const MsftMultisessionSequential: windows_core::GUID = windows_core::GUID::from_u128(0x27354122_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftRawCDImageCreator: windows_core::GUID = windows_core::GUID::from_u128(0x25983561_9d65_49ce_b335_40630d901227);
pub const MsftStreamConcatenate: windows_core::GUID = windows_core::GUID::from_u128(0x27354125_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamInterleave: windows_core::GUID = windows_core::GUID::from_u128(0x27354124_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamPrng001: windows_core::GUID = windows_core::GUID::from_u128(0x27354126_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamZero: windows_core::GUID = windows_core::GUID::from_u128(0x27354127_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteEngine2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412c_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteSpeedDescriptor: windows_core::GUID = windows_core::GUID::from_u128(0x27354123_7f64_5b0f_8f00_5d77afbe261e);
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
pub const NMP_PROCESS_POST: u32 = 1u32;
pub const PlatformEFI: PlatformId = PlatformId(239i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlatformId(pub i32);
pub const PlatformMac: PlatformId = PlatformId(2i32);
pub const PlatformPowerPC: PlatformId = PlatformId(1i32);
pub const PlatformX86: PlatformId = PlatformId(0i32);
pub const ProgressItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcb_975b_59be_a960_9a2a262853a5);
pub const ProgressItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc9_975b_59be_a960_9a2a262853a5);
pub const RECORDER_BURNING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(2u32);
pub const RECORDER_CDR: RECORDER_TYPES = RECORDER_TYPES(1i32);
pub const RECORDER_CDRW: RECORDER_TYPES = RECORDER_TYPES(2i32);
pub const RECORDER_DOING_NOTHING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(0u32);
pub const RECORDER_OPENED: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECORDER_TYPES(pub i32);
pub const RP_DELIVERED: u32 = 272u32;
pub const RP_DSN_HANDLED: u32 = 64u32;
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
pub const RP_ENPANDED: u32 = 8208u32;
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
pub const RP_EXPANDED: u32 = 8208u32;
pub const RP_FAILED: u32 = 2096u32;
pub const RP_GENERAL_FAILURE: u32 = 32u32;
pub const RP_HANDLED: u32 = 16u32;
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
pub const RP_UNRESOLVED: u32 = 4144u32;
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPropAttrArray {
    pub cValues: u32,
    pub aPropAttr: [u32; 1],
}
impl Default for SPropAttrArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SZ_PROGID_SMTPCAT: windows_core::PCSTR = windows_core::s!("Smtp.Cat");
pub const tagIMMPID_CPV_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0xa2a76b2a_e52d_11d1_aa64_00c04fa35b82);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tagIMMPID_GUIDLIST_ITEM {
    pub pguid: *const windows_core::GUID,
    pub dwStart: u32,
    pub dwLast: u32,
}
impl Default for tagIMMPID_GUIDLIST_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const tagIMMPID_MPV_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0xcbe69706_c9bd_11d1_9ff2_00c04fa37348);
pub const tagIMMPID_MP_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x13384cf0_b3c4_11d1_aa92_00aa006bc80b);
pub const tagIMMPID_NMP_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x7433a9aa_20e2_11d2_94d6_00c04fa379f1);
pub const tagIMMPID_RPV_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x79e82049_d320_11d1_9ff4_00c04fa37348);
pub const tagIMMPID_RP_STRUCT: windows_core::GUID = windows_core::GUID::from_u128(0x79e82048_d320_11d1_9ff4_00c04fa37348);
