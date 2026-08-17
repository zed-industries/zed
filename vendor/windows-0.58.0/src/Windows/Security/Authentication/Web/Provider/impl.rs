#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderBaseReportOperation_Impl: Sized {
    fn ReportCompleted(&self) -> windows_core::Result<()>;
    fn ReportError(&self, value: Option<&super::Core::WebProviderError>) -> windows_core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl windows_core::RuntimeName for IWebAccountProviderBaseReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderBaseReportOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAccountProviderBaseReportOperation_Vtbl
    where
        Identity: IWebAccountProviderBaseReportOperation_Impl,
    {
        unsafe extern "system" fn ReportCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderBaseReportOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAccountProviderBaseReportOperation_Impl::ReportCompleted(this).into()
        }
        unsafe extern "system" fn ReportError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderBaseReportOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAccountProviderBaseReportOperation_Impl::ReportError(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderBaseReportOperation, OFFSET>(),
            ReportCompleted: ReportCompleted::<Identity, OFFSET>,
            ReportError: ReportError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderBaseReportOperation as windows_core::Interface>::IID
    }
}
pub trait IWebAccountProviderOperation_Impl: Sized {
    fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind>;
}
impl windows_core::RuntimeName for IWebAccountProviderOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
}
impl IWebAccountProviderOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAccountProviderOperation_Vtbl
    where
        Identity: IWebAccountProviderOperation_Impl,
    {
        unsafe extern "system" fn Kind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAccountProviderOperation_Impl::Kind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderOperation, OFFSET>(), Kind: Kind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderOperation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderSilentReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserInteractionRequired(&self) -> windows_core::Result<()>;
    fn ReportUserInteractionRequiredWithError(&self, value: Option<&super::Core::WebProviderError>) -> windows_core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl windows_core::RuntimeName for IWebAccountProviderSilentReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderSilentReportOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAccountProviderSilentReportOperation_Vtbl
    where
        Identity: IWebAccountProviderSilentReportOperation_Impl,
    {
        unsafe extern "system" fn ReportUserInteractionRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderSilentReportOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAccountProviderSilentReportOperation_Impl::ReportUserInteractionRequired(this).into()
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderSilentReportOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAccountProviderSilentReportOperation_Impl::ReportUserInteractionRequiredWithError(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderSilentReportOperation, OFFSET>(),
            ReportUserInteractionRequired: ReportUserInteractionRequired::<Identity, OFFSET>,
            ReportUserInteractionRequiredWithError: ReportUserInteractionRequiredWithError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderSilentReportOperation as windows_core::Interface>::IID
    }
}
pub trait IWebAccountProviderTokenObjects_Impl: Sized {
    fn Operation(&self) -> windows_core::Result<IWebAccountProviderOperation>;
}
impl windows_core::RuntimeName for IWebAccountProviderTokenObjects {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
}
impl IWebAccountProviderTokenObjects_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAccountProviderTokenObjects_Vtbl
    where
        Identity: IWebAccountProviderTokenObjects_Impl,
    {
        unsafe extern "system" fn Operation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderTokenObjects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAccountProviderTokenObjects_Impl::Operation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderTokenObjects, OFFSET>(), Operation: Operation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects as windows_core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IWebAccountProviderTokenObjects2_Impl: Sized + IWebAccountProviderTokenObjects_Impl {
    fn User(&self) -> windows_core::Result<super::super::super::super::System::User>;
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for IWebAccountProviderTokenObjects2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
}
#[cfg(feature = "System")]
impl IWebAccountProviderTokenObjects2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAccountProviderTokenObjects2_Vtbl
    where
        Identity: IWebAccountProviderTokenObjects2_Impl,
    {
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderTokenObjects2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAccountProviderTokenObjects2_Impl::User(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderTokenObjects2, OFFSET>(), User: User::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IWebAccountProviderTokenOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
    fn ProviderRequest(&self) -> windows_core::Result<WebProviderTokenRequest>;
    fn ProviderResponses(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>;
    fn SetCacheExpirationTime(&self, value: &super::super::super::super::Foundation::DateTime) -> windows_core::Result<()>;
    fn CacheExpirationTime(&self) -> windows_core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IWebAccountProviderTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation";
}
#[cfg(feature = "Foundation_Collections")]
impl IWebAccountProviderTokenOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAccountProviderTokenOperation_Vtbl
    where
        Identity: IWebAccountProviderTokenOperation_Impl,
    {
        unsafe extern "system" fn ProviderRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderTokenOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAccountProviderTokenOperation_Impl::ProviderRequest(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderResponses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderTokenOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAccountProviderTokenOperation_Impl::ProviderResponses(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheExpirationTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderTokenOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAccountProviderTokenOperation_Impl::SetCacheExpirationTime(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CacheExpirationTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderTokenOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAccountProviderTokenOperation_Impl::CacheExpirationTime(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderTokenOperation, OFFSET>(),
            ProviderRequest: ProviderRequest::<Identity, OFFSET>,
            ProviderResponses: ProviderResponses::<Identity, OFFSET>,
            SetCacheExpirationTime: SetCacheExpirationTime::<Identity, OFFSET>,
            CacheExpirationTime: CacheExpirationTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenOperation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderUIReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserCanceled(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl windows_core::RuntimeName for IWebAccountProviderUIReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderUIReportOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAccountProviderUIReportOperation_Vtbl
    where
        Identity: IWebAccountProviderUIReportOperation_Impl,
    {
        unsafe extern "system" fn ReportUserCanceled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAccountProviderUIReportOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAccountProviderUIReportOperation_Impl::ReportUserCanceled(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderUIReportOperation, OFFSET>(),
            ReportUserCanceled: ReportUserCanceled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderUIReportOperation as windows_core::Interface>::IID
    }
}
