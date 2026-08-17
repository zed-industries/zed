pub trait IOplockBreakingHandler_Impl: Sized {
    fn OplockBreaking(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOplockBreakingHandler {}
impl IOplockBreakingHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOplockBreakingHandler_Vtbl
    where
        Identity: IOplockBreakingHandler_Impl,
    {
        unsafe extern "system" fn OplockBreaking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOplockBreakingHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOplockBreakingHandler_Impl::OplockBreaking(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OplockBreaking: OplockBreaking::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOplockBreakingHandler as windows_core::Interface>::IID
    }
}
pub trait IRandomAccessStreamFileAccessMode_Impl: Sized {
    fn GetMode(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IRandomAccessStreamFileAccessMode {}
impl IRandomAccessStreamFileAccessMode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRandomAccessStreamFileAccessMode_Vtbl
    where
        Identity: IRandomAccessStreamFileAccessMode_Impl,
    {
        unsafe extern "system" fn GetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileaccessmode: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStreamFileAccessMode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStreamFileAccessMode_Impl::GetMode(this) {
                Ok(ok__) => {
                    fileaccessmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMode: GetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamFileAccessMode as windows_core::Interface>::IID
    }
}
pub trait IStorageFolderHandleAccess_Impl: Sized {
    fn Create(&self, filename: &windows_core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Option<&IOplockBreakingHandler>) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
}
impl windows_core::RuntimeName for IStorageFolderHandleAccess {}
impl IStorageFolderHandleAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStorageFolderHandleAccess_Vtbl
    where
        Identity: IStorageFolderHandleAccess_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IStorageFolderHandleAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderHandleAccess_Impl::Create(this, core::mem::transmute(&filename), core::mem::transmute_copy(&creationoptions), core::mem::transmute_copy(&accessoptions), core::mem::transmute_copy(&sharingoptions), core::mem::transmute_copy(&options), windows_core::from_raw_borrowed(&oplockbreakinghandler)) {
                Ok(ok__) => {
                    interophandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageFolderHandleAccess as windows_core::Interface>::IID
    }
}
pub trait IStorageItemHandleAccess_Impl: Sized {
    fn Create(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Option<&IOplockBreakingHandler>) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
}
impl windows_core::RuntimeName for IStorageItemHandleAccess {}
impl IStorageItemHandleAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStorageItemHandleAccess_Vtbl
    where
        Identity: IStorageItemHandleAccess_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IStorageItemHandleAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageItemHandleAccess_Impl::Create(this, core::mem::transmute_copy(&accessoptions), core::mem::transmute_copy(&sharingoptions), core::mem::transmute_copy(&options), windows_core::from_raw_borrowed(&oplockbreakinghandler)) {
                Ok(ok__) => {
                    interophandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageItemHandleAccess as windows_core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleOplockCallback_Impl: Sized {
    fn OnBrokenCallback(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUnbufferedFileHandleOplockCallback {}
impl IUnbufferedFileHandleOplockCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUnbufferedFileHandleOplockCallback_Vtbl
    where
        Identity: IUnbufferedFileHandleOplockCallback_Impl,
    {
        unsafe extern "system" fn OnBrokenCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUnbufferedFileHandleOplockCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUnbufferedFileHandleOplockCallback_Impl::OnBrokenCallback(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnBrokenCallback: OnBrokenCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleOplockCallback as windows_core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleProvider_Impl: Sized {
    fn OpenUnbufferedFileHandle(&self, oplockbreakcallback: Option<&IUnbufferedFileHandleOplockCallback>) -> windows_core::Result<usize>;
    fn CloseUnbufferedFileHandle(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUnbufferedFileHandleProvider {}
impl IUnbufferedFileHandleProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUnbufferedFileHandleProvider_Vtbl
    where
        Identity: IUnbufferedFileHandleProvider_Impl,
    {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oplockbreakcallback: *mut core::ffi::c_void, filehandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: IUnbufferedFileHandleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUnbufferedFileHandleProvider_Impl::OpenUnbufferedFileHandle(this, windows_core::from_raw_borrowed(&oplockbreakcallback)) {
                Ok(ok__) => {
                    filehandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUnbufferedFileHandleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUnbufferedFileHandleProvider_Impl::CloseUnbufferedFileHandle(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenUnbufferedFileHandle: OpenUnbufferedFileHandle::<Identity, OFFSET>,
            CloseUnbufferedFileHandle: CloseUnbufferedFileHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleProvider as windows_core::Interface>::IID
    }
}
