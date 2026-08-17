pub trait IUserActivityContentInfo_Impl: Sized {
    fn ToJson(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IUserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityContentInfo";
}
impl IUserActivityContentInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserActivityContentInfo_Impl, const OFFSET: isize>() -> IUserActivityContentInfo_Vtbl {
        unsafe extern "system" fn ToJson<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUserActivityContentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUserActivityContentInfo_Impl::ToJson(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivityContentInfo, OFFSET>(), ToJson: ToJson::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivityContentInfo as windows_core::Interface>::IID
    }
}
