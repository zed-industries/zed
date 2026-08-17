#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
pub trait IGameListEntry_Impl: Sized {
    fn DisplayInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppDisplayInfo>;
    fn LaunchAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Category(&self) -> windows_core::Result<GameListCategory>;
    fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>;
    fn SetCategoryAsync(&self, value: GameListCategory) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
impl windows_core::RuntimeName for IGameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListEntry";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
impl IGameListEntry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGameListEntry_Vtbl
    where
        Identity: IGameListEntry_Impl,
    {
        unsafe extern "system" fn DisplayInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameListEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameListEntry_Impl::DisplayInfo(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameListEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameListEntry_Impl::LaunchAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GameListCategory) -> windows_core::HRESULT
        where
            Identity: IGameListEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameListEntry_Impl::Category(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameListEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameListEntry_Impl::Properties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: GameListCategory, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameListEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameListEntry_Impl::SetCategoryAsync(this, value) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameListEntry, OFFSET>(),
            DisplayInfo: DisplayInfo::<Identity, OFFSET>,
            LaunchAsync: LaunchAsync::<Identity, OFFSET>,
            Category: Category::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            SetCategoryAsync: SetCategoryAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameListEntry as windows_core::Interface>::IID
    }
}
