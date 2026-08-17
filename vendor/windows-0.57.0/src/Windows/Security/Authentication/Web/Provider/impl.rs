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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderBaseReportOperation_Vtbl {
        unsafe extern "system" fn ReportCompleted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWebAccountProviderBaseReportOperation_Impl::ReportCompleted(this).into()
        }
        unsafe extern "system" fn ReportError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWebAccountProviderBaseReportOperation_Impl::ReportError(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderBaseReportOperation, OFFSET>(),
            ReportCompleted: ReportCompleted::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderOperation_Vtbl {
        unsafe extern "system" fn Kind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccountProviderOperation_Impl::Kind(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderOperation, OFFSET>(), Kind: Kind::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderSilentReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserInteractionRequired<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWebAccountProviderSilentReportOperation_Impl::ReportUserInteractionRequired(this).into()
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWebAccountProviderSilentReportOperation_Impl::ReportUserInteractionRequiredWithError(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderSilentReportOperation, OFFSET>(),
            ReportUserInteractionRequired: ReportUserInteractionRequired::<Identity, Impl, OFFSET>,
            ReportUserInteractionRequiredWithError: ReportUserInteractionRequiredWithError::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: isize>() -> IWebAccountProviderTokenObjects_Vtbl {
        unsafe extern "system" fn Operation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccountProviderTokenObjects_Impl::Operation(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderTokenObjects, OFFSET>(),
            Operation: Operation::<Identity, Impl, OFFSET>,
        }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: isize>() -> IWebAccountProviderTokenObjects2_Vtbl {
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccountProviderTokenObjects2_Impl::User(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderTokenObjects2, OFFSET>(), User: User::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderTokenOperation_Vtbl {
        unsafe extern "system" fn ProviderRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccountProviderTokenOperation_Impl::ProviderRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderResponses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccountProviderTokenOperation_Impl::ProviderResponses(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheExpirationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWebAccountProviderTokenOperation_Impl::SetCacheExpirationTime(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CacheExpirationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccountProviderTokenOperation_Impl::CacheExpirationTime(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderTokenOperation, OFFSET>(),
            ProviderRequest: ProviderRequest::<Identity, Impl, OFFSET>,
            ProviderResponses: ProviderResponses::<Identity, Impl, OFFSET>,
            SetCacheExpirationTime: SetCacheExpirationTime::<Identity, Impl, OFFSET>,
            CacheExpirationTime: CacheExpirationTime::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: isize>() -> IWebAccountProviderUIReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserCanceled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWebAccountProviderUIReportOperation_Impl::ReportUserCanceled(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderUIReportOperation, OFFSET>(),
            ReportUserCanceled: ReportUserCanceled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderUIReportOperation as windows_core::Interface>::IID
    }
}
