pub trait ISceSvcAttachmentData_Impl: Sized {
    fn GetData(&self, scesvchandle: *mut core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut core::ffi::c_void, psceenumhandle: *mut u32) -> windows_core::Result<()>;
    fn Initialize(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: Option<&ISceSvcAttachmentPersistInfo>, pscesvchandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeBuffer(&self, pvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CloseHandle(&self, scesvchandle: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISceSvcAttachmentData {}
impl ISceSvcAttachmentData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISceSvcAttachmentData_Vtbl
    where
        Identity: ISceSvcAttachmentData_Impl,
    {
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scesvchandle: *mut core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut core::ffi::c_void, psceenumhandle: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISceSvcAttachmentData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISceSvcAttachmentData_Impl::GetData(this, core::mem::transmute_copy(&scesvchandle), core::mem::transmute_copy(&scetype), core::mem::transmute_copy(&ppvdata), core::mem::transmute_copy(&psceenumhandle)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: *mut core::ffi::c_void, pscesvchandle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISceSvcAttachmentData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISceSvcAttachmentData_Impl::Initialize(this, core::mem::transmute_copy(&lpservicename), core::mem::transmute_copy(&lptemplatename), windows_core::from_raw_borrowed(&lpscesvcpersistinfo), core::mem::transmute_copy(&pscesvchandle)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISceSvcAttachmentData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISceSvcAttachmentData_Impl::FreeBuffer(this, core::mem::transmute_copy(&pvdata)).into()
        }
        unsafe extern "system" fn CloseHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scesvchandle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISceSvcAttachmentData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISceSvcAttachmentData_Impl::CloseHandle(this, core::mem::transmute_copy(&scesvchandle)).into()
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
pub trait ISceSvcAttachmentPersistInfo_Impl: Sized {
    fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut core::ffi::c_void, ppvdata: *mut *mut core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsDirty(&self, lptemplatename: *mut i8) -> windows_core::HRESULT;
    fn FreeBuffer(&self, pvdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISceSvcAttachmentPersistInfo {}
impl ISceSvcAttachmentPersistInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISceSvcAttachmentPersistInfo_Vtbl
    where
        Identity: ISceSvcAttachmentPersistInfo_Impl,
    {
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut core::ffi::c_void, ppvdata: *mut *mut core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISceSvcAttachmentPersistInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISceSvcAttachmentPersistInfo_Impl::Save(this, core::mem::transmute_copy(&lptemplatename), core::mem::transmute_copy(&scesvchandle), core::mem::transmute_copy(&ppvdata), core::mem::transmute_copy(&pboverwriteall)).into()
        }
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lptemplatename: *mut i8) -> windows_core::HRESULT
        where
            Identity: ISceSvcAttachmentPersistInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISceSvcAttachmentPersistInfo_Impl::IsDirty(this, core::mem::transmute_copy(&lptemplatename))
        }
        unsafe extern "system" fn FreeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISceSvcAttachmentPersistInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISceSvcAttachmentPersistInfo_Impl::FreeBuffer(this, core::mem::transmute_copy(&pvdata)).into()
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
