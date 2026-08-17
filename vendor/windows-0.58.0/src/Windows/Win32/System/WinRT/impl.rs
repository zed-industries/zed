pub trait IAccountsSettingsPaneInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, accountssettingspane: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowManageAccountsForWindowAsync(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowAddAccountForWindowAsync(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAccountsSettingsPaneInterop {}
impl IAccountsSettingsPaneInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAccountsSettingsPaneInterop_Vtbl
    where
        Identity: IAccountsSettingsPaneInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, accountssettingspane: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccountsSettingsPaneInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccountsSettingsPaneInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&accountssettingspane)).into()
        }
        unsafe extern "system" fn ShowManageAccountsForWindowAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccountsSettingsPaneInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccountsSettingsPaneInterop_Impl::ShowManageAccountsForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncaction)).into()
        }
        unsafe extern "system" fn ShowAddAccountForWindowAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAccountsSettingsPaneInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccountsSettingsPaneInterop_Impl::ShowAddAccountForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncaction)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccountsSettingsPaneInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowManageAccountsForWindowAsync: ShowManageAccountsForWindowAsync::<Identity, OFFSET>,
            ShowAddAccountForWindowAsync: ShowAddAccountForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneInterop as windows_core::Interface>::IID
    }
}
pub trait IActivationFactory_Impl: Sized {
    fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl windows_core::RuntimeName for IActivationFactory {}
impl IActivationFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActivationFactory_Vtbl
    where
        Identity: IActivationFactory_Impl,
    {
        unsafe extern "system" fn ActivateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, instance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActivationFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActivationFactory_Impl::ActivateInstance(this) {
                Ok(ok__) => {
                    instance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivationFactory, OFFSET>(), ActivateInstance: ActivateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivationFactory as windows_core::Interface>::IID
    }
}
pub trait IAgileReference_Impl: Sized {
    fn Resolve(&self, riid: *const windows_core::GUID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAgileReference {}
impl IAgileReference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAgileReference_Vtbl
    where
        Identity: IAgileReference_Impl,
    {
        unsafe extern "system" fn Resolve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAgileReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAgileReference_Impl::Resolve(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobjectreference)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileReference as windows_core::Interface>::IID
    }
}
pub trait IApartmentShutdown_Impl: Sized {
    fn OnUninitialize(&self, ui64apartmentidentifier: u64);
}
impl windows_core::RuntimeName for IApartmentShutdown {}
impl IApartmentShutdown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IApartmentShutdown_Vtbl
    where
        Identity: IApartmentShutdown_Impl,
    {
        unsafe extern "system" fn OnUninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ui64apartmentidentifier: u64)
        where
            Identity: IApartmentShutdown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApartmentShutdown_Impl::OnUninitialize(this, core::mem::transmute_copy(&ui64apartmentidentifier))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUninitialize: OnUninitialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApartmentShutdown as windows_core::Interface>::IID
    }
}
pub trait IAppServiceConnectionExtendedExecution_Impl: Sized {
    fn OpenForExtendedExecutionAsync(&self, riid: *const windows_core::GUID, operation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAppServiceConnectionExtendedExecution {}
impl IAppServiceConnectionExtendedExecution_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppServiceConnectionExtendedExecution_Vtbl
    where
        Identity: IAppServiceConnectionExtendedExecution_Impl,
    {
        unsafe extern "system" fn OpenForExtendedExecutionAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, operation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppServiceConnectionExtendedExecution_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppServiceConnectionExtendedExecution_Impl::OpenForExtendedExecutionAsync(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&operation)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenForExtendedExecutionAsync: OpenForExtendedExecutionAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppServiceConnectionExtendedExecution as windows_core::Interface>::IID
    }
}
pub trait IBufferByteAccess_Impl: Sized {
    fn Buffer(&self) -> windows_core::Result<*mut u8>;
}
impl windows_core::RuntimeName for IBufferByteAccess {}
impl IBufferByteAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBufferByteAccess_Vtbl
    where
        Identity: IBufferByteAccess_Impl,
    {
        unsafe extern "system" fn Buffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IBufferByteAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBufferByteAccess_Impl::Buffer(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Buffer: Buffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBufferByteAccess as windows_core::Interface>::IID
    }
}
pub trait ICastingController_Impl: Sized {
    fn Initialize(&self, castingengine: Option<&windows_core::IUnknown>, castingsource: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Advise(&self, eventhandler: Option<&ICastingEventHandler>) -> windows_core::Result<u32>;
    fn UnAdvise(&self, cookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICastingController {}
impl ICastingController_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICastingController_Vtbl
    where
        Identity: ICastingController_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, castingengine: *mut core::ffi::c_void, castingsource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICastingController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICastingController_Impl::Initialize(this, windows_core::from_raw_borrowed(&castingengine), windows_core::from_raw_borrowed(&castingsource)).into()
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICastingController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICastingController_Impl::Connect(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICastingController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICastingController_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICastingController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICastingController_Impl::Advise(this, windows_core::from_raw_borrowed(&eventhandler)) {
                Ok(ok__) => {
                    cookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT
        where
            Identity: ICastingController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICastingController_Impl::UnAdvise(this, core::mem::transmute_copy(&cookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            UnAdvise: UnAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingController as windows_core::Interface>::IID
    }
}
pub trait ICastingEventHandler_Impl: Sized {
    fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> windows_core::Result<()>;
    fn OnError(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICastingEventHandler {}
impl ICastingEventHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICastingEventHandler_Vtbl
    where
        Identity: ICastingEventHandler_Impl,
    {
        unsafe extern "system" fn OnStateChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> windows_core::HRESULT
        where
            Identity: ICastingEventHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICastingEventHandler_Impl::OnStateChanged(this, core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn OnError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICastingEventHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICastingEventHandler_Impl::OnError(this, core::mem::transmute_copy(&errorstatus), core::mem::transmute(&errormessage)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ICastingSourceInfo_Impl: Sized {
    fn GetController(&self) -> windows_core::Result<ICastingController>;
    fn GetProperties(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ICastingSourceInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ICastingSourceInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICastingSourceInfo_Vtbl
    where
        Identity: ICastingSourceInfo_Impl,
    {
        unsafe extern "system" fn GetController<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, controller: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICastingSourceInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICastingSourceInfo_Impl::GetController(this) {
                Ok(ok__) => {
                    controller.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, props: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICastingSourceInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICastingSourceInfo_Impl::GetProperties(this) {
                Ok(ok__) => {
                    props.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetController: GetController::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingSourceInfo as windows_core::Interface>::IID
    }
}
pub trait ICoreInputInterop_Impl: Sized {
    fn SetInputSource(&self, value: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreInputInterop {}
impl ICoreInputInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreInputInterop_Vtbl
    where
        Identity: ICoreInputInterop_Impl,
    {
        unsafe extern "system" fn SetInputSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreInputInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreInputInterop_Impl::SetInputSource(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetMessageHandled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT
        where
            Identity: ICoreInputInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreInputInterop_Impl::SetMessageHandled(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInputSource: SetInputSource::<Identity, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputInterop as windows_core::Interface>::IID
    }
}
pub trait ICoreInputInterop2_Impl: Sized {
    fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn ChangeHostingContext(&self, newparentwindow: super::super::Foundation::HWND, newviewinstanceid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreInputInterop2 {}
impl ICoreInputInterop2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreInputInterop2_Vtbl
    where
        Identity: ICoreInputInterop2_Impl,
    {
        unsafe extern "system" fn WindowHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ICoreInputInterop2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreInputInterop2_Impl::WindowHandle(this) {
                Ok(ok__) => {
                    window.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeHostingContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newparentwindow: super::super::Foundation::HWND, newviewinstanceid: u32) -> windows_core::HRESULT
        where
            Identity: ICoreInputInterop2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreInputInterop2_Impl::ChangeHostingContext(this, core::mem::transmute_copy(&newparentwindow), core::mem::transmute_copy(&newviewinstanceid)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreInputInterop2, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            ChangeHostingContext: ChangeHostingContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputInterop2 as windows_core::Interface>::IID
    }
}
pub trait ICoreWindowAdapterInterop_Impl: Sized {
    fn AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetWindowClientAdapter(&self, value: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreWindowAdapterInterop {}
impl ICoreWindowAdapterInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreWindowAdapterInterop_Vtbl
    where
        Identity: ICoreWindowAdapterInterop_Impl,
    {
        unsafe extern "system" fn AppActivationClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowAdapterInterop_Impl::AppActivationClientAdapter(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationViewClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowAdapterInterop_Impl::ApplicationViewClientAdapter(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoreApplicationViewClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowAdapterInterop_Impl::CoreApplicationViewClientAdapter(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoloViewClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowAdapterInterop_Impl::HoloViewClientAdapter(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionerClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowAdapterInterop_Impl::PositionerClientAdapter(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemNavigationClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowAdapterInterop_Impl::SystemNavigationClientAdapter(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleBarClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowAdapterInterop_Impl::TitleBarClientAdapter(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowClientAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowAdapterInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindowAdapterInterop_Impl::SetWindowClientAdapter(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreWindowAdapterInterop, OFFSET>(),
            AppActivationClientAdapter: AppActivationClientAdapter::<Identity, OFFSET>,
            ApplicationViewClientAdapter: ApplicationViewClientAdapter::<Identity, OFFSET>,
            CoreApplicationViewClientAdapter: CoreApplicationViewClientAdapter::<Identity, OFFSET>,
            HoloViewClientAdapter: HoloViewClientAdapter::<Identity, OFFSET>,
            PositionerClientAdapter: PositionerClientAdapter::<Identity, OFFSET>,
            SystemNavigationClientAdapter: SystemNavigationClientAdapter::<Identity, OFFSET>,
            TitleBarClientAdapter: TitleBarClientAdapter::<Identity, OFFSET>,
            SetWindowClientAdapter: SetWindowClientAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowAdapterInterop as windows_core::Interface>::IID
    }
}
pub trait ICoreWindowComponentInterop_Impl: Sized {
    fn ConfigureComponentInput(&self, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetViewInstanceId(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ICoreWindowComponentInterop {}
impl ICoreWindowComponentInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreWindowComponentInterop_Vtbl
    where
        Identity: ICoreWindowComponentInterop_Impl,
    {
        unsafe extern "system" fn ConfigureComponentInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindowComponentInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindowComponentInterop_Impl::ConfigureComponentInput(this, core::mem::transmute_copy(&hostviewinstanceid), core::mem::transmute_copy(&hwndhost), windows_core::from_raw_borrowed(&inputsourcevisual)).into()
        }
        unsafe extern "system" fn GetViewInstanceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, componentviewinstanceid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICoreWindowComponentInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowComponentInterop_Impl::GetViewInstanceId(this) {
                Ok(ok__) => {
                    componentviewinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureComponentInput: ConfigureComponentInput::<Identity, OFFSET>,
            GetViewInstanceId: GetViewInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowComponentInterop as windows_core::Interface>::IID
    }
}
pub trait ICoreWindowInterop_Impl: Sized {
    fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreWindowInterop {}
impl ICoreWindowInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreWindowInterop_Vtbl
    where
        Identity: ICoreWindowInterop_Impl,
    {
        unsafe extern "system" fn WindowHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ICoreWindowInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowInterop_Impl::WindowHandle(this) {
                Ok(ok__) => {
                    hwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageHandled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT
        where
            Identity: ICoreWindowInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindowInterop_Impl::SetMessageHandled(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowInterop as windows_core::Interface>::IID
    }
}
pub trait ICorrelationVectorInformation_Impl: Sized {
    fn LastCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn NextCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNextCorrelationVectorForThread(&self, cv: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorrelationVectorInformation {}
impl ICorrelationVectorInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorrelationVectorInformation_Vtbl
    where
        Identity: ICorrelationVectorInformation_Impl,
    {
        unsafe extern "system" fn LastCorrelationVectorForThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ICorrelationVectorInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorrelationVectorInformation_Impl::LastCorrelationVectorForThread(this) {
                Ok(ok__) => {
                    cv.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextCorrelationVectorForThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ICorrelationVectorInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorrelationVectorInformation_Impl::NextCorrelationVectorForThread(this) {
                Ok(ok__) => {
                    cv.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNextCorrelationVectorForThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ICorrelationVectorInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorrelationVectorInformation_Impl::SetNextCorrelationVectorForThread(this, core::mem::transmute(&cv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorrelationVectorInformation, OFFSET>(),
            LastCorrelationVectorForThread: LastCorrelationVectorForThread::<Identity, OFFSET>,
            NextCorrelationVectorForThread: NextCorrelationVectorForThread::<Identity, OFFSET>,
            SetNextCorrelationVectorForThread: SetNextCorrelationVectorForThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorrelationVectorInformation as windows_core::Interface>::IID
    }
}
pub trait ICorrelationVectorSource_Impl: Sized {
    fn CorrelationVector(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for ICorrelationVectorSource {}
impl ICorrelationVectorSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorrelationVectorSource_Vtbl
    where
        Identity: ICorrelationVectorSource_Impl,
    {
        unsafe extern "system" fn CorrelationVector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ICorrelationVectorSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorrelationVectorSource_Impl::CorrelationVector(this) {
                Ok(ok__) => {
                    cv.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CorrelationVector: CorrelationVector::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorrelationVectorSource as windows_core::Interface>::IID
    }
}
pub trait IDragDropManagerInterop_Impl: Sized {
    fn GetForWindow(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDragDropManagerInterop {}
impl IDragDropManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDragDropManagerInterop_Vtbl
    where
        Identity: IDragDropManagerInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDragDropManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDragDropManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDragDropManagerInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDragDropManagerInterop as windows_core::Interface>::IID
    }
}
pub trait IHolographicSpaceInterop_Impl: Sized {
    fn CreateForWindow(&self, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, holographicspace: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHolographicSpaceInterop {}
impl IHolographicSpaceInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHolographicSpaceInterop_Vtbl
    where
        Identity: IHolographicSpaceInterop_Impl,
    {
        unsafe extern "system" fn CreateForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, holographicspace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHolographicSpaceInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolographicSpaceInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&holographicspace)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHolographicSpaceInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolographicSpaceInterop as windows_core::Interface>::IID
    }
}
pub trait IInputPaneInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, inputpane: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInputPaneInterop {}
impl IInputPaneInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInputPaneInterop_Vtbl
    where
        Identity: IInputPaneInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, inputpane: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInputPaneInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInputPaneInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&inputpane)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputPaneInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPaneInterop as windows_core::Interface>::IID
    }
}
pub trait ILanguageExceptionErrorInfo_Impl: Sized {
    fn GetLanguageException(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ILanguageExceptionErrorInfo {}
impl ILanguageExceptionErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILanguageExceptionErrorInfo_Vtbl
    where
        Identity: ILanguageExceptionErrorInfo_Impl,
    {
        unsafe extern "system" fn GetLanguageException<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageexception: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILanguageExceptionErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILanguageExceptionErrorInfo_Impl::GetLanguageException(this) {
                Ok(ok__) => {
                    languageexception.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLanguageException: GetLanguageException::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo as windows_core::Interface>::IID
    }
}
pub trait ILanguageExceptionErrorInfo2_Impl: Sized + ILanguageExceptionErrorInfo_Impl {
    fn GetPreviousLanguageExceptionErrorInfo(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2>;
    fn CapturePropagationContext(&self, languageexception: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPropagationContextHead(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2>;
}
impl windows_core::RuntimeName for ILanguageExceptionErrorInfo2 {}
impl ILanguageExceptionErrorInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILanguageExceptionErrorInfo2_Vtbl
    where
        Identity: ILanguageExceptionErrorInfo2_Impl,
    {
        unsafe extern "system" fn GetPreviousLanguageExceptionErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILanguageExceptionErrorInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILanguageExceptionErrorInfo2_Impl::GetPreviousLanguageExceptionErrorInfo(this) {
                Ok(ok__) => {
                    previouslanguageexceptionerrorinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePropagationContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageexception: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILanguageExceptionErrorInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILanguageExceptionErrorInfo2_Impl::CapturePropagationContext(this, windows_core::from_raw_borrowed(&languageexception)).into()
        }
        unsafe extern "system" fn GetPropagationContextHead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILanguageExceptionErrorInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILanguageExceptionErrorInfo2_Impl::GetPropagationContextHead(this) {
                Ok(ok__) => {
                    propagatedlanguageexceptionerrorinfohead.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ILanguageExceptionErrorInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPreviousLanguageExceptionErrorInfo: GetPreviousLanguageExceptionErrorInfo::<Identity, OFFSET>,
            CapturePropagationContext: CapturePropagationContext::<Identity, OFFSET>,
            GetPropagationContextHead: GetPropagationContextHead::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo2 as windows_core::Interface>::IID || iid == &<ILanguageExceptionErrorInfo as windows_core::Interface>::IID
    }
}
pub trait ILanguageExceptionStackBackTrace_Impl: Sized {
    fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILanguageExceptionStackBackTrace {}
impl ILanguageExceptionStackBackTrace_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILanguageExceptionStackBackTrace_Vtbl
    where
        Identity: ILanguageExceptionStackBackTrace_Impl,
    {
        unsafe extern "system" fn GetStackBackTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::HRESULT
        where
            Identity: ILanguageExceptionStackBackTrace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILanguageExceptionStackBackTrace_Impl::GetStackBackTrace(this, core::mem::transmute_copy(&maxframestocapture), core::mem::transmute_copy(&stackbacktrace), core::mem::transmute_copy(&framescaptured)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetStackBackTrace: GetStackBackTrace::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionStackBackTrace as windows_core::Interface>::IID
    }
}
pub trait ILanguageExceptionTransform_Impl: Sized {
    fn GetTransformedRestrictedErrorInfo(&self) -> windows_core::Result<IRestrictedErrorInfo>;
}
impl windows_core::RuntimeName for ILanguageExceptionTransform {}
impl ILanguageExceptionTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILanguageExceptionTransform_Vtbl
    where
        Identity: ILanguageExceptionTransform_Impl,
    {
        unsafe extern "system" fn GetTransformedRestrictedErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictederrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILanguageExceptionTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILanguageExceptionTransform_Impl::GetTransformedRestrictedErrorInfo(this) {
                Ok(ok__) => {
                    restrictederrorinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTransformedRestrictedErrorInfo: GetTransformedRestrictedErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionTransform as windows_core::Interface>::IID
    }
}
pub trait IMemoryBufferByteAccess_Impl: Sized {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMemoryBufferByteAccess {}
impl IMemoryBufferByteAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMemoryBufferByteAccess_Vtbl
    where
        Identity: IMemoryBufferByteAccess_Impl,
    {
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMemoryBufferByteAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMemoryBufferByteAccess_Impl::GetBuffer(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&capacity)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBuffer: GetBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMemoryBufferByteAccess as windows_core::Interface>::IID
    }
}
pub trait IMessageDispatcher_Impl: Sized {
    fn PumpMessages(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMessageDispatcher {}
impl IMessageDispatcher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMessageDispatcher_Vtbl
    where
        Identity: IMessageDispatcher_Impl,
    {
        unsafe extern "system" fn PumpMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMessageDispatcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessageDispatcher_Impl::PumpMessages(this).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMessageDispatcher, OFFSET>(), PumpMessages: PumpMessages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessageDispatcher as windows_core::Interface>::IID
    }
}
pub trait IPlayToManagerInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, playtomanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPlayToUIForWindow(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPlayToManagerInterop {}
impl IPlayToManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPlayToManagerInterop_Vtbl
    where
        Identity: IPlayToManagerInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, playtomanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPlayToManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPlayToManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&playtomanager)).into()
        }
        unsafe extern "system" fn ShowPlayToUIForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IPlayToManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPlayToManagerInterop_Impl::ShowPlayToUIForWindow(this, core::mem::transmute_copy(&appwindow)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayToManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowPlayToUIForWindow: ShowPlayToUIForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayToManagerInterop as windows_core::Interface>::IID
    }
}
pub trait IRestrictedErrorInfo_Impl: Sized {
    fn GetErrorDetails(&self, description: *mut windows_core::BSTR, error: *mut windows_core::HRESULT, restricteddescription: *mut windows_core::BSTR, capabilitysid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetReference(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IRestrictedErrorInfo {}
impl IRestrictedErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRestrictedErrorInfo_Vtbl
    where
        Identity: IRestrictedErrorInfo_Impl,
    {
        unsafe extern "system" fn GetErrorDetails<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>, error: *mut windows_core::HRESULT, restricteddescription: *mut core::mem::MaybeUninit<windows_core::BSTR>, capabilitysid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRestrictedErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRestrictedErrorInfo_Impl::GetErrorDetails(this, core::mem::transmute_copy(&description), core::mem::transmute_copy(&error), core::mem::transmute_copy(&restricteddescription), core::mem::transmute_copy(&capabilitysid)).into()
        }
        unsafe extern "system" fn GetReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRestrictedErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRestrictedErrorInfo_Impl::GetReference(this) {
                Ok(ok__) => {
                    reference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorDetails: GetErrorDetails::<Identity, OFFSET>,
            GetReference: GetReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRestrictedErrorInfo as windows_core::Interface>::IID
    }
}
pub trait IShareWindowCommandEventArgsInterop_Impl: Sized {
    fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
}
impl windows_core::RuntimeName for IShareWindowCommandEventArgsInterop {}
impl IShareWindowCommandEventArgsInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IShareWindowCommandEventArgsInterop_Vtbl
    where
        Identity: IShareWindowCommandEventArgsInterop_Impl,
    {
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IShareWindowCommandEventArgsInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IShareWindowCommandEventArgsInterop_Impl::GetWindow(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareWindowCommandEventArgsInterop as windows_core::Interface>::IID
    }
}
pub trait IShareWindowCommandSourceInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, sharewindowcommandsource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IShareWindowCommandSourceInterop {}
impl IShareWindowCommandSourceInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IShareWindowCommandSourceInterop_Vtbl
    where
        Identity: IShareWindowCommandSourceInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, sharewindowcommandsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IShareWindowCommandSourceInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IShareWindowCommandSourceInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&sharewindowcommandsource)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareWindowCommandSourceInterop as windows_core::Interface>::IID
    }
}
pub trait ISpatialInteractionManagerInterop_Impl: Sized {
    fn GetForWindow(&self, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, spatialinteractionmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialInteractionManagerInterop {}
impl ISpatialInteractionManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialInteractionManagerInterop_Vtbl
    where
        Identity: ISpatialInteractionManagerInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, spatialinteractionmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialInteractionManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialInteractionManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&spatialinteractionmanager)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpatialInteractionManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialInteractionManagerInterop as windows_core::Interface>::IID
    }
}
pub trait ISystemMediaTransportControlsInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, mediatransportcontrol: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISystemMediaTransportControlsInterop {}
impl ISystemMediaTransportControlsInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISystemMediaTransportControlsInterop_Vtbl
    where
        Identity: ISystemMediaTransportControlsInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, mediatransportcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMediaTransportControlsInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMediaTransportControlsInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&mediatransportcontrol)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISystemMediaTransportControlsInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsInterop as windows_core::Interface>::IID
    }
}
pub trait IUIViewSettingsInterop_Impl: Sized {
    fn GetForWindow(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIViewSettingsInterop {}
impl IUIViewSettingsInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUIViewSettingsInterop_Vtbl
    where
        Identity: IUIViewSettingsInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUIViewSettingsInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIViewSettingsInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIViewSettingsInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIViewSettingsInterop as windows_core::Interface>::IID
    }
}
pub trait IUserActivityInterop_Impl: Sized {
    fn CreateSessionForWindow(&self, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUserActivityInterop {}
impl IUserActivityInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUserActivityInterop_Vtbl
    where
        Identity: IUserActivityInterop_Impl,
    {
        unsafe extern "system" fn CreateSessionForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUserActivityInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUserActivityInterop_Impl::CreateSessionForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivityInterop, OFFSET>(),
            CreateSessionForWindow: CreateSessionForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivityInterop as windows_core::Interface>::IID
    }
}
pub trait IUserActivityRequestManagerInterop_Impl: Sized {
    fn GetForWindow(&self, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUserActivityRequestManagerInterop {}
impl IUserActivityRequestManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUserActivityRequestManagerInterop_Vtbl
    where
        Identity: IUserActivityRequestManagerInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUserActivityRequestManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUserActivityRequestManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivityRequestManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivityRequestManagerInterop as windows_core::Interface>::IID
    }
}
pub trait IUserActivitySourceHostInterop_Impl: Sized {
    fn SetActivitySourceHost(&self, activitysourcehost: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUserActivitySourceHostInterop {}
impl IUserActivitySourceHostInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUserActivitySourceHostInterop_Vtbl
    where
        Identity: IUserActivitySourceHostInterop_Impl,
    {
        unsafe extern "system" fn SetActivitySourceHost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitysourcehost: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IUserActivitySourceHostInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUserActivitySourceHostInterop_Impl::SetActivitySourceHost(this, core::mem::transmute(&activitysourcehost)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivitySourceHostInterop, OFFSET>(),
            SetActivitySourceHost: SetActivitySourceHost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivitySourceHostInterop as windows_core::Interface>::IID
    }
}
pub trait IUserConsentVerifierInterop_Impl: Sized {
    fn RequestVerificationForWindowAsync(&self, appwindow: super::super::Foundation::HWND, message: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUserConsentVerifierInterop {}
impl IUserConsentVerifierInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUserConsentVerifierInterop_Vtbl
    where
        Identity: IUserConsentVerifierInterop_Impl,
    {
        unsafe extern "system" fn RequestVerificationForWindowAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: core::mem::MaybeUninit<windows_core::HSTRING>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUserConsentVerifierInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUserConsentVerifierInterop_Impl::RequestVerificationForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&message), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserConsentVerifierInterop, OFFSET>(),
            RequestVerificationForWindowAsync: RequestVerificationForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserConsentVerifierInterop as windows_core::Interface>::IID
    }
}
pub trait IWeakReference_Impl: Sized {
    fn Resolve(&self, riid: *const windows_core::GUID, objectreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWeakReference {}
impl IWeakReference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWeakReference_Vtbl
    where
        Identity: IWeakReference_Impl,
    {
        unsafe extern "system" fn Resolve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, objectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWeakReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWeakReference_Impl::Resolve(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&objectreference)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReference as windows_core::Interface>::IID
    }
}
pub trait IWeakReferenceSource_Impl: Sized {
    fn GetWeakReference(&self) -> windows_core::Result<IWeakReference>;
}
impl windows_core::RuntimeName for IWeakReferenceSource {}
impl IWeakReferenceSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWeakReferenceSource_Vtbl
    where
        Identity: IWeakReferenceSource_Impl,
    {
        unsafe extern "system" fn GetWeakReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, weakreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWeakReferenceSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWeakReferenceSource_Impl::GetWeakReference(this) {
                Ok(ok__) => {
                    weakreference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWeakReference: GetWeakReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReferenceSource as windows_core::Interface>::IID
    }
}
pub trait IWebAuthenticationCoreManagerInterop_Impl: Sized {
    fn RequestTokenForWindowAsync(&self, appwindow: super::super::Foundation::HWND, request: Option<&windows_core::IInspectable>, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestTokenWithWebAccountForWindowAsync(&self, appwindow: super::super::Foundation::HWND, request: Option<&windows_core::IInspectable>, webaccount: Option<&windows_core::IInspectable>, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWebAuthenticationCoreManagerInterop {}
impl IWebAuthenticationCoreManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebAuthenticationCoreManagerInterop_Vtbl
    where
        Identity: IWebAuthenticationCoreManagerInterop_Impl,
    {
        unsafe extern "system" fn RequestTokenForWindowAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAuthenticationCoreManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAuthenticationCoreManagerInterop_Impl::RequestTokenForWindowAsync(this, core::mem::transmute_copy(&appwindow), windows_core::from_raw_borrowed(&request), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncinfo)).into()
        }
        unsafe extern "system" fn RequestTokenWithWebAccountForWindowAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut core::ffi::c_void, webaccount: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebAuthenticationCoreManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebAuthenticationCoreManagerInterop_Impl::RequestTokenWithWebAccountForWindowAsync(this, core::mem::transmute_copy(&appwindow), windows_core::from_raw_borrowed(&request), windows_core::from_raw_borrowed(&webaccount), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncinfo)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAuthenticationCoreManagerInterop, OFFSET>(),
            RequestTokenForWindowAsync: RequestTokenForWindowAsync::<Identity, OFFSET>,
            RequestTokenWithWebAccountForWindowAsync: RequestTokenWithWebAccountForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerInterop as windows_core::Interface>::IID
    }
}
