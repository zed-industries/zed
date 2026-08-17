#[cfg(feature = "deprecated")]
pub trait ISearchPaneQueryChangedEventArgs_Impl: Sized {
    fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn LinguisticDetails(&self) -> windows_core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for ISearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ISearchPaneQueryChangedEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchPaneQueryChangedEventArgs_Vtbl
    where
        Identity: ISearchPaneQueryChangedEventArgs_Impl,
    {
        unsafe extern "system" fn QueryText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISearchPaneQueryChangedEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchPaneQueryChangedEventArgs_Impl::QueryText(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ISearchPaneQueryChangedEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchPaneQueryChangedEventArgs_Impl::Language(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinguisticDetails<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchPaneQueryChangedEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchPaneQueryChangedEventArgs_Impl::LinguisticDetails(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISearchPaneQueryChangedEventArgs, OFFSET>(),
            QueryText: QueryText::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            LinguisticDetails: LinguisticDetails::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchPaneQueryChangedEventArgs as windows_core::Interface>::IID
    }
}
