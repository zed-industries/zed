pub const CCF_SCESVC_ATTACHMENT: windows_core::PCWSTR = windows_core::w!("CCF_SCESVC_ATTACHMENT");
pub const CCF_SCESVC_ATTACHMENT_DATA: windows_core::PCWSTR = windows_core::w!("CCF_SCESVC_ATTACHMENT_DATA");
windows_core::imp::define_interface!(ISceSvcAttachmentData, ISceSvcAttachmentData_Vtbl, 0x17c35fde_200d_11d1_affb_00c04fb984f9);
windows_core::imp::interface_hierarchy!(ISceSvcAttachmentData, windows_core::IUnknown);
impl ISceSvcAttachmentData {
    pub unsafe fn GetData(&self, scesvchandle: *mut core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut core::ffi::c_void, psceenumhandle: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), scesvchandle as _, scetype, ppvdata as _, psceenumhandle as _).ok() }
    }
    pub unsafe fn Initialize<P2>(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: P2, pscesvchandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ISceSvcAttachmentPersistInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lpservicename as _, lptemplatename as _, lpscesvcpersistinfo.param().abi(), pscesvchandle as _).ok() }
    }
    pub unsafe fn FreeBuffer(&self, pvdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pvdata as _).ok() }
    }
    pub unsafe fn CloseHandle(&self, scesvchandle: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseHandle)(windows_core::Interface::as_raw(self), scesvchandle as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SCESVC_INFO_TYPE, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i8, *mut i8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISceSvcAttachmentData_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, scesvchandle: *mut core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut core::ffi::c_void, psceenumhandle: *mut u32) -> windows_core::Result<()>;
    fn Initialize(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: windows_core::Ref<ISceSvcAttachmentPersistInfo>, pscesvchandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeBuffer(&self, pvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CloseHandle(&self, scesvchandle: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ISceSvcAttachmentData_Vtbl {
    pub const fn new<Identity: ISceSvcAttachmentData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scesvchandle: *mut core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut core::ffi::c_void, psceenumhandle: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISceSvcAttachmentData_Impl::GetData(this, core::mem::transmute_copy(&scesvchandle), core::mem::transmute_copy(&scetype), core::mem::transmute_copy(&ppvdata), core::mem::transmute_copy(&psceenumhandle)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: *mut core::ffi::c_void, pscesvchandle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISceSvcAttachmentData_Impl::Initialize(this, core::mem::transmute_copy(&lpservicename), core::mem::transmute_copy(&lptemplatename), core::mem::transmute_copy(&lpscesvcpersistinfo), core::mem::transmute_copy(&pscesvchandle)).into()
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISceSvcAttachmentData_Impl::FreeBuffer(this, core::mem::transmute_copy(&pvdata)).into()
            }
        }
        unsafe extern "system" fn CloseHandle<Identity: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scesvchandle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISceSvcAttachmentData_Impl::CloseHandle(this, core::mem::transmute_copy(&scesvchandle)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            CloseHandle: CloseHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISceSvcAttachmentData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISceSvcAttachmentData {}
windows_core::imp::define_interface!(ISceSvcAttachmentPersistInfo, ISceSvcAttachmentPersistInfo_Vtbl, 0x6d90e0d0_200d_11d1_affb_00c04fb984f9);
windows_core::imp::interface_hierarchy!(ISceSvcAttachmentPersistInfo, windows_core::IUnknown);
impl ISceSvcAttachmentPersistInfo {
    pub unsafe fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut core::ffi::c_void, ppvdata: *mut *mut core::ffi::c_void, pboverwriteall: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), lptemplatename as _, scesvchandle as _, ppvdata as _, pboverwriteall as _).ok() }
    }
    pub unsafe fn IsDirty(&self, lptemplatename: *mut i8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self), lptemplatename as _) }
    }
    pub unsafe fn FreeBuffer(&self, pvdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pvdata as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentPersistInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i8, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i8) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISceSvcAttachmentPersistInfo_Impl: windows_core::IUnknownImpl {
    fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut core::ffi::c_void, ppvdata: *mut *mut core::ffi::c_void, pboverwriteall: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsDirty(&self, lptemplatename: *mut i8) -> windows_core::HRESULT;
    fn FreeBuffer(&self, pvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ISceSvcAttachmentPersistInfo_Vtbl {
    pub const fn new<Identity: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Save<Identity: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut core::ffi::c_void, ppvdata: *mut *mut core::ffi::c_void, pboverwriteall: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISceSvcAttachmentPersistInfo_Impl::Save(this, core::mem::transmute_copy(&lptemplatename), core::mem::transmute_copy(&scesvchandle), core::mem::transmute_copy(&ppvdata), core::mem::transmute_copy(&pboverwriteall)).into()
            }
        }
        unsafe extern "system" fn IsDirty<Identity: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lptemplatename: *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISceSvcAttachmentPersistInfo_Impl::IsDirty(this, core::mem::transmute_copy(&lptemplatename))
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISceSvcAttachmentPersistInfo_Impl::FreeBuffer(this, core::mem::transmute_copy(&pvdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Save: Save::<Identity, OFFSET>,
            IsDirty: IsDirty::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISceSvcAttachmentPersistInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISceSvcAttachmentPersistInfo {}
pub type PFSCE_FREE_INFO = Option<unsafe extern "system" fn(pvserviceinfo: *mut core::ffi::c_void) -> u32>;
pub type PFSCE_LOG_INFO = Option<unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32>;
pub type PFSCE_QUERY_INFO = Option<unsafe extern "system" fn(scehandle: *mut core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: windows_core::BOOL, ppvinfo: *mut *mut core::ffi::c_void, psceenumhandle: *mut u32) -> u32>;
pub type PFSCE_SET_INFO = Option<unsafe extern "system" fn(scehandle: *mut core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: windows_core::BOOL, pvinfo: *mut core::ffi::c_void) -> u32>;
pub type PF_ConfigAnalyzeService = Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO) -> u32>;
pub type PF_UpdateService = Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32>;
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
pub const SCESTATUS_SUCCESS: i32 = 0i32;
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl Default for SCESVC_ANALYSIS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl Default for SCESVC_ANALYSIS_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut core::ffi::c_void,
    pub pfQueryInfo: PFSCE_QUERY_INFO,
    pub pfSetInfo: PFSCE_SET_INFO,
    pub pfFreeInfo: PFSCE_FREE_INFO,
    pub pfLogInfo: PFSCE_LOG_INFO,
}
impl Default for SCESVC_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl Default for SCESVC_CONFIGURATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl Default for SCESVC_CONFIGURATION_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCESVC_INFO_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCE_LOG_ERR_LEVEL(pub i32);
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(0i32);
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(3i32);
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(2i32);
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(1i32);
pub const SCE_ROOT_PATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\SeCEdit");
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(2i32);
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(0i32);
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(3i32);
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(1i32);
pub const cNodetypeSceAnalysisServices: windows_core::GUID = windows_core::GUID::from_u128(0x678050c7_1ff8_11d1_affb_00c04fb984f9);
pub const cNodetypeSceEventLog: windows_core::GUID = windows_core::GUID::from_u128(0x2ce06698_4bf3_11d1_8c30_00c04fb984f9);
pub const cNodetypeSceTemplateServices: windows_core::GUID = windows_core::GUID::from_u128(0x24a7f717_1f0c_11d1_affb_00c04fb984f9);
pub const lstruuidNodetypeSceAnalysisServices: windows_core::PCWSTR = windows_core::w!("{678050c7-1ff8-11d1-affb-00c04fb984f9}");
pub const lstruuidNodetypeSceEventLog: windows_core::PCWSTR = windows_core::w!("{2ce06698-4bf3-11d1-8c30-00c04fb984f9}");
pub const lstruuidNodetypeSceTemplateServices: windows_core::PCWSTR = windows_core::w!("{24a7f717-1f0c-11d1-affb-00c04fb984f9}");
pub const struuidNodetypeSceAnalysisServices: windows_core::PCSTR = windows_core::s!("{678050c7-1ff8-11d1-affb-00c04fb984f9}");
pub const struuidNodetypeSceEventLog: windows_core::PCSTR = windows_core::s!("{2ce06698-4bf3-11d1-8c30-00c04fb984f9}");
pub const struuidNodetypeSceTemplateServices: windows_core::PCSTR = windows_core::s!("{24a7f717-1f0c-11d1-affb-00c04fb984f9}");
