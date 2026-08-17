#[cfg(feature = "Win32_System_Com")]
pub trait IADsTSUserEx_Impl: Sized + super::Com::IDispatch_Impl {
    fn TerminalServicesProfilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesProfilePath(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TerminalServicesHomeDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesHomeDirectory(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TerminalServicesHomeDrive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesHomeDrive(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowLogon(&self) -> windows_core::Result<i32>;
    fn SetAllowLogon(&self, newval: i32) -> windows_core::Result<()>;
    fn EnableRemoteControl(&self) -> windows_core::Result<i32>;
    fn SetEnableRemoteControl(&self, newval: i32) -> windows_core::Result<()>;
    fn MaxDisconnectionTime(&self) -> windows_core::Result<i32>;
    fn SetMaxDisconnectionTime(&self, newval: i32) -> windows_core::Result<()>;
    fn MaxConnectionTime(&self) -> windows_core::Result<i32>;
    fn SetMaxConnectionTime(&self, newval: i32) -> windows_core::Result<()>;
    fn MaxIdleTime(&self) -> windows_core::Result<i32>;
    fn SetMaxIdleTime(&self, newval: i32) -> windows_core::Result<()>;
    fn ReconnectionAction(&self) -> windows_core::Result<i32>;
    fn SetReconnectionAction(&self, newval: i32) -> windows_core::Result<()>;
    fn BrokenConnectionAction(&self) -> windows_core::Result<i32>;
    fn SetBrokenConnectionAction(&self, newval: i32) -> windows_core::Result<()>;
    fn ConnectClientDrivesAtLogon(&self) -> windows_core::Result<i32>;
    fn SetConnectClientDrivesAtLogon(&self, newval: i32) -> windows_core::Result<()>;
    fn ConnectClientPrintersAtLogon(&self) -> windows_core::Result<i32>;
    fn SetConnectClientPrintersAtLogon(&self, newval: i32) -> windows_core::Result<()>;
    fn DefaultToMainPrinter(&self) -> windows_core::Result<i32>;
    fn SetDefaultToMainPrinter(&self, newval: i32) -> windows_core::Result<()>;
    fn TerminalServicesWorkDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesWorkDirectory(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TerminalServicesInitialProgram(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesInitialProgram(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsTSUserEx {}
#[cfg(feature = "Win32_System_Com")]
impl IADsTSUserEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IADsTSUserEx_Vtbl
    where
        Identity: IADsTSUserEx_Impl,
    {
        unsafe extern "system" fn TerminalServicesProfilePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::TerminalServicesProfilePath(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesProfilePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetTerminalServicesProfilePath(this, core::mem::transmute(&pnewval)).into()
        }
        unsafe extern "system" fn TerminalServicesHomeDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::TerminalServicesHomeDirectory(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetTerminalServicesHomeDirectory(this, core::mem::transmute(&pnewval)).into()
        }
        unsafe extern "system" fn TerminalServicesHomeDrive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::TerminalServicesHomeDrive(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDrive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetTerminalServicesHomeDrive(this, core::mem::transmute(&pnewval)).into()
        }
        unsafe extern "system" fn AllowLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::AllowLogon(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetAllowLogon(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn EnableRemoteControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::EnableRemoteControl(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableRemoteControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetEnableRemoteControl(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxDisconnectionTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::MaxDisconnectionTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDisconnectionTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetMaxDisconnectionTime(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxConnectionTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::MaxConnectionTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxConnectionTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetMaxConnectionTime(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxIdleTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::MaxIdleTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxIdleTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetMaxIdleTime(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ReconnectionAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::ReconnectionAction(this) {
                Ok(ok__) => {
                    pnewval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReconnectionAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetReconnectionAction(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn BrokenConnectionAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::BrokenConnectionAction(this) {
                Ok(ok__) => {
                    pnewval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrokenConnectionAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetBrokenConnectionAction(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ConnectClientDrivesAtLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::ConnectClientDrivesAtLogon(this) {
                Ok(ok__) => {
                    pnewval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectClientDrivesAtLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetConnectClientDrivesAtLogon(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ConnectClientPrintersAtLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::ConnectClientPrintersAtLogon(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectClientPrintersAtLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetConnectClientPrintersAtLogon(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DefaultToMainPrinter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::DefaultToMainPrinter(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultToMainPrinter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetDefaultToMainPrinter(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn TerminalServicesWorkDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::TerminalServicesWorkDirectory(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesWorkDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetTerminalServicesWorkDirectory(this, core::mem::transmute(&pnewval)).into()
        }
        unsafe extern "system" fn TerminalServicesInitialProgram<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IADsTSUserEx_Impl::TerminalServicesInitialProgram(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesInitialProgram<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IADsTSUserEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADsTSUserEx_Impl::SetTerminalServicesInitialProgram(this, core::mem::transmute(&pnewval)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TerminalServicesProfilePath: TerminalServicesProfilePath::<Identity, OFFSET>,
            SetTerminalServicesProfilePath: SetTerminalServicesProfilePath::<Identity, OFFSET>,
            TerminalServicesHomeDirectory: TerminalServicesHomeDirectory::<Identity, OFFSET>,
            SetTerminalServicesHomeDirectory: SetTerminalServicesHomeDirectory::<Identity, OFFSET>,
            TerminalServicesHomeDrive: TerminalServicesHomeDrive::<Identity, OFFSET>,
            SetTerminalServicesHomeDrive: SetTerminalServicesHomeDrive::<Identity, OFFSET>,
            AllowLogon: AllowLogon::<Identity, OFFSET>,
            SetAllowLogon: SetAllowLogon::<Identity, OFFSET>,
            EnableRemoteControl: EnableRemoteControl::<Identity, OFFSET>,
            SetEnableRemoteControl: SetEnableRemoteControl::<Identity, OFFSET>,
            MaxDisconnectionTime: MaxDisconnectionTime::<Identity, OFFSET>,
            SetMaxDisconnectionTime: SetMaxDisconnectionTime::<Identity, OFFSET>,
            MaxConnectionTime: MaxConnectionTime::<Identity, OFFSET>,
            SetMaxConnectionTime: SetMaxConnectionTime::<Identity, OFFSET>,
            MaxIdleTime: MaxIdleTime::<Identity, OFFSET>,
            SetMaxIdleTime: SetMaxIdleTime::<Identity, OFFSET>,
            ReconnectionAction: ReconnectionAction::<Identity, OFFSET>,
            SetReconnectionAction: SetReconnectionAction::<Identity, OFFSET>,
            BrokenConnectionAction: BrokenConnectionAction::<Identity, OFFSET>,
            SetBrokenConnectionAction: SetBrokenConnectionAction::<Identity, OFFSET>,
            ConnectClientDrivesAtLogon: ConnectClientDrivesAtLogon::<Identity, OFFSET>,
            SetConnectClientDrivesAtLogon: SetConnectClientDrivesAtLogon::<Identity, OFFSET>,
            ConnectClientPrintersAtLogon: ConnectClientPrintersAtLogon::<Identity, OFFSET>,
            SetConnectClientPrintersAtLogon: SetConnectClientPrintersAtLogon::<Identity, OFFSET>,
            DefaultToMainPrinter: DefaultToMainPrinter::<Identity, OFFSET>,
            SetDefaultToMainPrinter: SetDefaultToMainPrinter::<Identity, OFFSET>,
            TerminalServicesWorkDirectory: TerminalServicesWorkDirectory::<Identity, OFFSET>,
            SetTerminalServicesWorkDirectory: SetTerminalServicesWorkDirectory::<Identity, OFFSET>,
            TerminalServicesInitialProgram: TerminalServicesInitialProgram::<Identity, OFFSET>,
            SetTerminalServicesInitialProgram: SetTerminalServicesInitialProgram::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTSUserEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IAudioDeviceEndpoint_Impl: Sized {
    fn SetBuffer(&self, maxperiod: i64, u32latencycoefficient: u32) -> windows_core::Result<()>;
    fn GetRTCaps(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetEventDrivenCapable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn WriteExclusiveModeParametersToSharedMemory(&self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioDeviceEndpoint {}
impl IAudioDeviceEndpoint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioDeviceEndpoint_Vtbl
    where
        Identity: IAudioDeviceEndpoint_Impl,
    {
        unsafe extern "system" fn SetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> windows_core::HRESULT
        where
            Identity: IAudioDeviceEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioDeviceEndpoint_Impl::SetBuffer(this, core::mem::transmute_copy(&maxperiod), core::mem::transmute_copy(&u32latencycoefficient)).into()
        }
        unsafe extern "system" fn GetRTCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisrtcapable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioDeviceEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioDeviceEndpoint_Impl::GetRTCaps(this) {
                Ok(ok__) => {
                    pbisrtcapable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventDrivenCapable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbiseventcapable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioDeviceEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioDeviceEndpoint_Impl::GetEventDrivenCapable(this) {
                Ok(ok__) => {
                    pbiseventcapable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteExclusiveModeParametersToSharedMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> windows_core::HRESULT
        where
            Identity: IAudioDeviceEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioDeviceEndpoint_Impl::WriteExclusiveModeParametersToSharedMemory(this, core::mem::transmute_copy(&htargetprocess), core::mem::transmute_copy(&hnsperiod), core::mem::transmute_copy(&hnsbufferduration), core::mem::transmute_copy(&u32latencycoefficient), core::mem::transmute_copy(&pu32sharedmemorysize), core::mem::transmute_copy(&phsharedmemory)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBuffer: SetBuffer::<Identity, OFFSET>,
            GetRTCaps: GetRTCaps::<Identity, OFFSET>,
            GetEventDrivenCapable: GetEventDrivenCapable::<Identity, OFFSET>,
            WriteExclusiveModeParametersToSharedMemory: WriteExclusiveModeParametersToSharedMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioDeviceEndpoint as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IAudioEndpoint_Impl: Sized {
    fn GetFrameFormat(&self) -> windows_core::Result<*mut super::super::Media::Audio::WAVEFORMATEX>;
    fn GetFramesPerPacket(&self) -> windows_core::Result<u32>;
    fn GetLatency(&self) -> windows_core::Result<i64>;
    fn SetStreamFlags(&self, streamflags: u32) -> windows_core::Result<()>;
    fn SetEventHandle(&self, eventhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IAudioEndpoint {}
#[cfg(feature = "Win32_Media_Audio")]
impl IAudioEndpoint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpoint_Vtbl
    where
        Identity: IAudioEndpoint_Impl,
    {
        unsafe extern "system" fn GetFrameFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IAudioEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpoint_Impl::GetFrameFormat(this) {
                Ok(ok__) => {
                    ppformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesPerPacket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframesperpacket: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpoint_Impl::GetFramesPerPacket(this) {
                Ok(ok__) => {
                    pframesperpacket.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, platency: *mut i64) -> windows_core::HRESULT
        where
            Identity: IAudioEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpoint_Impl::GetLatency(this) {
                Ok(ok__) => {
                    platency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamflags: u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpoint_Impl::SetStreamFlags(this, core::mem::transmute_copy(&streamflags)).into()
        }
        unsafe extern "system" fn SetEventHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IAudioEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpoint_Impl::SetEventHandle(this, core::mem::transmute_copy(&eventhandle)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameFormat: GetFrameFormat::<Identity, OFFSET>,
            GetFramesPerPacket: GetFramesPerPacket::<Identity, OFFSET>,
            GetLatency: GetLatency::<Identity, OFFSET>,
            SetStreamFlags: SetStreamFlags::<Identity, OFFSET>,
            SetEventHandle: SetEventHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpoint as windows_core::Interface>::IID
    }
}
pub trait IAudioEndpointControl_Impl: Sized {
    fn Start(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEndpointControl {}
impl IAudioEndpointControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointControl_Vtbl
    where
        Identity: IAudioEndpointControl_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointControl_Impl::Start(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointControl_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointControl_Impl::Stop(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointControl as windows_core::Interface>::IID
    }
}
pub trait IAudioEndpointRT_Impl: Sized {
    fn GetCurrentPadding(&self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION);
    fn ProcessingComplete(&self);
    fn SetPinInactive(&self) -> windows_core::Result<()>;
    fn SetPinActive(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEndpointRT {}
impl IAudioEndpointRT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointRT_Vtbl
    where
        Identity: IAudioEndpointRT_Impl,
    {
        unsafe extern "system" fn GetCurrentPadding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION)
        where
            Identity: IAudioEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointRT_Impl::GetCurrentPadding(this, core::mem::transmute_copy(&ppadding), core::mem::transmute_copy(&paecurrentposition))
        }
        unsafe extern "system" fn ProcessingComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IAudioEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointRT_Impl::ProcessingComplete(this)
        }
        unsafe extern "system" fn SetPinInactive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointRT_Impl::SetPinInactive(this).into()
        }
        unsafe extern "system" fn SetPinActive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointRT_Impl::SetPinActive(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentPadding: GetCurrentPadding::<Identity, OFFSET>,
            ProcessingComplete: ProcessingComplete::<Identity, OFFSET>,
            SetPinInactive: SetPinInactive::<Identity, OFFSET>,
            SetPinActive: SetPinActive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointRT as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioInputEndpointRT_Impl: Sized {
    fn GetInputDataPointer(&self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION);
    fn ReleaseInputDataPointer(&self, u32framecount: u32, pdatapointer: usize);
    fn PulseEndpoint(&self);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl windows_core::RuntimeName for IAudioInputEndpointRT {}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioInputEndpointRT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioInputEndpointRT_Vtbl
    where
        Identity: IAudioInputEndpointRT_Impl,
    {
        unsafe extern "system" fn GetInputDataPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION)
        where
            Identity: IAudioInputEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioInputEndpointRT_Impl::GetInputDataPointer(this, core::mem::transmute_copy(&pconnectionproperty), core::mem::transmute_copy(&paetimestamp))
        }
        unsafe extern "system" fn ReleaseInputDataPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32framecount: u32, pdatapointer: usize)
        where
            Identity: IAudioInputEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioInputEndpointRT_Impl::ReleaseInputDataPointer(this, core::mem::transmute_copy(&u32framecount), core::mem::transmute_copy(&pdatapointer))
        }
        unsafe extern "system" fn PulseEndpoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IAudioInputEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioInputEndpointRT_Impl::PulseEndpoint(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputDataPointer: GetInputDataPointer::<Identity, OFFSET>,
            ReleaseInputDataPointer: ReleaseInputDataPointer::<Identity, OFFSET>,
            PulseEndpoint: PulseEndpoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioInputEndpointRT as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioOutputEndpointRT_Impl: Sized {
    fn GetOutputDataPointer(&self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize;
    fn ReleaseOutputDataPointer(&self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY);
    fn PulseEndpoint(&self);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl windows_core::RuntimeName for IAudioOutputEndpointRT {}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioOutputEndpointRT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioOutputEndpointRT_Vtbl
    where
        Identity: IAudioOutputEndpointRT_Impl,
    {
        unsafe extern "system" fn GetOutputDataPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize
        where
            Identity: IAudioOutputEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioOutputEndpointRT_Impl::GetOutputDataPointer(this, core::mem::transmute_copy(&u32framecount), core::mem::transmute_copy(&paetimestamp))
        }
        unsafe extern "system" fn ReleaseOutputDataPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY)
        where
            Identity: IAudioOutputEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioOutputEndpointRT_Impl::ReleaseOutputDataPointer(this, core::mem::transmute_copy(&pconnectionproperty))
        }
        unsafe extern "system" fn PulseEndpoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IAudioOutputEndpointRT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioOutputEndpointRT_Impl::PulseEndpoint(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputDataPointer: GetOutputDataPointer::<Identity, OFFSET>,
            ReleaseOutputDataPointer: ReleaseOutputDataPointer::<Identity, OFFSET>,
            PulseEndpoint: PulseEndpoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioOutputEndpointRT as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Reconnect(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn Settings(&self) -> windows_core::Result<IRemoteDesktopClientSettings>;
    fn Actions(&self) -> windows_core::Result<IRemoteDesktopClientActions>;
    fn TouchPointer(&self) -> windows_core::Result<IRemoteDesktopClientTouchPointer>;
    fn DeleteSavedCredentials(&self, servername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UpdateSessionDisplaySettings(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn attachEvent(&self, eventname: &windows_core::BSTR, callback: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn detachEvent(&self, eventname: &windows_core::BSTR, callback: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRemoteDesktopClient {}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRemoteDesktopClient_Vtbl
    where
        Identity: IRemoteDesktopClient_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClient_Impl::Connect(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClient_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn Reconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClient_Impl::Reconnect(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Settings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClient_Impl::Settings(this) {
                Ok(ok__) => {
                    settings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Actions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClient_Impl::Actions(this) {
                Ok(ok__) => {
                    actions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, touchpointer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClient_Impl::TouchPointer(this) {
                Ok(ok__) => {
                    touchpointer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSavedCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClient_Impl::DeleteSavedCredentials(this, core::mem::transmute(&servername)).into()
        }
        unsafe extern "system" fn UpdateSessionDisplaySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClient_Impl::UpdateSessionDisplaySettings(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn attachEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::BSTR>, callback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClient_Impl::attachEvent(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&callback)).into()
        }
        unsafe extern "system" fn detachEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::BSTR>, callback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClient_Impl::detachEvent(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&callback)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Reconnect: Reconnect::<Identity, OFFSET>,
            Settings: Settings::<Identity, OFFSET>,
            Actions: Actions::<Identity, OFFSET>,
            TouchPointer: TouchPointer::<Identity, OFFSET>,
            DeleteSavedCredentials: DeleteSavedCredentials::<Identity, OFFSET>,
            UpdateSessionDisplaySettings: UpdateSessionDisplaySettings::<Identity, OFFSET>,
            attachEvent: attachEvent::<Identity, OFFSET>,
            detachEvent: detachEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClient as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientActions_Impl: Sized + super::Com::IDispatch_Impl {
    fn SuspendScreenUpdates(&self) -> windows_core::Result<()>;
    fn ResumeScreenUpdates(&self) -> windows_core::Result<()>;
    fn ExecuteRemoteAction(&self, remoteaction: RemoteActionType) -> windows_core::Result<()>;
    fn GetSnapshot(&self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRemoteDesktopClientActions {}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientActions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRemoteDesktopClientActions_Vtbl
    where
        Identity: IRemoteDesktopClientActions_Impl,
    {
        unsafe extern "system" fn SuspendScreenUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientActions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientActions_Impl::SuspendScreenUpdates(this).into()
        }
        unsafe extern "system" fn ResumeScreenUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientActions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientActions_Impl::ResumeScreenUpdates(this).into()
        }
        unsafe extern "system" fn ExecuteRemoteAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaction: RemoteActionType) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientActions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientActions_Impl::ExecuteRemoteAction(this, core::mem::transmute_copy(&remoteaction)).into()
        }
        unsafe extern "system" fn GetSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientActions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClientActions_Impl::GetSnapshot(this, core::mem::transmute_copy(&snapshotencoding), core::mem::transmute_copy(&snapshotformat), core::mem::transmute_copy(&snapshotwidth), core::mem::transmute_copy(&snapshotheight)) {
                Ok(ok__) => {
                    snapshotdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SuspendScreenUpdates: SuspendScreenUpdates::<Identity, OFFSET>,
            ResumeScreenUpdates: ResumeScreenUpdates::<Identity, OFFSET>,
            ExecuteRemoteAction: ExecuteRemoteAction::<Identity, OFFSET>,
            GetSnapshot: GetSnapshot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClientActions as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn ApplySettings(&self, rdpfilecontents: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RetrieveSettings(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRdpProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn SetRdpProperty(&self, propertyname: &windows_core::BSTR, value: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRemoteDesktopClientSettings {}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRemoteDesktopClientSettings_Vtbl
    where
        Identity: IRemoteDesktopClientSettings_Impl,
    {
        unsafe extern "system" fn ApplySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdpfilecontents: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientSettings_Impl::ApplySettings(this, core::mem::transmute(&rdpfilecontents)).into()
        }
        unsafe extern "system" fn RetrieveSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdpfilecontents: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClientSettings_Impl::RetrieveSettings(this) {
                Ok(ok__) => {
                    rdpfilecontents.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRdpProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClientSettings_Impl::GetRdpProperty(this, core::mem::transmute(&propertyname)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRdpProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientSettings_Impl::SetRdpProperty(this, core::mem::transmute(&propertyname), core::mem::transmute(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ApplySettings: ApplySettings::<Identity, OFFSET>,
            RetrieveSettings: RetrieveSettings::<Identity, OFFSET>,
            GetRdpProperty: GetRdpProperty::<Identity, OFFSET>,
            SetRdpProperty: SetRdpProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClientSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientTouchPointer_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventsEnabled(&self, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EventsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPointerSpeed(&self, pointerspeed: u32) -> windows_core::Result<()>;
    fn PointerSpeed(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRemoteDesktopClientTouchPointer {}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientTouchPointer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRemoteDesktopClientTouchPointer_Vtbl
    where
        Identity: IRemoteDesktopClientTouchPointer_Impl,
    {
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientTouchPointer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientTouchPointer_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientTouchPointer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClientTouchPointer_Impl::Enabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientTouchPointer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientTouchPointer_Impl::SetEventsEnabled(this, core::mem::transmute_copy(&eventsenabled)).into()
        }
        unsafe extern "system" fn EventsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientTouchPointer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClientTouchPointer_Impl::EventsEnabled(this) {
                Ok(ok__) => {
                    eventsenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerSpeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerspeed: u32) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientTouchPointer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteDesktopClientTouchPointer_Impl::SetPointerSpeed(this, core::mem::transmute_copy(&pointerspeed)).into()
        }
        unsafe extern "system" fn PointerSpeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerspeed: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRemoteDesktopClientTouchPointer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRemoteDesktopClientTouchPointer_Impl::PointerSpeed(this) {
                Ok(ok__) => {
                    pointerspeed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEventsEnabled: SetEventsEnabled::<Identity, OFFSET>,
            EventsEnabled: EventsEnabled::<Identity, OFFSET>,
            SetPointerSpeed: SetPointerSpeed::<Identity, OFFSET>,
            PointerSpeed: PointerSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClientTouchPointer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRemoteSystemAdditionalInfoProvider_Impl: Sized {
    fn GetAdditionalInfo(&self, deduplicationid: *mut windows_core::HSTRING, riid: *const windows_core::GUID, mapview: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRemoteSystemAdditionalInfoProvider {}
impl IRemoteSystemAdditionalInfoProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRemoteSystemAdditionalInfoProvider_Vtbl
    where
        Identity: IRemoteSystemAdditionalInfoProvider_Impl,
    {
        unsafe extern "system" fn GetAdditionalInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deduplicationid: *mut core::mem::MaybeUninit<windows_core::HSTRING>, riid: *const windows_core::GUID, mapview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRemoteSystemAdditionalInfoProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteSystemAdditionalInfoProvider_Impl::GetAdditionalInfo(this, core::mem::transmute_copy(&deduplicationid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&mapview)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAdditionalInfo: GetAdditionalInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteSystemAdditionalInfoProvider as windows_core::Interface>::IID
    }
}
pub trait ITSGAccountingEngine_Impl: Sized {
    fn DoAccounting(&self, accountingdatatype: AAAccountingDataType, accountingdata: &AAAccountingData) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITSGAccountingEngine {}
impl ITSGAccountingEngine_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSGAccountingEngine_Vtbl
    where
        Identity: ITSGAccountingEngine_Impl,
    {
        unsafe extern "system" fn DoAccounting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: AAAccountingData) -> windows_core::HRESULT
        where
            Identity: ITSGAccountingEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAccountingEngine_Impl::DoAccounting(this, core::mem::transmute_copy(&accountingdatatype), core::mem::transmute(&accountingdata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoAccounting: DoAccounting::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAccountingEngine as windows_core::Interface>::IID
    }
}
pub trait ITSGAuthenticateUserSink_Impl: Sized {
    fn OnUserAuthenticated(&self, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn OnUserAuthenticationFailed(&self, context: usize, genericerrorcode: windows_core::HRESULT, specificerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ReauthenticateUser(&self, context: usize) -> windows_core::Result<()>;
    fn DisconnectUser(&self, context: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITSGAuthenticateUserSink {}
impl ITSGAuthenticateUserSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSGAuthenticateUserSink_Vtbl
    where
        Identity: ITSGAuthenticateUserSink_Impl,
    {
        unsafe extern "system" fn OnUserAuthenticated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::BSTR>, userdomain: core::mem::MaybeUninit<windows_core::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: ITSGAuthenticateUserSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthenticateUserSink_Impl::OnUserAuthenticated(this, core::mem::transmute(&username), core::mem::transmute(&userdomain), core::mem::transmute_copy(&context), core::mem::transmute_copy(&usertoken)).into()
        }
        unsafe extern "system" fn OnUserAuthenticationFailed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: usize, genericerrorcode: windows_core::HRESULT, specificerrorcode: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITSGAuthenticateUserSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthenticateUserSink_Impl::OnUserAuthenticationFailed(this, core::mem::transmute_copy(&context), core::mem::transmute_copy(&genericerrorcode), core::mem::transmute_copy(&specificerrorcode)).into()
        }
        unsafe extern "system" fn ReauthenticateUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: usize) -> windows_core::HRESULT
        where
            Identity: ITSGAuthenticateUserSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthenticateUserSink_Impl::ReauthenticateUser(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn DisconnectUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: usize) -> windows_core::HRESULT
        where
            Identity: ITSGAuthenticateUserSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthenticateUserSink_Impl::DisconnectUser(this, core::mem::transmute_copy(&context)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUserAuthenticated: OnUserAuthenticated::<Identity, OFFSET>,
            OnUserAuthenticationFailed: OnUserAuthenticationFailed::<Identity, OFFSET>,
            ReauthenticateUser: ReauthenticateUser::<Identity, OFFSET>,
            DisconnectUser: DisconnectUser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthenticateUserSink as windows_core::Interface>::IID
    }
}
pub trait ITSGAuthenticationEngine_Impl: Sized {
    fn AuthenticateUser(&self, mainsessionid: &windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: Option<&ITSGAuthenticateUserSink>) -> windows_core::Result<()>;
    fn CancelAuthentication(&self, mainsessionid: &windows_core::GUID, context: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITSGAuthenticationEngine {}
impl ITSGAuthenticationEngine_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSGAuthenticationEngine_Vtbl
    where
        Identity: ITSGAuthenticationEngine_Impl,
    {
        unsafe extern "system" fn AuthenticateUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSGAuthenticationEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthenticationEngine_Impl::AuthenticateUser(this, core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&cookiedata), core::mem::transmute_copy(&numcookiebytes), core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn CancelAuthentication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, context: usize) -> windows_core::HRESULT
        where
            Identity: ITSGAuthenticationEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthenticationEngine_Impl::CancelAuthentication(this, core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&context)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AuthenticateUser: AuthenticateUser::<Identity, OFFSET>,
            CancelAuthentication: CancelAuthentication::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthenticationEngine as windows_core::Interface>::IID
    }
}
pub trait ITSGAuthorizeConnectionSink_Impl: Sized {
    fn OnConnectionAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: &windows_core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITSGAuthorizeConnectionSink {}
impl ITSGAuthorizeConnectionSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSGAuthorizeConnectionSink_Vtbl
    where
        Identity: ITSGAuthorizeConnectionSink_Impl,
    {
        unsafe extern "system" fn OnConnectionAuthorized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> windows_core::HRESULT
        where
            Identity: ITSGAuthorizeConnectionSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthorizeConnectionSink_Impl::OnConnectionAuthorized(this, core::mem::transmute_copy(&hrin), core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&cbsohresponse), core::mem::transmute_copy(&pbsohresponse), core::mem::transmute_copy(&idletimeout), core::mem::transmute_copy(&sessiontimeout), core::mem::transmute_copy(&sessiontimeoutaction), core::mem::transmute_copy(&trustclass), core::mem::transmute_copy(&policyattributes)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnectionAuthorized: OnConnectionAuthorized::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthorizeConnectionSink as windows_core::Interface>::IID
    }
}
pub trait ITSGAuthorizeResourceSink_Impl: Sized {
    fn OnChannelAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: &windows_core::GUID, subsessionid: i32, allowedresourcenames: *const windows_core::BSTR, numallowedresourcenames: u32, failedresourcenames: *const windows_core::BSTR, numfailedresourcenames: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITSGAuthorizeResourceSink {}
impl ITSGAuthorizeResourceSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSGAuthorizeResourceSink_Vtbl
    where
        Identity: ITSGAuthorizeResourceSink_Impl,
    {
        unsafe extern "system" fn OnChannelAuthorized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, subsessionid: i32, allowedresourcenames: *const core::mem::MaybeUninit<windows_core::BSTR>, numallowedresourcenames: u32, failedresourcenames: *const core::mem::MaybeUninit<windows_core::BSTR>, numfailedresourcenames: u32) -> windows_core::HRESULT
        where
            Identity: ITSGAuthorizeResourceSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGAuthorizeResourceSink_Impl::OnChannelAuthorized(this, core::mem::transmute_copy(&hrin), core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&subsessionid), core::mem::transmute_copy(&allowedresourcenames), core::mem::transmute_copy(&numallowedresourcenames), core::mem::transmute_copy(&failedresourcenames), core::mem::transmute_copy(&numfailedresourcenames)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChannelAuthorized: OnChannelAuthorized::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthorizeResourceSink as windows_core::Interface>::IID
    }
}
pub trait ITSGPolicyEngine_Impl: Sized {
    fn AuthorizeConnection(&self, mainsessionid: &windows_core::GUID, username: &windows_core::BSTR, authtype: AAAuthSchemes, clientmachineip: &windows_core::BSTR, clientmachinename: &windows_core::BSTR, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: Option<&ITSGAuthorizeConnectionSink>) -> windows_core::Result<()>;
    fn AuthorizeResource(&self, mainsessionid: &windows_core::GUID, subsessionid: i32, username: &windows_core::BSTR, resourcenames: *const windows_core::BSTR, numresources: u32, alternateresourcenames: *const windows_core::BSTR, numalternateresourcename: u32, portnumber: u32, operation: &windows_core::BSTR, cookie: *const u8, numbytesincookie: u32, psink: Option<&ITSGAuthorizeResourceSink>) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn IsQuarantineEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITSGPolicyEngine {}
impl ITSGPolicyEngine_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITSGPolicyEngine_Vtbl
    where
        Identity: ITSGPolicyEngine_Impl,
    {
        unsafe extern "system" fn AuthorizeConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, username: core::mem::MaybeUninit<windows_core::BSTR>, authtype: AAAuthSchemes, clientmachineip: core::mem::MaybeUninit<windows_core::BSTR>, clientmachinename: core::mem::MaybeUninit<windows_core::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSGPolicyEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGPolicyEngine_Impl::AuthorizeConnection(this, core::mem::transmute(&mainsessionid), core::mem::transmute(&username), core::mem::transmute_copy(&authtype), core::mem::transmute(&clientmachineip), core::mem::transmute(&clientmachinename), core::mem::transmute_copy(&sohdata), core::mem::transmute_copy(&numsohbytes), core::mem::transmute_copy(&cookiedata), core::mem::transmute_copy(&numcookiebytes), core::mem::transmute_copy(&usertoken), windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn AuthorizeResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, subsessionid: i32, username: core::mem::MaybeUninit<windows_core::BSTR>, resourcenames: *const core::mem::MaybeUninit<windows_core::BSTR>, numresources: u32, alternateresourcenames: *const core::mem::MaybeUninit<windows_core::BSTR>, numalternateresourcename: u32, portnumber: u32, operation: core::mem::MaybeUninit<windows_core::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSGPolicyEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGPolicyEngine_Impl::AuthorizeResource(
                this,
                core::mem::transmute(&mainsessionid),
                core::mem::transmute_copy(&subsessionid),
                core::mem::transmute(&username),
                core::mem::transmute_copy(&resourcenames),
                core::mem::transmute_copy(&numresources),
                core::mem::transmute_copy(&alternateresourcenames),
                core::mem::transmute_copy(&numalternateresourcename),
                core::mem::transmute_copy(&portnumber),
                core::mem::transmute(&operation),
                core::mem::transmute_copy(&cookie),
                core::mem::transmute_copy(&numbytesincookie),
                windows_core::from_raw_borrowed(&psink),
            )
            .into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITSGPolicyEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITSGPolicyEngine_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn IsQuarantineEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, quarantineenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITSGPolicyEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITSGPolicyEngine_Impl::IsQuarantineEnabled(this) {
                Ok(ok__) => {
                    quarantineenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AuthorizeConnection: AuthorizeConnection::<Identity, OFFSET>,
            AuthorizeResource: AuthorizeResource::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            IsQuarantineEnabled: IsQuarantineEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGPolicyEngine as windows_core::Interface>::IID
    }
}
pub trait ITsSbBaseNotifySink_Impl: Sized {
    fn OnError(&self, hrerror: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbBaseNotifySink {}
impl ITsSbBaseNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbBaseNotifySink_Vtbl
    where
        Identity: ITsSbBaseNotifySink_Impl,
    {
        unsafe extern "system" fn OnError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITsSbBaseNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbBaseNotifySink_Impl::OnError(this, core::mem::transmute_copy(&hrerror)).into()
        }
        unsafe extern "system" fn OnReportStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> windows_core::HRESULT
        where
            Identity: ITsSbBaseNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbBaseNotifySink_Impl::OnReportStatus(this, core::mem::transmute_copy(&messagetype), core::mem::transmute_copy(&messageid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnError: OnError::<Identity, OFFSET>,
            OnReportStatus: OnReportStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbClientConnection_Impl: Sized {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LoadBalanceResult(&self) -> windows_core::Result<ITsSbLoadBalanceResult>;
    fn FarmName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PutContext(&self, contextid: &windows_core::BSTR, context: &windows_core::VARIANT, existingcontext: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetContext(&self, contextid: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn Environment(&self) -> windows_core::Result<ITsSbEnvironment>;
    fn get_ConnectionError(&self) -> windows_core::Result<()>;
    fn SamUserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClientConnectionPropertySet(&self) -> windows_core::Result<ITsSbClientConnectionPropertySet>;
    fn IsFirstAssignment(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn RdFarmType(&self) -> windows_core::Result<RD_FARM_TYPE>;
    fn UserSidString(&self) -> windows_core::Result<*mut i8>;
    fn GetDisconnectedSession(&self) -> windows_core::Result<ITsSbSession>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbClientConnection {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbClientConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbClientConnection_Vtbl
    where
        Identity: ITsSbClientConnection_Impl,
    {
        unsafe extern "system" fn UserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::UserName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::Domain(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialProgram<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::InitialProgram(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadBalanceResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::LoadBalanceResult(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FarmName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::FarmName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextid: core::mem::MaybeUninit<windows_core::BSTR>, context: core::mem::MaybeUninit<windows_core::VARIANT>, existingcontext: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbClientConnection_Impl::PutContext(this, core::mem::transmute(&contextid), core::mem::transmute(&context), core::mem::transmute_copy(&existingcontext)).into()
        }
        unsafe extern "system" fn GetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextid: core::mem::MaybeUninit<windows_core::BSTR>, context: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::GetContext(this, core::mem::transmute(&contextid)) {
                Ok(ok__) => {
                    context.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Environment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenvironment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::Environment(this) {
                Ok(ok__) => {
                    ppenvironment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ConnectionError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbClientConnection_Impl::get_ConnectionError(this).into()
        }
        unsafe extern "system" fn SamUserAccount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::SamUserAccount(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientConnectionPropertySet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::ClientConnectionPropertySet(this) {
                Ok(ok__) => {
                    pppropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstAssignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::IsFirstAssignment(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RdFarmType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::RdFarmType(this) {
                Ok(ok__) => {
                    prdfarmtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSidString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszusersidstring: *mut *mut i8) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::UserSidString(this) {
                Ok(ok__) => {
                    pszusersidstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisconnectedSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbClientConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbClientConnection_Impl::GetDisconnectedSession(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UserName: UserName::<Identity, OFFSET>,
            Domain: Domain::<Identity, OFFSET>,
            InitialProgram: InitialProgram::<Identity, OFFSET>,
            LoadBalanceResult: LoadBalanceResult::<Identity, OFFSET>,
            FarmName: FarmName::<Identity, OFFSET>,
            PutContext: PutContext::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            Environment: Environment::<Identity, OFFSET>,
            get_ConnectionError: get_ConnectionError::<Identity, OFFSET>,
            SamUserAccount: SamUserAccount::<Identity, OFFSET>,
            ClientConnectionPropertySet: ClientConnectionPropertySet::<Identity, OFFSET>,
            IsFirstAssignment: IsFirstAssignment::<Identity, OFFSET>,
            RdFarmType: RdFarmType::<Identity, OFFSET>,
            UserSidString: UserSidString::<Identity, OFFSET>,
            GetDisconnectedSession: GetDisconnectedSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbClientConnection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbClientConnectionPropertySet_Impl: Sized + ITsSbPropertySet_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbClientConnectionPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbClientConnectionPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbClientConnectionPropertySet_Vtbl
    where
        Identity: ITsSbClientConnectionPropertySet_Impl,
    {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbClientConnectionPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbEnvironment_Impl: Sized {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServerWeight(&self) -> windows_core::Result<u32>;
    fn EnvironmentPropertySet(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet>;
    fn SetEnvironmentPropertySet(&self, pval: Option<&ITsSbEnvironmentPropertySet>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbEnvironment {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbEnvironment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbEnvironment_Vtbl
    where
        Identity: ITsSbEnvironment_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbEnvironment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbEnvironment_Impl::Name(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerWeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITsSbEnvironment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbEnvironment_Impl::ServerWeight(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnvironmentPropertySet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbEnvironment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbEnvironment_Impl::EnvironmentPropertySet(this) {
                Ok(ok__) => {
                    pppropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentPropertySet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbEnvironment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbEnvironment_Impl::SetEnvironmentPropertySet(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            ServerWeight: ServerWeight::<Identity, OFFSET>,
            EnvironmentPropertySet: EnvironmentPropertySet::<Identity, OFFSET>,
            SetEnvironmentPropertySet: SetEnvironmentPropertySet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbEnvironment as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbEnvironmentPropertySet_Impl: Sized + ITsSbPropertySet_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbEnvironmentPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbEnvironmentPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbEnvironmentPropertySet_Vtbl
    where
        Identity: ITsSbEnvironmentPropertySet_Impl,
    {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbEnvironmentPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbFilterPluginStore_Impl: Sized {
    fn SaveProperties(&self, ppropertyset: Option<&ITsSbPropertySet>) -> windows_core::Result<()>;
    fn EnumerateProperties(&self) -> windows_core::Result<ITsSbPropertySet>;
    fn DeleteProperties(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbFilterPluginStore {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbFilterPluginStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbFilterPluginStore_Vtbl
    where
        Identity: ITsSbFilterPluginStore_Impl,
    {
        unsafe extern "system" fn SaveProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertyset: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbFilterPluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbFilterPluginStore_Impl::SaveProperties(this, windows_core::from_raw_borrowed(&ppropertyset)).into()
        }
        unsafe extern "system" fn EnumerateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbFilterPluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbFilterPluginStore_Impl::EnumerateProperties(this) {
                Ok(ok__) => {
                    pppropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbFilterPluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbFilterPluginStore_Impl::DeleteProperties(this, core::mem::transmute(&propertyname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveProperties: SaveProperties::<Identity, OFFSET>,
            EnumerateProperties: EnumerateProperties::<Identity, OFFSET>,
            DeleteProperties: DeleteProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbFilterPluginStore as windows_core::Interface>::IID
    }
}
pub trait ITsSbGenericNotifySink_Impl: Sized {
    fn OnCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetWaitTimeout(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
}
impl windows_core::RuntimeName for ITsSbGenericNotifySink {}
impl ITsSbGenericNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbGenericNotifySink_Vtbl
    where
        Identity: ITsSbGenericNotifySink_Impl,
    {
        unsafe extern "system" fn OnCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITsSbGenericNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbGenericNotifySink_Impl::OnCompleted(this, core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetWaitTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbGenericNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbGenericNotifySink_Impl::GetWaitTimeout(this) {
                Ok(ok__) => {
                    pfttimeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCompleted: OnCompleted::<Identity, OFFSET>,
            GetWaitTimeout: GetWaitTimeout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbGenericNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITsSbGlobalStore_Impl: Sized {
    fn QueryTarget(&self, providername: &windows_core::BSTR, targetname: &windows_core::BSTR, farmname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(&self, providername: &windows_core::BSTR, dwsessionid: u32, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbSession>;
    fn EnumerateFarms(&self, providername: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn EnumerateTargets(&self, providername: &windows_core::BSTR, farmname: &windows_core::BSTR, envname: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()>;
    fn EnumerateEnvironmentsByProvider(&self, providername: &windows_core::BSTR, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()>;
    fn EnumerateSessions(&self, providername: &windows_core::BSTR, targetname: &windows_core::BSTR, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, poolname: &windows_core::BSTR, initialprogram: &windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()>;
    fn GetFarmProperty(&self, farmname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITsSbGlobalStore {}
#[cfg(feature = "Win32_System_Com")]
impl ITsSbGlobalStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbGlobalStore_Vtbl
    where
        Identity: ITsSbGlobalStore_Impl,
    {
        unsafe extern "system" fn QueryTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: core::mem::MaybeUninit<windows_core::BSTR>, targetname: core::mem::MaybeUninit<windows_core::BSTR>, farmname: core::mem::MaybeUninit<windows_core::BSTR>, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbGlobalStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbGlobalStore_Impl::QueryTarget(this, core::mem::transmute(&providername), core::mem::transmute(&targetname), core::mem::transmute(&farmname)) {
                Ok(ok__) => {
                    pptarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: core::mem::MaybeUninit<windows_core::BSTR>, dwsessionid: u32, targetname: core::mem::MaybeUninit<windows_core::BSTR>, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbGlobalStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbGlobalStore_Impl::QuerySessionBySessionId(this, core::mem::transmute(&providername), core::mem::transmute_copy(&dwsessionid), core::mem::transmute(&targetname)) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateFarms<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: core::mem::MaybeUninit<windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITsSbGlobalStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbGlobalStore_Impl::EnumerateFarms(this, core::mem::transmute(&providername), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn EnumerateTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: core::mem::MaybeUninit<windows_core::BSTR>, farmname: core::mem::MaybeUninit<windows_core::BSTR>, envname: core::mem::MaybeUninit<windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::HRESULT
        where
            Identity: ITsSbGlobalStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbGlobalStore_Impl::EnumerateTargets(this, core::mem::transmute(&providername), core::mem::transmute(&farmname), core::mem::transmute(&envname), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn EnumerateEnvironmentsByProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: core::mem::MaybeUninit<windows_core::BSTR>, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::HRESULT
        where
            Identity: ITsSbGlobalStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbGlobalStore_Impl::EnumerateEnvironmentsByProvider(this, core::mem::transmute(&providername), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&ppval)).into()
        }
        unsafe extern "system" fn EnumerateSessions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: core::mem::MaybeUninit<windows_core::BSTR>, targetname: core::mem::MaybeUninit<windows_core::BSTR>, username: core::mem::MaybeUninit<windows_core::BSTR>, userdomain: core::mem::MaybeUninit<windows_core::BSTR>, poolname: core::mem::MaybeUninit<windows_core::BSTR>, initialprogram: core::mem::MaybeUninit<windows_core::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::HRESULT
        where
            Identity: ITsSbGlobalStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbGlobalStore_Impl::EnumerateSessions(this, core::mem::transmute(&providername), core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&userdomain), core::mem::transmute(&poolname), core::mem::transmute(&initialprogram), core::mem::transmute_copy(&psessionstate), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&ppval)).into()
        }
        unsafe extern "system" fn GetFarmProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, farmname: core::mem::MaybeUninit<windows_core::BSTR>, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, pvarvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbGlobalStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbGlobalStore_Impl::GetFarmProperty(this, core::mem::transmute(&farmname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryTarget: QueryTarget::<Identity, OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Identity, OFFSET>,
            EnumerateFarms: EnumerateFarms::<Identity, OFFSET>,
            EnumerateTargets: EnumerateTargets::<Identity, OFFSET>,
            EnumerateEnvironmentsByProvider: EnumerateEnvironmentsByProvider::<Identity, OFFSET>,
            EnumerateSessions: EnumerateSessions::<Identity, OFFSET>,
            GetFarmProperty: GetFarmProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbGlobalStore as windows_core::Interface>::IID
    }
}
pub trait ITsSbLoadBalanceResult_Impl: Sized {
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITsSbLoadBalanceResult {}
impl ITsSbLoadBalanceResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbLoadBalanceResult_Vtbl
    where
        Identity: ITsSbLoadBalanceResult_Impl,
    {
        unsafe extern "system" fn TargetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbLoadBalanceResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbLoadBalanceResult_Impl::TargetName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TargetName: TargetName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbLoadBalanceResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbLoadBalancing_Impl: Sized + ITsSbPlugin_Impl {
    fn GetMostSuitableTarget(&self, pconnection: Option<&ITsSbClientConnection>, plbsink: Option<&ITsSbLoadBalancingNotifySink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbLoadBalancing {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbLoadBalancing_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbLoadBalancing_Vtbl
    where
        Identity: ITsSbLoadBalancing_Impl,
    {
        unsafe extern "system" fn GetMostSuitableTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, plbsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbLoadBalancing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbLoadBalancing_Impl::GetMostSuitableTarget(this, windows_core::from_raw_borrowed(&pconnection), windows_core::from_raw_borrowed(&plbsink)).into()
        }
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(), GetMostSuitableTarget: GetMostSuitableTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbLoadBalancing as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
pub trait ITsSbLoadBalancingNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnGetMostSuitableTarget(&self, plbresult: Option<&ITsSbLoadBalanceResult>, fisnewconnection: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbLoadBalancingNotifySink {}
impl ITsSbLoadBalancingNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbLoadBalancingNotifySink_Vtbl
    where
        Identity: ITsSbLoadBalancingNotifySink_Impl,
    {
        unsafe extern "system" fn OnGetMostSuitableTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbresult: *mut core::ffi::c_void, fisnewconnection: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITsSbLoadBalancingNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbLoadBalancingNotifySink_Impl::OnGetMostSuitableTarget(this, windows_core::from_raw_borrowed(&plbresult), core::mem::transmute_copy(&fisnewconnection)).into()
        }
        Self { base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(), OnGetMostSuitableTarget: OnGetMostSuitableTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbLoadBalancingNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbOrchestration_Impl: Sized + ITsSbPlugin_Impl {
    fn PrepareTargetForConnect(&self, pconnection: Option<&ITsSbClientConnection>, porchestrationnotifysink: Option<&ITsSbOrchestrationNotifySink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbOrchestration {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbOrchestration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbOrchestration_Vtbl
    where
        Identity: ITsSbOrchestration_Impl,
    {
        unsafe extern "system" fn PrepareTargetForConnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, porchestrationnotifysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbOrchestration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbOrchestration_Impl::PrepareTargetForConnect(this, windows_core::from_raw_borrowed(&pconnection), windows_core::from_raw_borrowed(&porchestrationnotifysink)).into()
        }
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(), PrepareTargetForConnect: PrepareTargetForConnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbOrchestration as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
pub trait ITsSbOrchestrationNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnReadyToConnect(&self, ptarget: Option<&ITsSbTarget>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbOrchestrationNotifySink {}
impl ITsSbOrchestrationNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbOrchestrationNotifySink_Vtbl
    where
        Identity: ITsSbOrchestrationNotifySink_Impl,
    {
        unsafe extern "system" fn OnReadyToConnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbOrchestrationNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbOrchestrationNotifySink_Impl::OnReadyToConnect(this, windows_core::from_raw_borrowed(&ptarget)).into()
        }
        Self { base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(), OnReadyToConnect: OnReadyToConnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbOrchestrationNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlacement_Impl: Sized + ITsSbPlugin_Impl {
    fn QueryEnvironmentForTarget(&self, pconnection: Option<&ITsSbClientConnection>, pplacementsink: Option<&ITsSbPlacementNotifySink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbPlacement {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPlacement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbPlacement_Vtbl
    where
        Identity: ITsSbPlacement_Impl,
    {
        unsafe extern "system" fn QueryEnvironmentForTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, pplacementsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbPlacement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbPlacement_Impl::QueryEnvironmentForTarget(this, windows_core::from_raw_borrowed(&pconnection), windows_core::from_raw_borrowed(&pplacementsink)).into()
        }
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(), QueryEnvironmentForTarget: QueryEnvironmentForTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPlacement as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
pub trait ITsSbPlacementNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnQueryEnvironmentCompleted(&self, penvironment: Option<&ITsSbEnvironment>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbPlacementNotifySink {}
impl ITsSbPlacementNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbPlacementNotifySink_Vtbl
    where
        Identity: ITsSbPlacementNotifySink_Impl,
    {
        unsafe extern "system" fn OnQueryEnvironmentCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbPlacementNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbPlacementNotifySink_Impl::OnQueryEnvironmentCompleted(this, windows_core::from_raw_borrowed(&penvironment)).into()
        }
        Self { base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(), OnQueryEnvironmentCompleted: OnQueryEnvironmentCompleted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPlacementNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlugin_Impl: Sized {
    fn Initialize(&self, pprovider: Option<&ITsSbProvider>, pnotifysink: Option<&ITsSbPluginNotifySink>, ppropertyset: Option<&ITsSbPluginPropertySet>) -> windows_core::Result<()>;
    fn Terminate(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbPlugin {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbPlugin_Vtbl
    where
        Identity: ITsSbPlugin_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, pnotifysink: *mut core::ffi::c_void, ppropertyset: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbPlugin_Impl::Initialize(this, windows_core::from_raw_borrowed(&pprovider), windows_core::from_raw_borrowed(&pnotifysink), windows_core::from_raw_borrowed(&ppropertyset)).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITsSbPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbPlugin_Impl::Terminate(this, core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
pub trait ITsSbPluginNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnInitialized(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnTerminated(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbPluginNotifySink {}
impl ITsSbPluginNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbPluginNotifySink_Vtbl
    where
        Identity: ITsSbPluginNotifySink_Impl,
    {
        unsafe extern "system" fn OnInitialized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITsSbPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbPluginNotifySink_Impl::OnInitialized(this, core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnTerminated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbPluginNotifySink_Impl::OnTerminated(this).into()
        }
        Self {
            base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(),
            OnInitialized: OnInitialized::<Identity, OFFSET>,
            OnTerminated: OnTerminated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPluginNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPluginPropertySet_Impl: Sized + ITsSbPropertySet_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbPluginPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPluginPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbPluginPropertySet_Vtbl
    where
        Identity: ITsSbPluginPropertySet_Impl,
    {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPluginPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPropertySet_Impl: Sized + super::Com::StructuredStorage::IPropertyBag_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbPropertySet_Vtbl
    where
        Identity: ITsSbPropertySet_Impl,
    {
        Self { base__: super::Com::StructuredStorage::IPropertyBag_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbProvider_Impl: Sized {
    fn CreateTargetObject(&self, targetname: &windows_core::BSTR, environmentname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget>;
    fn CreateLoadBalanceResultObject(&self, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbLoadBalanceResult>;
    fn CreateSessionObject(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, domain: &windows_core::BSTR, sessionid: u32) -> windows_core::Result<ITsSbSession>;
    fn CreatePluginPropertySet(&self) -> windows_core::Result<ITsSbPluginPropertySet>;
    fn CreateTargetPropertySetObject(&self) -> windows_core::Result<ITsSbTargetPropertySet>;
    fn CreateEnvironmentObject(&self, name: &windows_core::BSTR, serverweight: u32) -> windows_core::Result<ITsSbEnvironment>;
    fn GetResourcePluginStore(&self) -> windows_core::Result<ITsSbResourcePluginStore>;
    fn GetFilterPluginStore(&self) -> windows_core::Result<ITsSbFilterPluginStore>;
    fn RegisterForNotification(&self, notificationtype: u32, resourcetomonitor: &windows_core::BSTR, ppluginnotification: Option<&ITsSbResourceNotification>) -> windows_core::Result<()>;
    fn UnRegisterForNotification(&self, notificationtype: u32, resourcetomonitor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetInstanceOfGlobalStore(&self) -> windows_core::Result<ITsSbGlobalStore>;
    fn CreateEnvironmentPropertySetObject(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbProvider {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbProvider_Vtbl
    where
        Identity: ITsSbProvider_Impl,
    {
        unsafe extern "system" fn CreateTargetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, environmentname: core::mem::MaybeUninit<windows_core::BSTR>, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::CreateTargetObject(this, core::mem::transmute(&targetname), core::mem::transmute(&environmentname)) {
                Ok(ok__) => {
                    pptarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLoadBalanceResultObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, pplbresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::CreateLoadBalanceResultObject(this, core::mem::transmute(&targetname)) {
                Ok(ok__) => {
                    pplbresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, username: core::mem::MaybeUninit<windows_core::BSTR>, domain: core::mem::MaybeUninit<windows_core::BSTR>, sessionid: u32, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::CreateSessionObject(this, core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&domain), core::mem::transmute_copy(&sessionid)) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePluginPropertySet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::CreatePluginPropertySet(this) {
                Ok(ok__) => {
                    pppropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetPropertySetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::CreateTargetPropertySetObject(this) {
                Ok(ok__) => {
                    pppropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnvironmentObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, serverweight: u32, ppenvironment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::CreateEnvironmentObject(this, core::mem::transmute(&name), core::mem::transmute_copy(&serverweight)) {
                Ok(ok__) => {
                    ppenvironment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourcePluginStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::GetResourcePluginStore(this) {
                Ok(ok__) => {
                    ppstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterPluginStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::GetFilterPluginStore(this) {
                Ok(ok__) => {
                    ppstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: u32, resourcetomonitor: core::mem::MaybeUninit<windows_core::BSTR>, ppluginnotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvider_Impl::RegisterForNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute(&resourcetomonitor), windows_core::from_raw_borrowed(&ppluginnotification)).into()
        }
        unsafe extern "system" fn UnRegisterForNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: u32, resourcetomonitor: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvider_Impl::UnRegisterForNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute(&resourcetomonitor)).into()
        }
        unsafe extern "system" fn GetInstanceOfGlobalStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppglobalstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::GetInstanceOfGlobalStore(this) {
                Ok(ok__) => {
                    ppglobalstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnvironmentPropertySetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbProvider_Impl::CreateEnvironmentPropertySetObject(this) {
                Ok(ok__) => {
                    pppropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTargetObject: CreateTargetObject::<Identity, OFFSET>,
            CreateLoadBalanceResultObject: CreateLoadBalanceResultObject::<Identity, OFFSET>,
            CreateSessionObject: CreateSessionObject::<Identity, OFFSET>,
            CreatePluginPropertySet: CreatePluginPropertySet::<Identity, OFFSET>,
            CreateTargetPropertySetObject: CreateTargetPropertySetObject::<Identity, OFFSET>,
            CreateEnvironmentObject: CreateEnvironmentObject::<Identity, OFFSET>,
            GetResourcePluginStore: GetResourcePluginStore::<Identity, OFFSET>,
            GetFilterPluginStore: GetFilterPluginStore::<Identity, OFFSET>,
            RegisterForNotification: RegisterForNotification::<Identity, OFFSET>,
            UnRegisterForNotification: UnRegisterForNotification::<Identity, OFFSET>,
            GetInstanceOfGlobalStore: GetInstanceOfGlobalStore::<Identity, OFFSET>,
            CreateEnvironmentPropertySetObject: CreateEnvironmentPropertySetObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbProvisioning_Impl: Sized + ITsSbPlugin_Impl {
    fn CreateVirtualMachines(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: Option<&ITsSbProvisioningPluginNotifySink>) -> windows_core::Result<()>;
    fn PatchVirtualMachines(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: Option<&ITsSbProvisioningPluginNotifySink>, pvmpatchinfo: *const VM_PATCH_INFO) -> windows_core::Result<()>;
    fn DeleteVirtualMachines(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: Option<&ITsSbProvisioningPluginNotifySink>) -> windows_core::Result<()>;
    fn CancelJob(&self, jobguid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbProvisioning {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbProvisioning_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbProvisioning_Vtbl
    where
        Identity: ITsSbProvisioning_Impl,
    {
        unsafe extern "system" fn CreateVirtualMachines<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobxmlstring: core::mem::MaybeUninit<windows_core::BSTR>, jobguid: core::mem::MaybeUninit<windows_core::BSTR>, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioning_Impl::CreateVirtualMachines(this, core::mem::transmute(&jobxmlstring), core::mem::transmute(&jobguid), windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn PatchVirtualMachines<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobxmlstring: core::mem::MaybeUninit<windows_core::BSTR>, jobguid: core::mem::MaybeUninit<windows_core::BSTR>, psink: *mut core::ffi::c_void, pvmpatchinfo: *const VM_PATCH_INFO) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioning_Impl::PatchVirtualMachines(this, core::mem::transmute(&jobxmlstring), core::mem::transmute(&jobguid), windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&pvmpatchinfo)).into()
        }
        unsafe extern "system" fn DeleteVirtualMachines<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobxmlstring: core::mem::MaybeUninit<windows_core::BSTR>, jobguid: core::mem::MaybeUninit<windows_core::BSTR>, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioning_Impl::DeleteVirtualMachines(this, core::mem::transmute(&jobxmlstring), core::mem::transmute(&jobguid), windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn CancelJob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobguid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioning_Impl::CancelJob(this, core::mem::transmute(&jobguid)).into()
        }
        Self {
            base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(),
            CreateVirtualMachines: CreateVirtualMachines::<Identity, OFFSET>,
            PatchVirtualMachines: PatchVirtualMachines::<Identity, OFFSET>,
            DeleteVirtualMachines: DeleteVirtualMachines::<Identity, OFFSET>,
            CancelJob: CancelJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbProvisioning as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
pub trait ITsSbProvisioningPluginNotifySink_Impl: Sized {
    fn OnJobCreated(&self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> windows_core::Result<()>;
    fn OnVirtualMachineStatusChanged(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnJobCompleted(&self, resultcode: windows_core::HRESULT, resultdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnJobCancelled(&self) -> windows_core::Result<()>;
    fn LockVirtualMachine(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> windows_core::Result<()>;
    fn OnVirtualMachineHostStatusChanged(&self, vmhost: &windows_core::BSTR, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbProvisioningPluginNotifySink {}
impl ITsSbProvisioningPluginNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbProvisioningPluginNotifySink_Vtbl
    where
        Identity: ITsSbProvisioningPluginNotifySink_Impl,
    {
        unsafe extern "system" fn OnJobCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioningPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioningPluginNotifySink_Impl::OnJobCreated(this, core::mem::transmute_copy(&pvmnotifyinfo)).into()
        }
        unsafe extern "system" fn OnVirtualMachineStatusChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioningPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioningPluginNotifySink_Impl::OnVirtualMachineStatusChanged(this, core::mem::transmute_copy(&pvmnotifyentry), core::mem::transmute_copy(&vmnotifystatus), core::mem::transmute_copy(&errorcode), core::mem::transmute(&errordescr)).into()
        }
        unsafe extern "system" fn OnJobCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultcode: windows_core::HRESULT, resultdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioningPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioningPluginNotifySink_Impl::OnJobCompleted(this, core::mem::transmute_copy(&resultcode), core::mem::transmute(&resultdescription)).into()
        }
        unsafe extern "system" fn OnJobCancelled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioningPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioningPluginNotifySink_Impl::OnJobCancelled(this).into()
        }
        unsafe extern "system" fn LockVirtualMachine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioningPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioningPluginNotifySink_Impl::LockVirtualMachine(this, core::mem::transmute_copy(&pvmnotifyentry)).into()
        }
        unsafe extern "system" fn OnVirtualMachineHostStatusChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vmhost: core::mem::MaybeUninit<windows_core::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbProvisioningPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbProvisioningPluginNotifySink_Impl::OnVirtualMachineHostStatusChanged(this, core::mem::transmute(&vmhost), core::mem::transmute_copy(&vmhostnotifystatus), core::mem::transmute_copy(&errorcode), core::mem::transmute(&errordescr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnJobCreated: OnJobCreated::<Identity, OFFSET>,
            OnVirtualMachineStatusChanged: OnVirtualMachineStatusChanged::<Identity, OFFSET>,
            OnJobCompleted: OnJobCompleted::<Identity, OFFSET>,
            OnJobCancelled: OnJobCancelled::<Identity, OFFSET>,
            LockVirtualMachine: LockVirtualMachine::<Identity, OFFSET>,
            OnVirtualMachineHostStatusChanged: OnVirtualMachineHostStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbProvisioningPluginNotifySink as windows_core::Interface>::IID
    }
}
pub trait ITsSbResourceNotification_Impl: Sized {
    fn NotifySessionChange(&self, changetype: TSSESSION_STATE, psession: Option<&ITsSbSession>) -> windows_core::Result<()>;
    fn NotifyTargetChange(&self, targetchangetype: u32, ptarget: Option<&ITsSbTarget>) -> windows_core::Result<()>;
    fn NotifyClientConnectionStateChange(&self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: Option<&ITsSbClientConnection>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbResourceNotification {}
impl ITsSbResourceNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbResourceNotification_Vtbl
    where
        Identity: ITsSbResourceNotification_Impl,
    {
        unsafe extern "system" fn NotifySessionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: TSSESSION_STATE, psession: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourceNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourceNotification_Impl::NotifySessionChange(this, core::mem::transmute_copy(&changetype), windows_core::from_raw_borrowed(&psession)).into()
        }
        unsafe extern "system" fn NotifyTargetChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetchangetype: u32, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourceNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourceNotification_Impl::NotifyTargetChange(this, core::mem::transmute_copy(&targetchangetype), windows_core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn NotifyClientConnectionStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourceNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourceNotification_Impl::NotifyClientConnectionStateChange(this, core::mem::transmute_copy(&changetype), windows_core::from_raw_borrowed(&pconnection)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifySessionChange: NotifySessionChange::<Identity, OFFSET>,
            NotifyTargetChange: NotifyTargetChange::<Identity, OFFSET>,
            NotifyClientConnectionStateChange: NotifyClientConnectionStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourceNotification as windows_core::Interface>::IID
    }
}
pub trait ITsSbResourceNotificationEx_Impl: Sized {
    fn NotifySessionChangeEx(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, domain: &windows_core::BSTR, sessionid: u32, sessionstate: TSSESSION_STATE) -> windows_core::Result<()>;
    fn NotifyTargetChangeEx(&self, targetname: &windows_core::BSTR, targetchangetype: u32) -> windows_core::Result<()>;
    fn NotifyClientConnectionStateChangeEx(&self, username: &windows_core::BSTR, domain: &windows_core::BSTR, initialprogram: &windows_core::BSTR, poolname: &windows_core::BSTR, targetname: &windows_core::BSTR, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbResourceNotificationEx {}
impl ITsSbResourceNotificationEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbResourceNotificationEx_Vtbl
    where
        Identity: ITsSbResourceNotificationEx_Impl,
    {
        unsafe extern "system" fn NotifySessionChangeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, username: core::mem::MaybeUninit<windows_core::BSTR>, domain: core::mem::MaybeUninit<windows_core::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbResourceNotificationEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourceNotificationEx_Impl::NotifySessionChangeEx(this, core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&domain), core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&sessionstate)).into()
        }
        unsafe extern "system" fn NotifyTargetChangeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, targetchangetype: u32) -> windows_core::HRESULT
        where
            Identity: ITsSbResourceNotificationEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourceNotificationEx_Impl::NotifyTargetChangeEx(this, core::mem::transmute(&targetname), core::mem::transmute_copy(&targetchangetype)).into()
        }
        unsafe extern "system" fn NotifyClientConnectionStateChangeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::BSTR>, domain: core::mem::MaybeUninit<windows_core::BSTR>, initialprogram: core::mem::MaybeUninit<windows_core::BSTR>, poolname: core::mem::MaybeUninit<windows_core::BSTR>, targetname: core::mem::MaybeUninit<windows_core::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> windows_core::HRESULT
        where
            Identity: ITsSbResourceNotificationEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourceNotificationEx_Impl::NotifyClientConnectionStateChangeEx(this, core::mem::transmute(&username), core::mem::transmute(&domain), core::mem::transmute(&initialprogram), core::mem::transmute(&poolname), core::mem::transmute(&targetname), core::mem::transmute_copy(&connectionchangetype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifySessionChangeEx: NotifySessionChangeEx::<Identity, OFFSET>,
            NotifyTargetChangeEx: NotifyTargetChangeEx::<Identity, OFFSET>,
            NotifyClientConnectionStateChangeEx: NotifyClientConnectionStateChangeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourceNotificationEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbResourcePlugin_Impl: Sized + ITsSbPlugin_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbResourcePlugin {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbResourcePlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbResourcePlugin_Vtbl
    where
        Identity: ITsSbResourcePlugin_Impl,
    {
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourcePlugin as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITsSbResourcePluginStore_Impl: Sized {
    fn QueryTarget(&self, targetname: &windows_core::BSTR, farmname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(&self, dwsessionid: u32, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbSession>;
    fn AddTargetToStore(&self, ptarget: Option<&ITsSbTarget>) -> windows_core::Result<()>;
    fn AddSessionToStore(&self, psession: Option<&ITsSbSession>) -> windows_core::Result<()>;
    fn AddEnvironmentToStore(&self, penvironment: Option<&ITsSbEnvironment>) -> windows_core::Result<()>;
    fn RemoveEnvironmentFromStore(&self, environmentname: &windows_core::BSTR, bignoreowner: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnumerateFarms(&self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn QueryEnvironment(&self, environmentname: &windows_core::BSTR) -> windows_core::Result<ITsSbEnvironment>;
    fn EnumerateEnvironments(&self, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()>;
    fn SaveTarget(&self, ptarget: Option<&ITsSbTarget>, bforcewrite: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SaveEnvironment(&self, penvironment: Option<&ITsSbEnvironment>, bforcewrite: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SaveSession(&self, psession: Option<&ITsSbSession>) -> windows_core::Result<()>;
    fn SetTargetProperty(&self, targetname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetEnvironmentProperty(&self, environmentname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetTargetState(&self, targetname: &windows_core::BSTR, newstate: TARGET_STATE) -> windows_core::Result<TARGET_STATE>;
    fn SetSessionState(&self, sbsession: Option<&ITsSbSession>) -> windows_core::Result<()>;
    fn EnumerateTargets(&self, farmname: &windows_core::BSTR, envname: &windows_core::BSTR, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()>;
    fn EnumerateSessions(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, poolname: &windows_core::BSTR, initialprogram: &windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()>;
    fn GetFarmProperty(&self, farmname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn DeleteTarget(&self, targetname: &windows_core::BSTR, hostname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTargetPropertyWithVersionCheck(&self, ptarget: Option<&ITsSbTarget>, propertyname: &windows_core::BSTR, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetEnvironmentPropertyWithVersionCheck(&self, penvironment: Option<&ITsSbEnvironment>, propertyname: &windows_core::BSTR, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn AcquireTargetLock(&self, targetname: &windows_core::BSTR, dwtimeout: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn ReleaseTargetLock(&self, pcontext: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn TestAndSetServerState(&self, poolname: &windows_core::BSTR, serverfqdn: &windows_core::BSTR, newstate: TARGET_STATE, teststate: TARGET_STATE) -> windows_core::Result<TARGET_STATE>;
    fn SetServerWaitingToStart(&self, poolname: &windows_core::BSTR, servername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetServerState(&self, poolname: &windows_core::BSTR, serverfqdn: &windows_core::BSTR) -> windows_core::Result<TARGET_STATE>;
    fn SetServerDrainMode(&self, serverfqdn: &windows_core::BSTR, drainmode: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITsSbResourcePluginStore {}
#[cfg(feature = "Win32_System_Com")]
impl ITsSbResourcePluginStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbResourcePluginStore_Vtbl
    where
        Identity: ITsSbResourcePluginStore_Impl,
    {
        unsafe extern "system" fn QueryTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, farmname: core::mem::MaybeUninit<windows_core::BSTR>, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbResourcePluginStore_Impl::QueryTarget(this, core::mem::transmute(&targetname), core::mem::transmute(&farmname)) {
                Ok(ok__) => {
                    pptarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsessionid: u32, targetname: core::mem::MaybeUninit<windows_core::BSTR>, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbResourcePluginStore_Impl::QuerySessionBySessionId(this, core::mem::transmute_copy(&dwsessionid), core::mem::transmute(&targetname)) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTargetToStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::AddTargetToStore(this, windows_core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn AddSessionToStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psession: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::AddSessionToStore(this, windows_core::from_raw_borrowed(&psession)).into()
        }
        unsafe extern "system" fn AddEnvironmentToStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::AddEnvironmentToStore(this, windows_core::from_raw_borrowed(&penvironment)).into()
        }
        unsafe extern "system" fn RemoveEnvironmentFromStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, environmentname: core::mem::MaybeUninit<windows_core::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::RemoveEnvironmentFromStore(this, core::mem::transmute(&environmentname), core::mem::transmute_copy(&bignoreowner)).into()
        }
        unsafe extern "system" fn EnumerateFarms<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::EnumerateFarms(this, core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn QueryEnvironment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, environmentname: core::mem::MaybeUninit<windows_core::BSTR>, ppenvironment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbResourcePluginStore_Impl::QueryEnvironment(this, core::mem::transmute(&environmentname)) {
                Ok(ok__) => {
                    ppenvironment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateEnvironments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::EnumerateEnvironments(this, core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn SaveTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SaveTarget(this, windows_core::from_raw_borrowed(&ptarget), core::mem::transmute_copy(&bforcewrite)).into()
        }
        unsafe extern "system" fn SaveEnvironment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SaveEnvironment(this, windows_core::from_raw_borrowed(&penvironment), core::mem::transmute_copy(&bforcewrite)).into()
        }
        unsafe extern "system" fn SaveSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psession: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SaveSession(this, windows_core::from_raw_borrowed(&psession)).into()
        }
        unsafe extern "system" fn SetTargetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SetTargetProperty(this, core::mem::transmute(&targetname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn SetEnvironmentProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, environmentname: core::mem::MaybeUninit<windows_core::BSTR>, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SetEnvironmentProperty(this, core::mem::transmute(&environmentname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn SetTargetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbResourcePluginStore_Impl::SetTargetState(this, core::mem::transmute(&targetname), core::mem::transmute_copy(&newstate)) {
                Ok(ok__) => {
                    poldstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sbsession: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SetSessionState(this, windows_core::from_raw_borrowed(&sbsession)).into()
        }
        unsafe extern "system" fn EnumerateTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, farmname: core::mem::MaybeUninit<windows_core::BSTR>, envname: core::mem::MaybeUninit<windows_core::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: core::mem::MaybeUninit<windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::EnumerateTargets(this, core::mem::transmute(&farmname), core::mem::transmute(&envname), core::mem::transmute_copy(&sortbyfieldid), core::mem::transmute(&sortybypropname), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn EnumerateSessions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, username: core::mem::MaybeUninit<windows_core::BSTR>, userdomain: core::mem::MaybeUninit<windows_core::BSTR>, poolname: core::mem::MaybeUninit<windows_core::BSTR>, initialprogram: core::mem::MaybeUninit<windows_core::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::EnumerateSessions(this, core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&userdomain), core::mem::transmute(&poolname), core::mem::transmute(&initialprogram), core::mem::transmute_copy(&psessionstate), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&ppval)).into()
        }
        unsafe extern "system" fn GetFarmProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, farmname: core::mem::MaybeUninit<windows_core::BSTR>, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, pvarvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::GetFarmProperty(this, core::mem::transmute(&farmname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn DeleteTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, hostname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::DeleteTarget(this, core::mem::transmute(&targetname), core::mem::transmute(&hostname)).into()
        }
        unsafe extern "system" fn SetTargetPropertyWithVersionCheck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SetTargetPropertyWithVersionCheck(this, windows_core::from_raw_borrowed(&ptarget), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn SetEnvironmentPropertyWithVersionCheck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SetEnvironmentPropertyWithVersionCheck(this, windows_core::from_raw_borrowed(&penvironment), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn AcquireTargetLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>, dwtimeout: u32, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbResourcePluginStore_Impl::AcquireTargetLock(this, core::mem::transmute(&targetname), core::mem::transmute_copy(&dwtimeout)) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseTargetLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::ReleaseTargetLock(this, windows_core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn TestAndSetServerState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poolname: core::mem::MaybeUninit<windows_core::BSTR>, serverfqdn: core::mem::MaybeUninit<windows_core::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbResourcePluginStore_Impl::TestAndSetServerState(this, core::mem::transmute(&poolname), core::mem::transmute(&serverfqdn), core::mem::transmute_copy(&newstate), core::mem::transmute_copy(&teststate)) {
                Ok(ok__) => {
                    pinitstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerWaitingToStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poolname: core::mem::MaybeUninit<windows_core::BSTR>, servername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SetServerWaitingToStart(this, core::mem::transmute(&poolname), core::mem::transmute(&servername)).into()
        }
        unsafe extern "system" fn GetServerState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poolname: core::mem::MaybeUninit<windows_core::BSTR>, serverfqdn: core::mem::MaybeUninit<windows_core::BSTR>, pstate: *mut TARGET_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbResourcePluginStore_Impl::GetServerState(this, core::mem::transmute(&poolname), core::mem::transmute(&serverfqdn)) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerDrainMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serverfqdn: core::mem::MaybeUninit<windows_core::BSTR>, drainmode: u32) -> windows_core::HRESULT
        where
            Identity: ITsSbResourcePluginStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbResourcePluginStore_Impl::SetServerDrainMode(this, core::mem::transmute(&serverfqdn), core::mem::transmute_copy(&drainmode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryTarget: QueryTarget::<Identity, OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Identity, OFFSET>,
            AddTargetToStore: AddTargetToStore::<Identity, OFFSET>,
            AddSessionToStore: AddSessionToStore::<Identity, OFFSET>,
            AddEnvironmentToStore: AddEnvironmentToStore::<Identity, OFFSET>,
            RemoveEnvironmentFromStore: RemoveEnvironmentFromStore::<Identity, OFFSET>,
            EnumerateFarms: EnumerateFarms::<Identity, OFFSET>,
            QueryEnvironment: QueryEnvironment::<Identity, OFFSET>,
            EnumerateEnvironments: EnumerateEnvironments::<Identity, OFFSET>,
            SaveTarget: SaveTarget::<Identity, OFFSET>,
            SaveEnvironment: SaveEnvironment::<Identity, OFFSET>,
            SaveSession: SaveSession::<Identity, OFFSET>,
            SetTargetProperty: SetTargetProperty::<Identity, OFFSET>,
            SetEnvironmentProperty: SetEnvironmentProperty::<Identity, OFFSET>,
            SetTargetState: SetTargetState::<Identity, OFFSET>,
            SetSessionState: SetSessionState::<Identity, OFFSET>,
            EnumerateTargets: EnumerateTargets::<Identity, OFFSET>,
            EnumerateSessions: EnumerateSessions::<Identity, OFFSET>,
            GetFarmProperty: GetFarmProperty::<Identity, OFFSET>,
            DeleteTarget: DeleteTarget::<Identity, OFFSET>,
            SetTargetPropertyWithVersionCheck: SetTargetPropertyWithVersionCheck::<Identity, OFFSET>,
            SetEnvironmentPropertyWithVersionCheck: SetEnvironmentPropertyWithVersionCheck::<Identity, OFFSET>,
            AcquireTargetLock: AcquireTargetLock::<Identity, OFFSET>,
            ReleaseTargetLock: ReleaseTargetLock::<Identity, OFFSET>,
            TestAndSetServerState: TestAndSetServerState::<Identity, OFFSET>,
            SetServerWaitingToStart: SetServerWaitingToStart::<Identity, OFFSET>,
            GetServerState: GetServerState::<Identity, OFFSET>,
            SetServerDrainMode: SetServerDrainMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourcePluginStore as windows_core::Interface>::IID
    }
}
pub trait ITsSbServiceNotification_Impl: Sized {
    fn NotifyServiceFailure(&self) -> windows_core::Result<()>;
    fn NotifyServiceSuccess(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbServiceNotification {}
impl ITsSbServiceNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbServiceNotification_Vtbl
    where
        Identity: ITsSbServiceNotification_Impl,
    {
        unsafe extern "system" fn NotifyServiceFailure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbServiceNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbServiceNotification_Impl::NotifyServiceFailure(this).into()
        }
        unsafe extern "system" fn NotifyServiceSuccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbServiceNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbServiceNotification_Impl::NotifyServiceSuccess(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyServiceFailure: NotifyServiceFailure::<Identity, OFFSET>,
            NotifyServiceSuccess: NotifyServiceSuccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbServiceNotification as windows_core::Interface>::IID
    }
}
pub trait ITsSbSession_Impl: Sized {
    fn SessionId(&self) -> windows_core::Result<u32>;
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetName(&self, targetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Username(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<TSSESSION_STATE>;
    fn SetState(&self, state: TSSESSION_STATE) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn SetCreateTime(&self, time: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn DisconnectTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn SetDisconnectTime(&self, time: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInitialProgram(&self, application: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientDisplay(&self) -> windows_core::Result<CLIENT_DISPLAY>;
    fn SetClientDisplay(&self, pclientdisplay: &CLIENT_DISPLAY) -> windows_core::Result<()>;
    fn ProtocolType(&self) -> windows_core::Result<u32>;
    fn SetProtocolType(&self, val: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITsSbSession {}
impl ITsSbSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbSession_Vtbl
    where
        Identity: ITsSbSession_Impl,
    {
        unsafe extern "system" fn SessionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::SessionId(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::TargetName(this) {
                Ok(ok__) => {
                    targetname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbSession_Impl::SetTargetName(this, core::mem::transmute(&targetname)).into()
        }
        unsafe extern "system" fn Username<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::Username(this) {
                Ok(ok__) => {
                    username.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, domain: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::Domain(this) {
                Ok(ok__) => {
                    domain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: TSSESSION_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbSession_Impl::SetState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::CreateTime(this) {
                Ok(ok__) => {
                    ptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbSession_Impl::SetCreateTime(this, core::mem::transmute(&time)).into()
        }
        unsafe extern "system" fn DisconnectTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::DisconnectTime(this) {
                Ok(ok__) => {
                    ptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisconnectTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbSession_Impl::SetDisconnectTime(this, core::mem::transmute(&time)).into()
        }
        unsafe extern "system" fn InitialProgram<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, app: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::InitialProgram(this) {
                Ok(ok__) => {
                    app.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialProgram<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, application: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbSession_Impl::SetInitialProgram(this, core::mem::transmute(&application)).into()
        }
        unsafe extern "system" fn ClientDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::ClientDisplay(this) {
                Ok(ok__) => {
                    pclientdisplay.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbSession_Impl::SetClientDisplay(this, core::mem::transmute(&pclientdisplay)).into()
        }
        unsafe extern "system" fn ProtocolType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbSession_Impl::ProtocolType(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocolType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: u32) -> windows_core::HRESULT
        where
            Identity: ITsSbSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbSession_Impl::SetProtocolType(this, core::mem::transmute_copy(&val)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SessionId: SessionId::<Identity, OFFSET>,
            TargetName: TargetName::<Identity, OFFSET>,
            SetTargetName: SetTargetName::<Identity, OFFSET>,
            Username: Username::<Identity, OFFSET>,
            Domain: Domain::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            SetCreateTime: SetCreateTime::<Identity, OFFSET>,
            DisconnectTime: DisconnectTime::<Identity, OFFSET>,
            SetDisconnectTime: SetDisconnectTime::<Identity, OFFSET>,
            InitialProgram: InitialProgram::<Identity, OFFSET>,
            SetInitialProgram: SetInitialProgram::<Identity, OFFSET>,
            ClientDisplay: ClientDisplay::<Identity, OFFSET>,
            SetClientDisplay: SetClientDisplay::<Identity, OFFSET>,
            ProtocolType: ProtocolType::<Identity, OFFSET>,
            SetProtocolType: SetProtocolType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbSession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTarget_Impl: Sized {
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetName(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FarmName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFarmName(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TargetFQDN(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetFQDN(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TargetNetbios(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetNetbios(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IpAddresses(&self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> windows_core::Result<()>;
    fn put_IpAddresses(&self, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> windows_core::Result<()>;
    fn TargetState(&self) -> windows_core::Result<TARGET_STATE>;
    fn SetTargetState(&self, state: TARGET_STATE) -> windows_core::Result<()>;
    fn TargetPropertySet(&self) -> windows_core::Result<ITsSbTargetPropertySet>;
    fn SetTargetPropertySet(&self, pval: Option<&ITsSbTargetPropertySet>) -> windows_core::Result<()>;
    fn EnvironmentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEnvironmentName(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NumSessions(&self) -> windows_core::Result<u32>;
    fn NumPendingConnections(&self) -> windows_core::Result<u32>;
    fn TargetLoad(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbTarget {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbTarget_Vtbl
    where
        Identity: ITsSbTarget_Impl,
    {
        unsafe extern "system" fn TargetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::TargetName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::SetTargetName(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn FarmName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::FarmName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFarmName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::SetFarmName(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn TargetFQDN<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetfqdnname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::TargetFQDN(this) {
                Ok(ok__) => {
                    targetfqdnname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetFQDN<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::SetTargetFQDN(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn TargetNetbios<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetnetbiosname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::TargetNetbios(this) {
                Ok(ok__) => {
                    targetnetbiosname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetNetbios<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::SetTargetNetbios(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn get_IpAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::get_IpAddresses(this, core::mem::transmute_copy(&sockaddr), core::mem::transmute_copy(&numaddresses)).into()
        }
        unsafe extern "system" fn put_IpAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::put_IpAddresses(this, core::mem::transmute_copy(&sockaddr), core::mem::transmute_copy(&numaddresses)).into()
        }
        unsafe extern "system" fn TargetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TARGET_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::TargetState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: TARGET_STATE) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::SetTargetState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn TargetPropertySet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::TargetPropertySet(this) {
                Ok(ok__) => {
                    pppropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetPropertySet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::SetTargetPropertySet(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn EnvironmentName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::EnvironmentName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTarget_Impl::SetEnvironmentName(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn NumSessions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumsessions: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::NumSessions(this) {
                Ok(ok__) => {
                    pnumsessions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumPendingConnections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpendingconnections: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::NumPendingConnections(this) {
                Ok(ok__) => {
                    pnumpendingconnections.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetLoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetload: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITsSbTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTarget_Impl::TargetLoad(this) {
                Ok(ok__) => {
                    ptargetload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetName: TargetName::<Identity, OFFSET>,
            SetTargetName: SetTargetName::<Identity, OFFSET>,
            FarmName: FarmName::<Identity, OFFSET>,
            SetFarmName: SetFarmName::<Identity, OFFSET>,
            TargetFQDN: TargetFQDN::<Identity, OFFSET>,
            SetTargetFQDN: SetTargetFQDN::<Identity, OFFSET>,
            TargetNetbios: TargetNetbios::<Identity, OFFSET>,
            SetTargetNetbios: SetTargetNetbios::<Identity, OFFSET>,
            get_IpAddresses: get_IpAddresses::<Identity, OFFSET>,
            put_IpAddresses: put_IpAddresses::<Identity, OFFSET>,
            TargetState: TargetState::<Identity, OFFSET>,
            SetTargetState: SetTargetState::<Identity, OFFSET>,
            TargetPropertySet: TargetPropertySet::<Identity, OFFSET>,
            SetTargetPropertySet: SetTargetPropertySet::<Identity, OFFSET>,
            EnvironmentName: EnvironmentName::<Identity, OFFSET>,
            SetEnvironmentName: SetEnvironmentName::<Identity, OFFSET>,
            NumSessions: NumSessions::<Identity, OFFSET>,
            NumPendingConnections: NumPendingConnections::<Identity, OFFSET>,
            TargetLoad: TargetLoad::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTargetPropertySet_Impl: Sized + ITsSbPropertySet_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbTargetPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTargetPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbTargetPropertySet_Vtbl
    where
        Identity: ITsSbTargetPropertySet_Impl,
    {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTargetPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITsSbTaskInfo_Impl: Sized {
    fn TargetId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StartTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn EndTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn Deadline(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn Identifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Context(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn Plugin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<RDV_TASK_STATUS>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITsSbTaskInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ITsSbTaskInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbTaskInfo_Vtbl
    where
        Identity: ITsSbTaskInfo_Impl,
    {
        unsafe extern "system" fn TargetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::TargetId(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::StartTime(this) {
                Ok(ok__) => {
                    pstarttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::EndTime(this) {
                Ok(ok__) => {
                    pendtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::Deadline(this) {
                Ok(ok__) => {
                    pdeadline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentifier: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::Identifier(this) {
                Ok(ok__) => {
                    pidentifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::Label(this) {
                Ok(ok__) => {
                    plabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::Context(this) {
                Ok(ok__) => {
                    pcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Plugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplugin: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::Plugin(this) {
                Ok(ok__) => {
                    pplugin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITsSbTaskInfo_Impl::Status(this) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetId: TargetId::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            EndTime: EndTime::<Identity, OFFSET>,
            Deadline: Deadline::<Identity, OFFSET>,
            Identifier: Identifier::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
            Plugin: Plugin::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTaskInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTaskPlugin_Impl: Sized + ITsSbPlugin_Impl {
    fn InitializeTaskPlugin(&self, pitssbtaskpluginnotifysink: Option<&ITsSbTaskPluginNotifySink>) -> windows_core::Result<()>;
    fn SetTaskQueue(&self, pszhostname: &windows_core::BSTR, sbtaskinfosize: u32, pitssbtaskinfo: *const Option<ITsSbTaskInfo>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbTaskPlugin {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTaskPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbTaskPlugin_Vtbl
    where
        Identity: ITsSbTaskPlugin_Impl,
    {
        unsafe extern "system" fn InitializeTaskPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitssbtaskpluginnotifysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTaskPlugin_Impl::InitializeTaskPlugin(this, windows_core::from_raw_borrowed(&pitssbtaskpluginnotifysink)).into()
        }
        unsafe extern "system" fn SetTaskQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhostname: core::mem::MaybeUninit<windows_core::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTaskPlugin_Impl::SetTaskQueue(this, core::mem::transmute(&pszhostname), core::mem::transmute_copy(&sbtaskinfosize), core::mem::transmute_copy(&pitssbtaskinfo)).into()
        }
        Self {
            base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(),
            InitializeTaskPlugin: InitializeTaskPlugin::<Identity, OFFSET>,
            SetTaskQueue: SetTaskQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTaskPlugin as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITsSbTaskPluginNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnSetTaskTime(&self, sztargetname: &windows_core::BSTR, taskstarttime: &super::super::Foundation::FILETIME, taskendtime: &super::super::Foundation::FILETIME, taskdeadline: &super::super::Foundation::FILETIME, sztasklabel: &windows_core::BSTR, sztaskidentifier: &windows_core::BSTR, sztaskplugin: &windows_core::BSTR, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnDeleteTaskTime(&self, sztargetname: &windows_core::BSTR, sztaskidentifier: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnUpdateTaskStatus(&self, sztargetname: &windows_core::BSTR, taskidentifier: &windows_core::BSTR, taskstatus: RDV_TASK_STATUS) -> windows_core::Result<()>;
    fn OnReportTasks(&self, szhostname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITsSbTaskPluginNotifySink {}
#[cfg(feature = "Win32_System_Com")]
impl ITsSbTaskPluginNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITsSbTaskPluginNotifySink_Vtbl
    where
        Identity: ITsSbTaskPluginNotifySink_Impl,
    {
        unsafe extern "system" fn OnSetTaskTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztargetname: core::mem::MaybeUninit<windows_core::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: core::mem::MaybeUninit<windows_core::BSTR>, sztaskidentifier: core::mem::MaybeUninit<windows_core::BSTR>, sztaskplugin: core::mem::MaybeUninit<windows_core::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTaskPluginNotifySink_Impl::OnSetTaskTime(this, core::mem::transmute(&sztargetname), core::mem::transmute(&taskstarttime), core::mem::transmute(&taskendtime), core::mem::transmute(&taskdeadline), core::mem::transmute(&sztasklabel), core::mem::transmute(&sztaskidentifier), core::mem::transmute(&sztaskplugin), core::mem::transmute_copy(&dwtaskstatus), core::mem::transmute_copy(&sacontext)).into()
        }
        unsafe extern "system" fn OnDeleteTaskTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztargetname: core::mem::MaybeUninit<windows_core::BSTR>, sztaskidentifier: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTaskPluginNotifySink_Impl::OnDeleteTaskTime(this, core::mem::transmute(&sztargetname), core::mem::transmute(&sztaskidentifier)).into()
        }
        unsafe extern "system" fn OnUpdateTaskStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztargetname: core::mem::MaybeUninit<windows_core::BSTR>, taskidentifier: core::mem::MaybeUninit<windows_core::BSTR>, taskstatus: RDV_TASK_STATUS) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTaskPluginNotifySink_Impl::OnUpdateTaskStatus(this, core::mem::transmute(&sztargetname), core::mem::transmute(&taskidentifier), core::mem::transmute_copy(&taskstatus)).into()
        }
        unsafe extern "system" fn OnReportTasks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhostname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITsSbTaskPluginNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITsSbTaskPluginNotifySink_Impl::OnReportTasks(this, core::mem::transmute(&szhostname)).into()
        }
        Self {
            base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(),
            OnSetTaskTime: OnSetTaskTime::<Identity, OFFSET>,
            OnDeleteTaskTime: OnDeleteTaskTime::<Identity, OFFSET>,
            OnUpdateTaskStatus: OnUpdateTaskStatus::<Identity, OFFSET>,
            OnReportTasks: OnReportTasks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTaskPluginNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
pub trait IWRdsEnhancedFastReconnectArbitrator_Impl: Sized {
    fn GetSessionForEnhancedFastReconnect(&self, psessionidarray: *const i32, dwsessioncount: u32) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IWRdsEnhancedFastReconnectArbitrator {}
impl IWRdsEnhancedFastReconnectArbitrator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsEnhancedFastReconnectArbitrator_Vtbl
    where
        Identity: IWRdsEnhancedFastReconnectArbitrator_Impl,
    {
        unsafe extern "system" fn GetSessionForEnhancedFastReconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWRdsEnhancedFastReconnectArbitrator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsEnhancedFastReconnectArbitrator_Impl::GetSessionForEnhancedFastReconnect(this, core::mem::transmute_copy(&psessionidarray), core::mem::transmute_copy(&dwsessioncount)) {
                Ok(ok__) => {
                    presultsessionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSessionForEnhancedFastReconnect: GetSessionForEnhancedFastReconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsEnhancedFastReconnectArbitrator as windows_core::Interface>::IID
    }
}
pub trait IWRdsGraphicsChannel_Impl: Sized {
    fn Write(&self, cbsize: u32, pbuffer: *const u8, pcontext: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Open(&self, pchannelevents: Option<&IWRdsGraphicsChannelEvents>, popencontext: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsGraphicsChannel {}
impl IWRdsGraphicsChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsGraphicsChannel_Vtbl
    where
        Identity: IWRdsGraphicsChannel_Impl,
    {
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannel_Impl::Write(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer), windows_core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannel_Impl::Close(this).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannelevents: *mut core::ffi::c_void, popencontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannel_Impl::Open(this, windows_core::from_raw_borrowed(&pchannelevents), windows_core::from_raw_borrowed(&popencontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Write: Write::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannel as windows_core::Interface>::IID
    }
}
pub trait IWRdsGraphicsChannelEvents_Impl: Sized {
    fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
    fn OnClose(&self) -> windows_core::Result<()>;
    fn OnChannelOpened(&self, openresult: windows_core::HRESULT, popencontext: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnDataSent(&self, pwritecontext: Option<&windows_core::IUnknown>, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> windows_core::Result<()>;
    fn OnMetricsUpdate(&self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsGraphicsChannelEvents {}
impl IWRdsGraphicsChannelEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsGraphicsChannelEvents_Vtbl
    where
        Identity: IWRdsGraphicsChannelEvents_Impl,
    {
        unsafe extern "system" fn OnDataReceived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannelEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannelEvents_Impl::OnDataReceived(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn OnClose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannelEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannelEvents_Impl::OnClose(this).into()
        }
        unsafe extern "system" fn OnChannelOpened<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, openresult: windows_core::HRESULT, popencontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannelEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannelEvents_Impl::OnChannelOpened(this, core::mem::transmute_copy(&openresult), windows_core::from_raw_borrowed(&popencontext)).into()
        }
        unsafe extern "system" fn OnDataSent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwritecontext: *mut core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannelEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannelEvents_Impl::OnDataSent(this, windows_core::from_raw_borrowed(&pwritecontext), core::mem::transmute_copy(&bcancelled), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer)).into()
        }
        unsafe extern "system" fn OnMetricsUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannelEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsGraphicsChannelEvents_Impl::OnMetricsUpdate(this, core::mem::transmute_copy(&bandwidth), core::mem::transmute_copy(&rtt), core::mem::transmute_copy(&lastsentbyteindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataReceived: OnDataReceived::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
            OnChannelOpened: OnChannelOpened::<Identity, OFFSET>,
            OnDataSent: OnDataSent::<Identity, OFFSET>,
            OnMetricsUpdate: OnMetricsUpdate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelEvents as windows_core::Interface>::IID
    }
}
pub trait IWRdsGraphicsChannelManager_Impl: Sized {
    fn CreateChannel(&self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> windows_core::Result<IWRdsGraphicsChannel>;
}
impl windows_core::RuntimeName for IWRdsGraphicsChannelManager {}
impl IWRdsGraphicsChannelManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsGraphicsChannelManager_Vtbl
    where
        Identity: IWRdsGraphicsChannelManager_Impl,
    {
        unsafe extern "system" fn CreateChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsGraphicsChannelManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsGraphicsChannelManager_Impl::CreateChannel(this, core::mem::transmute_copy(&pszchannelname), core::mem::transmute_copy(&channeltype)) {
                Ok(ok__) => {
                    ppvirtualchannel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateChannel: CreateChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelManager as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolConnection_Impl: Sized {
    fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWRdsProtocolLogonErrorRedirector>;
    fn AcceptConnection(&self) -> windows_core::Result<()>;
    fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()>;
    fn GetClientMonitorData(&self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> windows_core::Result<()>;
    fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()>;
    fn GetLicenseConnection(&self) -> windows_core::Result<IWRdsProtocolLicenseConnection>;
    fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn GetInputHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn GetVideoHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE_PTR>;
    fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()>;
    fn IsUserAllowedToLogon(&self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: &windows_core::PCWSTR, pusername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SessionArbitrationEnumeration(&self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()>;
    fn LogonNotify(&self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: &windows_core::PCWSTR, wszdomainname: &windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()>;
    fn PreDisconnect(&self, disconnectreason: u32) -> windows_core::Result<()>;
    fn DisconnectNotify(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()>;
    fn GetLastInputTime(&self) -> windows_core::Result<u64>;
    fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()>;
    fn CreateVirtualChannel(&self, szendpointname: &windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32) -> windows_core::Result<usize>;
    fn QueryProperty(&self, querytype: &windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetShadowConnection(&self) -> windows_core::Result<IWRdsProtocolShadowConnection>;
    fn NotifyCommandProcessCreated(&self, sessionid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolConnection {}
impl IWRdsProtocolConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolConnection_Vtbl
    where
        Identity: IWRdsProtocolConnection_Impl,
    {
        unsafe extern "system" fn GetLogonErrorRedirector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplogonerrorredir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolConnection_Impl::GetLogonErrorRedirector(this) {
                Ok(ok__) => {
                    pplogonerrorredir.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::AcceptConnection(this).into()
        }
        unsafe extern "system" fn GetClientData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::GetClientData(this, core::mem::transmute_copy(&pclientdata)).into()
        }
        unsafe extern "system" fn GetClientMonitorData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::GetClientMonitorData(this, core::mem::transmute_copy(&pnummonitors), core::mem::transmute_copy(&pprimarymonitor)).into()
        }
        unsafe extern "system" fn GetUserCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::GetUserCredentials(this, core::mem::transmute_copy(&pusercreds)).into()
        }
        unsafe extern "system" fn GetLicenseConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicenseconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolConnection_Impl::GetLicenseConnection(this) {
                Ok(ok__) => {
                    pplicenseconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::AuthenticateClientToSession(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::NotifySessionId(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&sessionhandle)).into()
        }
        unsafe extern "system" fn GetInputHandles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::GetInputHandles(this, core::mem::transmute_copy(&pkeyboardhandle), core::mem::transmute_copy(&pmousehandle), core::mem::transmute_copy(&pbeephandle)).into()
        }
        unsafe extern "system" fn GetVideoHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolConnection_Impl::GetVideoHandle(this) {
                Ok(ok__) => {
                    pvideohandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::ConnectNotify(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: windows_core::PCWSTR, pusername: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::IsUserAllowedToLogon(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&usertoken), core::mem::transmute(&pdomainname), core::mem::transmute(&pusername)).into()
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::SessionArbitrationEnumeration(this, core::mem::transmute_copy(&husertoken), core::mem::transmute_copy(&bsinglesessionperuserenabled), core::mem::transmute_copy(&psessionidarray), core::mem::transmute_copy(&pdwsessionidentifiercount)).into()
        }
        unsafe extern "system" fn LogonNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: windows_core::PCWSTR, wszdomainname: windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::LogonNotify(this, core::mem::transmute_copy(&hclienttoken), core::mem::transmute(&wszusername), core::mem::transmute(&wszdomainname), core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&pwrdsconnectionsettings)).into()
        }
        unsafe extern "system" fn PreDisconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectreason: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::PreDisconnect(this, core::mem::transmute_copy(&disconnectreason)).into()
        }
        unsafe extern "system" fn DisconnectNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::DisconnectNotify(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::Close(this).into()
        }
        unsafe extern "system" fn GetProtocolStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::GetProtocolStatus(this, core::mem::transmute_copy(&pprotocolstatus)).into()
        }
        unsafe extern "system" fn GetLastInputTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastinputtime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolConnection_Impl::GetLastInputTime(this) {
                Ok(ok__) => {
                    plastinputtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulerror: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::SetErrorInfo(this, core::mem::transmute_copy(&ulerror)).into()
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szendpointname: windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolConnection_Impl::CreateVirtualChannel(this, core::mem::transmute(&szendpointname), core::mem::transmute_copy(&bstatic), core::mem::transmute_copy(&requestedpriority)) {
                Ok(ok__) => {
                    phchannel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, querytype: windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::QueryProperty(this, core::mem::transmute(&querytype), core::mem::transmute_copy(&ulnumentriesin), core::mem::transmute_copy(&ulnumentriesout), core::mem::transmute_copy(&ppropertyentriesin), core::mem::transmute_copy(&ppropertyentriesout)).into()
        }
        unsafe extern "system" fn GetShadowConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshadowconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolConnection_Impl::GetShadowConnection(this) {
                Ok(ok__) => {
                    ppshadowconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyCommandProcessCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnection_Impl::NotifyCommandProcessCreated(this, core::mem::transmute_copy(&sessionid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Identity, OFFSET>,
            AcceptConnection: AcceptConnection::<Identity, OFFSET>,
            GetClientData: GetClientData::<Identity, OFFSET>,
            GetClientMonitorData: GetClientMonitorData::<Identity, OFFSET>,
            GetUserCredentials: GetUserCredentials::<Identity, OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Identity, OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Identity, OFFSET>,
            NotifySessionId: NotifySessionId::<Identity, OFFSET>,
            GetInputHandles: GetInputHandles::<Identity, OFFSET>,
            GetVideoHandle: GetVideoHandle::<Identity, OFFSET>,
            ConnectNotify: ConnectNotify::<Identity, OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Identity, OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Identity, OFFSET>,
            LogonNotify: LogonNotify::<Identity, OFFSET>,
            PreDisconnect: PreDisconnect::<Identity, OFFSET>,
            DisconnectNotify: DisconnectNotify::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Identity, OFFSET>,
            GetLastInputTime: GetLastInputTime::<Identity, OFFSET>,
            SetErrorInfo: SetErrorInfo::<Identity, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, OFFSET>,
            QueryProperty: QueryProperty::<Identity, OFFSET>,
            GetShadowConnection: GetShadowConnection::<Identity, OFFSET>,
            NotifyCommandProcessCreated: NotifyCommandProcessCreated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolConnection as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolConnectionCallback_Impl: Sized {
    fn OnReady(&self) -> windows_core::Result<()>;
    fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()>;
    fn StopScreenUpdates(&self) -> windows_core::Result<()>;
    fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()>;
    fn GetConnectionId(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWRdsProtocolConnectionCallback {}
impl IWRdsProtocolConnectionCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolConnectionCallback_Vtbl
    where
        Identity: IWRdsProtocolConnectionCallback_Impl,
    {
        unsafe extern "system" fn OnReady<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnectionCallback_Impl::OnReady(this).into()
        }
        unsafe extern "system" fn BrokenConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: u32, source: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnectionCallback_Impl::BrokenConnection(this, core::mem::transmute_copy(&reason), core::mem::transmute_copy(&source)).into()
        }
        unsafe extern "system" fn StopScreenUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnectionCallback_Impl::StopScreenUpdates(this).into()
        }
        unsafe extern "system" fn RedrawWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnectionCallback_Impl::RedrawWindow(this, core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn GetConnectionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolConnectionCallback_Impl::GetConnectionId(this) {
                Ok(ok__) => {
                    pconnectionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnReady: OnReady::<Identity, OFFSET>,
            BrokenConnection: BrokenConnection::<Identity, OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Identity, OFFSET>,
            RedrawWindow: RedrawWindow::<Identity, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionCallback as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolConnectionSettings_Impl: Sized {
    fn SetConnectionSetting(&self, propertyid: &windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetConnectionSetting(&self, propertyid: &windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolConnectionSettings {}
impl IWRdsProtocolConnectionSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolConnectionSettings_Vtbl
    where
        Identity: IWRdsProtocolConnectionSettings_Impl,
    {
        unsafe extern "system" fn SetConnectionSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnectionSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnectionSettings_Impl::SetConnectionSetting(this, core::mem::transmute(&propertyid), core::mem::transmute_copy(&ppropertyentriesin)).into()
        }
        unsafe extern "system" fn GetConnectionSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolConnectionSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolConnectionSettings_Impl::GetConnectionSetting(this, core::mem::transmute(&propertyid), core::mem::transmute_copy(&ppropertyentriesout)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetConnectionSetting: SetConnectionSetting::<Identity, OFFSET>,
            GetConnectionSetting: GetConnectionSetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionSettings as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolLicenseConnection_Impl: Sized {
    fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()>;
    fn SendClientLicense(&self, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::Result<()>;
    fn RequestClientLicense(&self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()>;
    fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolLicenseConnection {}
impl IWRdsProtocolLicenseConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolLicenseConnection_Vtbl
    where
        Identity: IWRdsProtocolLicenseConnection_Impl,
    {
        unsafe extern "system" fn RequestLicensingCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolLicenseConnection_Impl::RequestLicensingCapabilities(this, core::mem::transmute_copy(&pplicensecapabilities), core::mem::transmute_copy(&pcblicensecapabilities)).into()
        }
        unsafe extern "system" fn SendClientLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolLicenseConnection_Impl::SendClientLicense(this, core::mem::transmute_copy(&pclientlicense), core::mem::transmute_copy(&cbclientlicense)).into()
        }
        unsafe extern "system" fn RequestClientLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolLicenseConnection_Impl::RequestClientLicense(this, core::mem::transmute_copy(&reserve1), core::mem::transmute_copy(&reserve2), core::mem::transmute_copy(&ppclientlicense), core::mem::transmute_copy(&pcbclientlicense)).into()
        }
        unsafe extern "system" fn ProtocolComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcomplete: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolLicenseConnection_Impl::ProtocolComplete(this, core::mem::transmute_copy(&ulcomplete)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Identity, OFFSET>,
            SendClientLicense: SendClientLicense::<Identity, OFFSET>,
            RequestClientLicense: RequestClientLicense::<Identity, OFFSET>,
            ProtocolComplete: ProtocolComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolLicenseConnection as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolListener_Impl: Sized {
    fn GetSettings(&self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> windows_core::Result<WRDS_LISTENER_SETTINGS>;
    fn StartListen(&self, pcallback: Option<&IWRdsProtocolListenerCallback>) -> windows_core::Result<()>;
    fn StopListen(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolListener {}
impl IWRdsProtocolListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolListener_Vtbl
    where
        Identity: IWRdsProtocolListener_Impl,
    {
        unsafe extern "system" fn GetSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolListener_Impl::GetSettings(this, core::mem::transmute_copy(&wrdslistenersettinglevel)) {
                Ok(ok__) => {
                    pwrdslistenersettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartListen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolListener_Impl::StartListen(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn StopListen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolListener_Impl::StopListen(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSettings: GetSettings::<Identity, OFFSET>,
            StartListen: StartListen::<Identity, OFFSET>,
            StopListen: StopListen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolListener as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolListenerCallback_Impl: Sized {
    fn OnConnected(&self, pconnection: Option<&IWRdsProtocolConnection>, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> windows_core::Result<IWRdsProtocolConnectionCallback>;
}
impl windows_core::RuntimeName for IWRdsProtocolListenerCallback {}
impl IWRdsProtocolListenerCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolListenerCallback_Vtbl
    where
        Identity: IWRdsProtocolListenerCallback_Impl,
    {
        unsafe extern "system" fn OnConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolListenerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolListenerCallback_Impl::OnConnected(this, windows_core::from_raw_borrowed(&pconnection), core::mem::transmute_copy(&pwrdsconnectionsettings)) {
                Ok(ok__) => {
                    pcallback.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnected: OnConnected::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolListenerCallback as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolLogonErrorRedirector_Impl: Sized {
    fn OnBeginPainting(&self) -> windows_core::Result<()>;
    fn RedirectStatus(&self, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(&self, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
impl windows_core::RuntimeName for IWRdsProtocolLogonErrorRedirector {}
impl IWRdsProtocolLogonErrorRedirector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolLogonErrorRedirector_Vtbl
    where
        Identity: IWRdsProtocolLogonErrorRedirector_Impl,
    {
        unsafe extern "system" fn OnBeginPainting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolLogonErrorRedirector_Impl::OnBeginPainting(this).into()
        }
        unsafe extern "system" fn RedirectStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmessage: windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolLogonErrorRedirector_Impl::RedirectStatus(this, core::mem::transmute(&pszmessage)) {
                Ok(ok__) => {
                    presponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolLogonErrorRedirector_Impl::RedirectMessage(this, core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                Ok(ok__) => {
                    presponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectLogonError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolLogonErrorRedirector_Impl::RedirectLogonError(this, core::mem::transmute_copy(&ntsstatus), core::mem::transmute_copy(&ntssubstatus), core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                Ok(ok__) => {
                    presponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBeginPainting: OnBeginPainting::<Identity, OFFSET>,
            RedirectStatus: RedirectStatus::<Identity, OFFSET>,
            RedirectMessage: RedirectMessage::<Identity, OFFSET>,
            RedirectLogonError: RedirectLogonError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolLogonErrorRedirector as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolManager_Impl: Sized {
    fn Initialize(&self, piwrdssettings: Option<&IWRdsProtocolSettings>, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()>;
    fn CreateListener(&self, wszlistenername: &windows_core::PCWSTR) -> windows_core::Result<IWRdsProtocolListener>;
    fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()>;
    fn NotifySettingsChange(&self, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolManager {}
impl IWRdsProtocolManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolManager_Vtbl
    where
        Identity: IWRdsProtocolManager_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwrdssettings: *mut core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolManager_Impl::Initialize(this, windows_core::from_raw_borrowed(&piwrdssettings), core::mem::transmute_copy(&pwrdssettings)).into()
        }
        unsafe extern "system" fn CreateListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlistenername: windows_core::PCWSTR, pprotocollistener: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWRdsProtocolManager_Impl::CreateListener(this, core::mem::transmute(&wszlistenername)) {
                Ok(ok__) => {
                    pprotocollistener.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolManager_Impl::NotifyServiceStateChange(this, core::mem::transmute_copy(&ptsservicestatechange)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolManager_Impl::NotifySessionOfServiceStart(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolManager_Impl::NotifySessionOfServiceStop(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolManager_Impl::NotifySessionStateChange(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&eventid)).into()
        }
        unsafe extern "system" fn NotifySettingsChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolManager_Impl::NotifySettingsChange(this, core::mem::transmute_copy(&pwrdssettings)).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolManager_Impl::Uninitialize(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            CreateListener: CreateListener::<Identity, OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Identity, OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Identity, OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Identity, OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Identity, OFFSET>,
            NotifySettingsChange: NotifySettingsChange::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolManager as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolSettings_Impl: Sized {
    fn GetSettings(&self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> windows_core::Result<()>;
    fn MergeSettings(&self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolSettings {}
impl IWRdsProtocolSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolSettings_Vtbl
    where
        Identity: IWRdsProtocolSettings_Impl,
    {
        unsafe extern "system" fn GetSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolSettings_Impl::GetSettings(this, core::mem::transmute_copy(&wrdssettingtype), core::mem::transmute_copy(&wrdssettinglevel), core::mem::transmute_copy(&pwrdssettings)).into()
        }
        unsafe extern "system" fn MergeSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolSettings_Impl::MergeSettings(this, core::mem::transmute_copy(&pwrdssettings), core::mem::transmute_copy(&wrdsconnectionsettinglevel), core::mem::transmute_copy(&pwrdsconnectionsettings)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSettings: GetSettings::<Identity, OFFSET>,
            MergeSettings: MergeSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolSettings as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolShadowCallback_Impl: Sized {
    fn StopShadow(&self) -> windows_core::Result<()>;
    fn InvokeTargetShadow(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolShadowCallback {}
impl IWRdsProtocolShadowCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolShadowCallback_Vtbl
    where
        Identity: IWRdsProtocolShadowCallback_Impl,
    {
        unsafe extern "system" fn StopShadow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolShadowCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolShadowCallback_Impl::StopShadow(this).into()
        }
        unsafe extern "system" fn InvokeTargetShadow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolShadowCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolShadowCallback_Impl::InvokeTargetShadow(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StopShadow: StopShadow::<Identity, OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowCallback as windows_core::Interface>::IID
    }
}
pub trait IWRdsProtocolShadowConnection_Impl: Sized {
    fn Start(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: Option<&IWRdsProtocolShadowCallback>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn DoTarget(&self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsProtocolShadowConnection {}
impl IWRdsProtocolShadowConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsProtocolShadowConnection_Vtbl
    where
        Identity: IWRdsProtocolShadowConnection_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolShadowConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolShadowConnection_Impl::Start(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&hotkeyvk), core::mem::transmute_copy(&hotkeymodifiers), windows_core::from_raw_borrowed(&pshadowcallback)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolShadowConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolShadowConnection_Impl::Stop(this).into()
        }
        unsafe extern "system" fn DoTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWRdsProtocolShadowConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsProtocolShadowConnection_Impl::DoTarget(this, core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            DoTarget: DoTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowConnection as windows_core::Interface>::IID
    }
}
pub trait IWRdsWddmIddProps_Impl: Sized {
    fn GetHardwareId(&self, pdisplaydriverhardwareid: &windows_core::PCWSTR, count: u32) -> windows_core::Result<()>;
    fn OnDriverLoad(&self, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()>;
    fn EnableWddmIdd(&self, enabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsWddmIddProps {}
impl IWRdsWddmIddProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsWddmIddProps_Vtbl
    where
        Identity: IWRdsWddmIddProps_Impl,
    {
        unsafe extern "system" fn GetHardwareId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplaydriverhardwareid: windows_core::PCWSTR, count: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsWddmIddProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsWddmIddProps_Impl::GetHardwareId(this, core::mem::transmute(&pdisplaydriverhardwareid), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn OnDriverLoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IWRdsWddmIddProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsWddmIddProps_Impl::OnDriverLoad(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&driverhandle)).into()
        }
        unsafe extern "system" fn OnDriverUnload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsWddmIddProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsWddmIddProps_Impl::OnDriverUnload(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn EnableWddmIdd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWRdsWddmIddProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsWddmIddProps_Impl::EnableWddmIdd(this, core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHardwareId: GetHardwareId::<Identity, OFFSET>,
            OnDriverLoad: OnDriverLoad::<Identity, OFFSET>,
            OnDriverUnload: OnDriverUnload::<Identity, OFFSET>,
            EnableWddmIdd: EnableWddmIdd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsWddmIddProps as windows_core::Interface>::IID
    }
}
pub trait IWRdsWddmIddProps1_Impl: Sized {
    fn GetHardwareId(&self, pdisplaydriverhardwareid: &windows_core::PCWSTR, count: u32) -> windows_core::Result<()>;
    fn OnDriverLoad(&self, sessionid: u32, deviceinstance: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWRdsWddmIddProps1 {}
impl IWRdsWddmIddProps1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWRdsWddmIddProps1_Vtbl
    where
        Identity: IWRdsWddmIddProps1_Impl,
    {
        unsafe extern "system" fn GetHardwareId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplaydriverhardwareid: windows_core::PCWSTR, count: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsWddmIddProps1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsWddmIddProps1_Impl::GetHardwareId(this, core::mem::transmute(&pdisplaydriverhardwareid), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn OnDriverLoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, deviceinstance: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWRdsWddmIddProps1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsWddmIddProps1_Impl::OnDriverLoad(this, core::mem::transmute_copy(&sessionid), core::mem::transmute(&deviceinstance)).into()
        }
        unsafe extern "system" fn OnDriverUnload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT
        where
            Identity: IWRdsWddmIddProps1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWRdsWddmIddProps1_Impl::OnDriverUnload(this, core::mem::transmute_copy(&sessionid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHardwareId: GetHardwareId::<Identity, OFFSET>,
            OnDriverLoad: OnDriverLoad::<Identity, OFFSET>,
            OnDriverUnload: OnDriverUnload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsWddmIddProps1 as windows_core::Interface>::IID
    }
}
pub trait IWTSBitmapRenderService_Impl: Sized {
    fn GetMappedRenderer(&self, mappingid: u64, pmappedrenderercallback: Option<&IWTSBitmapRendererCallback>) -> windows_core::Result<IWTSBitmapRenderer>;
}
impl windows_core::RuntimeName for IWTSBitmapRenderService {}
impl IWTSBitmapRenderService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSBitmapRenderService_Vtbl
    where
        Identity: IWTSBitmapRenderService_Impl,
    {
        unsafe extern "system" fn GetMappedRenderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mappingid: u64, pmappedrenderercallback: *mut core::ffi::c_void, ppmappedrenderer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSBitmapRenderService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSBitmapRenderService_Impl::GetMappedRenderer(this, core::mem::transmute_copy(&mappingid), windows_core::from_raw_borrowed(&pmappedrenderercallback)) {
                Ok(ok__) => {
                    ppmappedrenderer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMappedRenderer: GetMappedRenderer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRenderService as windows_core::Interface>::IID
    }
}
pub trait IWTSBitmapRenderer_Impl: Sized {
    fn Render(&self, imageformat: &windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> windows_core::Result<()>;
    fn GetRendererStatistics(&self) -> windows_core::Result<BITMAP_RENDERER_STATISTICS>;
    fn RemoveMapping(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSBitmapRenderer {}
impl IWTSBitmapRenderer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSBitmapRenderer_Vtbl
    where
        Identity: IWTSBitmapRenderer_Impl,
    {
        unsafe extern "system" fn Render<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageformat: windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> windows_core::HRESULT
        where
            Identity: IWTSBitmapRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSBitmapRenderer_Impl::Render(this, core::mem::transmute(&imageformat), core::mem::transmute_copy(&dwwidth), core::mem::transmute_copy(&dwheight), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbimagebuffer), core::mem::transmute_copy(&pimagebuffer)).into()
        }
        unsafe extern "system" fn GetRendererStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IWTSBitmapRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSBitmapRenderer_Impl::GetRendererStatistics(this) {
                Ok(ok__) => {
                    pstatistics.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSBitmapRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSBitmapRenderer_Impl::RemoveMapping(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Render: Render::<Identity, OFFSET>,
            GetRendererStatistics: GetRendererStatistics::<Identity, OFFSET>,
            RemoveMapping: RemoveMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRenderer as windows_core::Interface>::IID
    }
}
pub trait IWTSBitmapRendererCallback_Impl: Sized {
    fn OnTargetSizeChanged(&self, rcnewsize: &super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSBitmapRendererCallback {}
impl IWTSBitmapRendererCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSBitmapRendererCallback_Vtbl
    where
        Identity: IWTSBitmapRendererCallback_Impl,
    {
        unsafe extern "system" fn OnTargetSizeChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IWTSBitmapRendererCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSBitmapRendererCallback_Impl::OnTargetSizeChanged(this, core::mem::transmute(&rcnewsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTargetSizeChanged: OnTargetSizeChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRendererCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWTSListener_Impl: Sized {
    fn GetConfiguration(&self) -> windows_core::Result<super::Com::StructuredStorage::IPropertyBag>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWTSListener {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWTSListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSListener_Vtbl
    where
        Identity: IWTSListener_Impl,
    {
        unsafe extern "system" fn GetConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSListener_Impl::GetConfiguration(this) {
                Ok(ok__) => {
                    pppropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetConfiguration: GetConfiguration::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSListener as windows_core::Interface>::IID
    }
}
pub trait IWTSListenerCallback_Impl: Sized {
    fn OnNewChannelConnection(&self, pchannel: Option<&IWTSVirtualChannel>, data: &windows_core::BSTR, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut Option<IWTSVirtualChannelCallback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSListenerCallback {}
impl IWTSListenerCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSListenerCallback_Vtbl
    where
        Identity: IWTSListenerCallback_Impl,
    {
        unsafe extern "system" fn OnNewChannelConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSListenerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSListenerCallback_Impl::OnNewChannelConnection(this, windows_core::from_raw_borrowed(&pchannel), core::mem::transmute(&data), core::mem::transmute_copy(&pbaccept), core::mem::transmute_copy(&ppcallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNewChannelConnection: OnNewChannelConnection::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSListenerCallback as windows_core::Interface>::IID
    }
}
pub trait IWTSPlugin_Impl: Sized {
    fn Initialize(&self, pchannelmgr: Option<&IWTSVirtualChannelManager>) -> windows_core::Result<()>;
    fn Connected(&self) -> windows_core::Result<()>;
    fn Disconnected(&self, dwdisconnectcode: u32) -> windows_core::Result<()>;
    fn Terminated(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSPlugin {}
impl IWTSPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSPlugin_Vtbl
    where
        Identity: IWTSPlugin_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannelmgr: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSPlugin_Impl::Initialize(this, windows_core::from_raw_borrowed(&pchannelmgr)).into()
        }
        unsafe extern "system" fn Connected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSPlugin_Impl::Connected(this).into()
        }
        unsafe extern "system" fn Disconnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdisconnectcode: u32) -> windows_core::HRESULT
        where
            Identity: IWTSPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSPlugin_Impl::Disconnected(this, core::mem::transmute_copy(&dwdisconnectcode)).into()
        }
        unsafe extern "system" fn Terminated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSPlugin_Impl::Terminated(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Connected: Connected::<Identity, OFFSET>,
            Disconnected: Disconnected::<Identity, OFFSET>,
            Terminated: Terminated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSPlugin as windows_core::Interface>::IID
    }
}
pub trait IWTSPluginServiceProvider_Impl: Sized {
    fn GetService(&self, serviceid: &windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IWTSPluginServiceProvider {}
impl IWTSPluginServiceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSPluginServiceProvider_Vtbl
    where
        Identity: IWTSPluginServiceProvider_Impl,
    {
        unsafe extern "system" fn GetService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: windows_core::GUID, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSPluginServiceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSPluginServiceProvider_Impl::GetService(this, core::mem::transmute(&serviceid)) {
                Ok(ok__) => {
                    ppunkobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSPluginServiceProvider as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolConnection_Impl: Sized {
    fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWTSProtocolLogonErrorRedirector>;
    fn SendPolicyData(&self, ppolicydata: *const WTS_POLICY_DATA) -> windows_core::Result<()>;
    fn AcceptConnection(&self) -> windows_core::Result<()>;
    fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()>;
    fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()>;
    fn GetLicenseConnection(&self) -> windows_core::Result<IWTSProtocolLicenseConnection>;
    fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn GetProtocolHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()>;
    fn IsUserAllowedToLogon(&self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: &windows_core::PCWSTR, pusername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SessionArbitrationEnumeration(&self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()>;
    fn LogonNotify(&self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: &windows_core::PCWSTR, wszdomainname: &windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn GetUserData(&self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> windows_core::Result<()>;
    fn DisconnectNotify(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()>;
    fn GetLastInputTime(&self) -> windows_core::Result<u64>;
    fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()>;
    fn SendBeep(&self, frequency: u32, duration: u32) -> windows_core::Result<()>;
    fn CreateVirtualChannel(&self, szendpointname: &windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32) -> windows_core::Result<usize>;
    fn QueryProperty(&self, querytype: &windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetShadowConnection(&self) -> windows_core::Result<IWTSProtocolShadowConnection>;
}
impl windows_core::RuntimeName for IWTSProtocolConnection {}
impl IWTSProtocolConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolConnection_Vtbl
    where
        Identity: IWTSProtocolConnection_Impl,
    {
        unsafe extern "system" fn GetLogonErrorRedirector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplogonerrorredir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolConnection_Impl::GetLogonErrorRedirector(this) {
                Ok(ok__) => {
                    pplogonerrorredir.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendPolicyData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::SendPolicyData(this, core::mem::transmute_copy(&ppolicydata)).into()
        }
        unsafe extern "system" fn AcceptConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::AcceptConnection(this).into()
        }
        unsafe extern "system" fn GetClientData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::GetClientData(this, core::mem::transmute_copy(&pclientdata)).into()
        }
        unsafe extern "system" fn GetUserCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::GetUserCredentials(this, core::mem::transmute_copy(&pusercreds)).into()
        }
        unsafe extern "system" fn GetLicenseConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicenseconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolConnection_Impl::GetLicenseConnection(this) {
                Ok(ok__) => {
                    pplicenseconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::AuthenticateClientToSession(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::NotifySessionId(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn GetProtocolHandles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::GetProtocolHandles(this, core::mem::transmute_copy(&pkeyboardhandle), core::mem::transmute_copy(&pmousehandle), core::mem::transmute_copy(&pbeephandle), core::mem::transmute_copy(&pvideohandle)).into()
        }
        unsafe extern "system" fn ConnectNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::ConnectNotify(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: windows_core::PCWSTR, pusername: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::IsUserAllowedToLogon(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&usertoken), core::mem::transmute(&pdomainname), core::mem::transmute(&pusername)).into()
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::SessionArbitrationEnumeration(this, core::mem::transmute_copy(&husertoken), core::mem::transmute_copy(&bsinglesessionperuserenabled), core::mem::transmute_copy(&psessionidarray), core::mem::transmute_copy(&pdwsessionidentifiercount)).into()
        }
        unsafe extern "system" fn LogonNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: windows_core::PCWSTR, wszdomainname: windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::LogonNotify(this, core::mem::transmute_copy(&hclienttoken), core::mem::transmute(&wszusername), core::mem::transmute(&wszdomainname), core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn GetUserData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::GetUserData(this, core::mem::transmute_copy(&ppolicydata), core::mem::transmute_copy(&pclientdata)).into()
        }
        unsafe extern "system" fn DisconnectNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::DisconnectNotify(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::Close(this).into()
        }
        unsafe extern "system" fn GetProtocolStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::GetProtocolStatus(this, core::mem::transmute_copy(&pprotocolstatus)).into()
        }
        unsafe extern "system" fn GetLastInputTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastinputtime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolConnection_Impl::GetLastInputTime(this) {
                Ok(ok__) => {
                    plastinputtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulerror: u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::SetErrorInfo(this, core::mem::transmute_copy(&ulerror)).into()
        }
        unsafe extern "system" fn SendBeep<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequency: u32, duration: u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::SendBeep(this, core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szendpointname: windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolConnection_Impl::CreateVirtualChannel(this, core::mem::transmute(&szendpointname), core::mem::transmute_copy(&bstatic), core::mem::transmute_copy(&requestedpriority)) {
                Ok(ok__) => {
                    phchannel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, querytype: windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnection_Impl::QueryProperty(this, core::mem::transmute(&querytype), core::mem::transmute_copy(&ulnumentriesin), core::mem::transmute_copy(&ulnumentriesout), core::mem::transmute_copy(&ppropertyentriesin), core::mem::transmute_copy(&ppropertyentriesout)).into()
        }
        unsafe extern "system" fn GetShadowConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshadowconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolConnection_Impl::GetShadowConnection(this) {
                Ok(ok__) => {
                    ppshadowconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Identity, OFFSET>,
            SendPolicyData: SendPolicyData::<Identity, OFFSET>,
            AcceptConnection: AcceptConnection::<Identity, OFFSET>,
            GetClientData: GetClientData::<Identity, OFFSET>,
            GetUserCredentials: GetUserCredentials::<Identity, OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Identity, OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Identity, OFFSET>,
            NotifySessionId: NotifySessionId::<Identity, OFFSET>,
            GetProtocolHandles: GetProtocolHandles::<Identity, OFFSET>,
            ConnectNotify: ConnectNotify::<Identity, OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Identity, OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Identity, OFFSET>,
            LogonNotify: LogonNotify::<Identity, OFFSET>,
            GetUserData: GetUserData::<Identity, OFFSET>,
            DisconnectNotify: DisconnectNotify::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Identity, OFFSET>,
            GetLastInputTime: GetLastInputTime::<Identity, OFFSET>,
            SetErrorInfo: SetErrorInfo::<Identity, OFFSET>,
            SendBeep: SendBeep::<Identity, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, OFFSET>,
            QueryProperty: QueryProperty::<Identity, OFFSET>,
            GetShadowConnection: GetShadowConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolConnection as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolConnectionCallback_Impl: Sized {
    fn OnReady(&self) -> windows_core::Result<()>;
    fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()>;
    fn StopScreenUpdates(&self) -> windows_core::Result<()>;
    fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()>;
    fn DisplayIOCtl(&self, displayioctl: *const WTS_DISPLAY_IOCTL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSProtocolConnectionCallback {}
impl IWTSProtocolConnectionCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolConnectionCallback_Vtbl
    where
        Identity: IWTSProtocolConnectionCallback_Impl,
    {
        unsafe extern "system" fn OnReady<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnectionCallback_Impl::OnReady(this).into()
        }
        unsafe extern "system" fn BrokenConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: u32, source: u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnectionCallback_Impl::BrokenConnection(this, core::mem::transmute_copy(&reason), core::mem::transmute_copy(&source)).into()
        }
        unsafe extern "system" fn StopScreenUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnectionCallback_Impl::StopScreenUpdates(this).into()
        }
        unsafe extern "system" fn RedrawWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnectionCallback_Impl::RedrawWindow(this, core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn DisplayIOCtl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolConnectionCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolConnectionCallback_Impl::DisplayIOCtl(this, core::mem::transmute_copy(&displayioctl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnReady: OnReady::<Identity, OFFSET>,
            BrokenConnection: BrokenConnection::<Identity, OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Identity, OFFSET>,
            RedrawWindow: RedrawWindow::<Identity, OFFSET>,
            DisplayIOCtl: DisplayIOCtl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolConnectionCallback as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolLicenseConnection_Impl: Sized {
    fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()>;
    fn SendClientLicense(&self, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::Result<()>;
    fn RequestClientLicense(&self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()>;
    fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSProtocolLicenseConnection {}
impl IWTSProtocolLicenseConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolLicenseConnection_Vtbl
    where
        Identity: IWTSProtocolLicenseConnection_Impl,
    {
        unsafe extern "system" fn RequestLicensingCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolLicenseConnection_Impl::RequestLicensingCapabilities(this, core::mem::transmute_copy(&pplicensecapabilities), core::mem::transmute_copy(&pcblicensecapabilities)).into()
        }
        unsafe extern "system" fn SendClientLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolLicenseConnection_Impl::SendClientLicense(this, core::mem::transmute_copy(&pclientlicense), core::mem::transmute_copy(&cbclientlicense)).into()
        }
        unsafe extern "system" fn RequestClientLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolLicenseConnection_Impl::RequestClientLicense(this, core::mem::transmute_copy(&reserve1), core::mem::transmute_copy(&reserve2), core::mem::transmute_copy(&ppclientlicense), core::mem::transmute_copy(&pcbclientlicense)).into()
        }
        unsafe extern "system" fn ProtocolComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcomplete: u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLicenseConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolLicenseConnection_Impl::ProtocolComplete(this, core::mem::transmute_copy(&ulcomplete)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Identity, OFFSET>,
            SendClientLicense: SendClientLicense::<Identity, OFFSET>,
            RequestClientLicense: RequestClientLicense::<Identity, OFFSET>,
            ProtocolComplete: ProtocolComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolLicenseConnection as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolListener_Impl: Sized {
    fn StartListen(&self, pcallback: Option<&IWTSProtocolListenerCallback>) -> windows_core::Result<()>;
    fn StopListen(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSProtocolListener {}
impl IWTSProtocolListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolListener_Vtbl
    where
        Identity: IWTSProtocolListener_Impl,
    {
        unsafe extern "system" fn StartListen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolListener_Impl::StartListen(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn StopListen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolListener_Impl::StopListen(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListen: StartListen::<Identity, OFFSET>,
            StopListen: StopListen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolListener as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolListenerCallback_Impl: Sized {
    fn OnConnected(&self, pconnection: Option<&IWTSProtocolConnection>) -> windows_core::Result<IWTSProtocolConnectionCallback>;
}
impl windows_core::RuntimeName for IWTSProtocolListenerCallback {}
impl IWTSProtocolListenerCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolListenerCallback_Vtbl
    where
        Identity: IWTSProtocolListenerCallback_Impl,
    {
        unsafe extern "system" fn OnConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, pcallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolListenerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolListenerCallback_Impl::OnConnected(this, windows_core::from_raw_borrowed(&pconnection)) {
                Ok(ok__) => {
                    pcallback.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnected: OnConnected::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolListenerCallback as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolLogonErrorRedirector_Impl: Sized {
    fn OnBeginPainting(&self) -> windows_core::Result<()>;
    fn RedirectStatus(&self, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(&self, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
impl windows_core::RuntimeName for IWTSProtocolLogonErrorRedirector {}
impl IWTSProtocolLogonErrorRedirector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolLogonErrorRedirector_Vtbl
    where
        Identity: IWTSProtocolLogonErrorRedirector_Impl,
    {
        unsafe extern "system" fn OnBeginPainting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolLogonErrorRedirector_Impl::OnBeginPainting(this).into()
        }
        unsafe extern "system" fn RedirectStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmessage: windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolLogonErrorRedirector_Impl::RedirectStatus(this, core::mem::transmute(&pszmessage)) {
                Ok(ok__) => {
                    presponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolLogonErrorRedirector_Impl::RedirectMessage(this, core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                Ok(ok__) => {
                    presponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectLogonError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolLogonErrorRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolLogonErrorRedirector_Impl::RedirectLogonError(this, core::mem::transmute_copy(&ntsstatus), core::mem::transmute_copy(&ntssubstatus), core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                Ok(ok__) => {
                    presponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBeginPainting: OnBeginPainting::<Identity, OFFSET>,
            RedirectStatus: RedirectStatus::<Identity, OFFSET>,
            RedirectMessage: RedirectMessage::<Identity, OFFSET>,
            RedirectLogonError: RedirectLogonError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolLogonErrorRedirector as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolManager_Impl: Sized {
    fn CreateListener(&self, wszlistenername: &windows_core::PCWSTR) -> windows_core::Result<IWTSProtocolListener>;
    fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSProtocolManager {}
impl IWTSProtocolManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolManager_Vtbl
    where
        Identity: IWTSProtocolManager_Impl,
    {
        unsafe extern "system" fn CreateListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlistenername: windows_core::PCWSTR, pprotocollistener: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSProtocolManager_Impl::CreateListener(this, core::mem::transmute(&wszlistenername)) {
                Ok(ok__) => {
                    pprotocollistener.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolManager_Impl::NotifyServiceStateChange(this, core::mem::transmute_copy(&ptsservicestatechange)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolManager_Impl::NotifySessionOfServiceStart(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolManager_Impl::NotifySessionOfServiceStop(this, core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolManager_Impl::NotifySessionStateChange(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&eventid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateListener: CreateListener::<Identity, OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Identity, OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Identity, OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Identity, OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolManager as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolShadowCallback_Impl: Sized {
    fn StopShadow(&self) -> windows_core::Result<()>;
    fn InvokeTargetShadow(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSProtocolShadowCallback {}
impl IWTSProtocolShadowCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolShadowCallback_Vtbl
    where
        Identity: IWTSProtocolShadowCallback_Impl,
    {
        unsafe extern "system" fn StopShadow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolShadowCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolShadowCallback_Impl::StopShadow(this).into()
        }
        unsafe extern "system" fn InvokeTargetShadow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolShadowCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolShadowCallback_Impl::InvokeTargetShadow(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StopShadow: StopShadow::<Identity, OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolShadowCallback as windows_core::Interface>::IID
    }
}
pub trait IWTSProtocolShadowConnection_Impl: Sized {
    fn Start(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: Option<&IWTSProtocolShadowCallback>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn DoTarget(&self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSProtocolShadowConnection {}
impl IWTSProtocolShadowConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSProtocolShadowConnection_Vtbl
    where
        Identity: IWTSProtocolShadowConnection_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolShadowConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolShadowConnection_Impl::Start(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&hotkeyvk), core::mem::transmute_copy(&hotkeymodifiers), windows_core::from_raw_borrowed(&pshadowcallback)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolShadowConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolShadowConnection_Impl::Stop(this).into()
        }
        unsafe extern "system" fn DoTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWTSProtocolShadowConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSProtocolShadowConnection_Impl::DoTarget(this, core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            DoTarget: DoTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolShadowConnection as windows_core::Interface>::IID
    }
}
pub trait IWTSSBPlugin_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<u32>;
    fn WTSSBX_MachineChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> windows_core::Result<()>;
    fn WTSSBX_SessionChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> windows_core::Result<()>;
    fn WTSSBX_GetMostSuitableServer(&self, username: &windows_core::PCWSTR, domainname: &windows_core::PCWSTR, applicationtype: &windows_core::PCWSTR, farmname: &windows_core::PCWSTR, pmachineid: *mut i32) -> windows_core::Result<()>;
    fn Terminated(&self) -> windows_core::Result<()>;
    fn WTSSBX_GetUserExternalSession(&self, username: &windows_core::PCWSTR, domainname: &windows_core::PCWSTR, applicationtype: &windows_core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSSBPlugin {}
impl IWTSSBPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSSBPlugin_Vtbl
    where
        Identity: IWTSSBPlugin_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugincapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWTSSBPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSSBPlugin_Impl::Initialize(this) {
                Ok(ok__) => {
                    plugincapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WTSSBX_MachineChangeNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> windows_core::HRESULT
        where
            Identity: IWTSSBPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSSBPlugin_Impl::WTSSBX_MachineChangeNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute_copy(&machineid), core::mem::transmute_copy(&pmachineinfo)).into()
        }
        unsafe extern "system" fn WTSSBX_SessionChangeNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> windows_core::HRESULT
        where
            Identity: IWTSSBPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSSBPlugin_Impl::WTSSBX_SessionChangeNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute_copy(&machineid), core::mem::transmute_copy(&numofsessions), core::mem::transmute_copy(&sessioninfo)).into()
        }
        unsafe extern "system" fn WTSSBX_GetMostSuitableServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: windows_core::PCWSTR, domainname: windows_core::PCWSTR, applicationtype: windows_core::PCWSTR, farmname: windows_core::PCWSTR, pmachineid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWTSSBPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSSBPlugin_Impl::WTSSBX_GetMostSuitableServer(this, core::mem::transmute(&username), core::mem::transmute(&domainname), core::mem::transmute(&applicationtype), core::mem::transmute(&farmname), core::mem::transmute_copy(&pmachineid)).into()
        }
        unsafe extern "system" fn Terminated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSSBPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSSBPlugin_Impl::Terminated(this).into()
        }
        unsafe extern "system" fn WTSSBX_GetUserExternalSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: windows_core::PCWSTR, domainname: windows_core::PCWSTR, applicationtype: windows_core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::HRESULT
        where
            Identity: IWTSSBPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSSBPlugin_Impl::WTSSBX_GetUserExternalSession(this, core::mem::transmute(&username), core::mem::transmute(&domainname), core::mem::transmute(&applicationtype), core::mem::transmute_copy(&redirectorinternalip), core::mem::transmute_copy(&psessionid), core::mem::transmute_copy(&pmachineconnectinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            WTSSBX_MachineChangeNotification: WTSSBX_MachineChangeNotification::<Identity, OFFSET>,
            WTSSBX_SessionChangeNotification: WTSSBX_SessionChangeNotification::<Identity, OFFSET>,
            WTSSBX_GetMostSuitableServer: WTSSBX_GetMostSuitableServer::<Identity, OFFSET>,
            Terminated: Terminated::<Identity, OFFSET>,
            WTSSBX_GetUserExternalSession: WTSSBX_GetUserExternalSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSSBPlugin as windows_core::Interface>::IID
    }
}
pub trait IWTSVirtualChannel_Impl: Sized {
    fn Write(&self, cbsize: u32, pbuffer: *const u8, preserved: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSVirtualChannel {}
impl IWTSVirtualChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSVirtualChannel_Vtbl
    where
        Identity: IWTSVirtualChannel_Impl,
    {
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSVirtualChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSVirtualChannel_Impl::Write(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer), windows_core::from_raw_borrowed(&preserved)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSVirtualChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSVirtualChannel_Impl::Close(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Write: Write::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannel as windows_core::Interface>::IID
    }
}
pub trait IWTSVirtualChannelCallback_Impl: Sized {
    fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
    fn OnClose(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWTSVirtualChannelCallback {}
impl IWTSVirtualChannelCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSVirtualChannelCallback_Vtbl
    where
        Identity: IWTSVirtualChannelCallback_Impl,
    {
        unsafe extern "system" fn OnDataReceived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> windows_core::HRESULT
        where
            Identity: IWTSVirtualChannelCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSVirtualChannelCallback_Impl::OnDataReceived(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn OnClose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSVirtualChannelCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWTSVirtualChannelCallback_Impl::OnClose(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataReceived: OnDataReceived::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannelCallback as windows_core::Interface>::IID
    }
}
pub trait IWTSVirtualChannelManager_Impl: Sized {
    fn CreateListener(&self, pszchannelname: &windows_core::PCSTR, uflags: u32, plistenercallback: Option<&IWTSListenerCallback>) -> windows_core::Result<IWTSListener>;
}
impl windows_core::RuntimeName for IWTSVirtualChannelManager {}
impl IWTSVirtualChannelManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWTSVirtualChannelManager_Vtbl
    where
        Identity: IWTSVirtualChannelManager_Impl,
    {
        unsafe extern "system" fn CreateListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszchannelname: windows_core::PCSTR, uflags: u32, plistenercallback: *mut core::ffi::c_void, pplistener: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWTSVirtualChannelManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWTSVirtualChannelManager_Impl::CreateListener(this, core::mem::transmute(&pszchannelname), core::mem::transmute_copy(&uflags), windows_core::from_raw_borrowed(&plistenercallback)) {
                Ok(ok__) => {
                    pplistener.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateListener: CreateListener::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannelManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspace_Impl: Sized {
    fn GetWorkspaceNames(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn StartRemoteApplication(&self, bstrworkspaceid: &windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn GetProcessId(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspace {}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspace_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspace_Vtbl
    where
        Identity: IWorkspace_Impl,
    {
        unsafe extern "system" fn GetWorkspaceNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IWorkspace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspace_Impl::GetWorkspaceNames(this) {
                Ok(ok__) => {
                    psawkspnames.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRemoteApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IWorkspace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspace_Impl::StartRemoteApplication(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute_copy(&psaparams)).into()
        }
        unsafe extern "system" fn GetProcessId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulprocessid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWorkspace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspace_Impl::GetProcessId(this) {
                Ok(ok__) => {
                    pulprocessid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWorkspaceNames: GetWorkspaceNames::<Identity, OFFSET>,
            StartRemoteApplication: StartRemoteApplication::<Identity, OFFSET>,
            GetProcessId: GetProcessId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspace2_Impl: Sized + IWorkspace_Impl {
    fn StartRemoteApplicationEx(&self, bstrworkspaceid: &windows_core::BSTR, bstrrequestingappid: &windows_core::BSTR, bstrrequestingappfamilyname: &windows_core::BSTR, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: &windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspace2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspace2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspace2_Vtbl
    where
        Identity: IWorkspace2_Impl,
    {
        unsafe extern "system" fn StartRemoteApplicationEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrrequestingappid: core::mem::MaybeUninit<windows_core::BSTR>, bstrrequestingappfamilyname: core::mem::MaybeUninit<windows_core::BSTR>, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: core::mem::MaybeUninit<windows_core::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IWorkspace2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspace2_Impl::StartRemoteApplicationEx(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrrequestingappid), core::mem::transmute(&bstrrequestingappfamilyname), core::mem::transmute_copy(&blaunchintoimmersiveclient), core::mem::transmute(&bstrimmersiveclientactivationcontext), core::mem::transmute_copy(&psaparams)).into()
        }
        Self { base__: IWorkspace_Vtbl::new::<Identity, OFFSET>(), StartRemoteApplicationEx: StartRemoteApplicationEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspace2 as windows_core::Interface>::IID || iid == &<IWorkspace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspace3_Impl: Sized + IWorkspace2_Impl {
    fn GetClaimsToken2(&self, bstrclaimshint: &windows_core::BSTR, bstruserhint: &windows_core::BSTR, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: &super::super::Foundation::RECT) -> windows_core::Result<windows_core::BSTR>;
    fn SetClaimsToken(&self, bstraccesstoken: &windows_core::BSTR, ullaccesstokenexpiration: u64, bstrrefreshtoken: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspace3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspace3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspace3_Vtbl
    where
        Identity: IWorkspace3_Impl,
    {
        unsafe extern "system" fn GetClaimsToken2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclaimshint: core::mem::MaybeUninit<windows_core::BSTR>, bstruserhint: core::mem::MaybeUninit<windows_core::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspace3_Impl::GetClaimsToken2(this, core::mem::transmute(&bstrclaimshint), core::mem::transmute(&bstruserhint), core::mem::transmute_copy(&claimcookie), core::mem::transmute_copy(&hwndcreduiparent), core::mem::transmute(&rectcreduiparent)) {
                Ok(ok__) => {
                    pbstraccesstoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClaimsToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccesstoken: core::mem::MaybeUninit<windows_core::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspace3_Impl::SetClaimsToken(this, core::mem::transmute(&bstraccesstoken), core::mem::transmute_copy(&ullaccesstokenexpiration), core::mem::transmute(&bstrrefreshtoken)).into()
        }
        Self {
            base__: IWorkspace2_Vtbl::new::<Identity, OFFSET>(),
            GetClaimsToken2: GetClaimsToken2::<Identity, OFFSET>,
            SetClaimsToken: SetClaimsToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspace3 as windows_core::Interface>::IID || iid == &<IWorkspace as windows_core::Interface>::IID || iid == &<IWorkspace2 as windows_core::Interface>::IID
    }
}
pub trait IWorkspaceClientExt_Impl: Sized {
    fn GetResourceId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetResourceDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IssueDisconnect(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWorkspaceClientExt {}
impl IWorkspaceClientExt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceClientExt_Vtbl
    where
        Identity: IWorkspaceClientExt_Impl,
    {
        unsafe extern "system" fn GetResourceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceClientExt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceClientExt_Impl::GetResourceId(this) {
                Ok(ok__) => {
                    bstrworkspaceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspacedisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceClientExt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceClientExt_Impl::GetResourceDisplayName(this) {
                Ok(ok__) => {
                    bstrworkspacedisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDisconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWorkspaceClientExt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceClientExt_Impl::IssueDisconnect(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResourceId: GetResourceId::<Identity, OFFSET>,
            GetResourceDisplayName: GetResourceDisplayName::<Identity, OFFSET>,
            IssueDisconnect: IssueDisconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceClientExt as windows_core::Interface>::IID
    }
}
pub trait IWorkspaceRegistration_Impl: Sized {
    fn AddResource(&self, punk: Option<&IWorkspaceClientExt>) -> windows_core::Result<u32>;
    fn RemoveResource(&self, dwcookieconnection: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWorkspaceRegistration {}
impl IWorkspaceRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceRegistration_Vtbl
    where
        Identity: IWorkspaceRegistration_Impl,
    {
        unsafe extern "system" fn AddResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWorkspaceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceRegistration_Impl::AddResource(this, windows_core::from_raw_borrowed(&punk)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookieconnection: u32) -> windows_core::HRESULT
        where
            Identity: IWorkspaceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceRegistration_Impl::RemoveResource(this, core::mem::transmute_copy(&dwcookieconnection)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddResource: AddResource::<Identity, OFFSET>,
            RemoveResource: RemoveResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceRegistration as windows_core::Interface>::IID
    }
}
pub trait IWorkspaceRegistration2_Impl: Sized + IWorkspaceRegistration_Impl {
    fn AddResourceEx(&self, punk: Option<&IWorkspaceClientExt>, bstreventloguploadaddress: &windows_core::BSTR, pdwcookie: *mut u32, correlationid: &windows_core::GUID) -> windows_core::Result<()>;
    fn RemoveResourceEx(&self, dwcookieconnection: u32, correlationid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWorkspaceRegistration2 {}
impl IWorkspaceRegistration2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceRegistration2_Vtbl
    where
        Identity: IWorkspaceRegistration2_Impl,
    {
        unsafe extern "system" fn AddResourceEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, bstreventloguploadaddress: core::mem::MaybeUninit<windows_core::BSTR>, pdwcookie: *mut u32, correlationid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWorkspaceRegistration2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceRegistration2_Impl::AddResourceEx(this, windows_core::from_raw_borrowed(&punk), core::mem::transmute(&bstreventloguploadaddress), core::mem::transmute_copy(&pdwcookie), core::mem::transmute(&correlationid)).into()
        }
        unsafe extern "system" fn RemoveResourceEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookieconnection: u32, correlationid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWorkspaceRegistration2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceRegistration2_Impl::RemoveResourceEx(this, core::mem::transmute_copy(&dwcookieconnection), core::mem::transmute(&correlationid)).into()
        }
        Self {
            base__: IWorkspaceRegistration_Vtbl::new::<Identity, OFFSET>(),
            AddResourceEx: AddResourceEx::<Identity, OFFSET>,
            RemoveResourceEx: RemoveResourceEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceRegistration2 as windows_core::Interface>::IID || iid == &<IWorkspaceRegistration as windows_core::Interface>::IID
    }
}
pub trait IWorkspaceReportMessage_Impl: Sized {
    fn RegisterErrorLogMessage(&self, bstrmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsErrorMessageRegistered(&self, bstrwkspid: &windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &windows_core::BSTR, dwerrorcode: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RegisterErrorEvent(&self, bstrwkspid: &windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &windows_core::BSTR, dwerrorcode: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWorkspaceReportMessage {}
impl IWorkspaceReportMessage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceReportMessage_Vtbl
    where
        Identity: IWorkspaceReportMessage_Impl,
    {
        unsafe extern "system" fn RegisterErrorLogMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceReportMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceReportMessage_Impl::RegisterErrorLogMessage(this, core::mem::transmute(&bstrmessage)).into()
        }
        unsafe extern "system" fn IsErrorMessageRegistered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrwkspid: core::mem::MaybeUninit<windows_core::BSTR>, dwerrortype: u32, bstrerrormessagetype: core::mem::MaybeUninit<windows_core::BSTR>, dwerrorcode: u32, pferrorexist: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWorkspaceReportMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceReportMessage_Impl::IsErrorMessageRegistered(this, core::mem::transmute(&bstrwkspid), core::mem::transmute_copy(&dwerrortype), core::mem::transmute(&bstrerrormessagetype), core::mem::transmute_copy(&dwerrorcode)) {
                Ok(ok__) => {
                    pferrorexist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterErrorEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrwkspid: core::mem::MaybeUninit<windows_core::BSTR>, dwerrortype: u32, bstrerrormessagetype: core::mem::MaybeUninit<windows_core::BSTR>, dwerrorcode: u32) -> windows_core::HRESULT
        where
            Identity: IWorkspaceReportMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceReportMessage_Impl::RegisterErrorEvent(this, core::mem::transmute(&bstrwkspid), core::mem::transmute_copy(&dwerrortype), core::mem::transmute(&bstrerrormessagetype), core::mem::transmute_copy(&dwerrorcode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterErrorLogMessage: RegisterErrorLogMessage::<Identity, OFFSET>,
            IsErrorMessageRegistered: IsErrorMessageRegistered::<Identity, OFFSET>,
            RegisterErrorEvent: RegisterErrorEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceReportMessage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceResTypeRegistry_Impl: Sized + super::Com::IDispatch_Impl {
    fn AddResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR, bstrlauncher: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRegisteredFileExtensions(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn GetResourceTypeInfo(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn ModifyResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR, bstrlauncher: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspaceResTypeRegistry {}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceResTypeRegistry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceResTypeRegistry_Vtbl
    where
        Identity: IWorkspaceResTypeRegistry_Impl,
    {
        unsafe extern "system" fn AddResourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: core::mem::MaybeUninit<windows_core::BSTR>, bstrlauncher: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceResTypeRegistry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceResTypeRegistry_Impl::AddResourceType(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension), core::mem::transmute(&bstrlauncher)).into()
        }
        unsafe extern "system" fn DeleteResourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceResTypeRegistry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceResTypeRegistry_Impl::DeleteResourceType(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension)).into()
        }
        unsafe extern "system" fn GetRegisteredFileExtensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IWorkspaceResTypeRegistry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceResTypeRegistry_Impl::GetRegisteredFileExtensions(this, core::mem::transmute_copy(&fmachinewide)) {
                Ok(ok__) => {
                    psafileextensions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: core::mem::MaybeUninit<windows_core::BSTR>, pbstrlauncher: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceResTypeRegistry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceResTypeRegistry_Impl::GetResourceTypeInfo(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension)) {
                Ok(ok__) => {
                    pbstrlauncher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyResourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: core::mem::MaybeUninit<windows_core::BSTR>, bstrlauncher: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceResTypeRegistry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceResTypeRegistry_Impl::ModifyResourceType(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension), core::mem::transmute(&bstrlauncher)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AddResourceType: AddResourceType::<Identity, OFFSET>,
            DeleteResourceType: DeleteResourceType::<Identity, OFFSET>,
            GetRegisteredFileExtensions: GetRegisteredFileExtensions::<Identity, OFFSET>,
            GetResourceTypeInfo: GetResourceTypeInfo::<Identity, OFFSET>,
            ModifyResourceType: ModifyResourceType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceResTypeRegistry as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptable_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisconnectWorkspace(&self, bstrworkspaceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartWorkspace(&self, bstrworkspaceid: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32) -> windows_core::Result<()>;
    fn IsWorkspaceCredentialSpecified(&self, bstrworkspaceid: &windows_core::BSTR, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorkspaceSSOEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ClearWorkspaceCredential(&self, bstrworkspaceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnAuthenticated(&self, bstrworkspaceid: &windows_core::BSTR, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisconnectWorkspaceByFriendlyName(&self, bstrworkspacefriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspaceScriptable {}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceScriptable_Vtbl
    where
        Identity: IWorkspaceScriptable_Impl,
    {
        unsafe extern "system" fn DisconnectWorkspace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable_Impl::DisconnectWorkspace(this, core::mem::transmute(&bstrworkspaceid)).into()
        }
        unsafe extern "system" fn StartWorkspace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrworkspaceparams: core::mem::MaybeUninit<windows_core::BSTR>, ltimeout: i32, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable_Impl::StartWorkspace(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrworkspaceparams), core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn IsWorkspaceCredentialSpecified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL, pbcredexist: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceScriptable_Impl::IsWorkspaceCredentialSpecified(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute_copy(&bcountunauthenticatedcredentials)) {
                Ok(ok__) => {
                    pbcredexist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorkspaceSSOEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbssoenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWorkspaceScriptable_Impl::IsWorkspaceSSOEnabled(this) {
                Ok(ok__) => {
                    pbssoenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearWorkspaceCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable_Impl::ClearWorkspaceCredential(this, core::mem::transmute(&bstrworkspaceid)).into()
        }
        unsafe extern "system" fn OnAuthenticated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable_Impl::OnAuthenticated(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn DisconnectWorkspaceByFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspacefriendlyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable_Impl::DisconnectWorkspaceByFriendlyName(this, core::mem::transmute(&bstrworkspacefriendlyname)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisconnectWorkspace: DisconnectWorkspace::<Identity, OFFSET>,
            StartWorkspace: StartWorkspace::<Identity, OFFSET>,
            IsWorkspaceCredentialSpecified: IsWorkspaceCredentialSpecified::<Identity, OFFSET>,
            IsWorkspaceSSOEnabled: IsWorkspaceSSOEnabled::<Identity, OFFSET>,
            ClearWorkspaceCredential: ClearWorkspaceCredential::<Identity, OFFSET>,
            OnAuthenticated: OnAuthenticated::<Identity, OFFSET>,
            DisconnectWorkspaceByFriendlyName: DisconnectWorkspaceByFriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceScriptable as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptable2_Impl: Sized + IWorkspaceScriptable_Impl {
    fn StartWorkspaceEx(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR, bstrredirectorname: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrappcontainer: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32) -> windows_core::Result<()>;
    fn ResourceDismissed(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspaceScriptable2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceScriptable2_Vtbl
    where
        Identity: IWorkspaceScriptable2_Impl,
    {
        unsafe extern "system" fn StartWorkspaceEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrworkspacefriendlyname: core::mem::MaybeUninit<windows_core::BSTR>, bstrredirectorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrappcontainer: core::mem::MaybeUninit<windows_core::BSTR>, bstrworkspaceparams: core::mem::MaybeUninit<windows_core::BSTR>, ltimeout: i32, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable2_Impl::StartWorkspaceEx(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrworkspacefriendlyname), core::mem::transmute(&bstrredirectorname), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrappcontainer), core::mem::transmute(&bstrworkspaceparams), core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn ResourceDismissed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrworkspacefriendlyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable2_Impl::ResourceDismissed(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrworkspacefriendlyname)).into()
        }
        Self {
            base__: IWorkspaceScriptable_Vtbl::new::<Identity, OFFSET>(),
            StartWorkspaceEx: StartWorkspaceEx::<Identity, OFFSET>,
            ResourceDismissed: ResourceDismissed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceScriptable2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWorkspaceScriptable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptable3_Impl: Sized + IWorkspaceScriptable2_Impl {
    fn StartWorkspaceEx2(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR, bstrredirectorname: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrappcontainer: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32, bstreventloguploadaddress: &windows_core::BSTR, correlationid: &windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspaceScriptable3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWorkspaceScriptable3_Vtbl
    where
        Identity: IWorkspaceScriptable3_Impl,
    {
        unsafe extern "system" fn StartWorkspaceEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrworkspacefriendlyname: core::mem::MaybeUninit<windows_core::BSTR>, bstrredirectorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrappcontainer: core::mem::MaybeUninit<windows_core::BSTR>, bstrworkspaceparams: core::mem::MaybeUninit<windows_core::BSTR>, ltimeout: i32, lflags: i32, bstreventloguploadaddress: core::mem::MaybeUninit<windows_core::BSTR>, correlationid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWorkspaceScriptable3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWorkspaceScriptable3_Impl::StartWorkspaceEx2(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrworkspacefriendlyname), core::mem::transmute(&bstrredirectorname), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrappcontainer), core::mem::transmute(&bstrworkspaceparams), core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&lflags), core::mem::transmute(&bstreventloguploadaddress), core::mem::transmute(&correlationid)).into()
        }
        Self { base__: IWorkspaceScriptable2_Vtbl::new::<Identity, OFFSET>(), StartWorkspaceEx2: StartWorkspaceEx2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceScriptable3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWorkspaceScriptable as windows_core::Interface>::IID || iid == &<IWorkspaceScriptable2 as windows_core::Interface>::IID
    }
}
pub trait ItsPubPlugin_Impl: Sized {
    fn GetResourceList(&self, userid: &windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> windows_core::Result<()>;
    fn GetResource(&self, alias: &windows_core::PCWSTR, flags: i32, resource: *mut pluginResource) -> windows_core::Result<()>;
    fn GetCacheLastUpdateTime(&self) -> windows_core::Result<u64>;
    fn pluginName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn pluginVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ResolveResource(&self, resourcetype: *mut u32, resourcelocation: windows_core::PWSTR, endpointname: windows_core::PWSTR, userid: &windows_core::PCWSTR, alias: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ItsPubPlugin {}
impl ItsPubPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ItsPubPlugin_Vtbl
    where
        Identity: ItsPubPlugin_Impl,
    {
        unsafe extern "system" fn GetResourceList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ItsPubPlugin_Impl::GetResourceList(this, core::mem::transmute(&userid), core::mem::transmute_copy(&pceapplistsize), core::mem::transmute_copy(&resourcelist)).into()
        }
        unsafe extern "system" fn GetResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alias: windows_core::PCWSTR, flags: i32, resource: *mut pluginResource) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ItsPubPlugin_Impl::GetResource(this, core::mem::transmute(&alias), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&resource)).into()
        }
        unsafe extern "system" fn GetCacheLastUpdateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastupdatetime: *mut u64) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ItsPubPlugin_Impl::GetCacheLastUpdateTime(this) {
                Ok(ok__) => {
                    lastupdatetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pluginName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ItsPubPlugin_Impl::pluginName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pluginVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ItsPubPlugin_Impl::pluginVersion(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcetype: *mut u32, resourcelocation: windows_core::PWSTR, endpointname: windows_core::PWSTR, userid: windows_core::PCWSTR, alias: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ItsPubPlugin_Impl::ResolveResource(this, core::mem::transmute_copy(&resourcetype), core::mem::transmute_copy(&resourcelocation), core::mem::transmute_copy(&endpointname), core::mem::transmute(&userid), core::mem::transmute(&alias)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResourceList: GetResourceList::<Identity, OFFSET>,
            GetResource: GetResource::<Identity, OFFSET>,
            GetCacheLastUpdateTime: GetCacheLastUpdateTime::<Identity, OFFSET>,
            pluginName: pluginName::<Identity, OFFSET>,
            pluginVersion: pluginVersion::<Identity, OFFSET>,
            ResolveResource: ResolveResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ItsPubPlugin as windows_core::Interface>::IID
    }
}
pub trait ItsPubPlugin2_Impl: Sized + ItsPubPlugin_Impl {
    fn GetResource2List(&self, userid: &windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> windows_core::Result<()>;
    fn GetResource2(&self, alias: &windows_core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> windows_core::Result<()>;
    fn ResolvePersonalDesktop(&self, userid: &windows_core::PCWSTR, poolid: &windows_core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn DeletePersonalDesktopAssignment(&self, userid: &windows_core::PCWSTR, poolid: &windows_core::PCWSTR, endpointname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ItsPubPlugin2 {}
impl ItsPubPlugin2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ItsPubPlugin2_Vtbl
    where
        Identity: ItsPubPlugin2_Impl,
    {
        unsafe extern "system" fn GetResource2List<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ItsPubPlugin2_Impl::GetResource2List(this, core::mem::transmute(&userid), core::mem::transmute_copy(&pceapplistsize), core::mem::transmute_copy(&resourcelist)).into()
        }
        unsafe extern "system" fn GetResource2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alias: windows_core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ItsPubPlugin2_Impl::GetResource2(this, core::mem::transmute(&alias), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&resource)).into()
        }
        unsafe extern "system" fn ResolvePersonalDesktop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, poolid: windows_core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ItsPubPlugin2_Impl::ResolvePersonalDesktop(this, core::mem::transmute(&userid), core::mem::transmute(&poolid), core::mem::transmute_copy(&epdresolutiontype), core::mem::transmute_copy(&ppdassignmenttype), core::mem::transmute_copy(&endpointname)).into()
        }
        unsafe extern "system" fn DeletePersonalDesktopAssignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, poolid: windows_core::PCWSTR, endpointname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ItsPubPlugin2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ItsPubPlugin2_Impl::DeletePersonalDesktopAssignment(this, core::mem::transmute(&userid), core::mem::transmute(&poolid), core::mem::transmute(&endpointname)).into()
        }
        Self {
            base__: ItsPubPlugin_Vtbl::new::<Identity, OFFSET>(),
            GetResource2List: GetResource2List::<Identity, OFFSET>,
            GetResource2: GetResource2::<Identity, OFFSET>,
            ResolvePersonalDesktop: ResolvePersonalDesktop::<Identity, OFFSET>,
            DeletePersonalDesktopAssignment: DeletePersonalDesktopAssignment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ItsPubPlugin2 as windows_core::Interface>::IID || iid == &<ItsPubPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ITSWkspEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _ITSWkspEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _ITSWkspEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _ITSWkspEvents_Vtbl
    where
        Identity: _ITSWkspEvents_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ITSWkspEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
