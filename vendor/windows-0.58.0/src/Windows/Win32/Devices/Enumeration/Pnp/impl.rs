pub trait IUPnPAddressFamilyControl_Impl: Sized {
    fn SetAddressFamily(&self, dwflags: i32) -> windows_core::Result<()>;
    fn GetAddressFamily(&self) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IUPnPAddressFamilyControl {}
impl IUPnPAddressFamilyControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPAddressFamilyControl_Vtbl
    where
        Identity: IUPnPAddressFamilyControl_Impl,
    {
        unsafe extern "system" fn SetAddressFamily<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT
        where
            Identity: IUPnPAddressFamilyControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPAddressFamilyControl_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetAddressFamily<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUPnPAddressFamilyControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPAddressFamilyControl_Impl::GetAddressFamily(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            GetAddressFamily: GetAddressFamily::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPAddressFamilyControl as windows_core::Interface>::IID
    }
}
pub trait IUPnPAsyncResult_Impl: Sized {
    fn AsyncOperationComplete(&self, ullrequestid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPAsyncResult {}
impl IUPnPAsyncResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPAsyncResult_Vtbl
    where
        Identity: IUPnPAsyncResult_Impl,
    {
        unsafe extern "system" fn AsyncOperationComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64) -> windows_core::HRESULT
        where
            Identity: IUPnPAsyncResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPAsyncResult_Impl::AsyncOperationComplete(this, core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPAsyncResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDescriptionDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn ReadyState(&self) -> windows_core::Result<i32>;
    fn Load(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoadAsync(&self, bstrurl: &windows_core::BSTR, punkcallback: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn LoadResult(&self) -> windows_core::Result<i32>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn RootDevice(&self) -> windows_core::Result<IUPnPDevice>;
    fn DeviceByUDN(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDescriptionDocument {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDescriptionDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDescriptionDocument_Vtbl
    where
        Identity: IUPnPDescriptionDocument_Impl,
    {
        unsafe extern "system" fn ReadyState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plreadystate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDescriptionDocument_Impl::ReadyState(this) {
                Ok(ok__) => {
                    plreadystate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDescriptionDocument_Impl::Load(this, core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn LoadAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, punkcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDescriptionDocument_Impl::LoadAsync(this, core::mem::transmute(&bstrurl), windows_core::from_raw_borrowed(&punkcallback)).into()
        }
        unsafe extern "system" fn LoadResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerror: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDescriptionDocument_Impl::LoadResult(this) {
                Ok(ok__) => {
                    phrerror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDescriptionDocument_Impl::Abort(this).into()
        }
        unsafe extern "system" fn RootDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppudrootdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDescriptionDocument_Impl::RootDevice(this) {
                Ok(ok__) => {
                    ppudrootdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceByUDN<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: core::mem::MaybeUninit<windows_core::BSTR>, ppuddevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDescriptionDocument_Impl::DeviceByUDN(this, core::mem::transmute(&bstrudn)) {
                Ok(ok__) => {
                    ppuddevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ReadyState: ReadyState::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            LoadAsync: LoadAsync::<Identity, OFFSET>,
            LoadResult: LoadResult::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            RootDevice: RootDevice::<Identity, OFFSET>,
            DeviceByUDN: DeviceByUDN::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocument as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IUPnPDescriptionDocumentCallback_Impl: Sized {
    fn LoadComplete(&self, hrloadresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPDescriptionDocumentCallback {}
impl IUPnPDescriptionDocumentCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDescriptionDocumentCallback_Vtbl
    where
        Identity: IUPnPDescriptionDocumentCallback_Impl,
    {
        unsafe extern "system" fn LoadComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrloadresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IUPnPDescriptionDocumentCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDescriptionDocumentCallback_Impl::LoadComplete(this, core::mem::transmute_copy(&hrloadresult)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadComplete: LoadComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocumentCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDevice_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn IsRootDevice(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn RootDevice(&self) -> windows_core::Result<IUPnPDevice>;
    fn ParentDevice(&self) -> windows_core::Result<IUPnPDevice>;
    fn HasChildren(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Children(&self) -> windows_core::Result<IUPnPDevices>;
    fn UniqueDeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PresentationURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ManufacturerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ManufacturerURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModelName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModelNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModelURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UPC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SerialNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IconURL(&self, bstrencodingformat: &windows_core::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Services(&self) -> windows_core::Result<IUPnPServices>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDevice {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDevice_Vtbl
    where
        Identity: IUPnPDevice_Impl,
    {
        unsafe extern "system" fn IsRootDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::IsRootDevice(this) {
                Ok(ok__) => {
                    pvarb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppudrootdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::RootDevice(this) {
                Ok(ok__) => {
                    ppudrootdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuddeviceparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::ParentDevice(this) {
                Ok(ok__) => {
                    ppuddeviceparent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildren<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::HasChildren(this) {
                Ok(ok__) => {
                    pvarb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppudchildren: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::Children(this) {
                Ok(ok__) => {
                    ppudchildren.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueDeviceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::UniqueDeviceName(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::Type(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::PresentationURL(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::ManufacturerName(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::ManufacturerURL(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::ModelName(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::ModelNumber(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::Description(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::ModelURL(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UPC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::UPC(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::SerialNumber(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrencodingformat: core::mem::MaybeUninit<windows_core::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::IconURL(this, core::mem::transmute(&bstrencodingformat), core::mem::transmute_copy(&lsizex), core::mem::transmute_copy(&lsizey), core::mem::transmute_copy(&lbitdepth)) {
                Ok(ok__) => {
                    pbstriconurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppusservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevice_Impl::Services(this) {
                Ok(ok__) => {
                    ppusservices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsRootDevice: IsRootDevice::<Identity, OFFSET>,
            RootDevice: RootDevice::<Identity, OFFSET>,
            ParentDevice: ParentDevice::<Identity, OFFSET>,
            HasChildren: HasChildren::<Identity, OFFSET>,
            Children: Children::<Identity, OFFSET>,
            UniqueDeviceName: UniqueDeviceName::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            PresentationURL: PresentationURL::<Identity, OFFSET>,
            ManufacturerName: ManufacturerName::<Identity, OFFSET>,
            ManufacturerURL: ManufacturerURL::<Identity, OFFSET>,
            ModelName: ModelName::<Identity, OFFSET>,
            ModelNumber: ModelNumber::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            ModelURL: ModelURL::<Identity, OFFSET>,
            UPC: UPC::<Identity, OFFSET>,
            SerialNumber: SerialNumber::<Identity, OFFSET>,
            IconURL: IconURL::<Identity, OFFSET>,
            Services: Services::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDevice as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceControl_Impl: Sized {
    fn Initialize(&self, bstrxmldesc: &windows_core::BSTR, bstrdeviceidentifier: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetServiceObject(&self, bstrudn: &windows_core::BSTR, bstrserviceid: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDeviceControl {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceControl_Vtbl
    where
        Identity: IUPnPDeviceControl_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxmldesc: core::mem::MaybeUninit<windows_core::BSTR>, bstrdeviceidentifier: core::mem::MaybeUninit<windows_core::BSTR>, bstrinitstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceControl_Impl::Initialize(this, core::mem::transmute(&bstrxmldesc), core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrinitstring)).into()
        }
        unsafe extern "system" fn GetServiceObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: core::mem::MaybeUninit<windows_core::BSTR>, bstrserviceid: core::mem::MaybeUninit<windows_core::BSTR>, ppdispservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDeviceControl_Impl::GetServiceObject(this, core::mem::transmute(&bstrudn), core::mem::transmute(&bstrserviceid)) {
                Ok(ok__) => {
                    ppdispservice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetServiceObject: GetServiceObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceControl as windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceControlHttpHeaders_Impl: Sized {
    fn GetAdditionalResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IUPnPDeviceControlHttpHeaders {}
impl IUPnPDeviceControlHttpHeaders_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceControlHttpHeaders_Vtbl
    where
        Identity: IUPnPDeviceControlHttpHeaders_Impl,
    {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhttpresponseheaders: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceControlHttpHeaders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDeviceControlHttpHeaders_Impl::GetAdditionalResponseHeaders(this) {
                Ok(ok__) => {
                    bstrhttpresponseheaders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAdditionalResponseHeaders: GetAdditionalResponseHeaders::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceControlHttpHeaders as windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IUPnPDeviceDocumentAccess {}
impl IUPnPDeviceDocumentAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccess_Vtbl
    where
        Identity: IUPnPDeviceDocumentAccess_Impl,
    {
        unsafe extern "system" fn GetDocumentURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocument: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceDocumentAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDeviceDocumentAccess_Impl::GetDocumentURL(this) {
                Ok(ok__) => {
                    pbstrdocument.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDocumentURL: GetDocumentURL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccess as windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceDocumentAccessEx_Impl: Sized {
    fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IUPnPDeviceDocumentAccessEx {}
impl IUPnPDeviceDocumentAccessEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceDocumentAccessEx_Vtbl
    where
        Identity: IUPnPDeviceDocumentAccessEx_Impl,
    {
        unsafe extern "system" fn GetDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocument: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceDocumentAccessEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDeviceDocumentAccessEx_Impl::GetDocument(this) {
                Ok(ok__) => {
                    pbstrdocument.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDocument: GetDocument::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccessEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinder_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn FindByType(&self, bstrtypeuri: &windows_core::BSTR, dwflags: u32) -> windows_core::Result<IUPnPDevices>;
    fn CreateAsyncFind(&self, bstrtypeuri: &windows_core::BSTR, dwflags: u32, punkdevicefindercallback: Option<&windows_core::IUnknown>) -> windows_core::Result<i32>;
    fn StartAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()>;
    fn CancelAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()>;
    fn FindByUDN(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDeviceFinder {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceFinder_Vtbl
    where
        Identity: IUPnPDeviceFinder_Impl,
    {
        unsafe extern "system" fn FindByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtypeuri: core::mem::MaybeUninit<windows_core::BSTR>, dwflags: u32, pdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDeviceFinder_Impl::FindByType(this, core::mem::transmute(&bstrtypeuri), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pdevices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncFind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtypeuri: core::mem::MaybeUninit<windows_core::BSTR>, dwflags: u32, punkdevicefindercallback: *mut core::ffi::c_void, plfinddata: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDeviceFinder_Impl::CreateAsyncFind(this, core::mem::transmute(&bstrtypeuri), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&punkdevicefindercallback)) {
                Ok(ok__) => {
                    plfinddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsyncFind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceFinder_Impl::StartAsyncFind(this, core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn CancelAsyncFind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceFinder_Impl::CancelAsyncFind(this, core::mem::transmute_copy(&lfinddata)).into()
        }
        unsafe extern "system" fn FindByUDN<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: core::mem::MaybeUninit<windows_core::BSTR>, pdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDeviceFinder_Impl::FindByUDN(this, core::mem::transmute(&bstrudn)) {
                Ok(ok__) => {
                    pdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FindByType: FindByType::<Identity, OFFSET>,
            CreateAsyncFind: CreateAsyncFind::<Identity, OFFSET>,
            StartAsyncFind: StartAsyncFind::<Identity, OFFSET>,
            CancelAsyncFind: CancelAsyncFind::<Identity, OFFSET>,
            FindByUDN: FindByUDN::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinder as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderAddCallbackWithInterface_Impl: Sized {
    fn DeviceAddedWithInterface(&self, lfinddata: i32, pdevice: Option<&IUPnPDevice>, pguidinterface: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDeviceFinderAddCallbackWithInterface {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceFinderAddCallbackWithInterface_Vtbl
    where
        Identity: IUPnPDeviceFinderAddCallbackWithInterface_Impl,
    {
        unsafe extern "system" fn DeviceAddedWithInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32, pdevice: *mut core::ffi::c_void, pguidinterface: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinderAddCallbackWithInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceFinderAddCallbackWithInterface_Impl::DeviceAddedWithInterface(this, core::mem::transmute_copy(&lfinddata), windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pguidinterface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeviceAddedWithInterface: DeviceAddedWithInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderAddCallbackWithInterface as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderCallback_Impl: Sized {
    fn DeviceAdded(&self, lfinddata: i32, pdevice: Option<&IUPnPDevice>) -> windows_core::Result<()>;
    fn DeviceRemoved(&self, lfinddata: i32, bstrudn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SearchComplete(&self, lfinddata: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDeviceFinderCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceFinderCallback_Vtbl
    where
        Identity: IUPnPDeviceFinderCallback_Impl,
    {
        unsafe extern "system" fn DeviceAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32, pdevice: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinderCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceFinderCallback_Impl::DeviceAdded(this, core::mem::transmute_copy(&lfinddata), windows_core::from_raw_borrowed(&pdevice)).into()
        }
        unsafe extern "system" fn DeviceRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32, bstrudn: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinderCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceFinderCallback_Impl::DeviceRemoved(this, core::mem::transmute_copy(&lfinddata), core::mem::transmute(&bstrudn)).into()
        }
        unsafe extern "system" fn SearchComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceFinderCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceFinderCallback_Impl::SearchComplete(this, core::mem::transmute_copy(&lfinddata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceAdded: DeviceAdded::<Identity, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, OFFSET>,
            SearchComplete: SearchComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderCallback as windows_core::Interface>::IID
    }
}
pub trait IUPnPDeviceProvider_Impl: Sized {
    fn Start(&self, bstrinitstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPDeviceProvider {}
impl IUPnPDeviceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDeviceProvider_Vtbl
    where
        Identity: IUPnPDeviceProvider_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinitstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceProvider_Impl::Start(this, core::mem::transmute(&bstrinitstring)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPDeviceProvider_Impl::Stop(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Start: Start::<Identity, OFFSET>, Stop: Stop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDevices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDevices {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPDevices_Vtbl
    where
        Identity: IUPnPDevices_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUPnPDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevices_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevices_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: core::mem::MaybeUninit<windows_core::BSTR>, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPDevices_Impl::get_Item(this, core::mem::transmute(&bstrudn)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDevices as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IUPnPEventSink_Impl: Sized {
    fn OnStateChanged(&self, cchanges: u32, rgdispidchanges: *const i32) -> windows_core::Result<()>;
    fn OnStateChangedSafe(&self, varsadispidchanges: &windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPEventSink {}
impl IUPnPEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPEventSink_Vtbl
    where
        Identity: IUPnPEventSink_Impl,
    {
        unsafe extern "system" fn OnStateChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> windows_core::HRESULT
        where
            Identity: IUPnPEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPEventSink_Impl::OnStateChanged(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&rgdispidchanges)).into()
        }
        unsafe extern "system" fn OnStateChangedSafe<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsadispidchanges: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUPnPEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPEventSink_Impl::OnStateChangedSafe(this, core::mem::transmute(&varsadispidchanges)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnStateChangedSafe: OnStateChangedSafe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPEventSink as windows_core::Interface>::IID
    }
}
pub trait IUPnPEventSource_Impl: Sized {
    fn Advise(&self, pessubscriber: Option<&IUPnPEventSink>) -> windows_core::Result<()>;
    fn Unadvise(&self, pessubscriber: Option<&IUPnPEventSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPEventSource {}
impl IUPnPEventSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPEventSource_Vtbl
    where
        Identity: IUPnPEventSource_Impl,
    {
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pessubscriber: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPEventSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPEventSource_Impl::Advise(this, windows_core::from_raw_borrowed(&pessubscriber)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pessubscriber: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPEventSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPEventSource_Impl::Unadvise(this, windows_core::from_raw_borrowed(&pessubscriber)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Advise: Advise::<Identity, OFFSET>, Unadvise: Unadvise::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPEventSource as windows_core::Interface>::IID
    }
}
pub trait IUPnPHttpHeaderControl_Impl: Sized {
    fn AddRequestHeaders(&self, bstrhttpheaders: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPHttpHeaderControl {}
impl IUPnPHttpHeaderControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPHttpHeaderControl_Vtbl
    where
        Identity: IUPnPHttpHeaderControl_Impl,
    {
        unsafe extern "system" fn AddRequestHeaders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhttpheaders: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPHttpHeaderControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPHttpHeaderControl_Impl::AddRequestHeaders(this, core::mem::transmute(&bstrhttpheaders)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddRequestHeaders: AddRequestHeaders::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPHttpHeaderControl as windows_core::Interface>::IID
    }
}
pub trait IUPnPRegistrar_Impl: Sized {
    fn RegisterDevice(&self, bstrxmldesc: &windows_core::BSTR, bstrprogiddevicecontrolclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterRunningDevice(&self, bstrxmldesc: &windows_core::BSTR, punkdevicecontrol: Option<&windows_core::IUnknown>, bstrinitstring: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterDeviceProvider(&self, bstrprovidername: &windows_core::BSTR, bstrprogidproviderclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetUniqueDeviceName(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrtemplateudn: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn UnregisterDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, fpermanent: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UnregisterDeviceProvider(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPRegistrar {}
impl IUPnPRegistrar_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPRegistrar_Vtbl
    where
        Identity: IUPnPRegistrar_Impl,
    {
        unsafe extern "system" fn RegisterDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxmldesc: core::mem::MaybeUninit<windows_core::BSTR>, bstrprogiddevicecontrolclass: core::mem::MaybeUninit<windows_core::BSTR>, bstrinitstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrcontainerid: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcepath: core::mem::MaybeUninit<windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPRegistrar_Impl::RegisterDevice(this, core::mem::transmute(&bstrxmldesc), core::mem::transmute(&bstrprogiddevicecontrolclass), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrcontainerid), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)) {
                Ok(ok__) => {
                    pbstrdeviceidentifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRunningDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxmldesc: core::mem::MaybeUninit<windows_core::BSTR>, punkdevicecontrol: *mut core::ffi::c_void, bstrinitstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcepath: core::mem::MaybeUninit<windows_core::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPRegistrar_Impl::RegisterRunningDevice(this, core::mem::transmute(&bstrxmldesc), windows_core::from_raw_borrowed(&punkdevicecontrol), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)) {
                Ok(ok__) => {
                    pbstrdeviceidentifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, bstrprogidproviderclass: core::mem::MaybeUninit<windows_core::BSTR>, bstrinitstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrcontainerid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPRegistrar_Impl::RegisterDeviceProvider(this, core::mem::transmute(&bstrprovidername), core::mem::transmute(&bstrprogidproviderclass), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrcontainerid)).into()
        }
        unsafe extern "system" fn GetUniqueDeviceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: core::mem::MaybeUninit<windows_core::BSTR>, bstrtemplateudn: core::mem::MaybeUninit<windows_core::BSTR>, pbstrudn: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPRegistrar_Impl::GetUniqueDeviceName(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrtemplateudn)) {
                Ok(ok__) => {
                    pbstrudn.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: core::mem::MaybeUninit<windows_core::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IUPnPRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPRegistrar_Impl::UnregisterDevice(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute_copy(&fpermanent)).into()
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPRegistrar_Impl::UnregisterDeviceProvider(this, core::mem::transmute(&bstrprovidername)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, OFFSET>,
            RegisterRunningDevice: RegisterRunningDevice::<Identity, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, OFFSET>,
            GetUniqueDeviceName: GetUniqueDeviceName::<Identity, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPRegistrar as windows_core::Interface>::IID
    }
}
pub trait IUPnPRemoteEndpointInfo_Impl: Sized {
    fn GetDwordValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<u32>;
    fn GetStringValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetGuidValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IUPnPRemoteEndpointInfo {}
impl IUPnPRemoteEndpointInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPRemoteEndpointInfo_Vtbl
    where
        Identity: IUPnPRemoteEndpointInfo_Impl,
    {
        unsafe extern "system" fn GetDwordValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluename: core::mem::MaybeUninit<windows_core::BSTR>, pdwvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUPnPRemoteEndpointInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPRemoteEndpointInfo_Impl::GetDwordValue(this, core::mem::transmute(&bstrvaluename)) {
                Ok(ok__) => {
                    pdwvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluename: core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPRemoteEndpointInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPRemoteEndpointInfo_Impl::GetStringValue(this, core::mem::transmute(&bstrvaluename)) {
                Ok(ok__) => {
                    pbstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuidValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluename: core::mem::MaybeUninit<windows_core::BSTR>, pguidvalue: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IUPnPRemoteEndpointInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPRemoteEndpointInfo_Impl::GetGuidValue(this, core::mem::transmute(&bstrvaluename)) {
                Ok(ok__) => {
                    pguidvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDwordValue: GetDwordValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPRemoteEndpointInfo as windows_core::Interface>::IID
    }
}
pub trait IUPnPReregistrar_Impl: Sized {
    fn ReregisterDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrxmldesc: &windows_core::BSTR, bstrprogiddevicecontrolclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<()>;
    fn ReregisterRunningDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrxmldesc: &windows_core::BSTR, punkdevicecontrol: Option<&windows_core::IUnknown>, bstrinitstring: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPReregistrar {}
impl IUPnPReregistrar_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPReregistrar_Vtbl
    where
        Identity: IUPnPReregistrar_Impl,
    {
        unsafe extern "system" fn ReregisterDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: core::mem::MaybeUninit<windows_core::BSTR>, bstrxmldesc: core::mem::MaybeUninit<windows_core::BSTR>, bstrprogiddevicecontrolclass: core::mem::MaybeUninit<windows_core::BSTR>, bstrinitstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrcontainerid: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcepath: core::mem::MaybeUninit<windows_core::BSTR>, nlifetime: i32) -> windows_core::HRESULT
        where
            Identity: IUPnPReregistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPReregistrar_Impl::ReregisterDevice(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrxmldesc), core::mem::transmute(&bstrprogiddevicecontrolclass), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrcontainerid), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)).into()
        }
        unsafe extern "system" fn ReregisterRunningDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: core::mem::MaybeUninit<windows_core::BSTR>, bstrxmldesc: core::mem::MaybeUninit<windows_core::BSTR>, punkdevicecontrol: *mut core::ffi::c_void, bstrinitstring: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcepath: core::mem::MaybeUninit<windows_core::BSTR>, nlifetime: i32) -> windows_core::HRESULT
        where
            Identity: IUPnPReregistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPReregistrar_Impl::ReregisterRunningDevice(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrxmldesc), windows_core::from_raw_borrowed(&punkdevicecontrol), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReregisterDevice: ReregisterDevice::<Identity, OFFSET>,
            ReregisterRunningDevice: ReregisterRunningDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPReregistrar as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPService_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn QueryStateVariable(&self, bstrvariablename: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn InvokeAction(&self, bstractionname: &windows_core::BSTR, vinactionargs: &windows_core::VARIANT, pvoutactionargs: *mut windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn ServiceTypeIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddCallback(&self, punkcallback: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastTransportStatus(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPService {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPService_Vtbl
    where
        Identity: IUPnPService_Impl,
    {
        unsafe extern "system" fn QueryStateVariable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvariablename: core::mem::MaybeUninit<windows_core::BSTR>, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUPnPService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPService_Impl::QueryStateVariable(this, core::mem::transmute(&bstrvariablename)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstractionname: core::mem::MaybeUninit<windows_core::BSTR>, vinactionargs: core::mem::MaybeUninit<windows_core::VARIANT>, pvoutactionargs: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pvretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUPnPService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPService_Impl::InvokeAction(this, core::mem::transmute(&bstractionname), core::mem::transmute(&vinactionargs), core::mem::transmute_copy(&pvoutactionargs)) {
                Ok(ok__) => {
                    pvretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPService_Impl::ServiceTypeIdentifier(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPService_Impl::AddCallback(this, windows_core::from_raw_borrowed(&punkcallback)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPService_Impl::Id(this) {
                Ok(ok__) => {
                    pbstrid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTransportStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUPnPService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPService_Impl::LastTransportStatus(this) {
                Ok(ok__) => {
                    plvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueryStateVariable: QueryStateVariable::<Identity, OFFSET>,
            InvokeAction: InvokeAction::<Identity, OFFSET>,
            ServiceTypeIdentifier: ServiceTypeIdentifier::<Identity, OFFSET>,
            AddCallback: AddCallback::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            LastTransportStatus: LastTransportStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPService as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IUPnPServiceAsync_Impl: Sized {
    fn BeginInvokeAction(&self, bstractionname: &windows_core::BSTR, vinactionargs: &windows_core::VARIANT, pasyncresult: Option<&IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut windows_core::VARIANT, pvretval: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn BeginQueryStateVariable(&self, bstrvariablename: &windows_core::BSTR, pasyncresult: Option<&IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn BeginSubscribeToEvents(&self, punkcallback: Option<&windows_core::IUnknown>, pasyncresult: Option<&IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndSubscribeToEvents(&self, ullrequestid: u64) -> windows_core::Result<()>;
    fn BeginSCPDDownload(&self, pasyncresult: Option<&IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndSCPDDownload(&self, ullrequestid: u64) -> windows_core::Result<windows_core::BSTR>;
    fn CancelAsyncOperation(&self, ullrequestid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPServiceAsync {}
impl IUPnPServiceAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPServiceAsync_Vtbl
    where
        Identity: IUPnPServiceAsync_Impl,
    {
        unsafe extern "system" fn BeginInvokeAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstractionname: core::mem::MaybeUninit<windows_core::BSTR>, vinactionargs: core::mem::MaybeUninit<windows_core::VARIANT>, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServiceAsync_Impl::BeginInvokeAction(this, core::mem::transmute(&bstractionname), core::mem::transmute(&vinactionargs), windows_core::from_raw_borrowed(&pasyncresult)) {
                Ok(ok__) => {
                    pullrequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInvokeAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pvretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPServiceAsync_Impl::EndInvokeAction(this, core::mem::transmute_copy(&ullrequestid), core::mem::transmute_copy(&pvoutactionargs), core::mem::transmute_copy(&pvretval)).into()
        }
        unsafe extern "system" fn BeginQueryStateVariable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvariablename: core::mem::MaybeUninit<windows_core::BSTR>, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServiceAsync_Impl::BeginQueryStateVariable(this, core::mem::transmute(&bstrvariablename), windows_core::from_raw_borrowed(&pasyncresult)) {
                Ok(ok__) => {
                    pullrequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQueryStateVariable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPServiceAsync_Impl::EndQueryStateVariable(this, core::mem::transmute_copy(&ullrequestid), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkcallback: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServiceAsync_Impl::BeginSubscribeToEvents(this, windows_core::from_raw_borrowed(&punkcallback), windows_core::from_raw_borrowed(&pasyncresult)) {
                Ok(ok__) => {
                    pullrequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPServiceAsync_Impl::EndSubscribeToEvents(this, core::mem::transmute_copy(&ullrequestid)).into()
        }
        unsafe extern "system" fn BeginSCPDDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServiceAsync_Impl::BeginSCPDDownload(this, windows_core::from_raw_borrowed(&pasyncresult)) {
                Ok(ok__) => {
                    pullrequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSCPDDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServiceAsync_Impl::EndSCPDDownload(this, core::mem::transmute_copy(&ullrequestid)) {
                Ok(ok__) => {
                    pbstrscpddoc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPServiceAsync_Impl::CancelAsyncOperation(this, core::mem::transmute_copy(&ullrequestid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginInvokeAction: BeginInvokeAction::<Identity, OFFSET>,
            EndInvokeAction: EndInvokeAction::<Identity, OFFSET>,
            BeginQueryStateVariable: BeginQueryStateVariable::<Identity, OFFSET>,
            EndQueryStateVariable: EndQueryStateVariable::<Identity, OFFSET>,
            BeginSubscribeToEvents: BeginSubscribeToEvents::<Identity, OFFSET>,
            EndSubscribeToEvents: EndSubscribeToEvents::<Identity, OFFSET>,
            BeginSCPDDownload: BeginSCPDDownload::<Identity, OFFSET>,
            EndSCPDDownload: EndSCPDDownload::<Identity, OFFSET>,
            CancelAsyncOperation: CancelAsyncOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceAsync as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPServiceCallback_Impl: Sized {
    fn StateVariableChanged(&self, pus: Option<&IUPnPService>, pcwszstatevarname: &windows_core::PCWSTR, vavalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ServiceInstanceDied(&self, pus: Option<&IUPnPService>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPServiceCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPServiceCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPServiceCallback_Vtbl
    where
        Identity: IUPnPServiceCallback_Impl,
    {
        unsafe extern "system" fn StateVariableChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pus: *mut core::ffi::c_void, pcwszstatevarname: windows_core::PCWSTR, vavalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPServiceCallback_Impl::StateVariableChanged(this, windows_core::from_raw_borrowed(&pus), core::mem::transmute(&pcwszstatevarname), core::mem::transmute(&vavalue)).into()
        }
        unsafe extern "system" fn ServiceInstanceDied<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pus: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPServiceCallback_Impl::ServiceInstanceDied(this, windows_core::from_raw_borrowed(&pus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StateVariableChanged: StateVariableChanged::<Identity, OFFSET>,
            ServiceInstanceDied: ServiceInstanceDied::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceCallback as windows_core::Interface>::IID
    }
}
pub trait IUPnPServiceDocumentAccess_Impl: Sized {
    fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IUPnPServiceDocumentAccess {}
impl IUPnPServiceDocumentAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPServiceDocumentAccess_Vtbl
    where
        Identity: IUPnPServiceDocumentAccess_Impl,
    {
        unsafe extern "system" fn GetDocumentURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceDocumentAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServiceDocumentAccess_Impl::GetDocumentURL(this) {
                Ok(ok__) => {
                    pbstrdocurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdoc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceDocumentAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServiceDocumentAccess_Impl::GetDocument(this) {
                Ok(ok__) => {
                    pbstrdoc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentURL: GetDocumentURL::<Identity, OFFSET>,
            GetDocument: GetDocument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceDocumentAccess as windows_core::Interface>::IID
    }
}
pub trait IUPnPServiceEnumProperty_Impl: Sized {
    fn SetServiceEnumProperty(&self, dwmask: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUPnPServiceEnumProperty {}
impl IUPnPServiceEnumProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPServiceEnumProperty_Vtbl
    where
        Identity: IUPnPServiceEnumProperty_Impl,
    {
        unsafe extern "system" fn SetServiceEnumProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT
        where
            Identity: IUPnPServiceEnumProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUPnPServiceEnumProperty_Impl::SetServiceEnumProperty(this, core::mem::transmute_copy(&dwmask)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetServiceEnumProperty: SetServiceEnumProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceEnumProperty as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPServices_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, bstrserviceid: &windows_core::BSTR) -> windows_core::Result<IUPnPService>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPServices {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPServices_Vtbl
    where
        Identity: IUPnPServices_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUPnPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServices_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServices_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceid: core::mem::MaybeUninit<windows_core::BSTR>, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPServices_Impl::get_Item(this, core::mem::transmute(&bstrserviceid)) {
                Ok(ok__) => {
                    ppservice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServices as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
