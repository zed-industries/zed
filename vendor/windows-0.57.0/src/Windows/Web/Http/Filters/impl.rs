pub trait IHttpFilter_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn SendRequestAsync(&self, request: Option<&super::HttpRequestMessage>) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
impl windows_core::RuntimeName for IHttpFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.IHttpFilter";
}
impl IHttpFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpFilter_Impl, const OFFSET: isize>() -> IHttpFilter_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IHttpFilter_Impl::SendRequestAsync(this, windows_core::from_raw_borrowed(&request)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IHttpFilter, OFFSET>(), SendRequestAsync: SendRequestAsync::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpFilter as windows_core::Interface>::IID
    }
}
