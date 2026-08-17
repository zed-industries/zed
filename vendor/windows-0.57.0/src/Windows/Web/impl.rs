#[cfg(feature = "Storage_Streams")]
pub trait IUriToStreamResolver_Impl: Sized {
    fn UriToStreamAsync(&self, uri: Option<&super::Foundation::Uri>) -> windows_core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IUriToStreamResolver {
    const NAME: &'static str = "Windows.Web.IUriToStreamResolver";
}
#[cfg(feature = "Storage_Streams")]
impl IUriToStreamResolver_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUriToStreamResolver_Impl, const OFFSET: isize>() -> IUriToStreamResolver_Vtbl {
        unsafe extern "system" fn UriToStreamAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUriToStreamResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUriToStreamResolver_Impl::UriToStreamAsync(this, windows_core::from_raw_borrowed(&uri)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUriToStreamResolver, OFFSET>(),
            UriToStreamAsync: UriToStreamAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriToStreamResolver as windows_core::Interface>::IID
    }
}
