#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetworkConnections_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut Option<INetworkConnection>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetworkConnections>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IEnumNetworkConnections {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworkConnections_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>() -> IEnumNetworkConnections_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvar: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetworkConnections_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumvar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetworkConnections_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetworkConnections_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetworkConnections_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetworkConnections_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumnetwork, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetworkConnections as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetworks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut Option<INetwork>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetworks>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IEnumNetworks {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworks_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>() -> IEnumNetworks_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvar: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetworks_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumvar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetworks_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetworks_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetworks_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetworks_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumnetwork, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetworks as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, sznetworknewname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, szdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetNetworkId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDomainType(&self) -> windows_core::Result<NLM_DOMAIN_TYPE>;
    fn GetNetworkConnections(&self) -> windows_core::Result<IEnumNetworkConnections>;
    fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> windows_core::Result<()>;
    fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY>;
    fn GetCategory(&self) -> windows_core::Result<NLM_NETWORK_CATEGORY>;
    fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetwork {}
#[cfg(feature = "Win32_System_Com")]
impl INetwork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>() -> INetwork_Vtbl {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psznetworkname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(psznetworkname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznetworknewname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetwork_Impl::SetName(this, core::mem::transmute(&sznetworknewname)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::GetDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(pszdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetwork_Impl::SetDescription(this, core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn GetNetworkId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgdguidnetworkid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::GetNetworkId(this) {
                Ok(ok__) => {
                    core::ptr::write(pgdguidnetworkid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::GetDomainType(this) {
                Ok(ok__) => {
                    core::ptr::write(pnetworktype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumnetworkconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::GetNetworkConnections(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumnetworkconnection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetwork_Impl::GetTimeCreatedAndConnected(this, core::mem::transmute_copy(&pdwlowdatetimecreated), core::mem::transmute_copy(&pdwhighdatetimecreated), core::mem::transmute_copy(&pdwlowdatetimeconnected), core::mem::transmute_copy(&pdwhighdatetimeconnected)).into()
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::IsConnectedToInternet(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisconnected, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::IsConnected(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisconnected, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::GetConnectivity(this) {
                Ok(ok__) => {
                    core::ptr::write(pconnectivity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork_Impl::GetCategory(this) {
                Ok(ok__) => {
                    core::ptr::write(pcategory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetwork_Impl::SetCategory(this, core::mem::transmute_copy(&newcategory)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetNetworkId: GetNetworkId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetTimeCreatedAndConnected: GetTimeCreatedAndConnected::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            SetCategory: SetCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetwork as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetwork2_Impl: Sized + INetwork_Impl {
    fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetwork2 {}
#[cfg(feature = "Win32_System_Com")]
impl INetwork2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork2_Impl, const OFFSET: isize>() -> INetwork2_Vtbl {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetwork2_Impl::IsDomainAuthenticatedBy(this, core::mem::transmute_copy(&domainauthenticationkind)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: INetwork_Vtbl::new::<Identity, Impl, OFFSET>(), IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetwork2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetwork as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkConnection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetNetwork(&self) -> windows_core::Result<INetwork>;
    fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY>;
    fn GetConnectionId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetAdapterId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDomainType(&self) -> windows_core::Result<NLM_DOMAIN_TYPE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetworkConnection {}
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>() -> INetworkConnection_Vtbl {
        unsafe extern "system" fn GetNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection_Impl::GetNetwork(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnetwork, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection_Impl::IsConnectedToInternet(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisconnected, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection_Impl::IsConnected(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisconnected, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection_Impl::GetConnectivity(this) {
                Ok(ok__) => {
                    core::ptr::write(pconnectivity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgdconnectionid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection_Impl::GetConnectionId(this) {
                Ok(ok__) => {
                    core::ptr::write(pgdconnectionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgdadapterid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection_Impl::GetAdapterId(this) {
                Ok(ok__) => {
                    core::ptr::write(pgdadapterid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection_Impl::GetDomainType(this) {
                Ok(ok__) => {
                    core::ptr::write(pdomaintype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, Impl, OFFSET>,
            GetAdapterId: GetAdapterId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkConnection2_Impl: Sized + INetworkConnection_Impl {
    fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetworkConnection2 {}
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection2_Impl, const OFFSET: isize>() -> INetworkConnection2_Vtbl {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnection2_Impl::IsDomainAuthenticatedBy(this, core::mem::transmute_copy(&domainauthenticationkind)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: INetworkConnection_Vtbl::new::<Identity, Impl, OFFSET>(), IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnection2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetworkConnection as windows_core::Interface>::IID
    }
}
pub trait INetworkConnectionCost_Impl: Sized {
    fn GetCost(&self) -> windows_core::Result<u32>;
    fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetworkConnectionCost {}
impl INetworkConnectionCost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>() -> INetworkConnectionCost_Vtbl {
        unsafe extern "system" fn GetCost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcost: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkConnectionCost_Impl::GetCost(this) {
                Ok(ok__) => {
                    core::ptr::write(pcost, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkConnectionCost_Impl::GetDataPlanStatus(this, core::mem::transmute_copy(&pdataplanstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnectionCost as windows_core::Interface>::IID
    }
}
pub trait INetworkConnectionCostEvents_Impl: Sized {
    fn ConnectionCostChanged(&self, connectionid: &windows_core::GUID, newcost: u32) -> windows_core::Result<()>;
    fn ConnectionDataPlanStatusChanged(&self, connectionid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetworkConnectionCostEvents {}
impl INetworkConnectionCostEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>() -> INetworkConnectionCostEvents_Vtbl {
        unsafe extern "system" fn ConnectionCostChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID, newcost: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkConnectionCostEvents_Impl::ConnectionCostChanged(this, core::mem::transmute(&connectionid), core::mem::transmute_copy(&newcost)).into()
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkConnectionCostEvents_Impl::ConnectionDataPlanStatusChanged(this, core::mem::transmute(&connectionid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectionCostChanged: ConnectionCostChanged::<Identity, Impl, OFFSET>,
            ConnectionDataPlanStatusChanged: ConnectionDataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnectionCostEvents as windows_core::Interface>::IID
    }
}
pub trait INetworkConnectionEvents_Impl: Sized {
    fn NetworkConnectionConnectivityChanged(&self, connectionid: &windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()>;
    fn NetworkConnectionPropertyChanged(&self, connectionid: &windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetworkConnectionEvents {}
impl INetworkConnectionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>() -> INetworkConnectionEvents_Vtbl {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkConnectionEvents_Impl::NetworkConnectionConnectivityChanged(this, core::mem::transmute(&connectionid), core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkConnectionEvents_Impl::NetworkConnectionPropertyChanged(this, core::mem::transmute(&connectionid), core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NetworkConnectionConnectivityChanged: NetworkConnectionConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkConnectionPropertyChanged: NetworkConnectionPropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnectionEvents as windows_core::Interface>::IID
    }
}
pub trait INetworkCostManager_Impl: Sized {
    fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
    fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
    fn SetDestinationAddresses(&self, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetworkCostManager {}
impl INetworkCostManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>() -> INetworkCostManager_Vtbl {
        unsafe extern "system" fn GetCost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkCostManager_Impl::GetCost(this, core::mem::transmute_copy(&pcost), core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkCostManager_Impl::GetDataPlanStatus(this, core::mem::transmute_copy(&pdataplanstatus), core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn SetDestinationAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkCostManager_Impl::SetDestinationAddresses(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&pdestipaddrlist), core::mem::transmute_copy(&bappend)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
            SetDestinationAddresses: SetDestinationAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkCostManager as windows_core::Interface>::IID
    }
}
pub trait INetworkCostManagerEvents_Impl: Sized {
    fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
    fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetworkCostManagerEvents {}
impl INetworkCostManagerEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>() -> INetworkCostManagerEvents_Vtbl {
        unsafe extern "system" fn CostChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkCostManagerEvents_Impl::CostChanged(this, core::mem::transmute_copy(&newcost), core::mem::transmute_copy(&pdestaddr)).into()
        }
        unsafe extern "system" fn DataPlanStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkCostManagerEvents_Impl::DataPlanStatusChanged(this, core::mem::transmute_copy(&pdestaddr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CostChanged: CostChanged::<Identity, Impl, OFFSET>,
            DataPlanStatusChanged: DataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkCostManagerEvents as windows_core::Interface>::IID
    }
}
pub trait INetworkEvents_Impl: Sized {
    fn NetworkAdded(&self, networkid: &windows_core::GUID) -> windows_core::Result<()>;
    fn NetworkDeleted(&self, networkid: &windows_core::GUID) -> windows_core::Result<()>;
    fn NetworkConnectivityChanged(&self, networkid: &windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()>;
    fn NetworkPropertyChanged(&self, networkid: &windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetworkEvents {}
impl INetworkEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>() -> INetworkEvents_Vtbl {
        unsafe extern "system" fn NetworkAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkEvents_Impl::NetworkAdded(this, core::mem::transmute(&networkid)).into()
        }
        unsafe extern "system" fn NetworkDeleted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkEvents_Impl::NetworkDeleted(this, core::mem::transmute(&networkid)).into()
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkEvents_Impl::NetworkConnectivityChanged(this, core::mem::transmute(&networkid), core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkPropertyChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkEvents_Impl::NetworkPropertyChanged(this, core::mem::transmute(&networkid), core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NetworkAdded: NetworkAdded::<Identity, Impl, OFFSET>,
            NetworkDeleted: NetworkDeleted::<Identity, Impl, OFFSET>,
            NetworkConnectivityChanged: NetworkConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkPropertyChanged: NetworkPropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkListManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> windows_core::Result<IEnumNetworks>;
    fn GetNetwork(&self, gdnetworkid: &windows_core::GUID) -> windows_core::Result<INetwork>;
    fn GetNetworkConnections(&self) -> windows_core::Result<IEnumNetworkConnections>;
    fn GetNetworkConnection(&self, gdnetworkconnectionid: &windows_core::GUID) -> windows_core::Result<INetworkConnection>;
    fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY>;
    fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> windows_core::Result<()>;
    fn ClearSimulatedProfileInfo(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetworkListManager {}
#[cfg(feature = "Win32_System_Com")]
impl INetworkListManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>() -> INetworkListManager_Vtbl {
        unsafe extern "system" fn GetNetworks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkListManager_Impl::GetNetworks(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumnetwork, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdnetworkid: windows_core::GUID, ppnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkListManager_Impl::GetNetwork(this, core::mem::transmute(&gdnetworkid)) {
                Ok(ok__) => {
                    core::ptr::write(ppnetwork, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkListManager_Impl::GetNetworkConnections(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdnetworkconnectionid: windows_core::GUID, ppnetworkconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkListManager_Impl::GetNetworkConnection(this, core::mem::transmute(&gdnetworkconnectionid)) {
                Ok(ok__) => {
                    core::ptr::write(ppnetworkconnection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkListManager_Impl::IsConnectedToInternet(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisconnected, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkListManager_Impl::IsConnected(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisconnected, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetworkListManager_Impl::GetConnectivity(this) {
                Ok(ok__) => {
                    core::ptr::write(pconnectivity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkListManager_Impl::SetSimulatedProfileInfo(this, core::mem::transmute_copy(&psimulatedinfo)).into()
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkListManager_Impl::ClearSimulatedProfileInfo(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetworks: GetNetworks::<Identity, Impl, OFFSET>,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetNetworkConnection: GetNetworkConnection::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            SetSimulatedProfileInfo: SetSimulatedProfileInfo::<Identity, Impl, OFFSET>,
            ClearSimulatedProfileInfo: ClearSimulatedProfileInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkListManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait INetworkListManagerEvents_Impl: Sized {
    fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetworkListManagerEvents {}
impl INetworkListManagerEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManagerEvents_Impl, const OFFSET: isize>() -> INetworkListManagerEvents_Vtbl {
        unsafe extern "system" fn ConnectivityChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetworkListManagerEvents_Impl::ConnectivityChanged(this, core::mem::transmute_copy(&newconnectivity)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectivityChanged: ConnectivityChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkListManagerEvents as windows_core::Interface>::IID
    }
}
