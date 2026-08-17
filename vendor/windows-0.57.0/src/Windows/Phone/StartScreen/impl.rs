#[cfg(feature = "UI_Notifications")]
pub trait IToastNotificationManagerStatics3_Impl: Sized {
    fn CreateToastNotifierForSecondaryTile(&self, tileid: &windows_core::HSTRING) -> windows_core::Result<super::super::UI::Notifications::ToastNotifier>;
}
#[cfg(feature = "UI_Notifications")]
impl windows_core::RuntimeName for IToastNotificationManagerStatics3 {
    const NAME: &'static str = "Windows.Phone.StartScreen.IToastNotificationManagerStatics3";
}
#[cfg(feature = "UI_Notifications")]
impl IToastNotificationManagerStatics3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>() -> IToastNotificationManagerStatics3_Vtbl {
        unsafe extern "system" fn CreateToastNotifierForSecondaryTile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tileid: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IToastNotificationManagerStatics3_Impl::CreateToastNotifierForSecondaryTile(this, core::mem::transmute(&tileid)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IToastNotificationManagerStatics3, OFFSET>(),
            CreateToastNotifierForSecondaryTile: CreateToastNotifierForSecondaryTile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics3 as windows_core::Interface>::IID
    }
}
