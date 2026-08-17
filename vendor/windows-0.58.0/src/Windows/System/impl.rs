#[cfg(feature = "UI_ViewManagement")]
pub trait ILauncherViewOptions_Impl: Sized {
    fn DesiredRemainingView(&self) -> windows_core::Result<super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_ViewManagement")]
impl windows_core::RuntimeName for ILauncherViewOptions {
    const NAME: &'static str = "Windows.System.ILauncherViewOptions";
}
#[cfg(feature = "UI_ViewManagement")]
impl ILauncherViewOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILauncherViewOptions_Vtbl
    where
        Identity: ILauncherViewOptions_Impl,
    {
        unsafe extern "system" fn DesiredRemainingView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> windows_core::HRESULT
        where
            Identity: ILauncherViewOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILauncherViewOptions_Impl::DesiredRemainingView(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRemainingView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> windows_core::HRESULT
        where
            Identity: ILauncherViewOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILauncherViewOptions_Impl::SetDesiredRemainingView(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILauncherViewOptions, OFFSET>(),
            DesiredRemainingView: DesiredRemainingView::<Identity, OFFSET>,
            SetDesiredRemainingView: SetDesiredRemainingView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILauncherViewOptions as windows_core::Interface>::IID
    }
}
