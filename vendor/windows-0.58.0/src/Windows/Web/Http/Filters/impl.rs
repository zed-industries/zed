pub trait IHttpFilter_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn SendRequestAsync(&self, request: Option<&super::HttpRequestMessage>) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
impl windows_core::RuntimeName for IHttpFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpFilter";
}
impl IHttpFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHttpFilter_Vtbl
    where
        Identity: IHttpFilter_Impl,
    {
        unsafe extern "system" fn SendRequestAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHttpFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpFilter_Impl::SendRequestAsync(this, windows_core::from_raw_borrowed(&request)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IHttpFilter, OFFSET>(), SendRequestAsync: SendRequestAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpFilter as windows_core::Interface>::IID
    }
}
