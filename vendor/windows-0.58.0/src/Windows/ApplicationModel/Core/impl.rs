pub trait ICoreApplicationUnhandledError_Impl: Sized {
    fn UnhandledErrorDetected(&self, handler: Option<&super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledErrorDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreApplicationUnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationUnhandledError";
}
impl ICoreApplicationUnhandledError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreApplicationUnhandledError_Vtbl
    where
        Identity: ICoreApplicationUnhandledError_Impl,
    {
        unsafe extern "system" fn UnhandledErrorDetected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreApplicationUnhandledError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreApplicationUnhandledError_Impl::UnhandledErrorDetected(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnhandledErrorDetected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreApplicationUnhandledError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreApplicationUnhandledError_Impl::RemoveUnhandledErrorDetected(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreApplicationUnhandledError, OFFSET>(),
            UnhandledErrorDetected: UnhandledErrorDetected::<Identity, OFFSET>,
            RemoveUnhandledErrorDetected: RemoveUnhandledErrorDetected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreApplicationUnhandledError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Core")]
pub trait IFrameworkView_Impl: Sized {
    fn Initialize(&self, applicationview: Option<&CoreApplicationView>) -> windows_core::Result<()>;
    fn SetWindow(&self, window: Option<&super::super::UI::Core::CoreWindow>) -> windows_core::Result<()>;
    fn Load(&self, entrypoint: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Run(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Core")]
impl windows_core::RuntimeName for IFrameworkView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkView";
}
#[cfg(feature = "UI_Core")]
impl IFrameworkView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFrameworkView_Vtbl
    where
        Identity: IFrameworkView_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationview: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFrameworkView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFrameworkView_Impl::Initialize(this, windows_core::from_raw_borrowed(&applicationview)).into()
        }
        unsafe extern "system" fn SetWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFrameworkView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFrameworkView_Impl::SetWindow(this, windows_core::from_raw_borrowed(&window)).into()
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, entrypoint: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IFrameworkView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFrameworkView_Impl::Load(this, core::mem::transmute(&entrypoint)).into()
        }
        unsafe extern "system" fn Run<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFrameworkView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFrameworkView_Impl::Run(this).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFrameworkView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFrameworkView_Impl::Uninitialize(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkView, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            SetWindow: SetWindow::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkView as windows_core::Interface>::IID
    }
}
pub trait IFrameworkViewSource_Impl: Sized {
    fn CreateView(&self) -> windows_core::Result<IFrameworkView>;
}
impl windows_core::RuntimeName for IFrameworkViewSource {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkViewSource";
}
impl IFrameworkViewSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFrameworkViewSource_Vtbl
    where
        Identity: IFrameworkViewSource_Impl,
    {
        unsafe extern "system" fn CreateView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFrameworkViewSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFrameworkViewSource_Impl::CreateView(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkViewSource, OFFSET>(), CreateView: CreateView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkViewSource as windows_core::Interface>::IID
    }
}
