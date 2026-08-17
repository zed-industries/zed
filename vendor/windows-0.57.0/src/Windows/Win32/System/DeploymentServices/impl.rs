#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportCacheable_Impl: Sized + super::Com::IDispatch_Impl {
    fn Dirty(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Discard(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportCacheable {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCacheable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>() -> IWdsTransportCacheable_Vtbl {
        unsafe extern "system" fn Dirty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportCacheable_Impl::Dirty(this) {
                Ok(ok__) => {
                    core::ptr::write(pbdirty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discard<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportCacheable_Impl::Discard(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportCacheable_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportCacheable_Impl::Commit(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Dirty: Dirty::<Identity, Impl, OFFSET>,
            Discard: Discard::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportCacheable as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IWdsTransportSession>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MacAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IpAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PercentCompletion(&self) -> windows_core::Result<u32>;
    fn JoinDuration(&self) -> windows_core::Result<u32>;
    fn CpuUtilization(&self) -> windows_core::Result<u32>;
    fn MemoryUtilization(&self) -> windows_core::Result<u32>;
    fn NetworkUtilization(&self) -> windows_core::Result<u32>;
    fn UserIdentity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Disconnect(&self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportClient {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>() -> IWdsTransportClient_Vtbl {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::Session(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportsession, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pulid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszmacaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::MacAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszmacaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszipaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::IpAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszipaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentCompletion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpercentcompletion: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::PercentCompletion(this) {
                Ok(ok__) => {
                    core::ptr::write(pulpercentcompletion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljoinduration: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::JoinDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(puljoinduration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUtilization<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcpuutilization: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::CpuUtilization(this) {
                Ok(ok__) => {
                    core::ptr::write(pulcpuutilization, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemoryUtilization<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmemoryutilization: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::MemoryUtilization(this) {
                Ok(ok__) => {
                    core::ptr::write(pulmemoryutilization, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkUtilization<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnetworkutilization: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::NetworkUtilization(this) {
                Ok(ok__) => {
                    core::ptr::write(pulnetworkutilization, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserIdentity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszuseridentity: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportClient_Impl::UserIdentity(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszuseridentity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportClient_Impl::Disconnect(this, core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            MacAddress: MacAddress::<Identity, Impl, OFFSET>,
            IpAddress: IpAddress::<Identity, Impl, OFFSET>,
            PercentCompletion: PercentCompletion::<Identity, Impl, OFFSET>,
            JoinDuration: JoinDuration::<Identity, Impl, OFFSET>,
            CpuUtilization: CpuUtilization::<Identity, Impl, OFFSET>,
            MemoryUtilization: MemoryUtilization::<Identity, Impl, OFFSET>,
            NetworkUtilization: NetworkUtilization::<Identity, Impl, OFFSET>,
            UserIdentity: UserIdentity::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportClient as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn get_Item(&self, ulindex: u32) -> windows_core::Result<super::Com::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>() -> IWdsTransportCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pulcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportCollection_Impl::get_Item(this, core::mem::transmute_copy(&ulindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportConfigurationManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn ServicePolicy(&self) -> windows_core::Result<IWdsTransportServicePolicy>;
    fn DiagnosticsPolicy(&self) -> windows_core::Result<IWdsTransportDiagnosticsPolicy>;
    fn get_WdsTransportServicesRunning(&self, brealtimestatus: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EnableWdsTransportServices(&self) -> windows_core::Result<()>;
    fn DisableWdsTransportServices(&self) -> windows_core::Result<()>;
    fn StartWdsTransportServices(&self) -> windows_core::Result<()>;
    fn StopWdsTransportServices(&self) -> windows_core::Result<()>;
    fn RestartWdsTransportServices(&self) -> windows_core::Result<()>;
    fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportConfigurationManager {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>() -> IWdsTransportConfigurationManager_Vtbl {
        unsafe extern "system" fn ServicePolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportservicepolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportConfigurationManager_Impl::ServicePolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportservicepolicy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiagnosticsPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportConfigurationManager_Impl::DiagnosticsPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportdiagnosticspolicy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_WdsTransportServicesRunning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brealtimestatus: super::super::Foundation::VARIANT_BOOL, pbservicesrunning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportConfigurationManager_Impl::get_WdsTransportServicesRunning(this, core::mem::transmute_copy(&brealtimestatus)) {
                Ok(ok__) => {
                    core::ptr::write(pbservicesrunning, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableWdsTransportServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportConfigurationManager_Impl::EnableWdsTransportServices(this).into()
        }
        unsafe extern "system" fn DisableWdsTransportServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportConfigurationManager_Impl::DisableWdsTransportServices(this).into()
        }
        unsafe extern "system" fn StartWdsTransportServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportConfigurationManager_Impl::StartWdsTransportServices(this).into()
        }
        unsafe extern "system" fn StopWdsTransportServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportConfigurationManager_Impl::StopWdsTransportServices(this).into()
        }
        unsafe extern "system" fn RestartWdsTransportServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportConfigurationManager_Impl::RestartWdsTransportServices(this).into()
        }
        unsafe extern "system" fn NotifyWdsTransportServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportConfigurationManager_Impl::NotifyWdsTransportServices(this, core::mem::transmute_copy(&servicenotification)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServicePolicy: ServicePolicy::<Identity, Impl, OFFSET>,
            DiagnosticsPolicy: DiagnosticsPolicy::<Identity, Impl, OFFSET>,
            get_WdsTransportServicesRunning: get_WdsTransportServicesRunning::<Identity, Impl, OFFSET>,
            EnableWdsTransportServices: EnableWdsTransportServices::<Identity, Impl, OFFSET>,
            DisableWdsTransportServices: DisableWdsTransportServices::<Identity, Impl, OFFSET>,
            StartWdsTransportServices: StartWdsTransportServices::<Identity, Impl, OFFSET>,
            StopWdsTransportServices: StopWdsTransportServices::<Identity, Impl, OFFSET>,
            RestartWdsTransportServices: RestartWdsTransportServices::<Identity, Impl, OFFSET>,
            NotifyWdsTransportServices: NotifyWdsTransportServices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportConfigurationManager2_Impl: Sized + IWdsTransportConfigurationManager_Impl {
    fn MulticastSessionPolicy(&self) -> windows_core::Result<IWdsTransportMulticastSessionPolicy>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportConfigurationManager2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>() -> IWdsTransportConfigurationManager2_Vtbl {
        unsafe extern "system" fn MulticastSessionPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportConfigurationManager2_Impl::MulticastSessionPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportmulticastsessionpolicy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWdsTransportConfigurationManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            MulticastSessionPolicy: MulticastSessionPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportConfigurationManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportContent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Namespace(&self) -> windows_core::Result<IWdsTransportNamespace>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RetrieveSessions(&self) -> windows_core::Result<IWdsTransportCollection>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportContent {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>() -> IWdsTransportContent_Vtbl {
        unsafe extern "system" fn Namespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContent_Impl::Namespace(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContent_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pulid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContent_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveSessions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportsessions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContent_Impl::RetrieveSessions(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportsessions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportContent_Impl::Terminate(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            RetrieveSessions: RetrieveSessions::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportContent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportContentProvider_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializationRoutine(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportContentProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContentProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>() -> IWdsTransportContentProvider_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContentProvider_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContentProvider_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszfilepath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContentProvider_Impl::FilePath(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszfilepath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializationRoutine<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszinitializationroutine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportContentProvider_Impl::InitializationRoutine(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszinitializationroutine, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            FilePath: FilePath::<Identity, Impl, OFFSET>,
            InitializationRoutine: InitializationRoutine::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportContentProvider as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportDiagnosticsPolicy_Impl: Sized + IWdsTransportCacheable_Impl {
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Components(&self) -> windows_core::Result<u32>;
    fn SetComponents(&self, ulcomponents: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportDiagnosticsPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportDiagnosticsPolicy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>() -> IWdsTransportDiagnosticsPolicy_Vtbl {
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportDiagnosticsPolicy_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportDiagnosticsPolicy_Impl::SetEnabled(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn Components<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcomponents: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportDiagnosticsPolicy_Impl::Components(this) {
                Ok(ok__) => {
                    core::ptr::write(pulcomponents, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcomponents: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportDiagnosticsPolicy_Impl::SetComponents(this, core::mem::transmute_copy(&ulcomponents)).into()
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Components: Components::<Identity, Impl, OFFSET>,
            SetComponents: SetComponents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportDiagnosticsPolicy as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetWdsTransportServer(&self, bszservername: &windows_core::BSTR) -> windows_core::Result<IWdsTransportServer>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportManager {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportManager_Impl, const OFFSET: isize>() -> IWdsTransportManager_Vtbl {
        unsafe extern "system" fn GetWdsTransportServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszservername: core::mem::MaybeUninit<windows_core::BSTR>, ppwdstransportserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportManager_Impl::GetWdsTransportServer(this, core::mem::transmute(&bszservername)) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportserver, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetWdsTransportServer: GetWdsTransportServer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportMulticastSessionPolicy_Impl: Sized + IWdsTransportCacheable_Impl {
    fn SlowClientHandling(&self) -> windows_core::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE>;
    fn SetSlowClientHandling(&self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::Result<()>;
    fn AutoDisconnectThreshold(&self) -> windows_core::Result<u32>;
    fn SetAutoDisconnectThreshold(&self, ulthreshold: u32) -> windows_core::Result<()>;
    fn MultistreamStreamCount(&self) -> windows_core::Result<u32>;
    fn SetMultistreamStreamCount(&self, ulstreamcount: u32) -> windows_core::Result<()>;
    fn SlowClientFallback(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSlowClientFallback(&self, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportMulticastSessionPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportMulticastSessionPolicy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>() -> IWdsTransportMulticastSessionPolicy_Vtbl {
        unsafe extern "system" fn SlowClientHandling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportMulticastSessionPolicy_Impl::SlowClientHandling(this) {
                Ok(ok__) => {
                    core::ptr::write(pslowclienthandling, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientHandling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportMulticastSessionPolicy_Impl::SetSlowClientHandling(this, core::mem::transmute_copy(&slowclienthandling)).into()
        }
        unsafe extern "system" fn AutoDisconnectThreshold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulthreshold: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportMulticastSessionPolicy_Impl::AutoDisconnectThreshold(this) {
                Ok(ok__) => {
                    core::ptr::write(pulthreshold, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDisconnectThreshold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulthreshold: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportMulticastSessionPolicy_Impl::SetAutoDisconnectThreshold(this, core::mem::transmute_copy(&ulthreshold)).into()
        }
        unsafe extern "system" fn MultistreamStreamCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstreamcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportMulticastSessionPolicy_Impl::MultistreamStreamCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pulstreamcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultistreamStreamCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstreamcount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportMulticastSessionPolicy_Impl::SetMultistreamStreamCount(this, core::mem::transmute_copy(&ulstreamcount)).into()
        }
        unsafe extern "system" fn SlowClientFallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclientfallback: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportMulticastSessionPolicy_Impl::SlowClientFallback(this) {
                Ok(ok__) => {
                    core::ptr::write(pbclientfallback, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientFallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportMulticastSessionPolicy_Impl::SetSlowClientFallback(this, core::mem::transmute_copy(&bclientfallback)).into()
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SlowClientHandling: SlowClientHandling::<Identity, Impl, OFFSET>,
            SetSlowClientHandling: SetSlowClientHandling::<Identity, Impl, OFFSET>,
            AutoDisconnectThreshold: AutoDisconnectThreshold::<Identity, Impl, OFFSET>,
            SetAutoDisconnectThreshold: SetAutoDisconnectThreshold::<Identity, Impl, OFFSET>,
            MultistreamStreamCount: MultistreamStreamCount::<Identity, Impl, OFFSET>,
            SetMultistreamStreamCount: SetMultistreamStreamCount::<Identity, Impl, OFFSET>,
            SlowClientFallback: SlowClientFallback::<Identity, Impl, OFFSET>,
            SetSlowClientFallback: SetSlowClientFallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportMulticastSessionPolicy as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespace_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bszname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFriendlyName(&self, bszfriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bszdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ContentProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetContentProvider(&self, bszcontentprovider: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Configuration(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConfiguration(&self, bszconfiguration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Registered(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Tombstoned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TombstoneTime(&self) -> windows_core::Result<f64>;
    fn TransmissionStarted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Register(&self) -> windows_core::Result<()>;
    fn Deregister(&self, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWdsTransportNamespace>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn RetrieveContents(&self) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportNamespace {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespace_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>() -> IWdsTransportNamespace_Vtbl {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pulid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::SetName(this, core::mem::transmute(&bszname)).into()
        }
        unsafe extern "system" fn FriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszfriendlyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszfriendlyname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::SetFriendlyName(this, core::mem::transmute(&bszfriendlyname)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::SetDescription(this, core::mem::transmute(&bszdescription)).into()
        }
        unsafe extern "system" fn ContentProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszcontentprovider: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::ContentProvider(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszcontentprovider, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszcontentprovider: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::SetContentProvider(this, core::mem::transmute(&bszcontentprovider)).into()
        }
        unsafe extern "system" fn Configuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszconfiguration: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Configuration(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszconfiguration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszconfiguration: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::SetConfiguration(this, core::mem::transmute(&bszconfiguration)).into()
        }
        unsafe extern "system" fn Registered<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregistered: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Registered(this) {
                Ok(ok__) => {
                    core::ptr::write(pbregistered, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tombstoned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbtombstoned: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Tombstoned(this) {
                Ok(ok__) => {
                    core::ptr::write(pbtombstoned, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TombstoneTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptombstonetime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::TombstoneTime(this) {
                Ok(ok__) => {
                    core::ptr::write(ptombstonetime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStarted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbtransmissionstarted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::TransmissionStarted(this) {
                Ok(ok__) => {
                    core::ptr::write(pbtransmissionstarted, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::Register(this).into()
        }
        unsafe extern "system" fn Deregister<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::Deregister(this, core::mem::transmute_copy(&bterminatesessions)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportnamespaceclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportnamespaceclone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespace_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn RetrieveContents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportcontents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespace_Impl::RetrieveContents(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportcontents, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ContentProvider: ContentProvider::<Identity, Impl, OFFSET>,
            SetContentProvider: SetContentProvider::<Identity, Impl, OFFSET>,
            Configuration: Configuration::<Identity, Impl, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            Registered: Registered::<Identity, Impl, OFFSET>,
            Tombstoned: Tombstoned::<Identity, Impl, OFFSET>,
            TombstoneTime: TombstoneTime::<Identity, Impl, OFFSET>,
            TransmissionStarted: TransmissionStarted::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Deregister: Deregister::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            RetrieveContents: RetrieveContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespace as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceAutoCast_Impl: Sized + IWdsTransportNamespace_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportNamespaceAutoCast {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceAutoCast_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceAutoCast_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceAutoCast_Vtbl {
        Self { base__: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceAutoCast as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateNamespace(&self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: &windows_core::BSTR, bszcontentprovider: &windows_core::BSTR, bszconfiguration: &windows_core::BSTR) -> windows_core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespace(&self, bsznamespacename: &windows_core::BSTR) -> windows_core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespaces(&self, bszcontentprovider: &windows_core::BSTR, bsznamespacename: &windows_core::BSTR, bincludetombstones: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportNamespaceManager {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceManager_Vtbl {
        unsafe extern "system" fn CreateNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: core::mem::MaybeUninit<windows_core::BSTR>, bszcontentprovider: core::mem::MaybeUninit<windows_core::BSTR>, bszconfiguration: core::mem::MaybeUninit<windows_core::BSTR>, ppwdstransportnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespaceManager_Impl::CreateNamespace(this, core::mem::transmute_copy(&namespacetype), core::mem::transmute(&bsznamespacename), core::mem::transmute(&bszcontentprovider), core::mem::transmute(&bszconfiguration)) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsznamespacename: core::mem::MaybeUninit<windows_core::BSTR>, ppwdstransportnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespaceManager_Impl::RetrieveNamespace(this, core::mem::transmute(&bsznamespacename)) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszcontentprovider: core::mem::MaybeUninit<windows_core::BSTR>, bsznamespacename: core::mem::MaybeUninit<windows_core::BSTR>, bincludetombstones: super::super::Foundation::VARIANT_BOOL, ppwdstransportnamespaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespaceManager_Impl::RetrieveNamespaces(this, core::mem::transmute(&bszcontentprovider), core::mem::transmute(&bsznamespacename), core::mem::transmute_copy(&bincludetombstones)) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportnamespaces, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateNamespace: CreateNamespace::<Identity, Impl, OFFSET>,
            RetrieveNamespace: RetrieveNamespace::<Identity, Impl, OFFSET>,
            RetrieveNamespaces: RetrieveNamespaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCast_Impl: Sized + IWdsTransportNamespace_Impl {
    fn StartTransmission(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportNamespaceScheduledCast {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCast_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCast_Vtbl {
        unsafe extern "system" fn StartTransmission<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespaceScheduledCast_Impl::StartTransmission(this).into()
        }
        Self { base__: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, OFFSET>(), StartTransmission: StartTransmission::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCast as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastAutoStart_Impl: Sized + IWdsTransportNamespaceScheduledCast_Impl {
    fn MinimumClients(&self) -> windows_core::Result<u32>;
    fn SetMinimumClients(&self, ulminimumclients: u32) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<f64>;
    fn SetStartTime(&self, starttime: f64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportNamespaceScheduledCastAutoStart {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
        unsafe extern "system" fn MinimumClients<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulminimumclients: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespaceScheduledCastAutoStart_Impl::MinimumClients(this) {
                Ok(ok__) => {
                    core::ptr::write(pulminimumclients, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumClients<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulminimumclients: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespaceScheduledCastAutoStart_Impl::SetMinimumClients(this, core::mem::transmute_copy(&ulminimumclients)).into()
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstarttime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportNamespaceScheduledCastAutoStart_Impl::StartTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pstarttime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportNamespaceScheduledCastAutoStart_Impl::SetStartTime(this, core::mem::transmute_copy(&starttime)).into()
        }
        Self {
            base__: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, OFFSET>(),
            MinimumClients: MinimumClients::<Identity, Impl, OFFSET>,
            SetMinimumClients: SetMinimumClients::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastAutoStart as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID || iid == &<IWdsTransportNamespaceScheduledCast as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastManualStart_Impl: Sized + IWdsTransportNamespaceScheduledCast_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportNamespaceScheduledCastManualStart {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastManualStart_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
        Self { base__: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastManualStart as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID || iid == &<IWdsTransportNamespaceScheduledCast as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetupManager(&self) -> windows_core::Result<IWdsTransportSetupManager>;
    fn ConfigurationManager(&self) -> windows_core::Result<IWdsTransportConfigurationManager>;
    fn NamespaceManager(&self) -> windows_core::Result<IWdsTransportNamespaceManager>;
    fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportServer {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>() -> IWdsTransportServer_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServer_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportsetupmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServer_Impl::SetupManager(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportsetupmanager, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigurationManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportconfigurationmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServer_Impl::ConfigurationManager(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportconfigurationmanager, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportnamespacemanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServer_Impl::NamespaceManager(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportnamespacemanager, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectClient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServer_Impl::DisconnectClient(this, core::mem::transmute_copy(&ulclientid), core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetupManager: SetupManager::<Identity, Impl, OFFSET>,
            ConfigurationManager: ConfigurationManager::<Identity, Impl, OFFSET>,
            NamespaceManager: NamespaceManager::<Identity, Impl, OFFSET>,
            DisconnectClient: DisconnectClient::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServer2_Impl: Sized + IWdsTransportServer_Impl {
    fn TftpManager(&self) -> windows_core::Result<IWdsTransportTftpManager>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportServer2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer2_Impl, const OFFSET: isize>() -> IWdsTransportServer2_Vtbl {
        unsafe extern "system" fn TftpManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransporttftpmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServer2_Impl::TftpManager(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransporttftpmanager, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWdsTransportServer_Vtbl::new::<Identity, Impl, OFFSET>(), TftpManager: TftpManager::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServer2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportServer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServicePolicy_Impl: Sized + IWdsTransportCacheable_Impl {
    fn get_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>;
    fn put_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::Result<()>;
    fn get_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<windows_core::BSTR>;
    fn put_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<windows_core::BSTR>;
    fn put_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartPort(&self) -> windows_core::Result<u32>;
    fn SetStartPort(&self, ulstartport: u32) -> windows_core::Result<()>;
    fn EndPort(&self) -> windows_core::Result<u32>;
    fn SetEndPort(&self, ulendport: u32) -> windows_core::Result<()>;
    fn NetworkProfile(&self) -> windows_core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE>;
    fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportServicePolicy {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>() -> IWdsTransportServicePolicy_Vtbl {
        unsafe extern "system" fn get_IpAddressSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy_Impl::get_IpAddressSource(this, core::mem::transmute_copy(&addresstype)) {
                Ok(ok__) => {
                    core::ptr::write(psourcetype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_IpAddressSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy_Impl::put_IpAddressSource(this, core::mem::transmute_copy(&addresstype), core::mem::transmute_copy(&sourcetype)).into()
        }
        unsafe extern "system" fn get_StartIpAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy_Impl::get_StartIpAddress(this, core::mem::transmute_copy(&addresstype)) {
                Ok(ok__) => {
                    core::ptr::write(pbszstartipaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_StartIpAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy_Impl::put_StartIpAddress(this, core::mem::transmute_copy(&addresstype), core::mem::transmute(&bszstartipaddress)).into()
        }
        unsafe extern "system" fn get_EndIpAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy_Impl::get_EndIpAddress(this, core::mem::transmute_copy(&addresstype)) {
                Ok(ok__) => {
                    core::ptr::write(pbszendipaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_EndIpAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy_Impl::put_EndIpAddress(this, core::mem::transmute_copy(&addresstype), core::mem::transmute(&bszendipaddress)).into()
        }
        unsafe extern "system" fn StartPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstartport: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy_Impl::StartPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pulstartport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartport: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy_Impl::SetStartPort(this, core::mem::transmute_copy(&ulstartport)).into()
        }
        unsafe extern "system" fn EndPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulendport: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy_Impl::EndPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pulendport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulendport: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy_Impl::SetEndPort(this, core::mem::transmute_copy(&ulendport)).into()
        }
        unsafe extern "system" fn NetworkProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy_Impl::NetworkProfile(this) {
                Ok(ok__) => {
                    core::ptr::write(pprofiletype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy_Impl::SetNetworkProfile(this, core::mem::transmute_copy(&profiletype)).into()
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_IpAddressSource: get_IpAddressSource::<Identity, Impl, OFFSET>,
            put_IpAddressSource: put_IpAddressSource::<Identity, Impl, OFFSET>,
            get_StartIpAddress: get_StartIpAddress::<Identity, Impl, OFFSET>,
            put_StartIpAddress: put_StartIpAddress::<Identity, Impl, OFFSET>,
            get_EndIpAddress: get_EndIpAddress::<Identity, Impl, OFFSET>,
            put_EndIpAddress: put_EndIpAddress::<Identity, Impl, OFFSET>,
            StartPort: StartPort::<Identity, Impl, OFFSET>,
            SetStartPort: SetStartPort::<Identity, Impl, OFFSET>,
            EndPort: EndPort::<Identity, Impl, OFFSET>,
            SetEndPort: SetEndPort::<Identity, Impl, OFFSET>,
            NetworkProfile: NetworkProfile::<Identity, Impl, OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServicePolicy2_Impl: Sized + IWdsTransportServicePolicy_Impl {
    fn UdpPortPolicy(&self) -> windows_core::Result<WDSTRANSPORT_UDP_PORT_POLICY>;
    fn SetUdpPortPolicy(&self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::Result<()>;
    fn TftpMaximumBlockSize(&self) -> windows_core::Result<u32>;
    fn SetTftpMaximumBlockSize(&self, ultftpmaximumblocksize: u32) -> windows_core::Result<()>;
    fn EnableTftpVariableWindowExtension(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableTftpVariableWindowExtension(&self, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportServicePolicy2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicy2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>() -> IWdsTransportServicePolicy2_Vtbl {
        unsafe extern "system" fn UdpPortPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy2_Impl::UdpPortPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(pudpportpolicy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUdpPortPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy2_Impl::SetUdpPortPolicy(this, core::mem::transmute_copy(&udpportpolicy)).into()
        }
        unsafe extern "system" fn TftpMaximumBlockSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy2_Impl::TftpMaximumBlockSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pultftpmaximumblocksize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTftpMaximumBlockSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultftpmaximumblocksize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy2_Impl::SetTftpMaximumBlockSize(this, core::mem::transmute_copy(&ultftpmaximumblocksize)).into()
        }
        unsafe extern "system" fn EnableTftpVariableWindowExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabletftpvariablewindowextension: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportServicePolicy2_Impl::EnableTftpVariableWindowExtension(this) {
                Ok(ok__) => {
                    core::ptr::write(pbenabletftpvariablewindowextension, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTftpVariableWindowExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportServicePolicy2_Impl::SetEnableTftpVariableWindowExtension(this, core::mem::transmute_copy(&benabletftpvariablewindowextension)).into()
        }
        Self {
            base__: IWdsTransportServicePolicy_Vtbl::new::<Identity, Impl, OFFSET>(),
            UdpPortPolicy: UdpPortPolicy::<Identity, Impl, OFFSET>,
            SetUdpPortPolicy: SetUdpPortPolicy::<Identity, Impl, OFFSET>,
            TftpMaximumBlockSize: TftpMaximumBlockSize::<Identity, Impl, OFFSET>,
            SetTftpMaximumBlockSize: SetTftpMaximumBlockSize::<Identity, Impl, OFFSET>,
            EnableTftpVariableWindowExtension: EnableTftpVariableWindowExtension::<Identity, Impl, OFFSET>,
            SetEnableTftpVariableWindowExtension: SetEnableTftpVariableWindowExtension::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID || iid == &<IWdsTransportServicePolicy as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Content(&self) -> windows_core::Result<IWdsTransportContent>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn NetworkInterfaceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn NetworkInterfaceAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TransferRate(&self) -> windows_core::Result<u32>;
    fn MasterClientId(&self) -> windows_core::Result<u32>;
    fn RetrieveClients(&self) -> windows_core::Result<IWdsTransportCollection>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportSession {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>() -> IWdsTransportSession_Vtbl {
        unsafe extern "system" fn Content<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSession_Impl::Content(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportcontent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSession_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(pulid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsznetworkinterfacename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSession_Impl::NetworkInterfaceName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbsznetworkinterfacename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsznetworkinterfaceaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSession_Impl::NetworkInterfaceAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbsznetworkinterfaceaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferRate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultransferrate: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSession_Impl::TransferRate(this) {
                Ok(ok__) => {
                    core::ptr::write(pultransferrate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MasterClientId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmasterclientid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSession_Impl::MasterClientId(this) {
                Ok(ok__) => {
                    core::ptr::write(pulmasterclientid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveClients<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportclients: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSession_Impl::RetrieveClients(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransportclients, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportSession_Impl::Terminate(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Content: Content::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            NetworkInterfaceName: NetworkInterfaceName::<Identity, Impl, OFFSET>,
            NetworkInterfaceAddress: NetworkInterfaceAddress::<Identity, Impl, OFFSET>,
            TransferRate: TransferRate::<Identity, Impl, OFFSET>,
            MasterClientId: MasterClientId::<Identity, Impl, OFFSET>,
            RetrieveClients: RetrieveClients::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportSession as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSetupManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Version(&self) -> windows_core::Result<u64>;
    fn InstalledFeatures(&self) -> windows_core::Result<u32>;
    fn Protocols(&self) -> windows_core::Result<u32>;
    fn RegisterContentProvider(&self, bszname: &windows_core::BSTR, bszdescription: &windows_core::BSTR, bszfilepath: &windows_core::BSTR, bszinitializationroutine: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeregisterContentProvider(&self, bszname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportSetupManager {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>() -> IWdsTransportSetupManager_Vtbl {
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullversion: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSetupManager_Impl::Version(this) {
                Ok(ok__) => {
                    core::ptr::write(pullversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledFeatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulinstalledfeatures: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSetupManager_Impl::InstalledFeatures(this) {
                Ok(ok__) => {
                    core::ptr::write(pulinstalledfeatures, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocols<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulprotocols: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSetupManager_Impl::Protocols(this) {
                Ok(ok__) => {
                    core::ptr::write(pulprotocols, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterContentProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszname: core::mem::MaybeUninit<windows_core::BSTR>, bszdescription: core::mem::MaybeUninit<windows_core::BSTR>, bszfilepath: core::mem::MaybeUninit<windows_core::BSTR>, bszinitializationroutine: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportSetupManager_Impl::RegisterContentProvider(this, core::mem::transmute(&bszname), core::mem::transmute(&bszdescription), core::mem::transmute(&bszfilepath), core::mem::transmute(&bszinitializationroutine)).into()
        }
        unsafe extern "system" fn DeregisterContentProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWdsTransportSetupManager_Impl::DeregisterContentProvider(this, core::mem::transmute(&bszname)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Version: Version::<Identity, Impl, OFFSET>,
            InstalledFeatures: InstalledFeatures::<Identity, Impl, OFFSET>,
            Protocols: Protocols::<Identity, Impl, OFFSET>,
            RegisterContentProvider: RegisterContentProvider::<Identity, Impl, OFFSET>,
            DeregisterContentProvider: DeregisterContentProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSetupManager2_Impl: Sized + IWdsTransportSetupManager_Impl {
    fn TftpCapabilities(&self) -> windows_core::Result<u32>;
    fn ContentProviders(&self) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportSetupManager2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>() -> IWdsTransportSetupManager2_Vtbl {
        unsafe extern "system" fn TftpCapabilities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultftpcapabilities: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSetupManager2_Impl::TftpCapabilities(this) {
                Ok(ok__) => {
                    core::ptr::write(pultftpcapabilities, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprovidercollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportSetupManager2_Impl::ContentProviders(this) {
                Ok(ok__) => {
                    core::ptr::write(ppprovidercollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWdsTransportSetupManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            TftpCapabilities: TftpCapabilities::<Identity, Impl, OFFSET>,
            ContentProviders: ContentProviders::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportSetupManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportTftpClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn FileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IpAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Timeout(&self) -> windows_core::Result<u32>;
    fn CurrentFileOffset(&self) -> windows_core::Result<u64>;
    fn FileSize(&self) -> windows_core::Result<u64>;
    fn BlockSize(&self) -> windows_core::Result<u32>;
    fn WindowSize(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportTftpClient {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>() -> IWdsTransportTftpClient_Vtbl {
        unsafe extern "system" fn FileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpClient_Impl::FileName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszfilename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszipaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpClient_Impl::IpAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbszipaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultimeout: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpClient_Impl::Timeout(this) {
                Ok(ok__) => {
                    core::ptr::write(pultimeout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFileOffset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pul64currentoffset: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpClient_Impl::CurrentFileOffset(this) {
                Ok(ok__) => {
                    core::ptr::write(pul64currentoffset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pul64filesize: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpClient_Impl::FileSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pul64filesize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulblocksize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpClient_Impl::BlockSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pulblocksize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulwindowsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpClient_Impl::WindowSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pulwindowsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FileName: FileName::<Identity, Impl, OFFSET>,
            IpAddress: IpAddress::<Identity, Impl, OFFSET>,
            Timeout: Timeout::<Identity, Impl, OFFSET>,
            CurrentFileOffset: CurrentFileOffset::<Identity, Impl, OFFSET>,
            FileSize: FileSize::<Identity, Impl, OFFSET>,
            BlockSize: BlockSize::<Identity, Impl, OFFSET>,
            WindowSize: WindowSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportTftpClient as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportTftpManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn RetrieveTftpClients(&self) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWdsTransportTftpManager {}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpManager_Impl, const OFFSET: isize>() -> IWdsTransportTftpManager_Vtbl {
        unsafe extern "system" fn RetrieveTftpClients<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransporttftpclients: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWdsTransportTftpManager_Impl::RetrieveTftpClients(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwdstransporttftpclients, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), RetrieveTftpClients: RetrieveTftpClients::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportTftpManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
