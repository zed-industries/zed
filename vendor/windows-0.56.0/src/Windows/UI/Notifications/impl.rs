#[cfg(feature = "Foundation_Collections")]
pub trait IAdaptiveNotificationContent_Impl: Sized {
    fn Kind(&self) -> windows_core::Result<AdaptiveNotificationContentKind>;
    fn Hints(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IAdaptiveNotificationContent {
    const NAME: &'static str = "Windows.UI.Notifications.IAdaptiveNotificationContent";
}
#[cfg(feature = "Foundation_Collections")]
impl IAdaptiveNotificationContent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: isize>() -> IAdaptiveNotificationContent_Vtbl {
        unsafe extern "system" fn Kind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut AdaptiveNotificationContentKind) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdaptiveNotificationContent_Impl::Kind(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdaptiveNotificationContent_Impl::Hints(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAdaptiveNotificationContent, OFFSET>(),
            Kind: Kind::<Identity, Impl, OFFSET>,
            Hints: Hints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdaptiveNotificationContent as windows_core::Interface>::IID
    }
}
