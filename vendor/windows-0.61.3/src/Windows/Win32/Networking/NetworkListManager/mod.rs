#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumNetworkConnections, IEnumNetworkConnections_Vtbl, 0xdcb00006_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumNetworkConnections {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumNetworkConnections, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworkConnections {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetworkConnection>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetworkConnections> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworkConnections_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetworkConnections_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut Option<INetworkConnection>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetworkConnections>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetworkConnections_Vtbl {
    pub const fn new<Identity: IEnumNetworkConnections_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvar: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetworkConnections_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetworkConnections_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetworkConnections_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetworkConnections_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetworkConnections_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumnetwork.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetworkConnections as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumNetworkConnections {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumNetworks, IEnumNetworks_Vtbl, 0xdcb00003_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumNetworks {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumNetworks, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworks {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetwork>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetworks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetworks_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut Option<INetwork>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetworks>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetworks_Vtbl {
    pub const fn new<Identity: IEnumNetworks_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvar: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetworks_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetworks_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetworks_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetworks_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetworks_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumnetwork.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetworks as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumNetworks {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetwork, INetwork_Vtbl, 0xdcb00002_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetwork {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetwork, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetwork {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, sznetworknewname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sznetworknewname)).ok() }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, szdescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(szdescription)).ok() }
    }
    pub unsafe fn GetNetworkId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDomainType(&self) -> windows_core::Result<NLM_DOMAIN_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDomainType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNetworkConnections(&self) -> windows_core::Result<IEnumNetworkConnections> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTimeCreatedAndConnected)(windows_core::Interface::as_raw(self), pdwlowdatetimecreated as _, pdwhighdatetimecreated as _, pdwlowdatetimeconnected as _, pdwhighdatetimeconnected as _).ok() }
    }
    pub unsafe fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnectedToInternet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<NLM_NETWORK_CATEGORY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCategory)(windows_core::Interface::as_raw(self), newcategory).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT,
    pub GetNetworkConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTimeCreatedAndConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub IsConnectedToInternet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_NETWORK_CATEGORY) -> windows_core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_NETWORK_CATEGORY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetwork_Impl: super::super::System::Com::IDispatch_Impl {
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetwork_Vtbl {
    pub const fn new<Identity: INetwork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psznetworkname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::GetName(this) {
                    Ok(ok__) => {
                        psznetworkname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznetworknewname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetwork_Impl::SetName(this, core::mem::transmute(&sznetworknewname)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetwork_Impl::SetDescription(this, core::mem::transmute(&szdescription)).into()
            }
        }
        unsafe extern "system" fn GetNetworkId<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgdguidnetworkid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::GetNetworkId(this) {
                    Ok(ok__) => {
                        pgdguidnetworkid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::GetDomainType(this) {
                    Ok(ok__) => {
                        pnetworktype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumnetworkconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::GetNetworkConnections(this) {
                    Ok(ok__) => {
                        ppenumnetworkconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetwork_Impl::GetTimeCreatedAndConnected(this, core::mem::transmute_copy(&pdwlowdatetimecreated), core::mem::transmute_copy(&pdwhighdatetimecreated), core::mem::transmute_copy(&pdwlowdatetimeconnected), core::mem::transmute_copy(&pdwhighdatetimeconnected)).into()
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::IsConnectedToInternet(this) {
                    Ok(ok__) => {
                        pbisconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsConnected<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        pbisconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::GetConnectivity(this) {
                    Ok(ok__) => {
                        pconnectivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategory<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork_Impl::GetCategory(this) {
                    Ok(ok__) => {
                        pcategory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCategory<Identity: INetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetwork_Impl::SetCategory(this, core::mem::transmute_copy(&newcategory)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetNetworkId: GetNetworkId::<Identity, OFFSET>,
            GetDomainType: GetDomainType::<Identity, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, OFFSET>,
            GetTimeCreatedAndConnected: GetTimeCreatedAndConnected::<Identity, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
            SetCategory: SetCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetwork as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetwork {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetwork2, INetwork2_Vtbl, 0xb5550abb_3391_4310_804f_25dcc325ed81);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetwork2 {
    type Target = INetwork;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetwork2, windows_core::IUnknown, super::super::System::Com::IDispatch, INetwork);
#[cfg(feature = "Win32_System_Com")]
impl INetwork2 {
    pub unsafe fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDomainAuthenticatedBy)(windows_core::Interface::as_raw(self), domainauthenticationkind, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetwork2_Vtbl {
    pub base__: INetwork_Vtbl,
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_DOMAIN_AUTHENTICATION_KIND, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetwork2_Impl: INetwork_Impl {
    fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetwork2_Vtbl {
    pub const fn new<Identity: INetwork2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: INetwork2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetwork2_Impl::IsDomainAuthenticatedBy(this, core::mem::transmute_copy(&domainauthenticationkind)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: INetwork_Vtbl::new::<Identity, OFFSET>(), IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetwork2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetwork as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetwork2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetworkConnection, INetworkConnection_Vtbl, 0xdcb00005_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetworkConnection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetworkConnection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection {
    pub unsafe fn GetNetwork(&self) -> windows_core::Result<INetwork> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetwork)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnectedToInternet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectionId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAdapterId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapterId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDomainType(&self) -> windows_core::Result<NLM_DOMAIN_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDomainType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnectedToInternet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetAdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkConnection_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetNetwork(&self) -> windows_core::Result<INetwork>;
    fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY>;
    fn GetConnectionId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetAdapterId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDomainType(&self) -> windows_core::Result<NLM_DOMAIN_TYPE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetworkConnection_Vtbl {
    pub const fn new<Identity: INetworkConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNetwork<Identity: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection_Impl::GetNetwork(this) {
                    Ok(ok__) => {
                        ppnetwork.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection_Impl::IsConnectedToInternet(this) {
                    Ok(ok__) => {
                        pbisconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsConnected<Identity: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        pbisconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection_Impl::GetConnectivity(this) {
                    Ok(ok__) => {
                        pconnectivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectionId<Identity: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgdconnectionid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection_Impl::GetConnectionId(this) {
                    Ok(ok__) => {
                        pgdconnectionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAdapterId<Identity: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgdadapterid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection_Impl::GetAdapterId(this) {
                    Ok(ok__) => {
                        pgdadapterid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: INetworkConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection_Impl::GetDomainType(this) {
                    Ok(ok__) => {
                        pdomaintype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetNetwork: GetNetwork::<Identity, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, OFFSET>,
            GetAdapterId: GetAdapterId::<Identity, OFFSET>,
            GetDomainType: GetDomainType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetworkConnection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetworkConnection2, INetworkConnection2_Vtbl, 0x00e676ed_5a35_4738_92eb_8581738d0f0a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetworkConnection2 {
    type Target = INetworkConnection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetworkConnection2, windows_core::IUnknown, super::super::System::Com::IDispatch, INetworkConnection);
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection2 {
    pub unsafe fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDomainAuthenticatedBy)(windows_core::Interface::as_raw(self), domainauthenticationkind, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnection2_Vtbl {
    pub base__: INetworkConnection_Vtbl,
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_DOMAIN_AUTHENTICATION_KIND, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkConnection2_Impl: INetworkConnection_Impl {
    fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetworkConnection2_Vtbl {
    pub const fn new<Identity: INetworkConnection2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: INetworkConnection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnection2_Impl::IsDomainAuthenticatedBy(this, core::mem::transmute_copy(&domainauthenticationkind)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: INetworkConnection_Vtbl::new::<Identity, OFFSET>(), IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnection2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetworkConnection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetworkConnection2 {}
windows_core::imp::define_interface!(INetworkConnectionCost, INetworkConnectionCost_Vtbl, 0xdcb0000a_570f_4a9b_8d69_199fdba5723b);
windows_core::imp::interface_hierarchy!(INetworkConnectionCost, windows_core::IUnknown);
impl INetworkConnectionCost {
    pub unsafe fn GetCost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDataPlanStatus)(windows_core::Interface::as_raw(self), pdataplanstatus as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DATAPLAN_STATUS) -> windows_core::HRESULT,
}
pub trait INetworkConnectionCost_Impl: windows_core::IUnknownImpl {
    fn GetCost(&self) -> windows_core::Result<u32>;
    fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> windows_core::Result<()>;
}
impl INetworkConnectionCost_Vtbl {
    pub const fn new<Identity: INetworkConnectionCost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCost<Identity: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcost: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkConnectionCost_Impl::GetCost(this) {
                    Ok(ok__) => {
                        pcost.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkConnectionCost_Impl::GetDataPlanStatus(this, core::mem::transmute_copy(&pdataplanstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnectionCost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetworkConnectionCost {}
windows_core::imp::define_interface!(INetworkConnectionCostEvents, INetworkConnectionCostEvents_Vtbl, 0xdcb0000b_570f_4a9b_8d69_199fdba5723b);
windows_core::imp::interface_hierarchy!(INetworkConnectionCostEvents, windows_core::IUnknown);
impl INetworkConnectionCostEvents {
    pub unsafe fn ConnectionCostChanged(&self, connectionid: windows_core::GUID, newcost: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectionCostChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid), newcost).ok() }
    }
    pub unsafe fn ConnectionDataPlanStatusChanged(&self, connectionid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectionDataPlanStatusChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCostEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectionCostChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub ConnectionDataPlanStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait INetworkConnectionCostEvents_Impl: windows_core::IUnknownImpl {
    fn ConnectionCostChanged(&self, connectionid: &windows_core::GUID, newcost: u32) -> windows_core::Result<()>;
    fn ConnectionDataPlanStatusChanged(&self, connectionid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl INetworkConnectionCostEvents_Vtbl {
    pub const fn new<Identity: INetworkConnectionCostEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectionCostChanged<Identity: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID, newcost: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkConnectionCostEvents_Impl::ConnectionCostChanged(this, core::mem::transmute(&connectionid), core::mem::transmute_copy(&newcost)).into()
            }
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Identity: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkConnectionCostEvents_Impl::ConnectionDataPlanStatusChanged(this, core::mem::transmute(&connectionid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectionCostChanged: ConnectionCostChanged::<Identity, OFFSET>,
            ConnectionDataPlanStatusChanged: ConnectionDataPlanStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnectionCostEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetworkConnectionCostEvents {}
windows_core::imp::define_interface!(INetworkConnectionEvents, INetworkConnectionEvents_Vtbl, 0xdcb00007_570f_4a9b_8d69_199fdba5723b);
windows_core::imp::interface_hierarchy!(INetworkConnectionEvents, windows_core::IUnknown);
impl INetworkConnectionEvents {
    pub unsafe fn NetworkConnectionConnectivityChanged(&self, connectionid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NetworkConnectionConnectivityChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid), newconnectivity).ok() }
    }
    pub unsafe fn NetworkConnectionPropertyChanged(&self, connectionid: windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NetworkConnectionPropertyChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid), flags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NetworkConnectionConnectivityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub NetworkConnectionPropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::HRESULT,
}
pub trait INetworkConnectionEvents_Impl: windows_core::IUnknownImpl {
    fn NetworkConnectionConnectivityChanged(&self, connectionid: &windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()>;
    fn NetworkConnectionPropertyChanged(&self, connectionid: &windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::Result<()>;
}
impl INetworkConnectionEvents_Vtbl {
    pub const fn new<Identity: INetworkConnectionEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Identity: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkConnectionEvents_Impl::NetworkConnectionConnectivityChanged(this, core::mem::transmute(&connectionid), core::mem::transmute_copy(&newconnectivity)).into()
            }
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Identity: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkConnectionEvents_Impl::NetworkConnectionPropertyChanged(this, core::mem::transmute(&connectionid), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NetworkConnectionConnectivityChanged: NetworkConnectionConnectivityChanged::<Identity, OFFSET>,
            NetworkConnectionPropertyChanged: NetworkConnectionPropertyChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkConnectionEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetworkConnectionEvents {}
windows_core::imp::define_interface!(INetworkCostManager, INetworkCostManager_Vtbl, 0xdcb00008_570f_4a9b_8d69_199fdba5723b);
windows_core::imp::interface_hierarchy!(INetworkCostManager, windows_core::IUnknown);
impl INetworkCostManager {
    pub unsafe fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCost)(windows_core::Interface::as_raw(self), pcost as _, pdestipaddr).ok() }
    }
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDataPlanStatus)(windows_core::Interface::as_raw(self), pdataplanstatus as _, pdestipaddr).ok() }
    }
    pub unsafe fn SetDestinationAddresses(&self, pdestipaddrlist: &[NLM_SOCKADDR], bappend: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationAddresses)(windows_core::Interface::as_raw(self), pdestipaddrlist.len().try_into().unwrap(), core::mem::transmute(pdestipaddrlist.as_ptr()), bappend).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *const NLM_SOCKADDR) -> windows_core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DATAPLAN_STATUS, *const NLM_SOCKADDR) -> windows_core::HRESULT,
    pub SetDestinationAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const NLM_SOCKADDR, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait INetworkCostManager_Impl: windows_core::IUnknownImpl {
    fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
    fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
    fn SetDestinationAddresses(&self, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl INetworkCostManager_Vtbl {
    pub const fn new<Identity: INetworkCostManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCost<Identity: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkCostManager_Impl::GetCost(this, core::mem::transmute_copy(&pcost), core::mem::transmute_copy(&pdestipaddr)).into()
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkCostManager_Impl::GetDataPlanStatus(this, core::mem::transmute_copy(&pdataplanstatus), core::mem::transmute_copy(&pdestipaddr)).into()
            }
        }
        unsafe extern "system" fn SetDestinationAddresses<Identity: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkCostManager_Impl::SetDestinationAddresses(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&pdestipaddrlist), core::mem::transmute_copy(&bappend)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, OFFSET>,
            SetDestinationAddresses: SetDestinationAddresses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkCostManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetworkCostManager {}
windows_core::imp::define_interface!(INetworkCostManagerEvents, INetworkCostManagerEvents_Vtbl, 0xdcb00009_570f_4a9b_8d69_199fdba5723b);
windows_core::imp::interface_hierarchy!(INetworkCostManagerEvents, windows_core::IUnknown);
impl INetworkCostManagerEvents {
    pub unsafe fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CostChanged)(windows_core::Interface::as_raw(self), newcost, pdestaddr).ok() }
    }
    pub unsafe fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DataPlanStatusChanged)(windows_core::Interface::as_raw(self), pdestaddr).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CostChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const NLM_SOCKADDR) -> windows_core::HRESULT,
    pub DataPlanStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const NLM_SOCKADDR) -> windows_core::HRESULT,
}
pub trait INetworkCostManagerEvents_Impl: windows_core::IUnknownImpl {
    fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
    fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()>;
}
impl INetworkCostManagerEvents_Vtbl {
    pub const fn new<Identity: INetworkCostManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CostChanged<Identity: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkCostManagerEvents_Impl::CostChanged(this, core::mem::transmute_copy(&newcost), core::mem::transmute_copy(&pdestaddr)).into()
            }
        }
        unsafe extern "system" fn DataPlanStatusChanged<Identity: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkCostManagerEvents_Impl::DataPlanStatusChanged(this, core::mem::transmute_copy(&pdestaddr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CostChanged: CostChanged::<Identity, OFFSET>,
            DataPlanStatusChanged: DataPlanStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkCostManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetworkCostManagerEvents {}
windows_core::imp::define_interface!(INetworkEvents, INetworkEvents_Vtbl, 0xdcb00004_570f_4a9b_8d69_199fdba5723b);
windows_core::imp::interface_hierarchy!(INetworkEvents, windows_core::IUnknown);
impl INetworkEvents {
    pub unsafe fn NetworkAdded(&self, networkid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NetworkAdded)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid)).ok() }
    }
    pub unsafe fn NetworkDeleted(&self, networkid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NetworkDeleted)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid)).ok() }
    }
    pub unsafe fn NetworkConnectivityChanged(&self, networkid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NetworkConnectivityChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid), newconnectivity).ok() }
    }
    pub unsafe fn NetworkPropertyChanged(&self, networkid: windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NetworkPropertyChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid), flags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NetworkAdded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub NetworkDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub NetworkConnectivityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub NetworkPropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::HRESULT,
}
pub trait INetworkEvents_Impl: windows_core::IUnknownImpl {
    fn NetworkAdded(&self, networkid: &windows_core::GUID) -> windows_core::Result<()>;
    fn NetworkDeleted(&self, networkid: &windows_core::GUID) -> windows_core::Result<()>;
    fn NetworkConnectivityChanged(&self, networkid: &windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()>;
    fn NetworkPropertyChanged(&self, networkid: &windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::Result<()>;
}
impl INetworkEvents_Vtbl {
    pub const fn new<Identity: INetworkEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NetworkAdded<Identity: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkEvents_Impl::NetworkAdded(this, core::mem::transmute(&networkid)).into()
            }
        }
        unsafe extern "system" fn NetworkDeleted<Identity: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkEvents_Impl::NetworkDeleted(this, core::mem::transmute(&networkid)).into()
            }
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Identity: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkEvents_Impl::NetworkConnectivityChanged(this, core::mem::transmute(&networkid), core::mem::transmute_copy(&newconnectivity)).into()
            }
        }
        unsafe extern "system" fn NetworkPropertyChanged<Identity: INetworkEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkid: windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkEvents_Impl::NetworkPropertyChanged(this, core::mem::transmute(&networkid), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NetworkAdded: NetworkAdded::<Identity, OFFSET>,
            NetworkDeleted: NetworkDeleted::<Identity, OFFSET>,
            NetworkConnectivityChanged: NetworkConnectivityChanged::<Identity, OFFSET>,
            NetworkPropertyChanged: NetworkPropertyChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetworkEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetworkListManager, INetworkListManager_Vtbl, 0xdcb00000_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetworkListManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetworkListManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetworkListManager {
    pub unsafe fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> windows_core::Result<IEnumNetworks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNetwork(&self, gdnetworkid: windows_core::GUID) -> windows_core::Result<INetwork> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetwork)(windows_core::Interface::as_raw(self), core::mem::transmute(gdnetworkid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNetworkConnections(&self) -> windows_core::Result<IEnumNetworkConnections> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNetworkConnection(&self, gdnetworkconnectionid: windows_core::GUID) -> windows_core::Result<INetworkConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNetworkConnection)(windows_core::Interface::as_raw(self), core::mem::transmute(gdnetworkconnectionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnectedToInternet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSimulatedProfileInfo)(windows_core::Interface::as_raw(self), psimulatedinfo).ok() }
    }
    pub unsafe fn ClearSimulatedProfileInfo(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearSimulatedProfileInfo)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetNetworks: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_ENUM_NETWORK, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetworkConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetworkConnection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnectedToInternet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub SetSimulatedProfileInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const NLM_SIMULATED_PROFILE_INFO) -> windows_core::HRESULT,
    pub ClearSimulatedProfileInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkListManager_Impl: super::super::System::Com::IDispatch_Impl {
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetworkListManager_Vtbl {
    pub const fn new<Identity: INetworkListManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNetworks<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkListManager_Impl::GetNetworks(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppenumnetwork.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNetwork<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdnetworkid: windows_core::GUID, ppnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkListManager_Impl::GetNetwork(this, core::mem::transmute(&gdnetworkid)) {
                    Ok(ok__) => {
                        ppnetwork.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkListManager_Impl::GetNetworkConnections(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNetworkConnection<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdnetworkconnectionid: windows_core::GUID, ppnetworkconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkListManager_Impl::GetNetworkConnection(this, core::mem::transmute(&gdnetworkconnectionid)) {
                    Ok(ok__) => {
                        ppnetworkconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkListManager_Impl::IsConnectedToInternet(this) {
                    Ok(ok__) => {
                        pbisconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsConnected<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkListManager_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        pbisconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkListManager_Impl::GetConnectivity(this) {
                    Ok(ok__) => {
                        pconnectivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkListManager_Impl::SetSimulatedProfileInfo(this, core::mem::transmute_copy(&psimulatedinfo)).into()
            }
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Identity: INetworkListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkListManager_Impl::ClearSimulatedProfileInfo(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetNetworks: GetNetworks::<Identity, OFFSET>,
            GetNetwork: GetNetwork::<Identity, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, OFFSET>,
            GetNetworkConnection: GetNetworkConnection::<Identity, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, OFFSET>,
            SetSimulatedProfileInfo: SetSimulatedProfileInfo::<Identity, OFFSET>,
            ClearSimulatedProfileInfo: ClearSimulatedProfileInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkListManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetworkListManager {}
windows_core::imp::define_interface!(INetworkListManagerEvents, INetworkListManagerEvents_Vtbl, 0xdcb00001_570f_4a9b_8d69_199fdba5723b);
windows_core::imp::interface_hierarchy!(INetworkListManagerEvents, windows_core::IUnknown);
impl INetworkListManagerEvents {
    pub unsafe fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectivityChanged)(windows_core::Interface::as_raw(self), newconnectivity).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectivityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_CONNECTIVITY) -> windows_core::HRESULT,
}
pub trait INetworkListManagerEvents_Impl: windows_core::IUnknownImpl {
    fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()>;
}
impl INetworkListManagerEvents_Vtbl {
    pub const fn new<Identity: INetworkListManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectivityChanged<Identity: INetworkListManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkListManagerEvents_Impl::ConnectivityChanged(this, core::mem::transmute_copy(&newconnectivity)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectivityChanged: ConnectivityChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkListManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetworkListManagerEvents {}
pub const NA_AllowMerge: windows_core::PCWSTR = windows_core::w!("NA_AllowMerge");
pub const NA_CategoryReadOnly: windows_core::PCWSTR = windows_core::w!("NA_CategoryReadOnly");
pub const NA_CategorySetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_CategorySetByPolicy");
pub const NA_DescriptionReadOnly: windows_core::PCWSTR = windows_core::w!("NA_DescriptionReadOnly");
pub const NA_DescriptionSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_DescriptionSetByPolicy");
pub const NA_DomainAuthenticationFailed: windows_core::PCWSTR = windows_core::w!("NA_DomainAuthenticationFailed");
pub const NA_IconReadOnly: windows_core::PCWSTR = windows_core::w!("NA_IconReadOnly");
pub const NA_IconSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_IconSetByPolicy");
pub const NA_InternetConnectivityV4: windows_core::PCWSTR = windows_core::w!("NA_InternetConnectivityV4");
pub const NA_InternetConnectivityV6: windows_core::PCWSTR = windows_core::w!("NA_InternetConnectivityV6");
pub const NA_NameReadOnly: windows_core::PCWSTR = windows_core::w!("NA_NameReadOnly");
pub const NA_NameSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_NameSetByPolicy");
pub const NA_NetworkClass: windows_core::PCWSTR = windows_core::w!("NA_NetworkClass");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_CONNECTION_COST(pub i32);
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(524288i32);
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(131072i32);
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(2i32);
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(65536i32);
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = NLM_CONNECTION_COST(262144i32);
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = NLM_CONNECTION_COST(0i32);
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(1i32);
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = NLM_CONNECTION_COST(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_CONNECTION_PROPERTY_CHANGE(pub i32);
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = NLM_CONNECTION_PROPERTY_CHANGE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_CONNECTIVITY(pub i32);
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = NLM_CONNECTIVITY(0i32);
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(64i32);
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(32i32);
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1i32);
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(16i32);
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1024i32);
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(512i32);
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(2i32);
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(256i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: windows_core::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::super::Foundation::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_DOMAIN_AUTHENTICATION_KIND(pub i32);
pub const NLM_DOMAIN_AUTHENTICATION_KIND_LDAP: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(1i32);
pub const NLM_DOMAIN_AUTHENTICATION_KIND_NONE: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(0i32);
pub const NLM_DOMAIN_AUTHENTICATION_KIND_TLS: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_DOMAIN_TYPE(pub i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(2i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(1i32);
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_ENUM_NETWORK(pub i32);
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(3i32);
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(1i32);
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_INTERNET_CONNECTIVITY(pub i32);
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(4i32);
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(2i32);
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(1i32);
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_NETWORK_CATEGORY(pub i32);
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(2i32);
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(1i32);
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_NETWORK_CLASS(pub i32);
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(2i32);
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLM_NETWORK_PROPERTY_CHANGE(pub i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(16i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(1i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(2i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(8i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(4i32);
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NLM_SIMULATED_PROFILE_INFO {
    pub ProfileName: [u16; 256],
    pub cost: NLM_CONNECTION_COST,
    pub UsageInMegabytes: u32,
    pub DataLimitInMegabytes: u32,
}
impl Default for NLM_SIMULATED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl Default for NLM_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
pub const NetworkListManager: windows_core::GUID = windows_core::GUID::from_u128(0xdcb00c01_570f_4a9b_8d69_199fdba5723b);
