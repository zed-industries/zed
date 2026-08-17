pub trait ISysStorageProviderEventSource_Impl: Sized {
    fn EventReceived(&self, handler: Option<&super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISysStorageProviderEventSource {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
}
impl ISysStorageProviderEventSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISysStorageProviderEventSource_Vtbl
    where
        Identity: ISysStorageProviderEventSource_Impl,
    {
        unsafe extern "system" fn EventReceived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ISysStorageProviderEventSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderEventSource_Impl::EventReceived(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventReceived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ISysStorageProviderEventSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISysStorageProviderEventSource_Impl::RemoveEventReceived(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderEventSource, OFFSET>(),
            EventReceived: EventReceived::<Identity, OFFSET>,
            RemoveEventReceived: RemoveEventReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderEventSource as windows_core::Interface>::IID
    }
}
pub trait ISysStorageProviderHandlerFactory_Impl: Sized {
    fn GetHttpRequestProvider(&self, syncrootid: &windows_core::HSTRING) -> windows_core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(&self, syncrootid: &windows_core::HSTRING, eventname: &windows_core::HSTRING) -> windows_core::Result<ISysStorageProviderEventSource>;
}
impl windows_core::RuntimeName for ISysStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
}
impl ISysStorageProviderHandlerFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISysStorageProviderHandlerFactory_Vtbl
    where
        Identity: ISysStorageProviderHandlerFactory_Impl,
    {
        unsafe extern "system" fn GetHttpRequestProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncrootid: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISysStorageProviderHandlerFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderHandlerFactory_Impl::GetHttpRequestProvider(this, core::mem::transmute(&syncrootid)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncrootid: core::mem::MaybeUninit<windows_core::HSTRING>, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISysStorageProviderHandlerFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderHandlerFactory_Impl::GetEventSource(this, core::mem::transmute(&syncrootid), core::mem::transmute(&eventname)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderHandlerFactory, OFFSET>(),
            GetHttpRequestProvider: GetHttpRequestProvider::<Identity, OFFSET>,
            GetEventSource: GetEventSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderHandlerFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Web_Http")]
pub trait ISysStorageProviderHttpRequestProvider_Impl: Sized {
    fn SendRequestAsync(&self, request: Option<&super::super::super::Web::Http::HttpRequestMessage>) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
#[cfg(feature = "Web_Http")]
impl windows_core::RuntimeName for ISysStorageProviderHttpRequestProvider {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
}
#[cfg(feature = "Web_Http")]
impl ISysStorageProviderHttpRequestProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISysStorageProviderHttpRequestProvider_Vtbl
    where
        Identity: ISysStorageProviderHttpRequestProvider_Impl,
    {
        unsafe extern "system" fn SendRequestAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISysStorageProviderHttpRequestProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderHttpRequestProvider_Impl::SendRequestAsync(this, windows_core::from_raw_borrowed(&request)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderHttpRequestProvider, OFFSET>(),
            SendRequestAsync: SendRequestAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderHttpRequestProvider as windows_core::Interface>::IID
    }
}
